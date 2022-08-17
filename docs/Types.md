# Data Types
Fire provides several different data types to use in your code, and ways of creating your own. 

## Literals
### Number
---
Numbers are the simplest possible type.
They can be created just by writing down something like `1.0`.
Numbers can have underscores anywhere inside of them for readability. `1_000_000.0` is valid!

#### Int
Integers are a subset of Numbers. They're easily created by writing something like `1` or `8`.
Integers will be automatically converted to Numbers if needed, but not the other way around.


### String
--- 
Strings are extremely basic, they can be created `"by typing something like this"` or `'like this'`.
Strings can be escaped using `\`. This can be used to achieve newlines and quotes inside of the text.

Percent codes will be accepted by diamondfire within strings, however they're heavily discouraged because they can affect the integrity and safety of the typing and selection system. (This goes for any % code)


### Booleans
---
Booleans are extremely simple, they're either true or false, and are created by writing `true` or `false`.


### Lists
---
A list can be written like this: `["a", "b", 5]`.
Creating a list this way will give it the `List<T>` type if all values are T, and `List<Any>` otherwise. 
`List<Int>(1, 2, 3, 4)` is another valid way to create a list. (In fact, the [] syntax just calls `List<T>(...)`)

### Dictionaries
---
A dictionary can be written like this: `{ name: "AshlieUwU", gender: "F" }`
Creating a dictionary this way will give it the `Dictionary<T>` type if all values are T, and `Dictionary<Any>` otherwise.
`Dictionary<String>(["name", "gender"], ["AshlieUwU", "F"])` is another valid way to create a dictionary.
(In fact, the {} syntax just calls `Dictionary<T>(...)`)

## Custom Types
### Enums
---
Enums are the simplest custom type, and are used if you want a choice for something (In the standard library, they're used to represent certain block parameters)
They can be defined like this:
```
enum Gender: String {
  MALE = "M",
  FEMALE = "F",
  OTHER = "O"
}
```
This specifies an enum "Gender", that masks the type "String" (That is, they're really just a String with special names).
And then there are three types specified.

If no value is specifically needed, a shorcut can be used:
```
enum Gender {
  MALE, 
  FEMALE, 
  OTHER
}
```
When compiled, this is used:
```
enum Gender: Int {
  MALE = 0,
  FEMALE = 1,
  OTHER = 2
}
```


### Structs
A struct is a way of grouping data together, and are crucial to good programs.
Here is a simple example:
```
struct Vector {
  x: Num,
  y: Num,
  z: Num
}
```
A struct is a glorified, and type-safe dictionary.
