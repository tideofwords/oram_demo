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

            // Get size of ORAM from user
            let mut input = String::new();
            let n: usize;
            loop {
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(num) => {
                        n = num;
                        break;
                    },
                    Err(_) => {
                        print!("Invalid input, please enter an integer.\n");
                    }
                }
            }
            let oram = Oram::new(n);
            Fe { oram }
        }

        // convert boolean to "egg" or "noegg" for printing
        fn to_string(&self, result: Option<bool>) -> &str {
            match result {
                Some(val) => if val {"egg"} else {"noegg"},
                None => "None",
            }
        }

        // one cycle of the main loop for user interaction
        pub fn get_and_execute_instruction(&mut self) {
            println!("");
            let mut instr: Option<Instruction>;
            loop{
                self.oram.say(String::from("Please enter an instruction."));
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                    instr = Instruction::from(input);
                if instr.is_some() {
                    break;
                }
                self.oram.say(String::from("Invalid instruction"));
            }
            let result: Option<bool> = self.oram.execute_instruction(instr.unwrap());
            let output: String;
            match instr.unwrap() {
                Instruction::Read(read) => {
                    output = format!(
                        "ORAM says: Read value {:?} from position {:?}", 
                        self.to_string(result),
                        read.idx,
                    )
                },
                Instruction::Write(write) => {
                    output = format!(
                        "ORAM says: Wrote value {:?} to position {:?}",
                        self.to_string(Some(write.value)),
                        write.idx,
                    )
                }
            }
            self.oram.say(output);
        }
    }
}