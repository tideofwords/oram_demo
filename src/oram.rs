
pub mod oram{
    use crate::tree::tree::*;
    use colored::Colorize;
    use rand::rngs::ThreadRng;
    use std::collections::HashSet;

    // An ORAM takes read/write instructions from a client (Fe)
    // and issues read/write instructions to a memory (Tree).
    // The ORAM scrambles the access patterns, so that the memory
    // does not learn which items the client reads and writes.
    pub struct Oram {
        tree: Tree,
        addrs: Vec<usize>,
        rng: ThreadRng,
    }

    #[derive(Clone, Copy)]
    pub struct ReadInstruction {
        pub idx: usize,
    }

    #[derive(Clone, Copy)]
    pub struct WriteInstruction {
        pub idx: usize,
        pub value: bool,
    }

    // An instruction that the client issues to oram.
    // The instruction has one of the following two forms:
    //    read idx
    //    write idx val
    #[derive(Clone, Copy)]
    pub enum Instruction {
        Read(ReadInstruction),
        Write(WriteInstruction),
    }

    impl Instruction {
        pub fn from(input: String) -> Option<Self> {
            let words: Vec<&str> = input.split_whitespace().collect();
            match words[0].to_lowercase().as_str() {
                "read" => {
                    if words.len() <= 1 {
                        return None;
                    }
                    let idx: usize = match words[1].parse::<usize>() {
                        Result::Ok(x) => x,
                        Result::Err(_) => return None,
                    };
                    Some(Instruction::Read(ReadInstruction{idx: idx}))
                },
                "write" => {
                    if words.len() <= 2 {
                        return None;
                    }
                    let idx: usize = match words[1].parse::<usize>() {
                        Result::Ok(x) => x,
                        Result::Err(_) => return None,
                    };
                    if words.len() <= 2 {
                        return None;
                    }
                    let value: bool = words[2] == "egg";
                    Some(Instruction::Write(WriteInstruction{idx: idx, value: value}))
                }
                "q" => {
                    println!("{}", "ORAM says: Goodbye!".red().bold());
                    std::process::exit(0);
                }
                _ => {
                    None
                }
            }
        }
    }

    impl Oram {
        // Initialize ORAM with virtual memory size of n
        // and randomly initialize the address array
        pub fn new(n: usize) -> Self {
            let mut depth: usize = 2;
            while 1 << depth < 2 * n {
                depth += 1;
            }
            let tree = Tree::new(depth);
            let mut rng = rand::thread_rng();
            let mut addrs: Vec<usize> = vec![0; n];
            for i in 0..n {
                addrs[i] = tree.random_leaf(&mut rng);
            }
            let instance: Self = Self {
                tree: tree,
                addrs: addrs,
                rng: rng,
            };
            instance.say(format!("Initializing ORAM with depth {:?}", depth));
            instance
        }

        pub fn say(&self, message: String) {
            let full_message = format!("ORAM says: {}", message);
            println!("{}", full_message.red().bold());
        }

        fn say_to_memory(&self, message: String) {
            let full_message = format!("Memory, {}", message);
            println!("{}", full_message.yellow().bold());
        }

        // Helper function to decide whether a block
        // (with address eviction_bucket_addr)
        // can be evicted to block number block_id
        fn is_eligible(
            &self,
            eviction_bucket_addr: usize,
            block_id: usize,
        ) -> bool {
            self.tree.is_ancestor(
                eviction_bucket_addr,
                self.addrs[block_id],
            )
        }

        // Execute a read or write instruction from the client
        pub fn execute_instruction(
            &mut self,
            instr: Instruction,
        ) -> Option<bool> {
            let mut read_value: Option<bool> = None;

            let idx = match instr {
                Instruction::Read(read) => read.idx,
                Instruction::Write(write) => write.idx,
            };

            let leaf_addr = self.addrs[idx];
            let new_addr = self.tree.random_leaf(&mut self.rng);
            self.addrs[idx] = new_addr;

            // Command to memory: 
            // Read all blocks from a path on the binary tree
            self.say_to_memory(format!("please read and clear the path to leaf {:?}", leaf_addr));
            let path_of_buckets = self.tree.read_and_clear_path(leaf_addr);

            let mut all_blocks: HashSet<Block> = path_of_buckets
                .iter()
                .flat_map(|bucket| bucket.blocks())
                .filter(|opt| opt.is_some())
                .map(|opt| opt.unwrap())
                .collect();

            let idx_block_opt = all_blocks
                .iter()
                .find(|block| block.idx == idx)
                .cloned();

            // Carry out the instruction: 
            // read or write the one value the user wants
            match instr {
                Instruction::Read(_) => {
                    read_value = Some(idx_block_opt?.value);
                }
                Instruction::Write(write) => {
                    let new_block = Block{ 
                        idx: idx,
                        value: write.value,
                    };
                    if idx_block_opt.is_some() {
                        all_blocks.remove(&idx_block_opt.unwrap());
                    }
                    all_blocks.insert(new_block);
                }
            }


            // Write all the blocks back to the same path on the tree.
            // This is a greedy algorithm,
            // starting at the leaf node, writing as many blocks as possible,
            // and working back to the root.
            let mut addr = leaf_addr;

            while addr >= self.tree.root_idx() {
                self.say_to_memory(format!(
                    "please write the following to bucket number {:?}", addr
                ));
                let capacity = self.tree.nodes[addr].capacity();
                let eligible_blocks: Vec<Block> = all_blocks
                    .iter()
                    .filter(|block| 
                        self.is_eligible(addr, block.idx)
                    )
                    .take(capacity)
                    .cloned()
                    .collect();
                for block in eligible_blocks {
                    self.say_to_memory(format!("{:?}", block));
                    self.tree.write_block_to_bucket(
                        addr, 
                        block
                    );
                    all_blocks.remove(&block);
                }
                addr = self.tree.parent_idx(addr);
            }

            read_value
        }
    }
}