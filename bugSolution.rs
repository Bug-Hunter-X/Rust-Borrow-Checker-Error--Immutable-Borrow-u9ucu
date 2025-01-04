fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    
    *y += 1; // Modify x through the mutable reference y
    println!("x = {}", x); // Prints x = 6
    
    //This is now okay because y is a mutable reference
    *y -= 1; // Modify x through y
    println!("x = {}", x); //Prints x = 5
} 