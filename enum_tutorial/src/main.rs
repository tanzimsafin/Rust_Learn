enum Color{
    Green,
    Yellow,
    Red
}


fn color_signal(light:Color)->String{
    match light{
        Color::Green=>"Go ".to_string(),
        Color::Yellow=>"Hold".to_string(),
        Color::Red=>"Stop".to_string()
    }
}
fn addi(first_number:i32,second_number:i32)->Option<i32>{
    Some(first_number+second_number)
}
fn main() {
     println!("{}",color_signal(Color::Green));
     println!("{}",color_signal(Color::Yellow));
     println!("{}",color_signal(Color::Red));
     let first_number =0;
     let second_number = 0;
    
 
    //  let absent_number: Option<i32> = None;

     let addition= addi(first_number,second_number);
    
    match addition{
        Some(r)=>println!("{}",r),
        None=>println!("No value came")
    }
}
