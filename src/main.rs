use hello::greet;
use rand::{thread_rng, Rng};

const STARTING_MISSILES: i32  = 8;
const READY_AMOUNT: i32  = 2;

fn main() {
    excercise_1_variables();
    excercise_2_functions();
    greet();

    let random = thread_rng().gen_range(0, 101);

    let bunnies = 2;
    let typed: i32 = 2;
    let (carrots, lettuce) = (14, 6);

    let mut bunny = 3;
    bunny = 4;

    // bunnies = 45;

    const BUNNIES_CONSTANT: f64 = 3.14;

    let x = 5;

    {
        let y = 6;
        println!("{}, {}", x, y);
    }

    // println!("{}, {}", x, y); // Error

    {
        let x = 7;
        println!("{}", x); // Prints 7
    }

    println!("{}", x); // Prints 5

    // Shadowing
    let mut z = 3;
    let z = z;

    // let meme = 'Something';
    // let meme = make_image(meme);

    // Initialization

    let unassigned: i32;

    // println!("{}", unassigned); // Throws an Error
}

fn excercise_1_variables() {
  let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

  println!("Firing {} of my {} missiles...", ready, missiles);

  missiles = missiles - ready;

  println!("{} missiles left", missiles);
}

fn tail_expression(qty: f64) -> f64 {
    // If the last expression doesn't have a semicolon, it will be returned
    qty * 2.0
}


fn excercise_2_functions() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.

    let area = area_of(width, height);
    
    println!("Area is {}", area);

    // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
    //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    x * y
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}