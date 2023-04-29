# Syntax Spec
This defines the syntax of the Blackstone programming language.

# Example 1
This example represents a player event left click block.
it is a LeftClick PlayerEvent block followed by a PlayerAction SendMessage block with a text of "Block Location:" inside followed by a game value of EventBlockLocation. Check underground for example.

> Cake
```ts
plot.playerEvent.leftClick(event: PlayerLeftClickEvent) {
    selector player = event.player; 
    // we like this more
    player as event.player;
    player.sendMessage(
        "Block Location:" + GameValue(EventTag.BlockLocation) 
        textValueMerging=SendMessageTag.NoSpaces
    );
}

plot.playerEvent.rightClick(event) { // Type anotations are optional
    var local.s = 5;
    // This also works
    var s = 5;
}

fn funnyFunction(player) { // PROBLEM (FUTURE ME PROBLEM)
    var list = ["a", "b", "c"];
    for (it in list) {
        player.sendMessage("I like " + it)
    }
    while ()

    loop {

    }
}



```

> Endi
```rs
// comment
PlayerEvent(LeftClick) {
    player.SendMessage("Block Location: ", GameValue(EventBlockLocation))
}

PlayerEvent(RightClick) {
    var s = 5; // prefix with `game.` or `save.` to change variable scope
}

Function FunnyFunction() {
    var list = createList(
        "a",
        "b",
        "c"
    );
    Repeat ForEach(it, list) {
        default.sendMessage("I like ", it);
    }
}
```

> Camila (Current Working) 
```rs
PlayerEvent (Join) {
    default as event.default;
    default.SendMessage(
        "Block Location: ", 
        GameValue.EventBlockLocation(Default) |
        "No Spaces", "Regular"
    );
    // also acceptable:
    default.SendMessage("Block Location: ", GameValue.EventBlockLocation(Default) | "No Spaces", "Regular");
}

PlayerEvent (RightClick) {
    var local.s = 5;
}

func funnyFunction {
    var local.list = ["a", "b", "c"];
    repeat (ForEach(local.list, it)) {
        default.SendMessage(
            it |
            "Add Spaces", "Centered"
        )
    }
}
```

> Attempted Merge
```ts

playerEvent(Join) {
    default.sendMessage("hi");
}

func funnyFunction {
    var local.list = ["a", "b", "c"];
}
```


# Select Object?
```ts
selection my_selection as select {
     AllPlayers().FilterCondition::IsLookingAt() 
    };
selection player as select {default};
```