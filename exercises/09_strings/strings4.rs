// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {
    
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}


//string_slice("blue");
// "blue" is a string literal, which is of type &str.

// string("red".to_string());
// .to_string() converts a &str into a String.

// string(String::from("hi"));
// String::from creates a String from a &str.

// string("rust is fun!".to_owned());
// .to_owned() converts a &str into a String.

// string("nice weather".into());
// .into() creates a String from a &str via type inference.

// string(format!("Interpolation {}", "Station"));
// format! creates a String using interpolation.

// string_slice(&String::from("abc")[0..1]);
// Slicing ([0..1]) on a String returns a &str of the first byte.

// string_slice(" hello there ".trim());
// .trim() removes whitespace and returns a &str.

// string("Happy Monday!".replace("Mon", "Tues"));
// .replace() modifies the string and returns a String.

// string("mY sHiFt KeY iS sTiCkY".to_lowercase());
// .to_lowercase() returns a String with all characters in lowercase.