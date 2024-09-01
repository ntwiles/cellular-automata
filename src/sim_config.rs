pub struct SimConfig {
    pub debug: bool,
}

// TODO: Config is all-or-nothing for now as a convenience (and since there's only one option), but
// making each field optional would be more flexible.
impl Default for SimConfig {
    fn default() -> Self {
        Self { debug: false }
    }
}
