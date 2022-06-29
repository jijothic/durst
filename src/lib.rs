struct Durst {
}

struct MachineSet{}

struct DurstBuilder {
    descriptors: Vec<MachineSet>
}

impl Default for BurstBuilder {
    fn default() -> self {
        DurstBuilder {
            descriptors: Vec::new(), 
        }
    }
}
impl Durst {
    
    pub fn add_set(&mut self, description: MachineSet){}

    pub fn run()
}

fn main() {
    let mut b = DurstBuilder::default;
    b.add_set(MachineSet::new("t2.micro", "ami-e1aa89b", |ssh| {
        ssh.exec("sudo yum install vim");
    }));
    b.run(||)
}
