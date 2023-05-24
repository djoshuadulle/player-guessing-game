# player-guessing-game
A game to test your player jersey number recognition skills. Displays a jersey number and player options for you to choose from.  

Quickstart (from top level directory): `cargo run ./atl-utd_roster.csv 3`  

# Requirements
Game requires a player roster, i.e. a CSV file of players names and jersey numbers. See the example: `atl-utd_roster.csv`.  

# Running the game
There are two command line arguments:  
1) The path to the roster CSV file  
2) The number of player options to choose from (allows two to full roster)  

To run: `cargo run [path to roster csv] [number of player options]`  
 
Example usage with three player options (from top level directory): `cargo run ./atl-utd_roster.csv 3`  
