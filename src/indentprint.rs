fn get_indent(indention_text: &str, indention: usize) -> String {
    if indention < indention_text.len() {
        return "".to_string();
    }
    ' '.to_string().repeat(indention - indention_text.len())
}

pub fn print(print_text: &str, indention_text: &str, indention: usize) {
    let indent = get_indent(indention_text, indention);
    print!("{}{}", indent, print_text);
}

pub fn println(print_text: &str, indention_text: &str, indention: usize) {
    let indent = get_indent(indention_text, indention);
    println!("{}{}", indent, print_text);
}
