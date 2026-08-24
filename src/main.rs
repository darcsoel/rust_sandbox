mod enums;
mod secondary;
mod structures_sandbox;

fn take_control(name: String) -> String {
    let mut some_new = String::from("test");
    some_new.push_str(&name);
    return some_new;
}

fn triple(number: i32) -> i32 {
    return number * 3;
}

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) -> String {
    let data = data.to_uppercase();

    println!("{data}");
    return data;
}

fn main() {
    println!("Hello, world!");

    let name = String::from("hello");
    let returned = take_control(name);

    println!("{returned}");

    let num = 2;
    let tripled = triple(num);

    println!("{tripled}");

    let mut counter = 0;
    let some_number = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("{some_number}");

    println!("iterating over range");
    for number in 1..10 {
        println!("{number}");
    }

    println!("reverse order iterator");
    for number in (1..10).rev() {
        println!("{number}");
    }

    // let a = 1..100; // iterator, 1..99 included
    let a = [42; 100]; // 100 elements, each has value of 42

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        let temp = a.len();
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed, more that [{temp}]");
    }

    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name, age) = cat;

    println!("{name} is {age} years old");

    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("last char from reference = {last_char}");

    let data2 = string_uppercase(data.clone());

    println!("{data}");
    println!("{data2}");
}

// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a: [i32; 4] = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.

    // let v = vec![10, 20, 30, 40];
    let v = Vec::from(a);

    return (a, v);
}

fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(*element * 2);
    }

    output
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec2(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec2(vec0.clone());
        assert_eq!(vec1, [22, 44, 66, 88]);
        assert_eq!(vec0, [22, 44, 66]);
    }

    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }

    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
