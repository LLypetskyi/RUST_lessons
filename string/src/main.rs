fn main() {
    //Strings

    let s: String = String::new();
    let s1: String = String::from("Say hello!");
    let mut s2: String = "Say hello".to_string();

    s2.push_str(" World! "); // + string in the end of s2
    s2.push(':'); // + char - only 1 symbol in the end of s2
    s2.push(')');
    println!("{}", s2);

    //concatenation
    let s3: String = "Name".to_string();
    let s4: String = " Surname".to_string();
    let s5 = "Face".to_string();

    let rez = s3 + &s4;
    println!("{}", rez);
    let rez1 = format!("{} {}", rez, s5);
    println!("{}", rez1);

    //character-by-character search in String
    let some_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce egestas";

    for ch in some_text.chars() {
        print!("{}=>", ch);
    }
}
