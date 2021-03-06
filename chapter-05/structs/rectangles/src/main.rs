#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rectangle = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rectangle)
  );

  println!("rect1 is {:#?}", rectangle);
  dbg!(&rectangle);
  dbg!(8 * 9);
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
