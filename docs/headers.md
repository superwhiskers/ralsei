# headers

across the various servers, one can find a variety of headers being
sent in http requests. because of their potentially incriminating
value, this document serves as documentation of their construction
and purpose

## account client (`*.account.nintendo.net`/`account.nintendo.net`)

### common

| field                            | type    | description                                                                                                                                            |
| -------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `X-Nintendo-Platform-ID`         | u1      | 1 on wiiu, 0 on 3ds                                                                                                                                    |
| `X-Nintendo-Device-Type`         | u2      | 1 on a developer console, 2 on a retail console                                                                                                        |
| `X-Nintendo-Device-ID`           | u64     | the id of the `X-Nintendo-Device-Cert` in decimal                                                                                                      |
| `X-Nintendo-Serial-Number`       | str     | the console's serial number, minus the check digit                                                                                                     |
| `X-Nintendo-System-Version`      | str[4]  | [the version number of a title](#system-version)                                                                                                       |
| `X-Nintendo-Region`              | bits[7] | [a bitfield detailing the region](#region-information)                                                                                                 |
| `X-Nintendo-Country`             | str[2]  | any valid iso 3166-1 alpha-2 country code                                                                                                              |
| `X-Nintendo-Client-ID`           | str[32] | [a hexadecimal string](#client-id-and-secret)                                                                                                          |
| `X-Nintendo-Client-Secret`       | str[32] | ^                                                                                                                                                      |
| `X-Nintendo-FPD-Version`         | u16     | four zeroes. no differences have been documented                                                                                                       |
| `X-Nintendo-Environment`         | str[2]  | `Lx`/`Dx`/`Sx`/`Tx`/`Jx`, `L1` by default                                                                                                              |
| `X-Nintendo-Title-ID`            | str[16] | the whole id of the title                                                                                                                              |
| `X-Nintendo-Unique-ID`           | str[5]  | the unique id section of the title id left-padded to 5 digits if less (all o3ds titles have 5 max, n3ds exclusives have 2 in the sixth leftmost digit) |
| `X-Nintendo-Application-Version` | str[4]  | the major version of the title in hexadecimal                                                                                                          |
| `X-Nintendo-Device-Cert`         | str     | [the device certificate encoded in base64](#device-cert)                                                                                               |

#### unsure

these following headers are ones that are not known to
be provided by all consoles, and as such, they go here.
none of them are not important to emulate a console,
but they may be useful for extremely accurate mimicking
of one to avoid being banned

| field             | type   | description                                              |
| ----------------- | ------ | -------------------------------------------------------- |
| `Accept-Language` | str[2] | a valid ISO 639-1 language code                          |
| `Accept`          | str    | a valid content-type to accept. the 3ds sets it to `*/*` |

#### exceptions

whereas the `User-Agent` header is sometimes provided in
requests to other servers, it is not in requests to the
account server (at least on the 3ds)

#### region information

the `X-Nintendo-Region` field has a specific layout of its
corresponding bitfield type

| bit | region |
| --- | ------ |
| 0   | JPN    |
| 1   | USA    |
| 2   | EUR    |
| 3   | AUS    |
| 4   | CHN    |
| 5   | KOR    |
| 6   | TWN    |

footnote: `AUS` is not a game region, it instead accepts
games from the `EUR` region

#### system version

the `X-Nintendo-System-Version` header gets its information
from a specific title, which varies across regions and
consoles

the following table contains the title id of each title
whose version is used as the information for
`X-Nintendo-System-Version`

| region | console | title id         |
| ------ | ------- | ---------------- |
| JPN    | 3ds     | 000400DB00016202 |
|        | n3ds    | 000400DB20016202 |
|        | wiiu    | 0005001010041000 |
| USA    | 3ds     | 000400DB00016302 |
|        | n3ds    | 000400DB20016302 |
|        | wiiu    | 0005001010041100 |
| EUR    | 3ds     | 000400DB00016102 |
|        | n3ds    | 000400DB20016102 |
|        | wiiu    | 0005001010041200 |
| CHN    | 3ds     | 000400DB00016402 |
| KOR    | 3ds     | 000400DB00016502 |
|        | n3ds    | 000400DB20016502 |
| TWN    | 3ds     | 000400DB00016602 |

on 3ds, the title is referred to as `nver`, whereas on the
wiiu, it is referred to as `version.bin`

#### client id and secret

the `X-Nintendo-Client-ID` and `X-Nintendo-Client-Secret`
headers are two headers provided by both the 3ds and wii u
to the account server (and so far, it appears to only be
the account server). contrary to what they are named, they
do not appear to be console-specific (device granularity)
and instead differ on more general boundaries

here are some known pairs

| console | id                               | secret                           |
| ------- | -------------------------------- | -------------------------------- |
| wiiu    | a2efa818a34fa16b8afbc8a74eba3eda | c91cdb5658bd4954ade78533a339cf9a |
| 3ds     | daf6227853bcbdce3d75baee8332b    | 3eff548eac636e2bf45bb7b375e7b6b0 |
| 3ds     | ea25c66c26b403376b4c5ed94ab9cdea | d137be62cb6a2b831cad8c013b92fb55 |

they appear to be oauth client id/secrets, and also appear
to have the property of region and console (family)
specificity. aside from that, there isn't much else known
about them

#### device cert

the `X-Nintendo-Device-Cert` header contains a console
unique certificate that is used for signing titles. on
the 3ds, it is sent in all requests made to the account
server, whereas on the wii u, it is only sent in some

it consists of 384 base64-encoded bytes. the layout of
them is as follows:

| offset | size | description                   |
| ------ | ---- | ----------------------------- |
| 0x000  | 0x04 | the signature type (0x010005) |
| 0x004  | 0x7c | the signature (+ padding)     |
| 0x080  | 0x40 | certificate issuer id         |
| 0x0c0  | 0x04 | key type                      |
| 0x0c4  | 0x40 | certificate name              |
| 0x104  | 0x04 | key id                        |
| 0x108  | 0x78 | ecdsa public key (+ padding)  |

the signature type follows this format. in the certificate,
it is prepended with a null byte (`0x00`)

| console | console type | signature type | description       |
| ------- | ------------ | -------------- | ----------------- |
| wiiu    | retail       | 0x010005       | ecdsa with sha256 |
|         | debug        | 0x010002       | ecdsa with sha1   |
| 3ds     | retail       | 0x010005       | ecdsa with sha256 |

the issuer id is one of the following, depending on the
console

| console | type                | issuer id                           |
| ------- | ------------------- | ----------------------------------- |
| wiiu    | (presumably) retail | `Root-CA<%08X>-MS<%08X>`            |
| 3ds     | retail              | `Nintendo CA - G3_NintendoCTR2prod` |
|         | developer           | `Nintendo CA - G3_NintendoCTR2dev`  |

the certificate name section is one of the following

| console | certificate name format |
| ------- | ----------------------- |
| wiiu    | `NG<%08X>`              |
| 3ds     | `CT<%08X>-<%02X>`       |

in the 3ds format, the first parameter is your device id,
and the second is either `00` for a retail certificate or
`01` for a development one. in the wii u format, the only
parameter is the device id

more detailed information regarding the device
certificate, and more generally, the certificate format
of the 3ds (and, given how well they match up, the wii u)
can be found [here](https://www.3dbrew.org/wiki/CTCert)
and [here](https://www.3dbrew.org/wiki/Certificates),
respectively

kinnay's documentation of the certificate format can be
found
[here](https://github.com/Kinnay/NintendoClients/wiki/Account-Server#device-certificate)
which is what led me to merge the two into a single
section

### 3ds

| field                     | type   | description                                                    |
| ------------------------- | ------ | -------------------------------------------------------------- |
| `X-Nintendo-API-Version`  | u16    | the account server api version the client is using (always v1) |
| `X-Nintendo-Device-Model` | str[3] | [the device codename string](#device-model)                    |

footnote:

- unlike on the wiiu, there do not appear to be
  any headers that are not provided on all
  endpoints, with the exception of the
  `Authorization` header provided on necessary
  endpoints
- on some endpoints, there are two
  `X-Nintendo-Device-Cert` headers provided
  that are exactly the same, instead of one.
  whether this is 3ds-specific remains to be
  seen. this is speculated to be the result
  of nintendo adding the certificate to all
  endpoints, duplicating it on the ones that
  it was already provided to

#### device model

the device model string corresponds to the device codename
string, as returned by `CFG:GetSystemModel`

| model  | codename string |
| ------ | --------------- |
| 3ds    | CTR             |
| 3dsxl  | SPR             |
| n3ds   | KTR             |
| 2ds    | FTR             |
| n3dsxl | RED             |
| n2dsxl | JAN             |

### wiiu

there are no known headers that only the wii u provides to
the account server

