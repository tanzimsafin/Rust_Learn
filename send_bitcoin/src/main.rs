use std::io;
use rand::Rng;
fn send_bitcoin(){
    let address=vec!["Alex","Bob","Morean"];
    for address in &address{
        println!("{}",address);
    }
    let mut receiver_address=String::new();
    io::stdin().read_line(&mut receiver_address);
    let mut number=String::new();
    io::stdin().read_line(&mut number);
    if address.contains(&receiver_address.trim()){
        println!("{} send to the account of {}",number.trim(),receiver_address.trim());
    }else{
        println!("Wrong address");
    }
    
}
fn receieve_bitcoin(){
    println!("Hey you are going to receieve bitcoin!");
    println!("1....2....3");
    let amount = rand::thread_rng().gen_range(1..10);
    println!("hey you receive {} bitcoin",amount)
}
fn invalid_error(){
    println!("Hey sorry This is not possible according to our policy! try again happy coding 🤓 ");
}
fn main() {
    let welcome=String::from("Welcome to crypto world");
    println!("{}",welcome);
    //  take input staring 
    println!("Do you want to (s) Bitcoint or (r)");
    let mut command =String::new();
     io::stdin().read_line( &mut command).expect("Sorry unable to reach");
    if command.trim().eq("s"){
        send_bitcoin();
    }else if command.trim().eq("r"){
        receieve_bitcoin();
    }else{
        invalid_error();
    }
}
