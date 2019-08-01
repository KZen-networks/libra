use crate::{
    cluster::Cluster,
    effects::{Effect, Reboot},
    experiments::Experiment,
    instance::Instance,
};
use failure;
<<<<<<< HEAD
use std::{thread, time::Duration};
=======
use std::{collections::HashSet, fmt, thread, time::Duration};
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847

pub struct RebootRandomValidator {
    instance: Instance,
}

impl RebootRandomValidator {
    pub fn new(cluster: &Cluster) -> Self {
        Self {
            instance: cluster.random_instance(),
        }
    }
}

impl Experiment for RebootRandomValidator {
<<<<<<< HEAD
=======
    fn affected_validators(&self) -> HashSet<String> {
        let mut r = HashSet::new();
        r.insert(self.instance.short_hash().clone());
        r
    }

>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
    fn run(&self) -> failure::Result<()> {
        let reboot = Reboot::new(self.instance.clone());
        reboot.apply()?;
        while !reboot.is_complete() {
            thread::sleep(Duration::from_secs(5));
        }
        Ok(())
    }
}
<<<<<<< HEAD
=======

impl fmt::Display for RebootRandomValidator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reboot {}", self.instance)
    }
}
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
