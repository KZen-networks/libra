use failure::{self, prelude::*};
use std::{
    ffi::OsStr,
    fmt,
    process::{Command, Stdio},
};

#[derive(Clone)]
pub struct Instance {
<<<<<<< HEAD
=======
    short_hash: String,
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
    ip: String,
}

impl Instance {
<<<<<<< HEAD
    pub fn new<I>(ip: I) -> Instance
    where
        I: Into<String>,
    {
        Instance { ip: ip.into() }
=======
    pub fn new(short_hash: String, ip: String) -> Instance {
        Instance { short_hash, ip }
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
    }

    pub fn run_cmd<I, S>(&self, args: I) -> failure::Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let ssh_dest = format!("ec2-user@{}", self.ip);
        let ssh_args = vec!["-i", "/libra_rsa", ssh_dest.as_str()];
        let mut ssh_cmd = Command::new("ssh");
        ssh_cmd.args(ssh_args).args(args).stderr(Stdio::null());
        let status = ssh_cmd.status()?;
        ensure!(
            status.success(),
            "Failed with code {}",
            status.code().unwrap_or(-1)
        );
        Ok(())
    }

    pub fn check_ac_port(&self) -> bool {
        let mut cmd = Command::new("nc");
        cmd.args(vec!["-w", "1", "-z", self.ip.as_str(), "30307"]);
        let status = cmd.status();
        match status {
            Err(..) => false,
            Ok(exit_status) => exit_status.success(),
        }
    }
<<<<<<< HEAD
=======

    pub fn short_hash(&self) -> &String {
        &self.short_hash
    }
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
<<<<<<< HEAD
        write!(f, "{}", self.ip)
=======
        write!(f, "{}({})", self.short_hash, self.ip)
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
    }
}
