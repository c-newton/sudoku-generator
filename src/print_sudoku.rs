use crate::SudokuSolution;

const OPEN_ARRAY_DELIM: &'static str = "[";
const CLOSE_ARRAY_DELIM: &'static str = "]";

const OPEN_OBJECT_DELIM: &'static str = "{";
const CLOSE_OBJECT_DELIM: &'static str = "}";

pub fn print_json_set(solutions: &Vec<SudokuSolution>) -> String {
    let rows: Vec<String> = solutions.iter().map(print_json).collect();

    format!(
        "{}{}{}",
        OPEN_ARRAY_DELIM,
        rows.join(",\n"),
        CLOSE_ARRAY_DELIM
    )
}

fn print_json(suln: &SudokuSolution) -> String {
    let num_clues = format!("\"numClues\":{},", suln.num_clues);
    let solution = format!(
        "\"solution\":{}{}{},",
        OPEN_ARRAY_DELIM,
        suln.solution.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","),
        CLOSE_ARRAY_DELIM
    );
    let puzzle = format!(
        "\"puzzle\":{}{}{}",
        OPEN_ARRAY_DELIM,
        suln.puzzle.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","),
        CLOSE_ARRAY_DELIM
    );

    format!(
        "{}{}{}{}{}",
        OPEN_OBJECT_DELIM, num_clues, solution, puzzle, CLOSE_OBJECT_DELIM
    )
}
