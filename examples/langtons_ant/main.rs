use cellular_automata::sim::run_sim;
use pixels::Error;

mod langtons_ant;

fn main() -> Result<(), Error> {
    let sim = Box::new(langtons_ant::LangtonsAnt::new());
    run_sim(sim, None)
}
