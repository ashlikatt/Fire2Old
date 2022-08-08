# Annotations
Annotations are used to signal information to the compiler. They are intended for internal use and are not recommended to be used outside of the standard library.

An annotation can be added before a function, process, struct, etc. (Any sort of declaration)



## @Event
---
Used before a struct definition to mark it as being an Event.

`@Event(type: String, name: String, args: String...)`
1. type - The type of event as it appears on the sign. ("PLAYER EVENT", "ENTITY EVENT", etc)
2. name - The name of the event as it appears on the sign. ("Join", "RightClick", etc)
3. args - List of Game Values to be supplied to the constructor when the event is called. ("Event Block Side", "Event Command", etc. `/i nbt` on a Game Value for more information)

Example stdlib:
```java
@Event("PLAYER EVENT", "Command", "Event Command", "Event Command Arguments")
struct CommandEvent {
    command: String,
    arguments: List<String>
}
```



## @Compile
---
Used before a function or process definition to override how it is compiled into DiamondFire.

`@Compile(a: String, b: String, c: String, d: String, chest: Any...)`
1. a - Top line of sign, this determines block type.
2. b - Second line of sign.
3. c - Third line of sign.
4. d - Fourth line of sign.
5. chest - Values to put in the chest. Can parse Strings, numbers, variables, etc

Example stdlib:
```java
@Compile("SET VARIABLE","Average","","",v,n)
fn average(v: Var<Number>, n: Number...) {} // No code needed
```