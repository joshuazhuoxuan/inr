
fn somethingString(somethingString:String) -> String {
    println!("{}",somethingString);
    somethingString
}


fn main() {

    let mut s1 =String::from("joshua");
    s1.push_str("is handson");
    somethingString(s1);



}

