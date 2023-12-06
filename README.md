# Sigmar's Garden Solver

A solver for the Sigmar's Garden minigame in [Opus Magnum](https://www.zachtronics.com/opus-magnum/) by [Zachtronics](https://www.zachtronics.com/).

It takes a screenshot of the game and determines the state of the game using edge-detection with some reference images. The game is then solved and moves are performed by simulating mouse clicks.

While I'm pretty sure that all puzzles are solvable, the solver has a 5 second timeout, since some puzzles require a lot of internal backtracking to get to a valid solution. There's a ~15% chance for it to timeout and give up, starting a new puzzle instead.
