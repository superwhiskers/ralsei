//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::{FromPrimitive, ToPrimitive};
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{error, fmt};

/// A representation of a Nintendo Network error xml document
#[serde(rename = "errors", default)]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Errors {
    /// A vector of [`Error`] types
    ///
    /// [`Error`]: ./struct.Error.html
    #[serde(rename = "error", default)]
    pub errors: Vec<Error>,
}

impl Errors {
    /// Returns the first [`Error`] or `None` if there are none
    ///
    /// [`Error`]: ./struct.Error.html
    pub fn first(&self) -> Option<&Error> {
        self.errors.first()
    }

    /// Returns the first [`Error`]'s [`ErrorCode`] or `None` if there are none
    ///
    /// [`Error`]: ./struct.Error.html
    /// [`ErrorCode`]: ./struct.ErrorCode.html
    pub fn first_code(&self) -> Option<&ErrorCode> {
        self.errors.first().map(|v| &v.code)
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // here, we assume that there is only one error present. this presumption has held true in
        // all known cases, so we believe that there is no need to handle the edge case of there
        // being multiple
        if let Some(error) = self.errors.get(0) {
            error.fmt(formatter)
        } else {
            write!(formatter, "No error has arised")
        }
    }
}

impl error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // ditto
        self.errors
            .get(0)
            .map::<&(dyn error::Error + 'static), _>(|v| v)
    }
}

/// A Nintendo Network account server error
#[serde(rename = "error", default)]
#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Error {
    /// The cause of the error
    pub cause: Option<String>,

    /// The error code. Appears to always be represented as four digits right-aligned
    pub code: ErrorCode,

    /// The error message
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Error code `{:0>4}` arised with message `{}` and cause `{}`",
            self.code.to_u16().unwrap_or(0),
            self.message,
            self.cause.as_ref().unwrap_or(&"no cause".to_string())
        )
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.code)
    }
}

/// A Visitor for u16 values
struct U16Visitor;

impl<'de> Visitor<'de> for U16Visitor {
    type Value = u16;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an integer between 0 and 2^16-1")
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }
}

/// A container for a Nintendo Network account server error code, handling unknown values as well
/// as known ones
#[derive(thiserror::Error, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ErrorCode {
    #[error("{0}")]
    Known(#[from] ErrorCodeValue),

    #[error("An unknown error has occurred ({0})")]
    Unknown(u16),
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl FromPrimitive for ErrorCode {
    fn from_i64(n: i64) -> Option<Self> {
        Some(match ErrorCodeValue::from_u16(n as u16) {
            Some(known_code) => Self::Known(known_code),
            None => Self::Unknown(n as u16),
        })
    }

    fn from_i16(n: i16) -> Option<Self> {
        Some(match ErrorCodeValue::from_u16(n as u16) {
            Some(known_code) => Self::Known(known_code),
            None => Self::Unknown(n as u16),
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(match ErrorCodeValue::from_u16(n as u16) {
            Some(known_code) => Self::Known(known_code),
            None => Self::Unknown(n as u16),
        })
    }

    fn from_u16(n: u16) -> Option<Self> {
        Some(match ErrorCodeValue::from_u16(n) {
            Some(known_code) => Self::Known(known_code),
            None => Self::Unknown(n),
        })
    }
}

impl ToPrimitive for ErrorCode {
    fn to_i64(&self) -> Option<i64> {
        Some(match self {
            Self::Known(code) => code.to_i64().unwrap_or(0),
            Self::Unknown(code) => i64::from(*code),
        })
    }

    fn to_i16(&self) -> Option<i16> {
        Some(match self {
            Self::Known(code) => code.to_i16().unwrap_or(0),
            Self::Unknown(code) => *code as i16, // lossy
        })
    }

    fn to_u64(&self) -> Option<u64> {
        Some(match self {
            Self::Known(code) => code.to_u64().unwrap_or(0),
            Self::Unknown(code) => u64::from(*code),
        })
    }

    fn to_u16(&self) -> Option<u16> {
        Some(match self {
            Self::Known(code) => code.to_u16().unwrap_or(0),
            Self::Unknown(code) => *code,
        })
    }
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_u16(self.to_u16().unwrap_or(0))?)
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_u16(deserializer.deserialize_u16(U16Visitor)?).unwrap_or(Self::Unknown(0)))
    }
}

