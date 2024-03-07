// use crate::block::Block;
use crate::blockchain::Blockchain;

// use log::info;

mod errors;
mod block;
mod blockchain;

fn main() {
    let mut b = Blockchain::new().unwrap();
    let _ = b.add_block("data 1".to_string());
    let _ = b.add_block("data 2".to_string());
    let _ = b.add_block("data 3".to_string());

    for item in b.iter() {
      println!("item {:?}",item)
    }
}
