# Matryoshka tic-tac-toe

![Matryoshka tic-tac-toe in real life](media/matryoshka-tic-tac-toe.gif)

![Wonder if this is still a solved game](media/ed.png)

## No.

The computational requirements explode after 5 pieces.

The 6 pieces of the video are unsolved.

## But yes.
Through exhaustive searching with an efficient min-max algorithm, all game
states for 3, 4 and 5 piece per player games have been checked.

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
        - Any move will pin every core of a 64 vCPU Graviton instance
      - Alpha-beta pruning
      - Pruning of symmetrical branches
    - Lookup table
      - Fully solved for games of 3/4/5 pieces

You can't win. It is hard to avoid losing.

  - Lookup generator
    - `src/lookup-generator/main.rs -> target/release/lookup-generator --help`
    - Very, very slow for 6-piece games
    - Solving 5-pieces took around 192 vCPU hours. 6-pieces will take around
      16000


![Example game](media/example-game.png)
