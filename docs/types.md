# types

throughout this documentation, a specific notation is used to
represent the "type" of a value. this document explains it.

## basic types

| type | description                                   |
| ---- | --------------------------------------------- |
| uN   | an unsigned integer of N bits                 |
| iN   | a signed integer of N bits (two's compliment) |
| fN   | a floating-point value (ieee-754)             |
| str  | a string value (composed of characters)       |
| char | an ascii character                            |
| bits | a bitfield                                    |

## advanced typing

### str

a `str` can be restricted to N characters with the following notation

```
str[N]
```

### bits

a `bits` can be restricted in size to N bits with the following notation

```
bits[N]
```

## interpolated typing

in this documentation, c printf format specifiers are used.
