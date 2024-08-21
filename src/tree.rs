pub mod tree{
    use colored::Colorize;
    use rand::{Rng, rngs::ThreadRng};

    pub const BUCKET_SIZE: usize = 3;
    pub const STASH_SIZE: usize = 5;

    // A Block is a single memory block
    // that the ORAM client wants to store
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Block {
        pub idx: usize,
        pub value: bool,
    }

    // A Holder is a container for a small number of Blocks.
    // Holders come in two types, Bucket and Stash,
    // which differ only in the number of Blocks they store.
    pub trait Holder {
        fn capacity(&self) -> usize;
        fn blocks_mut(&mut self) -> &mut Vec<Option<Block>>;
        fn blocks(&self) -> &Vec<Option<Block>>;
        //fn clear(&mut self);
        //fn is_full(&self) -> bool;
        //fn write_block(&mut self, block: &Block);

        /* fn clear(&mut self) -> () {
            *self.blocks_mut() = vec![None; self.capacity()];
        }

        fn is_full(&self) -> bool {
            self.blocks().iter().all(|&x| x.is_some())
        } */
        
        fn write_block(&mut self, block: &Block) -> () {
            let idx: usize = self.blocks_mut().iter()
                .position(|&x| x.is_none())
                .expect("Block is full");
            self.blocks_mut()[idx] = Some(*block);
            return;
        }
    }

    #[derive(Clone, Debug)]
    pub struct Bucket {
        pub capacity: usize,
        pub blocks: Vec<Option<Block>>,
    }

    impl Bucket {
        pub fn new() -> Bucket {
            let capacity: usize = BUCKET_SIZE;
            let blocks = vec![None; capacity];
            Bucket {capacity: capacity, blocks: blocks}
        }
    }

    impl Holder for Bucket {
        fn capacity(&self) -> usize {
            self.capacity
        }

        fn blocks(&self) -> &Vec<Option<Block>> {
            &self.blocks
        }

        fn blocks_mut(&mut self) -> &mut Vec<Option<Block>> {
            &mut self.blocks
        }
    }

    #[derive(Clone, Debug)]
    pub struct Stash {
        pub capacity: usize,
        pub blocks: Vec<Option<Block>>,
    }

    impl Stash {
        fn new() -> Stash {
            let capacity: usize = STASH_SIZE;
            let blocks = vec![None; capacity];
            Stash {capacity: capacity, blocks: blocks}
        }
    }

    impl Holder for Stash {
        fn capacity(&self) -> usize {
            self.capacity
        }

        fn blocks(&self) -> &Vec<Option<Block>> {
            &self.blocks
        }

        fn blocks_mut(&mut self) -> &mut Vec<Option<Block>> {
            &mut self.blocks
        }
    }

    #[derive(Clone, Debug)]
    pub enum TreeNode {
        Bucket(Bucket),
        Stash(Stash),
    }

    impl Holder for TreeNode {
        fn capacity(&self) -> usize {
            match self {
                TreeNode::Bucket(bucket) => bucket.capacity(),
                TreeNode::Stash(stash) => stash.capacity(),
            }
        }

        fn blocks(&self) -> &Vec<Option<Block>> {
            match self {
                TreeNode::Bucket(bucket) => bucket.blocks(),
                TreeNode::Stash(stash) => stash.blocks(),
            }
        }

        fn blocks_mut(&mut self) -> &mut Vec<Option<Block>> {
            match self {
                TreeNode::Bucket(bucket) => bucket.blocks_mut(),
                TreeNode::Stash(stash) => stash.blocks_mut(),
            }
        }
    }

    // The main data structure on the memory side is a binary tree
    // of Holders: a Stash at the root, and a Bucket at each non-root node.
    // ORAM interacts with memory by reading and writing Blocks
    // to specified Holders:
    // in each round, ORAM will choose a leaf node, and
    // read and rewrite every Block in every Bucket on the path
    // to that leaf.

    // The binary tree is implemented as a Vec of length (1 << depth) + 1;
    // the root is indexed at 1 for convenience.
    pub struct Tree {
        depth: usize,
        pub nodes: Vec<TreeNode>,
    }

    pub trait TreeOps {
        fn clear_bucket(&mut self, idx: usize) -> ();
        fn write_block_to_bucket(&mut self, idx: usize, block: Block) -> (); 
        fn random_leaf(&self, rng: &mut ThreadRng) -> usize;
        fn read_and_clear_path(&mut self, leaf: usize) -> Vec<TreeNode>;

        fn parent_idx(&self, idx: usize) -> usize {
            idx / 2
        }
        fn root_idx(&self) -> usize {
            1
        }
        fn is_ancestor(&self, a: usize, d: usize) -> bool {
            let mut x: usize = d;
            while x > a {
                x /= 2;
            }
            x == a
        }
    }

    impl Tree {
        pub fn new(depth: usize) -> Self {
            let mut nodes: Vec<TreeNode> = vec![TreeNode::Bucket(Bucket::new()); (1 << depth) + 1];
            nodes[1] = TreeNode::Stash(Stash::new());
            Tree {depth: depth, nodes: nodes}
        }
    }

    impl TreeOps for Tree {
        fn clear_bucket(&mut self, idx: usize) -> () {
            let empty_node: TreeNode = match self.nodes[idx] {
                TreeNode::Bucket(_) => TreeNode::Bucket(Bucket::new()),
                TreeNode::Stash(_) => TreeNode::Stash(Stash::new()),
            };
            self.nodes[idx] = empty_node;
        }

        fn write_block_to_bucket(&mut self, idx: usize, block: Block) -> () {
            self.nodes[idx].write_block(&block);
        }

        fn random_leaf(&self, rng: &mut ThreadRng) -> usize {
            let a: usize = (1 << (self.depth - 1)) + 1;
            let b: usize = (1 << self.depth) + 1;
            rng.gen_range(a..b)
        }

        fn read_and_clear_path(&mut self, leaf: usize) -> Vec<TreeNode> {
            assert!(2 * leaf > 1 << self.depth);
            assert!(leaf <= 1 << self.depth);

            let mut buckets_on_path: Vec<TreeNode> = Vec::new();
            let mut idx = leaf;
            while idx >= self.root_idx() {
                buckets_on_path.push(
                    self.nodes[idx].clone()
                );
                self.clear_bucket(idx);
                idx = self.parent_idx(idx);
            }

            println!("{}", format!("{:?}", buckets_on_path).cyan());
            buckets_on_path
        }
    }
}

