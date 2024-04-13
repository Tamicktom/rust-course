fn main() {
    println!("Exercice 1:");
    exercice_1();

    println!("\nExercice 2:");
    exercice_2();

    println!("\nExercice 3:");
    exercice_3();

    println!("\nExercice 4:");
    exercice_4();
}

// Create three variables with the names: val1, val2, and ans.
// We want to perform a simple operation of generating the modulo of val1 and val2.
// Set val1 to 5 and val2 to 2.
// Assign the answer to the ans variable.
// Before executing your code, what do you think the answer will be?

fn exercice_1() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;

    println!("{}", ans); // 1
}

// Create a vector and put in the values "2, 4, 6, 8, 10".
// Once you have created the vector perform the following:
// print out the current values,
// remove the value 10,
// add the value 12,
// and then print the vector back out to confirm your results.

fn exercice_2() {
    let mut vector = vec![2, 4, 6, 8, 10];

    println!("{:?}", vector); // [2, 4, 6, 8, 10]

    // find the index of 10
    let index = vector.iter().position(|&x| x == 10).unwrap();

    // remove the value 10
    vector.remove(index);

    // add the value 12
    vector.push(12);

    println!("{:?}", vector); // [2, 4, 6, 8, 12]
}

// Create a function called "concat_string".
// Create a string variable and assign the value "Hello" to it.
// The function is going to take one argument that is of type string and is going to return a String.
// Inside this function, concatenate the string " World".
// Print out the results in main() to confirm your results.

fn exercice_3() {
    let s = "Hello".to_string();
    let result = concat_string(s);

    println!("{}", result); // Hello World
}

fn concat_string(s: String) -> String {
    let world = " World".to_string();

    s + &world
}

// Create a function called control_flow.
// This is going to take one argument that is an integer.
// Based on this integer, print out the following:
// "The value is one",
// "The value is greater than 50",
// "The value is less than 25",
// or "The value is greater than 25 but less than 50".

fn exercice_4() {
    control_flow(1); // The value is one
    control_flow(51); // The value is greater than 50
    control_flow(24); // The value is less than 25
    control_flow(30); // The value is greater than 25 but less than 50
}

fn control_flow(value: i32) {
    if value == 1 {
        println!("The value is one");
    } else if value > 50 {
        println!("The value is greater than 50");
    } else if value < 25 {
        println!("The value is less than 25");
    } else {
        println!("The value is greater than 25 but less than 50");
    }
}
