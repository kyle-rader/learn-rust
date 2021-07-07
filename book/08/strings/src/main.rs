use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let data_str: &str = "Hello, strings 🚀";
    let data_string = String::from("foo bar baz");

    println!("{}", data_str);

    let mut s: String = data_str.to_string();
    println!("{}", s);

    s.push_str("💚💨");
    println!("{}", s);
    s.push('💤');
    println!("{}", s);

    let s: String = s + " " + &data_string;
    //              ^ this s is moved here and no longer valid.
    //                But we can reclaiom the name in the assignment.
    println!("{}", s);

    // Better string concats
    let s2 = format!("{}-{}-{}", data_str, data_string, s);
    println!("{}", s2);

    // Handling unicode with unicode-segmentation
    let s = "नमस्ते";
    for c in s.chars() { // actually returns 6 Char instances
        print!("{} ", c);
    }
    println!("");
    
    // splits into 4 graphemes, the meaningful characters or letters of this Hindi word.
    for c in s.graphemes(true).collect::<Vec<&str>>() { 
        print!("{} ", c);
    }
    println!("");
}
