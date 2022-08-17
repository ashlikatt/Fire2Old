# Annotations
Annotations are used to signal extra information to the compiler / IDE. They don't exist in compiled code.

An annotation can be added before a function, process, struct, enum, var declaration, etc. (Any sort of declaration)


## Safe Annotations
These can safely be used.

### @Save
---
Used before a top-level variable declaration to mark it as being a saved variable. 



## Dangerous Annotations
These can seriously break code if used incorrectly. Intended for internal use.


### @Event
---
Used before a struct definition to mark it as being an Event.

`@Event(type: String, name: String, args: Dictionary<String>)`
1. type - The type of event as it appears on the sign. ("PLAYER EVENT", "ENTITY EVENT", etc)
2. name - The name of the event as it appears on the sign. ("Join", "RightClick", etc)
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

`@Compile(blocks: List<std.compile.CodeBlock>)`
1. blocks - CodeBlocks to place instead of a regular call / definition.

Example stdlib:
```java
@Compile("SET VARIABLE","Average","","",v,n)
fn average(v: Var<Number>, n: Number...) {} // No code needed
```



### @ValueItem
---
Used before a struct to mark a type as being able to convert to a DF item, and define how to do so.

`@ValueItem()` TODO
