fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("World!");

    let concatenated_String = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_String);
}

fn concatenate_strings(a:&str,b:&str) -> String {
    let mut result = String::new();
    result.push_str(a);
    result.push_str(b);
    result
}