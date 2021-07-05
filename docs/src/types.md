# types

throughout the documentation, there is a specific notation used to denote the "type" of a value.
while it may be immediately familiar to some readers, this section explains exactly what each
"type" means

## basic types

| type     | description                                            |
| -------- | ------------------------------------------------------ |
| uN       | an unsigned integer of N bits                          |
| ux       | an unsigned integer with an unspecified number of bits |
| iN       | a signed integer of N bits (two's compliment)          |
| ix       | a signed integer with an unspecified number of bits    |
| fN       | a floating-point value (ieee-754)                      |
| str      | a string value (composed of characters)                |
| char     | an ascii character                                     |
| bits     | a bitfield                                             |
| array[T] | a homogenous sequence of instances of type T           |

## advanced typing

### str

a `str` can be restricted to `N` characters with the following notation

```
str[N]
```

### bits

a `bits` can be restricted in size to `N` bits with the following notation

```
bits[N]
```

### arrays

an `array[T]` can be restricted in length to `N` instances of `T` with the following notation

```
array[T][N]
```

## string interpolation

for string interpolation, refer to the documentation for the `printf()` c function to understand
what each format specifier means
