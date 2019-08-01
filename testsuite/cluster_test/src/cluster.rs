<<<<<<< HEAD
use crate::instance::Instance;
use failure::{self, prelude::*};
use rand::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

=======
use crate::{aws::Aws, instance::Instance};
use failure::{self, prelude::*};
use rand::prelude::*;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Filter};

#[derive(Clone)]
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
pub struct Cluster {
    instances: Vec<Instance>, // guaranteed non-empty
}

impl Cluster {
<<<<<<< HEAD
    pub fn discover() -> failure::Result<Self> {
        let f = File::open("instances.txt")?;
        let f = BufReader::new(f);
        let mut instances = vec![];
        for line in f.lines() {
            instances.push(Instance::new(line?));
=======
    pub fn discover(aws: &Aws) -> failure::Result<Self> {
        let filters = vec![
            Filter {
                name: Some("tag:Workspace".into()),
                values: Some(vec![aws.workplace().clone()]),
            },
            Filter {
                name: Some("tag:Role".into()),
                values: Some(vec!["validator".into()]),
            },
        ];
        let result = aws
            .ec2()
            .describe_instances(DescribeInstancesRequest {
                filters: Some(filters),
                max_results: Some(1000),
                dry_run: None,
                instance_ids: None,
                next_token: None,
            })
            .sync()
            .expect("Failed to describe aws instances");
        let mut instances = vec![];
        for reservation in result.reservations.expect("no reservations") {
            for aws_instance in reservation.instances.expect("no instances") {
                let ip = aws_instance
                    .private_ip_address
                    .expect("Instance does not have private IP address");
                let tags = aws_instance.tags.expect("Instance does not have tags");
                let peer_id = tags
                    .into_iter()
                    .find(|tag| tag.key == Some("PeerId".into()))
                    .expect("No peer id")
                    .value
                    .expect("PeerId tag has no value");
                let short_hash = peer_id[..8].into();
                instances.push(Instance::new(short_hash, ip));
            }
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
        }
        ensure!(!instances.is_empty(), "instances.txt is empty");
        Ok(Self { instances })
    }

    pub fn random_instance(&self) -> Instance {
        let mut rnd = rand::thread_rng();
        self.instances.choose(&mut rnd).unwrap().clone()
    }
<<<<<<< HEAD
=======

    pub fn instances(&self) -> &Vec<Instance> {
        &self.instances
    }
>>>>>>> 05c40c977badf052b9efcc4e0180e3628bee2847
}
