#![allow(dead_code)]
mod utility;
pub use utility::vector::*;

fn main() {
    let vec1 = Vector3::new(1.0, 1.0, 1.0); 
    println!("curr vec is:{:#?}", vec1);
}
