fn main() {
    println!("Exercice 1:");
    exercice_1();

    println!("\nExercice 2:");
    exercice_2();
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
