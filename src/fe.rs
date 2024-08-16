pub mod fe {
    use crate::oram::oram::{Instruction, Oram};
    use std::io;

    pub struct Fe {
        oram: Oram,
    }

    impl Fe {
        pub fn new() -> Self {
            print!("Welcome to ORAM!\n");
            print!("Size of virtual memory?\n");

            let mut input = String::new();
            let mut N: usize;
            loop {
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(num) => {
                        N = num;
                        break;
                    },
                    Err(_) => {
                        print!("Invalid input, please enter an integer.\n");
                    }
                }
            }
            let mut oram = Oram::new(N);
            Fe { oram }
        }

        pub fn get_and_execute_instruction(&mut self) {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let instr: Instruction = Instruction::from(input);
            let result: Option<bool> = self.oram.execute_instruction(instr);
            let mut output: String = String::new();
            match instr {
                Instruction::Read(read) => {
                    output = format!(
                        "ORAM says: Read value {:?} from position {:?}", 
                        result,
                        read.idx,
                    )
                },
                Instruction::Write(write) => {
                    output = format!(
                        "ORAM says: Wrote value {:?} to position {:?}",
                        write.value,
                        write.idx,
                    )
                }
            }
            self.oram.say(output);
        }
    }
}