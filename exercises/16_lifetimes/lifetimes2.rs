// Don't change this function.
fn longest<'a>(x: &'a str, _y: & str) -> &'a str {
    x
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
