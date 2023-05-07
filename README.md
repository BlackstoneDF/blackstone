# Blackstone

Blackstone is a programming language designed to help create plots on the MCDiamondFire Minecraft server.

## Community Links

[Discord](https://discord.gg/c7qzkNAURV)
[DF Website](https://mcdiamondfire.com)
For ease of use with Blackstone, we recommend downloading the [Recode Mod](https://github.com/homchom/recode).

## Installation

### Building from Source

You can install the compiler using the Cargo toolchain.

```text
cargo install shulker
```
It will automatically handle building from source for you.

### Releases Page

You can also install it through our `Releases` page on the sidebar. During official releases, we will compile binaries for Windows & Linux.

## Features

### Code Blocks

- [x] Player Event
- [x] Player Action
- [x] If Player
- [x] Entity Event
- [x] Entity Action
- [x] If Entity
- [x] Game Action
- [x] If Game
- [x] Set Variable
- [x] If Variable
- [x] Repeat
- [x] Function
- [x] Call Function
- [x] Process
- [x] Start Process
- [x] Else
- [x] Select Object
- [ ] NOT Functionality

### Values

- [x] Items (Vanilla, not customizable)
- [x] Text
- [x] Number
- [x] Variable
- [x] Location
- [ ] Sound
- [ ] Vector
- [ ] Sound
- [ ] Particle
- [ ] Potion
- [ ] Game Value

### Bonus / Planned Features

- [ ] Function Parameters
- [ ] Object-Oriented Programming
- [ ] Compiler-Enforced Strong Typing
- [ ] Tests
- [ ] Customizable Items

Note that none of the bonus features are *guaranteed* to be implemented.
We do think they would be useful, though.

### Shulker / Compiler Commands

- [x] version
- [ ] init
- [x] build (script)
- [x] build-all
- [x] build-stdout (script)
- [ ] build_test
- [ ] add (package)

### DFS Suggestions

None yet! Let us know if there's some cool ones you'd like to see in our Discord!

## Special Thanks

`todo!()`

## Documentation

All code must be inside an Event, Function, or Procedure. Here's some basic examples:

```rs
playerEvent(Join) {
    //supports comments too!
    default.sendMessage("Hello world!");
}

func FunnyFunction {
    // variables must be manually scoped - local, game, or save
    var game.joke = "Among us";
    // defaults to local
    var list = ["a", "b", "c"];
    // default.sendMessage(...) == print(...)
    default.sendMessage(joke);
}
```
