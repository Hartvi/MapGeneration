
// define a new Rust module named `arrays`
pub mod arrayss {
    // define an array of integers with a length of 5
    pub const MY_ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
    pub const YS: [i32; 500] = [0; 500];
    
    // define a function to print the array elements
    pub fn print_array() {
        for element in MY_ARRAY.iter() {
            println!("{}", element);
        }
    }
}