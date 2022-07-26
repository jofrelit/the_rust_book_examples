#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


#[derive(Debug)]
enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: &UsCoin) -> u8 {
    match coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(_) => 25,
    }
}




fn main() {

    let coin_array = [
        UsCoin::Penny,
        UsCoin::Nickel,
        UsCoin::Dime,
        UsCoin::Quarter(UsState::Alabama),
    ];

    for coin in coin_array {
        let coin_value = value_in_cents(&coin);
        println!("The value of a {:?} is {} cent", coin, coin_value);
    }




}
