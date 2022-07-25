fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);




    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
        r1.push_str(" world"); //Adding " world" to the string r1 - scoped s.
        println!("{r1}"); //Prints: Hello world
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{r2}"); //Prints: Hello world
    println!("{s}"); //Prints: Hello world


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

