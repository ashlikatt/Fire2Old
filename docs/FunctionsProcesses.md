# Functions & Processes
Functions and processes are the building blocks of any Fire program, so they must be understood.
Both functions and processes are used in certain situations, and are not to be used interchangeably, in general a function is almost always the correct choice.



## Functions
---
### Basics
---
Functions are quite simple to declare.
Here is a basic function that will do... something... when called.
```rs
fn myFunc() {
    // Do stuff
}
```
A function can be called like this: `myFunc()`, quite simple.

### Parameters
---
Using a function like before, however, is not incredibly useful outside of splitting up your code. We can give more functionality by adding *parameters*.
```rs
// Returns the larger of the two numbers
fn max(a: Num, b: Num) {
    // TODO Implement this...
}
```
This is stating "The function's name is max, and it requires two number parameters". Attempting to provide more or less parameters, or mismatched types will result in an error. (Note that Ints can be passed into this function because they will be auto-converted)

You would call such a function with something such as: `max(2, 3)`.

### Returning
---
Fire supports returning values from functions!

Let's return to our previous `max` function and give it output! 
We can give a type to the whole function to give it a return type.
```rs
fn max(a: Num, b: Num): Num {
    if (a > b) {
        return a; // "a" is the correct type, okay!
    } else {
        return b; // "b" is the correct type, okay!
    }
}
```
Now we have a working function! 
We instead call it like so: `max(2, 3)`. Or, more relevantly, `var = max(2, 3)`.





## Processes
---
Processes are very different than functions in many ways.
1. A process cannot return values at all.
2. A process is its own thread and runs independently of others
3. A process can be subscribed to an event, while a function cannot.

Those are the main differences. A process can still accept parameters.

They're defined with the `proc` keyword.
Here is an extremely simple example:
```rs
proc myProcess() {

}
```
You would call it like so `myProcess()`

One of the main uses of processes is for listening to events. 
To make a process run when an event is triggered, its only parameter must be a valid event.

Here is an example:
```rs
proc joinEvent(e: JoinEvent) {

}
```
This process will be triggered whenever a player joins the plot.
An event-subscribed process can still be called just like a normal process, given that you provide a valid JoinEvent input.
Additionally, you can have multiple of the same event in a program, this is important for certain libraries.
(As it stands, there is no way to assign a "priority" to event calls, however it shouldn't matter in well-written code)

### Special Events
Fire has some built-in events that DiamondFire does not for convienence.
These are:
1. `InitializeEvent` - Runs when the plot starts for the first time / after a varpurge. This event initialized all @Saved variables.
2. `StartupEvent` - Runs when a player joins while no others are online, AKA when you plot starts up. This event initializes all global variables.
3. `TickEvent` - Runs every tick.
