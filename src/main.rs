
mod day01;


trait Solvable {
    type InputT;
    type SolutionT: std::fmt::Debug;
    fn parse(input: &str) -> Self::InputT;
    fn solve(input: Self::InputT) -> Self::SolutionT;
}

fn main() {
    let input = include_str!("./../inputs/day01.input").trim();
    let input = day01::Solution::parse(input);
    let solution = day01::Solution::solve(input);
    dbg!(solution);
    // hmmm is havign the whole input in memory a good idea ?... hmmm lets not care about it heheheheh THIS ISNT GOONNA RUN ON PROD BWAHHHAHAHAHA
    // and the input is smol anyway
    // Depending on the day no read appropriate input files call the approprite solutions....
    // we could just include_str!() our way but we can't leak the input.....
    // oh wait but i don't have to distribute my solution exe.
    // welp, anything works.
}
