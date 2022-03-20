fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple((width, height))
    );
    let rect = Rectangle { width, height };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect)
    );

    println!("The area of the rectangle is {} square pixels", rect.area());
    println!("rect is {:?}", rect); // rect的类型 需要派生Debug功能
    println!("rect is {:#?}", rect);

    println!(
        "can hold: {}",
        rect.can_hold(Rectangle {
            width: 20,
            height: 10
        })
    );
    println!("rect is {:?}", Rectangle::square(60));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
// 使用元组作为参数类型
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// 使用结构体作为参数类型
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)] // 通过注解来派生 Debug trait功能
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
