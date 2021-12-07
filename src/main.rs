fn main() {
    let test: Vec<&str> = vec!["hi", "hello", "mitochondria is the powerhouse of the cell"];

    let test = test
        .iter()
        .filter(|s| s.contains("h")) // `s` has the type &&&str
        .map(|s| s.to_string()) // `s` has the type &&str
        .collect::<Vec<String>>();

    println!("Values: {:?}", test);
}
