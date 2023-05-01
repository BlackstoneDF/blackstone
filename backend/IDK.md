

```rs

PlayerEvent(event: LeftClickEvent) {
    default as event.default; // semicolons :)
    default.sendMessage(
        "Block Location: ", GameValue.EventBlockLocation(/* There are no params for event tags*/) 
        | SendMessageSpacing.NoSpaces, SendMessageAlignment.Regular // Eh naming for the enums
    ); 
}

PlayerEvent(event: RightClickEvent) { // Type anotations are optional
    var local.s = 5;
    // This also works
    var s = 5;
}
// fn is good >:(
fn funnyFunction(event: any) { // implicitly passed in and _ means that it is going to be disgarded
    player as event.default;
    var list = ["a", "b", "c"];

    Repeat ForEach(it, list) {
        player.sendMessage("I like ", it);
    }

    //Another proposal (No hard coding required)

    forEach(list, it) {
        player.sendMessage("I like ", it);
    }
}
```

```ts
func forEach(list: List, it: any, clusure) {
    //sus Struff that is going to be executed
}
```