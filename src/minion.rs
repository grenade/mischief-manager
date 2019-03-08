extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate uuid;

use rusoto_core::{
  Region
};
use rusoto_ec2::{
  Ec2,
  Ec2Client,
  RunInstancesRequest
};
use uuid::Uuid;

pub fn spawn(worker_type: String) {
  println!("worker type: {}", worker_type);

  let client_token = format!("{}-{}", worker_type, Uuid::new_v4());
  println!("client token: {}", client_token);

  let client = Ec2Client::new(Region::UsWest2);
  let run_instances_request: RunInstancesRequest = RunInstancesRequest {
    min_count: 1,
    max_count: 1,
    key_name: Some(format!("mozilla-taskcluster-worker-{}", worker_type)),
    client_token: Some(client_token),
    image_id: Some("ami-08d1120db609ed869".to_string()),
    instance_type: Some("c5.4xlarge".to_string()),
    //security_groups: Some(vec!["rdp-only".to_string(), "ssh-only".to_string()]),
    //security_group_ids: Some(vec!["sg-3bd7bf41".to_string(), "sg-s5bd6be21".to_string()]),
    //subnet_id: Some("subnet-f94cb29f".to_string()),
    instance_initiated_shutdown_behavior: Some("stop".to_string()),
    ..Default::default()
  };

  match client.run_instances(run_instances_request).sync() {
    Ok(output) => {
      match output.instances {
        Some(instances) => {
          println!("instances instantiated:");
          for instance in instances {
            println!("{:?}", instance.instance_id);
          }
        }
        None => println!("no instances instantiated!"),
      }
    }
    Err(error) => {
      println!("Error: {:?}", error);
    }
  }
}