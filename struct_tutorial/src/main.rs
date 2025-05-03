#[derive(Debug,Clone)]
struct User{
    name:String,
    locationl:String,
    age:u32,
    active:bool
}

impl User{
    fn age_check(&self) ->String{
        if self.age<18{
            String::from("Your age is below 18")
        }else{
            String::from("Your age is avobe 18")
        }
    }
    fn print_hello(){ // associated function does't use &self as reference
        println!("{}","Hello Solana");
    }
}
fn main() {
    let mut u1=User{
        name:String::from("Terror"),
        locationl:String::from("Dhaka"),
        age:22,
        active:true
    };
    let s1=u1.age_check();
    println!("{}",s1);
    println!("{}",u1.name);
    u1.name=String::from("Solana");
    println!("{}",u1.name);

    let u2=User{
        name:String::from("ETH"),
        ..u1.clone() // we can use this to use other object parameters
    };
    println!("{}",u2.active);
    println!("{:?}",u1);
    User::print_hello();
}
