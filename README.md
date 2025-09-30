## Things to do next
- correct coord offset for dominos being placed (anchor tile should always be beneath hand/cursor)
- fix the turn order such that placement happens before the second round of tile selection

Turn goes: Place1, Pick1, Place2, Pick2, Place3, Pick3, Place4, Pick4
Not: Pick1, Pick2, Pick3, Pick4, Place1, Place2, Place3, Place4

- work on logic that centers the domino_map in the box each time
- Check logic for the socket layout, its located in grid.rs and is a total mess, maybe its working already, but havent checked it out
- sockets should overlay correctly on the active player's colored box zone. Also needs to be centered to overlay on the domino_map correctly
- make logic for score calculation. will be important in the near future.
- Redo the layout of gui to give draft zone more room. Also update the Const in gui.rs to adjust for the new domino size, if that will be enlarged
- I have another todo list in player.rs, with some stuff I forgot to mention here
- TODOs are also scattered throughout the codebase, I haven't bothered to track them all down though.
- ask if you have any questions!