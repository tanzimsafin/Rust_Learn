// use serde::{Serialize,Deserialize};
// #[derive(Serialize,Deserialize,Debug)]
use borsh::{BorshSerialize,BorshDeserialize};
use std::fmt::Debug;
use std::cmp::PartialEq;
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct Point{
    x:i32,
    y:i32
}
fn main(){
    let point=Point{
        x:32,
        y:55
    };
    let ser = borsh::to_vec(&point).unwrap();
     let deser:Point=borsh::from_slice(&ser).unwrap();


    //serialize to json format
    // let ser=serde_json::to_string(&point).unwrap();
    println!("{:?}",ser);
    println!("{:?}",deser);
    // //deserialize to orginal format
    // let deser:Point=serde_json::from_str(&ser).unwrap();
    // println!("{:?}",deser);
}