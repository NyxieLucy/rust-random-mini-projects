// standart imports nothing too fancy 
use std::io::{self, Write};

// a simple struct with string value for quote
struct Quotes {
    quote : String,
}
/*  function with parameter that uses '&mut' so that you can modify the vector
without taking ownership */
/*i just remembered that i have to mention that the parameter tells the function what to do */
fn adding_quote(quote_list : &mut Vec<Quotes>) {
    let mut input = String::new();
    println!("what's the quote you wanna add?: ");
    io::stdout().flush().expect("failed to flush");
// also mind keeping stdout before the stdin so that you can see the output and stuff
    io::stdin().read_line(&mut input).expect("failed to read");

    // added an instance that links the value of the Quotes struct with out input
    let new_q = Quotes{quote: input.trim().to_string()}; // okay i added the trim so that we get rid of the '/n' annoying and 'to_string' to consider it a string again
    /* and finaly adding it to the vector, remember you write the place where to store
     then the value you'd store */
    quote_list.push(new_q);
}

fn main() {
    // that's the vector, nothing too wow, if ya already know the basics you won't need this line you're reading now
    let mut quote_list: Vec<Quotes> = Vec::new();
    //here we called the function...
    adding_quote(&mut quote_list);
    // and finally print the output
    println!("the first quote is: {}", quote_list[0].quote)
}