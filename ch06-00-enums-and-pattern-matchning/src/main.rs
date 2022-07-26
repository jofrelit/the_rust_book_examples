use std::fmt;

enum UsState {
    Alabama,
    Alaska,
}

impl fmt::Display for UsState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = match self {
            UsState::Alabama => "Alabama",
            UsState::Alaska => "Alaska",
        };
        write!(f, "{}", typename)
    }

}

enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}
impl fmt::Display for UsCoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = match self {
            UsCoin::Penny => "Penny",
            UsCoin::Nickel => "Nickel",
            UsCoin::Dime => "Dime",
            UsCoin::Quarter(_) => "Quarter",
        };
        write!(f, "{}", typename)
    }
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
        UsCoin::Quarter(Some(UsState::Alabama)),
        UsCoin::Quarter(Some(UsState::Alaska)),
        UsCoin::Quarter(None),
    ];

    for coin in coin_array {
        let coin_value = value_in_cents(&coin);

        match &coin {
           UsCoin::Quarter(optional_state) =>  {
                if let Some(state) = optional_state {
                    println!("The value of a {} {} is {} cent",state, coin, coin_value);
                } else {
                    println!("The value of a {} is {} cent", coin, coin_value); 
                }
            }
            _ => {
                println!("The value of a {} is {} cent", coin, coin_value);
            }
        }
        
    }




}
