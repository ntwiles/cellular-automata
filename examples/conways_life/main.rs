mod conway;

use cellular_automata::sim::run_sim;
use pixels::Error;

fn main() -> Result<(), Error> {
    let sim = Box::new(conway::Conway::new());
    run_sim(sim)
}
