fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut clone = s.clone(); //clone of string
    clone.push_str("World!"); //push modification
    clone //return changed string
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}