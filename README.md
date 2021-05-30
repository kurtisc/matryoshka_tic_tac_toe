# Matryoshka tic-tac-toe

![Matryoshka tic-tac-toe in real life](media/matryoshka-tic-tac-toe.gif)

![Wonder if this is still a solved game](media/ed.png)

~~No.~~

~~The computational requirements explode after 5 pieces.~~

~~The 6 pieces of the video are unsolved.~~

## Yes.
Through exhaustive searching with an efficient min-max algorithm, all game
states for 3, 4, 5 and 6 piece per player games have been checked.

## Features

  - A 2-player, local game of matryoshka tic-tac-toe
    - `src/game/main.rs -> target/release/game`

Harder than it looks!

  - Solver
    - `src/solver/main.rs -> target/release/solver`
    - A single player game against the CPU
    - Min-max algorithm
      - Efficient
      - Highly parallel
        - Any move in a 7-piece game will pin every core of a 64 vCPU Graviton instance
      - Alpha-beta pruning
      - Pruning of symmetrical branches
      - Heuristics for the most expansive search-spaces
    - Lookup table
      - Fully solved for games of 3/4/5/6 pieces

You can't win. It is hard to avoid losing.

  - Lookup generator
    - `src/lookup-generator/main.rs -> target/release/lookup-generator --help`


![Example game](media/example-game.png)
