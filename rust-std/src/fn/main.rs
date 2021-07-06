fn main() {

    println!("this is main");
    another_function();
    println!("{}",retfive());
}
fn another_function(){
    println!("this is in another_function");
}

fn retfive() -> i32 {
    let x=2;
    return x;
}