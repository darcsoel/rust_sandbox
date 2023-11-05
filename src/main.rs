fn take_control(name: String) -> String {
    let mut some_new = String::from("test");
    some_new.push_str(&name);
    return some_new;
}

fn triple(number: i32) -> i32 {
    return number * 3;
}

fn main() {
    println!("Hello, world!");

    let name = String::from("hello");
    let returned = take_control(name);

    println!("{returned}");

    let num = 2;
    let tripled = triple(num);

    println!("{tripled}")

}
