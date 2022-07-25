
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    
    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    println!("The area of the rectangle is {}", area(rect));

}


fn area(rec : Rectangle) -> u32  {
    rec.width * rec.height
}