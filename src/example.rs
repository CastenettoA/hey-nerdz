pub fn _vector() {
    // vec with '!' allow to use vec with array syntax.
    let mut numbers = vec![1, 2, 3, 4, 5];
    print_elements(&numbers);

    increment_elements(&mut numbers);
    print_elements(&numbers);
}

fn print_elements(vec: &Vec<i32>) {
    for &num in vec.iter() {
        println!("n: {}", num);
    }
}

fn increment_elements(vec: &mut Vec<i32>) {
    for num in vec.iter_mut() {
        // *num dereferenzia num per modificarne il valore
        // 'num = ...' will not work because is a reference to a value, not a direct access to it.
        // the '*num' give direct access to the value stored in the vector, allowing modifications
        *num *= 2;
    }
}