# Tourney
A program that runs a game tournament in parallell.

# The Game
The game being played is a 'game-theory' game similar to the [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma); simple and only contains a few rules and possible options for each participant.
## Rules
The game is played in a non-deterministic amount of rounds. A round, or a move, consists of each player choosing a ply. Each player has full knowledge of all moves that has been played this game.
1. There are three possible plies:<br>
  🔴 Red<br>
  🟢 Green<br>
  🔵 Blue
2. Scores are calculated after each move according to the following matrix:
  <div align="center">
    
  | 1 \\ 2    | **Red** | **Green** | **Blue** |
  | :-------: | :-----: | :-------: | :------: |
  | **Red**   | 1, 1    | 3, 0      | 1, -1    |
  | **Green** | 0, 3    | 2, 2      | 1, -1    |
  | **Blue**  | -1, 1   | -1, 1     | 0, 0     |
  </div>
  
  * Notice that the upper left 2x2 matrix corresponds to the prisoner's dilemma.
  * Also notice that choosing blue essentially equals giving your opponent one of your own points.
3. At the end of all rounds, the player that has played the most blue ply's gets their score multiplied by 2.

# Acknowledgements
The creation of this project was inspired by [this video](https://www.youtube.com/watch?v=mScpHTIi-kM).
