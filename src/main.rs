use std::time::Instant;
use letter_boxed_bot::*;

fn solve(game: &Game) {
    let start = Instant::now();

    let solutions = Solver::solve(game);

    let max_first_word_len = solutions
        .iter()
        .map(|s| s[0].len())
        .max()
        .unwrap();
    let max_second_word_len = solutions
        .iter()
        .map(|s| s[1].len())
        .max()
        .unwrap();


    println!("{}", "-".repeat(max_first_word_len + max_second_word_len + 3));

    println!("{}", game.format_board());

    for s in &solutions {
        println!("{} {:>max_first_word_len$} | {:<max_second_word_len$}", s.join("").len(), s[0], s[1]);
    }

    println!("\n{} solutions found in {:?}", solutions.len(), start.elapsed());
    println!("{}\n", "-".repeat(max_first_word_len + max_second_word_len + 3));
}

fn main() {
    let s = Instant::now();

    if let Some(game) = Game::today() {
        solve(&game);
        Solver::solve(&game);
    };

    eprintln!("Finished in {:?}", s.elapsed());
}
