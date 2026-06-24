fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.

    // <author>: this is shadowing. like replacing an existing value with a variable name
    //           that exists without types concern.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
