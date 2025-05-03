// struct Reactangle{
//     width:u32,
//     height:u32,
// }
// impl Reactangle{
//     fn area(&self) -> u32 {
//        return 2* self.width * self.height
//     }
//     fn normal(a:u32)->(String,u32){
//         return("the number is".to_string(),a)
//     }
// }
// fn main(){
//     let data=Reactangle {width:20,height:20};
//     let area_=data.area();
//     println!("{}",area_);
//     let normal_data=Reactangle::normal(45);
//     println!("{:?}",normal_data)
// }
enum  Shape{
    Square(f32),
    Circle(f32),
    Rect(f32,f32)
}
fn area(s:&Shape)->f32{
   match s{
    Shape::Circle(radius)=>3.1416*radius*radius,
    Shape::Rect(length,width)=>length*width,
    Shape::Square(side)=>side*side
   }
}
fn perimeter(s:&Shape)->f32{
   match s{
     Shape::Circle(radius)=>2.0*3.1416*radius,
     Shape::Rect(length,width)=>2.0*(length+width),
     Shape::Square(side)=>4.0*side,
   }

   }
fn main(){
    let area_cal=area(&Shape::Circle(10.0));
    let area_cal2=area(&Shape::Square(10.0));
    let area_cal3=area(&&Shape::Rect(10.0,20.0));
    println!("{}",area_cal);
    println!("{}",area_cal2);
    println!("{}",area_cal3);
    let perimeter_cal=perimeter(&Shape::Circle(10.0));
    let perimeter_cal2=perimeter(&&Shape::Square(10.0));
    let perimeter_cal3=perimeter(&&Shape::Rect(10.0,20.0));
    println!("{}",perimeter_cal);
    println!("{}",perimeter_cal2);
    println!("{}",perimeter_cal3);
}