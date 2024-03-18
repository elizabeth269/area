struct Rectangle {
    height: u32,
    breadth: u32,
}

fn main() {
    let rect = Rectangle {
        height: 30,
        breadth: 40,
    };
    println!("the area of the rectangle is {} square unit", area(&rect))
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.breadth
}
