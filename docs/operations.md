# Operations
Fire contains a few operations, here they are:

## Math
---
`a + b` - Adds two Nums or Ints.
`a - b` - Subtracts two Nums or Ints.
`a * b` - Multiplies two Nums or Ints.
`a / b` - Divides two Nums or Ints. Will return `0` if invalid divide.
`a % b` - Modulo of two Nums or Ints.

## Strings
---
`a & b` - Concatenates two values (if they have toString implemented)

## Boolean
---
`!a` - NOT of a boolean
`a && b` - AND of two booleans
`a || b` - OR of two booleans

## Indexing
---
`a[b]` - Index b of a
`a[b] = c` - Set index b of a to c

## Misc
---
`a as B` - Forces a to be of type B for the type-checker. Doesn't affect compiled code. Inherently unsafe.
Ex:
```
let a: Any = 55;
let b = a as Int;
```
