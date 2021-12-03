use fastrand;
use lazy_static::lazy_static;

static ONE_MASK: u32 = 1;
static TWO_MASK: u32 = 2;
static THREE_MASK: u32 = 4;
static FOUR_MASK: u32 = 8;
static FIVE_MASK: u32 = 16;
static SIX_MASK: u32 = 32;
static SEVEN_MASK: u32 = 64;
static EIGHT_MASK: u32 = 128;
static NINE_MASK: u32 = 256;
static ALL_MASK: u32 = 0b111111111;

// Index lookups for the 3x3 boxes that contain "box peers"
static BOX_1: [usize; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];
static BOX_2: [usize; 9] = [3, 4, 5, 12, 13, 14, 21, 22, 23];
static BOX_3: [usize; 9] = [6, 7, 8, 15, 16, 17, 24, 25, 26];
static BOX_4: [usize; 9] = [27, 28, 29, 36, 37, 38, 45, 46, 47];
static BOX_5: [usize; 9] = [30, 31, 32, 39, 40, 41, 48, 49, 50];
static BOX_6: [usize; 9] = [33, 34, 35, 42, 43, 44, 51, 52, 53];
static BOX_7: [usize; 9] = [54, 55, 56, 63, 64, 65, 72, 73, 74];
static BOX_8: [usize; 9] = [57, 58, 59, 66, 67, 68, 75, 76, 77];
static BOX_9: [usize; 9] = [60, 61, 62, 69, 70, 71, 78, 79, 80];
// Reverse lookup from 'cell' to 'box'
static BOX_REVERSE_LOOKUP: [&[usize; 9]; 81] = [
    &BOX_1, &BOX_1, &BOX_1, &BOX_2, &BOX_2, &BOX_2, &BOX_3, &BOX_3, &BOX_3, //
    &BOX_1, &BOX_1, &BOX_1, &BOX_2, &BOX_2, &BOX_2, &BOX_3, &BOX_3, &BOX_3, //
    &BOX_1, &BOX_1, &BOX_1, &BOX_2, &BOX_2, &BOX_2, &BOX_3, &BOX_3, &BOX_3, //
    &BOX_4, &BOX_4, &BOX_4, &BOX_5, &BOX_5, &BOX_5, &BOX_6, &BOX_6, &BOX_6, //
    &BOX_4, &BOX_4, &BOX_4, &BOX_5, &BOX_5, &BOX_5, &BOX_6, &BOX_6, &BOX_6, //
    &BOX_4, &BOX_4, &BOX_4, &BOX_5, &BOX_5, &BOX_5, &BOX_6, &BOX_6, &BOX_6, //
    &BOX_7, &BOX_7, &BOX_7, &BOX_8, &BOX_8, &BOX_8, &BOX_9, &BOX_9, &BOX_9, //
    &BOX_7, &BOX_7, &BOX_7, &BOX_8, &BOX_8, &BOX_8, &BOX_9, &BOX_9, &BOX_9, //
    &BOX_7, &BOX_7, &BOX_7, &BOX_8, &BOX_8, &BOX_8, &BOX_9, &BOX_9, &BOX_9, //
];

// Since we use 1 bit per digit, 9 digits means 9 bits = 512 different mask values.
// We can then just store all 512 combinations using their literal value as an index into an array of 512 elements.
// Then given the 'mask' of elements we can return a pre-allocated array of elements.
lazy_static! {
    static ref OPTION_SETS: Vec<Vec<u32>> = {
        let mut m = Vec::with_capacity(512);
        for idx in 0..512 {
            let mut set = Vec::new();
            if idx & ONE_MASK != 0 {
                set.push(ONE_MASK);
            }
            if idx & TWO_MASK != 0 {
                set.push(TWO_MASK);
            }
            if idx & THREE_MASK != 0 {
                set.push(THREE_MASK);
            }
            if idx & FOUR_MASK != 0 {
                set.push(FOUR_MASK);
            }
            if idx & FIVE_MASK != 0 {
                set.push(FIVE_MASK);
            }
            if idx & SIX_MASK != 0 {
                set.push(SIX_MASK);
            }
            if idx & SEVEN_MASK != 0 {
                set.push(SEVEN_MASK);
            }
            if idx & EIGHT_MASK != 0 {
                set.push(EIGHT_MASK);
            }
            if idx & NINE_MASK != 0 {
                set.push(NINE_MASK);
            }
            m.push(set);
        }
        m
    };
}

fn is_grid_filled(grid: [u32; 81]) -> bool {
    !grid.contains(&0)
}

fn get_available_options_from_peer_mask(peer_mask: u32) -> &'static Vec<u32> {
    // Begin with 'all' options mask, and subtract any peer elements (by `&` with the inverted peer mask)
    let options_mask = ALL_MASK & (!peer_mask);

    // Use the options mask as a lookup into the pre-calculated options sets.
    &OPTION_SETS[options_mask as usize]
}

// Build up a bit-mask of all occupied peer values.
// A peer is any occupied cell in the same row, column or 'box' as the specified cell.
// e.g. the mask `110000000` would indicate that both 9 and 8 are occupied, but all others are available.
// e.g. the mask `000000001` would indicate that the digit 1 is occupied.
fn get_peer_mask(grid: &[u32; 81], index: &usize) -> u32 {
    let mut mask = 0u32; // Starting with all 0s

    // Add row peers
    for x in (index / 9) * 9..((index / 9) * 9 + 9) {
        mask |= grid[x];
    }

    // Add column peers
    for x in (index % 9..81).step_by(9) {
        mask |= grid[x];
    }

    // Add box peers
    for x in BOX_REVERSE_LOOKUP[*index].iter() {
        mask |= grid[*x];
    }

    mask
}

