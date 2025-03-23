fn main() {
    let mut x = 4; //mut to let variables change (mutatable)

    println!("x is: {x}"); //this is okay
    //Rust doesnt like if you change the variable before using its original value
    x = 52;

    println!("x is also: {}", x); //this is also okay


    let y = 20;
    println!("y is: {y}");
    let y = y + 40; //Overwriting for a non mutatable variable is okay
    println!("y is also: {y}");

    { //another scope made

        let y = y - 80; //variables with same name in diff scope is okay (will not overwrite)
        //Using variables from and exterior scope with the same name is okay (will get confusing)
        println!("why is y: {y}?");
    }

    const UM_HI: u32 = 60; //Type needs to be specified for constants
    //Redefining is NOT okay
    println!("{UM_HI}"); 

    /*
        u8 range 0 -> 2^8 - 1 (negative numbers are NOT okay)
        i8 range -2^7 -> 2^7 -1  (negatives are okay)
     */

    let uhhhh = true; //Not specifying data type is okay (boolean type is 'bool')
    println!("{uhhhh}");

    let mut tup = (1, false, 'f'); //tuple is fixed length of elements & immutable. Mixing data types is okay (pretty cool)
    //adding mut is okay
    tup.2 = 'q';
    println!("{}", tup.2);  //This is okay. Printing normally is NOT okay.

}
