
// Majority of rust's built-in types are similar to types you would see in other systems programming languages. 
// There are keywords for true/false, numbers, and strings. But because its a systems programming language, you will 
// also see things like u8, i8 which stands for unsigned integer and signed integer respectively. 



fn main() {
    println!("Running data types training program, get ready for all the education!");
    println!("=====================================================================");
    
    // For starters, lets see what happens when we add two integer values together
    // Uncomment the code below and run the program 

    // let var1: i8 = -15;
    // let var2: i8 = 15; 

    // println!("Running addition operation on {} and {}", var1, var2);
    // let var3 = var1 + var2;
    // println!("The result is {}", var3);
    
    // end coding block 

    // the answer above should be 0 because we're adding 15 and -15 together 
    // because these are classified as signed integers, we can add a negative and a positive number together
    // but what if we use an unsigned integer in the same scenario?
    // uncomment the code below to observe the error 

    // let var1: u8 = -15;
    // let var2: u8 = 15; 

    // notice an error that emerges for the -15 value. This is because we defined our variable as an unsigned integer 

    // ------------- ---------------------------------------------------------
    // What if in the above scenario we wanted to use the number 150?
    // uncomment below to find out!

    // let var1: i8 = 150;

    // you'll notice either when compiling or when your code analyzer reviews the code that 150 is too large. This is because 
    // in systems programming languages your goal is typically to get the best performance and one way to achieve that is
    // to define your memory layouts. Javascript, Java, C#, etc. actually do offer keywords like long, double, etc. but 
    // most people just use int because its easier to manage but at the cost of performance.

    // if we want to use a number like 150 we'll have to use the next "increment" which is i16




}
