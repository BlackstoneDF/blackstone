# Blackstone & Shulker

Blackstone is a programming language designed to help create plots on the MCDiamondFire Minecraft server. Shulker is a build tool that allows you to use the Blackstone programming language.

Shulker also allows you to distribute your code publicly - whether you just want your game to be open-source or you want to make a library others can use.

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
- [x] Entity Event
- [x] Player Action
- [x] Entity Action
- [x] Game Action
- [x] If Player
- [x] If Entity
- [x] If Game
- [x] Set Variable
- [x] If Variable
- [ ] Repeat
- [x] Function
- [x] Call Function
- [x] Process
- [x] Start Process
- [ ] Else
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

# Build Utilities
Shulker will allow you to define your game & library in the `Shulker.toml` file.

## Shulker.toml
For example, a `Shulker.toml` for a game would be:
```yaml
[game]
name = "Islands 2"
id = 51025
```
And for a library, an example one would be:
```yaml
[library]
name = "Entity Utilities"
id = "entity_utilities"
description = "This is a library used to make manipulating entities just better."
creator = "Endistic"
```

## Publishing & Creating Libraries
`todo!()`

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
