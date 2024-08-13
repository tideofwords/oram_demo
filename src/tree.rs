
mod tree{
    pub const BUCKET_SIZE: usize = 3;
    pub const STASH_SIZE: usize = 5;

    #[derive(Copy, Clone)]
    pub struct Block {
        pub idx: usize,
        pub value: bool,
    }

    pub trait Holder {
        fn new() -> Self;
        fn capacity(&self) -> usize;
        fn blocks_mut(&mut self) -> &mut Vec<Option<Block>>;
        fn blocks(&self) -> &Vec<Option<Block>>;
        //fn clear(&mut self);
        //fn is_full(&self) -> bool;
        //fn write_block(&mut self, block: &Block);

        fn clear(&mut self) -> () {
            *self.blocks_mut() = vec![None; self.capacity()];
        }

        fn is_full(&self) -> bool {
            self.blocks().iter().all(|&x| x.is_some())
        }
        
        fn write_block(&mut self, block: &Block) -> () {
            let idx: usize = self.blocks_mut().iter()
                .position(|&x| x.is_none())
                .expect("Block is full");
            self.blocks_mut()[idx] = Some(*block);
            return;
        }
    }


    pub struct Bucket {
        pub capacity: usize,
        pub blocks: Vec<Option<Block>>,
    }

    impl Holder for Bucket {
        fn new() -> Bucket {
            let capacity: usize = BUCKET_SIZE;
            let blocks = vec![None; capacity];
            Bucket {capacity: capacity, blocks: blocks}
        }

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

    pub struct Stash {
        pub capacity: usize,
        pub blocks: Vec<Option<Block>>,
    }

    impl Holder for Stash {
        fn new() -> Stash {
            let capacity: usize = STASH_SIZE;
            let blocks = vec![None; capacity];
            Stash {capacity: capacity, blocks: blocks}
        }

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
}

#[cfg(test)]
mod test_bucket {
    use super::tree::*;

    #[test]
    fn test_is_full() {
        let mut bucket = Bucket::new();
        assert!(!bucket.is_full());
        bucket.write_block(&Block{idx: 0, value: true});
        bucket.write_block(&Block{idx: 1, value: true});
        assert!(!bucket.is_full());
        bucket.write_block(&Block{idx: 2, value: false});
        assert!(bucket.is_full());
    }

    #[test]
    #[should_panic]
    fn test_overfull() {
        let mut bucket = Bucket::new();
        bucket.write_block(&Block{idx: 0, value: true});
        bucket.write_block(&Block{idx: 1, value: true});
        bucket.write_block(&Block{idx: 2, value: false});
        bucket.write_block(&Block{idx: 3, value: false});
    }
}