use std::io;
use std::collections::HashMap;

struct SshConnection;

struct MachineSetup<F> {
    instance_type: String,
    ami: String,
    setup: F,
}

impl MachineSetup {
    pub fn new<F>(instance_type: String, ami: String, setup: F) -> Self
    where
        F: Fn(&mut SshConnection) -> io::Result<()>,
    {
        MachineSetup {
            instance_type,
            ami,
            setup,
        }
    }
}
struct BurstBuilder {
    descriptors: HashMap<String, (MachineSetup, u32)>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptors: Default::default(),
        }
    }
}
impl BurstBuilder {
    pub fn add_set(&mut self, name: String, number: u32, description: MachineSetup) {}

    pub fn run() {}
}

fn main() {
    let mut builder = BurstBuilder::default();

    builder.add_set(
        "server",
        1,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
            // yum install apache
        }),
    );

    builder.add_set(
        "client",
        2,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
            // git clone ...
        }),
    );

    builder.run(|vms: HashMap<String, MachineSetup>| {
        let server_ip = vms["server"][0].ip;
        let cmd = format!("ping {}", server_ip);

        vms["client"].for_each_parallel(|client| {
            client.exec(cmd);
        })
    });
}
