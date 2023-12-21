const STARTING_MISSILES: i32  = 8;
const READY_AMOUNT: i32  = 2;

fn main() {
    variables();
    
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

fn variables() {
  let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

  println!("Firing {} of my {} missiles...", ready, missiles);

  missiles = missiles - ready;

  println!("{} missiles left", missiles);
}
