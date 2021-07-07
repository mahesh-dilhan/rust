fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s1 = s2;
    println!("{}, world!", s1);
   // println!("{}, world!", s2);// will not work as this s2 already moved //


    let y1 = String::from("hello");
    let y2 = y1;

    //println!("{}, world!", y1);
    println!("{}, world!", y2)
}