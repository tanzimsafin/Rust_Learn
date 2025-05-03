pub trait BoundRect<T> {
    fn area(&self) -> T;
}

struct Shape<T> {
    height: T,
    width: T,
}

impl<T> BoundRect<T> for Shape<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.height * self.width
    }
}

fn main() {
    let area_sq = Shape {
        height: 30,
        width: 30,
    };
    println!("Area: {}", area_sq.area());
    println!("Hello, world!");
}