/// A Nintendo Network account server error code
#[non_exhaustive]
#[derive(
    thiserror::Error, FromPrimitive, ToPrimitive, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum ErrorCodeValue {
    #[error("The parameters provided are formatted incorrectly")]
    BadParameterFormat = 1,

    #[error("This request is formatted incorrectly")]
    BadRequestFormat = 2,

    #[error("This request is missing a parameter")]
    MissingRequestParameter = 3,

    #[error("This client is unauthorized")]
    UnauthorizedClient = 4,

    #[error("The account token provided in this request is invalid")]
    InvalidAccountToken = 5,

    #[error("The account token is expired")]
    ExpiredAccountToken = 6,

    #[error("This request is forbidden")]
    ForbiddenRequest = 7,

    #[error("This request points to a nonexistant endpoint")]
    RequestNotFound = 8,

    #[error("This request uses the wrong HTTP method")]
    WrongHTTPMethod = 9,

    #[error("The platform id provided in the request is invalid")]
    InvalidPlatformId = 10,

    #[error("A system update is required to access this service")]
    SystemUpdateRequired = 11,

    #[error("The device in use has been banned from all services")]
    BannedDevice = 12,

    #[error("The account id provided in this request already exists")]
    AccountIdExists = 100,

    #[error("The account id provided in this request is invalid")]
    InvalidAccountId = 101,

    #[error("The email provided in this request is invalid")]
    InvalidMailAddress = 103,

    #[error("This device is unauthorized")]
    UnauthorizedDevice = 104,

    #[error("This device is unable to register any more accounts")]
    RegistrationLimitReached = 105,

    #[error("The account id and/or password is incorrect")]
    WrongAccountPassword = 106,

    #[error("The account country and device country do not match")]
    CountryMismatch = 107,

    #[error("The account in use has been banned from all services")]
    BannedAccount = 108,

    #[error("This device is unlinked to the provided account")]
    DeviceMismatch = 110,

    #[error("This account id has changed")]
    AccountIdChanged = 111,

    #[error("This account has been deleted")]
    AccountDeleted = 112,

    // UnauthorizedDevice = 113, -- according to kinnay's docs, this one also means "unauthorized
    // device" -- yet i already have a variant dedicated to that under code 0104 ??
    #[error("The COPPA agreement has not been accepted")]
    CoppaNotAccepted = 114,

    #[error("This device has reached its account association limit")]
    AssociationLimitReached = 115,

    #[error("The confirmation code provided is incorrect")]
    WrongConfirmationCode = 116,

    #[error("The confirmation code provided has expired")]
    ExpiredConfirmationCode = 117,

    #[error("The game server id and unique id are not linked")]
    GameServerIdUniqueIdNotLinked = 118,

    #[error("The account in use has been banned from this application")]
    BannedAccountInApplication = 119,

    #[error("The device in use has been banned from this application")]
    BannedDeviceInApplication = 120,

    #[error("The account in use has been banned from this NEX service")]
    BannedAccountInNexService = 121,

    #[error("The device in use has been banned from this NEX service")]
    BannedDeviceInNexService = 122,

    #[error("This service has closed its operations")]
    ServiceClosed = 123,

    #[error("An update of your application is required to use this service")]
    ApplicationUpdateRequired = 124,

    #[error("The client and unique id are not linked")] // unsure ??
    ClientUniqueIdNotLinked = 125,

    #[error("The account in use has been banned from this independent service")]
    BannedAccountInIndependentService = 126,

    #[error("The device in use has been banned from this independent service")]
    BannedDeviceInIndependentService = 127,

    #[error("The email in use has not been validated")]
    MailAddressNotValidated = 128,

    #[error("The birthdate or email address is wrong")] // unsure ??
    WrongBirthdateOrMailAddress = 129,

    #[error("The requested PID was not found")]
    PidNotFound = 130,

    #[error("The email address is incorrect")] // unsure ??
    WrongAccountMail = 131,

    #[error("The account in use has been temporarily banned from all services")]
    TempbannedAccount = 132,

    // TempbannedDevice = 0133, ??
    #[error("The account in use has been temporarily banned from this application")]
    TempbannedAccountInApplication = 134,

    // TempbannedDeviceInApplication = 0135, ??
    #[error("The account in use has been temporarily banned from this NEX service")]
    TempbannedAccountInNexService = 136,

    #[error("The device in use has been temporarily banned from this NEX service")]
    TempbannedDeviceInNexService = 137,

    #[error("The account in use has been temporarily banned from this independent service")]
    TempbannedAccountInIndependentService = 138,

    #[error("The device in use has been temporarily banned from this independent service")]
    TempbannedDeviceInIndependentService = 139,

    #[error("The COPPA agreement has been cancelled")] // unsure ??
    CoppaAgreementCancelled = 142,

    #[error("This device is inactive")] // unsure ??
    DeviceInactive = 143,

    #[error("The EULA has not been accepted")]
    EulaNotAccepted = 1004,

    #[error("The provided unique id is invalid")]
    InvalidUniqueId = 1006,

    #[error("The requested NEX account was not found")]
    NexAccountNotFound = 1016,

    #[error("The requested game environment was not found for this game server id")]
    GameServerIdEnvironmentNotFound = 1017,

    #[error("The server was unable to generate a token")]
    TokenGenerationFailed = 1018,

    #[error("The provided NEX client id is invalid")]
    InvalidNexClientId = 1019,

    #[error("The provided client key is invalid")]
    InvalidClientKey = 1020,

    #[error("The requested game server id is invalid")]
    InvalidGameServerId = 1021,

    #[error("The requested client id is invalid/not found")]
    InvalidClientId = 1022,

    #[error("The provided email address is incorrect")]
    WrongMailAddress = 1023,

    #[error("...")] // unsure ??
    MasterPinNotFound = 1024,

    #[error("...")] // unsure ??
    MailTextNotFound = 1025,

    #[error("The server was unable to send an email to the provided address")]
    SendMailFailure = 1031,

    #[error("...")] // unsure ??
    DomainAccountAlreadyExists = 1032,

    #[error("Too many `forgot password` attempts have been made")]
    ExcessiveMailSendRequest = 1033,

    #[error("A general error has occurred while doing a credit card operation")]
    CreditCardGeneralFailure = 1035,

    #[error("The date has expired on the provided credit card")]
    CreditCardDateExpired = 1036,

    #[error("The credit card has been declined")]
    CreditCardDeclined = 1037,

    #[error("The provided credit card number is invalid")] // / these two seem redundant?
    InvalidCreditCardNumber = 1038, // |
    // |
    #[error("The provided credit card number is incorrect")] // /
    WrongCreditCardNumber = 1039,

    #[error("The provided credit card date is invalid")]
    InvalidCreditCardDate = 1040,

    #[error("The provided credit card has been blacklisted")]
    CreditCardBlacklisted = 1041,

    #[error("The provided credit card's pin is invalid")] // / ditto
    InvalidCreditCardPin = 1042, // |
    // |
    #[error("The provided credit card's pin is incorrect")] // /
    WrongCreditCardPin = 1043,

    #[error("The provided location is invalid")]
    InvalidLocation = 1044,

    #[error("The provided postal code is invalid")]
    InvalidPostalCode = 1045,

    #[error("The device country and EULA country do not match")]
    DeviceEulaCountryMismatch = 1046,

    #[error("The requested EULA country is invalid")]
    InvalidEulaCountry = 1100,

    #[error("The requested EULA country and version pair are invalid")]
    InvalidEulaCountryAndVersion = 1101,

    #[error("The endpoint you are requesting requires parental controls")]
    ParentalControlsRequired = 1103,

    #[error("The provided account id is invalid")]
    AccountIdFormatInvalid = 1104,

    #[error("The provided email and/or password is incorrect")]
    WrongAccountPasswordOrMailAddress = 1105,

    #[error("Authentication attempts for this service are locked")] // unsure ??
    AuthenticationLocked = 1106,

    #[error("The provided account id and password are the same")]
    AccountIdPasswordSame = 1107,

    #[error("The requested approval id was not found")] // unsure ??
    ApprovalIdNotFound = 1111,

    #[error("The account you requested is pending migration")] // unsure ??
    PendingMigration = 1115,

    #[error("The provided email address' domain name is invalid")]
    InvalidMailAddressDomainName = 1125,

    #[error("The provided email address' domain name is unresolvable")]
    UnresolvableMailAddressDomainName = 1126,

    #[error("No country was provided in the request")]
    UnprovidedCountry = 1200,

    #[error("Unable to process request")]
    BadRequestError = 1600,

    #[error("An internal server error occurred")]
    InternalServerError = 2001,

    #[error("The servers are under maintenance")]
    UnderMaintenance = 2002,

    #[error("The Nintendo Network service has ended")]
    NintendoNetworkClosed = 2999,
}
