mod reboot_random_validator;

pub use reboot_random_validator::RebootRandomValidator;
<<<<<<< HEAD

pub trait Experiment {
=======
use std::{collections::HashSet, fmt::Display};

pub trait Experiment: Display {
    fn affected_validators(&self) -> HashSet<String>;
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
    fn run(&self) -> failure::Result<()>;
}
