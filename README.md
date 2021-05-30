# Matryoshka tic-tac-toe

![Matryoshka tic-tac-toe in real life](media/matryoshka-tic-tac-toe.gif)

![Wonder if this is still a solved game](media/ed.png)

## No.

The computational requirements explode after 5 pieces.

The 6 pieces of the video are unsolved.

## But yes.
Through exhaustive searching with an efficient min-max algorithm, all game
states for 3, 4 and 5 piece per player games have been checked.

![Example game](media/example-game.png)

### src/game/main.rs -> target/release/game

A 2-player, local game of matryoshka tic-tac-toe

### src/solver/main.rs -> target/release/solver

A 1-player game of matryoshka tic-tac-toe, with fewer pieces, where the CPU chooses the best move.
