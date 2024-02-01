use cellular_automata::sim::run_sim;
use pixels::Error;

mod rock_paper_scissors;

fn main() -> Result<(), Error> {
    let sim = Box::new(rock_paper_scissors::RockPaperScissors::new());
    run_sim(sim)
}
