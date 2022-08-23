# Annotations
Annotations are used to signal extra information to the compiler / IDE. They don't exist in compiled code.

An annotation can be added before a function, process, struct, enum, var declaration, etc. (Any sort of declaration)


## Safe Annotations
These can safely be used.

### @Save
---
Used before a top-level variable declaration to mark it as being a saved variable. 

### @Const
---
Used before a parameter to a function or process to specify that only consistent values may be passed.
```java
fn myFun(@Const a: String);
...
myFun("Cool!") // Valid
let a = "Cool!";
myFun(a) // Invalid
```

### @Pure
---
Mark a function as being "pure". A pure function gets no data from outside itself.
Example:
```java
@Pure
fn safeDivide(a: Num, b: Num): Num {
    if (b==0) raise "DivByZero";
    return a/b;
}
```
This function is pure because it only uses its parameters.
The compiler will throw an error if a non-pure function is marked with @Pure, and likewise will automatically mark valid functions with @Pure during compile-time.
(This annotation is used by the compiler for optimizing code)


## Dangerous Annotations
These can seriously break code if used incorrectly. Intended for internal use.


### @Event
---
Used before a struct definition to mark it as being an Event.

`@Event(block: std::compile::CodeBlock, args: Dict<Any>)`
1. block - The event codeblock.
3. args - Dict of Game Values to build the struct when the event is called. ("Event Block Side", "Event Command", etc. Use `/i tag list` on a Game Value for more information). All struct values must be supplied.

Example stdlib:
```java
@Event("PLAYER EVENT", "Command", {command: "Event Command", arguments: "Event Command Arguments"})
struct CommandEvent {
    command: String,
    arguments: List<String>
}
```



### @Compile
---
Used before a function or process definition to override how it is compiled into DiamondFire.

`@Compile(blocks: List<std::compile::CodeBlock>)`
1. blocks - CodeBlocks to place instead of a regular call / definition.

Example stdlib:
```java
@Compile("SET VARIABLE","Average","","",v,n)
fn average(v: Var<Number>, n: Number...) {} // No code needed
```



### @ValueItem
---
Used before a struct to mark a type as being able to convert to a raw DF item (Strings, numbers, etc), however the process for the conversion is hard-coded and handled internally. 

Example:
```
@ValueItem
struct Loc {
    x: Num,
    y: Num,
    z: Num,
    pitch: Num,
    yaw: Num
}
```