// Takes a grid of 'mask' values (0b0 to 0b111111111) and returns a grid of decimal values (0 to 9)
fn transform_mask_to_digit(mask_grid: [u32; 81]) -> [u8; 81] {
    let mut digit_result: [u8; 81] = [0; 81];

    for idx in 0..81 {
        let masked = mask_grid[idx];
        if masked & ONE_MASK != 0 {
            digit_result[idx] = 1;
        } else if masked & TWO_MASK != 0 {
            digit_result[idx] = 2;
        } else if masked & THREE_MASK != 0 {
            digit_result[idx] = 3;
        } else if masked & FOUR_MASK != 0 {
            digit_result[idx] = 4;
        } else if masked & FIVE_MASK != 0 {
            digit_result[idx] = 5;
        } else if masked & SIX_MASK != 0 {
            digit_result[idx] = 6;
        } else if masked & SEVEN_MASK != 0 {
            digit_result[idx] = 7;
        } else if masked & EIGHT_MASK != 0 {
            digit_result[idx] = 8;
        } else if masked & NINE_MASK != 0 {
            digit_result[idx] = 9;
        }
    }

    digit_result
}

// Takes a grid of decimal values (0 to 9) and returns a grid of 'mask' values (0b0 to 0b111111111)
fn transform_digit_to_mask(digit_grid: [u8; 81]) -> [u32; 81] {
    let mut mask_result: [u32; 81] = [0; 81];

    for idx in 0..81 {
        mask_result[idx] = match digit_grid[idx] {
            1 => ONE_MASK,
            2 => TWO_MASK,
            3 => THREE_MASK,
            4 => FOUR_MASK,
            5 => FIVE_MASK,
            6 => SIX_MASK,
            7 => SEVEN_MASK,
            8 => EIGHT_MASK,
            9 => NINE_MASK,
            _ => 0,
        }
    }

    mask_result
}

// A solvable grid is one that only leads to a single valid solution.
// Returns false if multiple solutions can be found from the current clues.
fn is_solvable_grid(mut grid: [u32; 81], solution_counter: &mut u8) -> bool {
    // Assuming we've been given a grid with at least 1 element removed...
    let index = grid.iter().position(|item| *item == 0).unwrap();

    let peer_mask = get_peer_mask(&grid, &index);
    let options = get_available_options_from_peer_mask(peer_mask);

    for opt in options.iter() {
        // Set the current index, if the grid is not solved we will recurse and the next empty cell will be solved.
        grid[index] = *opt;
        if is_grid_filled(grid) {
            *solution_counter += 1;
            break;
        } else {
            if !is_solvable_grid(grid, solution_counter) {
                // We've found at least 2 solutions - return immediately.
                return false;
            }
        }
    }

    // Unset the element so further iterations can be attempted.
    grid[index] = 0;

    // Return True if the number of solutions (so far) is 0 or 1.
    *solution_counter <= 1
}

// Generates a valid (pre-solved) sudoku grid.
pub fn generate_filled_grid() -> [u8; 81] {
    let mut grid: [u32; 81] = [0; 81];
    let mut option_sets: Vec<Vec<u32>> = Vec::new();
    let mut index = 0;
    while index < 81 {
        if let Some(options) = option_sets.get_mut(index) {
            if options.is_empty() {
                // We have no valid options, remove the previous value and try again.
                // This is effectively recursive, we will continue stepping backwards removing values until we find a valid option.
                option_sets.pop();
                grid[index - 1] = 0;
                index -= 1;
            } else {
                // Try the next option at random and move on to the next cell.
                let idx = fastrand::usize(..options.len());
                let current_option = options.remove(idx);
                grid[index] = current_option;
                index += 1;
            }
        } else {
            // Generate a new options list for this node, allow the loop to recycle to rerun this position.
            let peer_mask = get_peer_mask(&grid, &index);
            option_sets.push(get_available_options_from_peer_mask(peer_mask).clone());
        }
    }
    transform_mask_to_digit(grid)
}

// Takes a valid (pre-solved) sudoku grid and removes elements to create a puzzle.
// difficulty - the number of failed attempts to remove a random cell clue before returning a result (0-50,000)
// returns - a valid sudoku puzzle with only 1 solution.
pub fn create_puzzle_from_complete_grid(grid: [u8; 81], difficulty: u32) -> [u8; 81] {
    assert!(difficulty <= 50_000);
    let mut grid = transform_digit_to_mask(grid);

    let mut attempts = difficulty;

    while attempts > 0 {
        // Find a random element that isn't unset.
        let index = loop {
            let rnd = fastrand::usize(..81);
            if grid[rnd] != 0 {
                break rnd;
            }
        };

        let removed_value = grid[index];
        grid[index] = 0;

        let puzzle = grid.clone();

        if !is_solvable_grid(puzzle, &mut 0) {
            // Decrement remaining attempts, restore removed value.
            attempts -= 1;
            grid[index] = removed_value;
        }
    }

    transform_mask_to_digit(grid)
}
