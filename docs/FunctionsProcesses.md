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
Using a function like this, however, is not incredibly useful apart from splitting up your code. We can give more functionality by adding *parameters*.
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
We're going to use the special `Var<T>` type to accept a variable that we can modify.
Let's implement its body while we're at it!
```rs
fn max(out: Var<Int>, a: Num, b: Num) {
    if (a > b) {
        out = a; // "a" is the correct type, okay!
    } else {
        out = b; // "b" is the correct type, okay!
    }
}
```
Now we have a working function! 
We instead call it like so: `max(var, 2, 3)`.

The reason this works is `Var<T>` is a special type. A variable (of type T) must be passed into it. However, instead of using the variable's value, the variable itself is passed, and can hence be set or modified.

### Selection Parameters
---
A Selection Parameter is used whenever a function might need to be able to run on various players/entities. They can be declared by prefixing the function with a `*`:
```rs
fn *startingGear() {
    // Implicitly use SELECTION::giveItems(...)
    giveItems(createItem("iron_sword")); 
}
```
That's really about it. If a function is not marked as needing a selection parameter, then that function cannot use `SELECTION`. This prevents the function from calling things such as giveItem or sendMessage unless using an explicit, known selector, such as `ALL`.

Something to note, is Selections are local to a function / process in Fire.
While they can still be manipulated like normal, a function has no information about the caller's selection unless the caller passes `SELECTION`. (In this case that could look like `SELECTION::startingGear()`)





## Processes
---
Processes are very different than functions in many ways.
1. A process cannot return values at all.
2. A process is its own thread and runs independently of others
3. A process can be subscribed to an event, while a function cannot.

Those are the main differences. A process can still accept both regular parameters and selection parameters.

They're defined with the `proc` keyword.
Here is an extremely simple example:
```rs
proc myProcess() {

}
```

One of the main uses of processes is for listening to events. 
To make a process run when an event is triggered, its only parameter must be a valid event, some events will require the process to accept a selection parameter, and others will forbid it.

Here is an example:
```rs
proc *joinEvent(e: JoinEvent) {

}
```
This process will be triggered whenever a player joins the plot.

### Special Events
Fire has some built-in events that DiamondFire does not for convienence.
These are:
1. `InitializeEvent` - Runs when the plot starts for the first time / after a varpurge. This event initialized all SAVEd variables.
2. `StartupEvent` - Runs when a player joins while no others are online, AKA when you plot starts up. This event initializes all DEFAULT variables.
3. `TickEvent` - Runs every tick.