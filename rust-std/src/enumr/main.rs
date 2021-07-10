enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let cn = Coin::Penny;
    println!("{}",value_in_cents(cn));

    let ni = Coin::Nickel;
    println!("{}",value_in_cents(ni));

    let di = Coin::Dime;
    println!("{}",value_in_cents(di));

    let qu = Coin::Quarter;
    println!("{}",value_in_cents(qu));

}
