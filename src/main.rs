mod fe;
mod tree;
mod oram;

fn main() {
    let mut fe = crate::fe::fe::Fe::new();
    loop {
        fe.get_and_execute_instruction();
    }
}
