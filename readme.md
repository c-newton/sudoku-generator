## Usage

Running this will generate a file "sudoku-puzzles-out.json" containing a valid JSON array of puzzles with both their solved and unsolved versions.

You may specify the number of puzzles to generate **per logical core** and the minimum and maximum 'difficulty' to generate.

Increasing difficulty will *drastically* increase run time.

Usage:  
cargo run --release $puzzlesPerCore $minDifficulty $maxDifficulty

Example:  
cargo run --release 100 3 5
