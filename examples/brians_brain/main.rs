mod brians_brain;

use cellular_automata::sim::run_sim;
use pixels::Error;

fn main() -> Result<(), Error> {
    let sim = Box::new(brians_brain::BriansBrain::new());
    run_sim(sim)
}
