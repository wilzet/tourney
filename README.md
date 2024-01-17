# Tourney
A program written in `Rust` that runs a game tournament in parallel. The game and tournament configurations are explained by further reading this `README`. Thank you! 🙂

# The Game
The game being played is a 'game-theory' game similar to the [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma); simple and only contains a few rules and possible options for each participant.
## Rules
The game is played in a non-deterministic amount of rounds. A round, or a move, consists of each player choosing a color. Each player has full knowledge of all moves that has been played this game.
1. There are three possible color options:<br>
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
3. At the end of all rounds, the player that has played the most blue color options gets their score multiplied by 2.

The player with the highest score is deemed the winner. In the case of equal scores, it's a draw.

# Tournament
In its current state, the tournament has all participating programs playing against eachother for one game each. At the end of the tournament programs are listed in descending order according to their average score across their games.

## Configuration
The program accepts a few command line arguments:

 * `--min <u32>` - The minimum amount of rounds. The provided value must be greater than 0 and less than `std::u32::MAX`. The default value is 70 if `--min` is not provided.
   
 * `--max <u32>` - The maximum amount of rounds The provided value must be greater than 0 and less than `std::u32::MAX`. The default value is 100 if `--max` is not provided.

 * `--games` - The program will display all the games outcomes if this argument is provided. The default behaviour is thus to not display all the games outcomes.

 * `--threads <u32>` - Specify the amount of threads used. The provided value must be greater than 0 and less than or equal to 64. The default value is 20 if `--threads` is not provided.

> [!IMPORTANT]
> If only `--min` is provided, the config will have exactly `--min` amount of rounds. Likewise if only `--max` is provided, the config will have exactly `--max` amount of rounds. When both or neither of the arguments are supplied, the amount of rounds is a uniform random integer value between the minimum and maximum amount of rounds (inclusive).

> [!TIP]
> Great values for `--min` and `--max` will cause the program to have a long runtime. Stick to a smaller amount of rounds, perhaps no more than 100,000.

---

### Example 1
```console
cargo run -- --min 100 --max 200 --games --threads 10
```
The tournament will have between 100 and 200 rounds (inclusive), every game and corresponding score will be printed, and the process is specified to be run on 10 threads.
### Example 2
<table align="center">
<tr>
<th width=300>(a)</th>
<th width=300>(b)</th>
</tr>
<tr>
<td>
    
```console
cargo run -- --min 100
```
</td>
<td>
  
```console
cargo run -- --max 100
```
</td>
</tr>
</table>

The tournament will run for exactly 100 rounds. The commands are equivalent.
### Example 3
```console
cargo run -- --max 100 --min 200
```
This will cause a `panic!` since `min > max`.

# Acknowledgements
The creation of this project was inspired by [this video](https://www.youtube.com/watch?v=mScpHTIi-kM).
