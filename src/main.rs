fn main() {
    let number_array = [1, 10, 2, -1, 5, 0];
    let string_array = ["abc", "def", "Banana", "John Smith", "john smith"];

    println!(
        "The largest number in number_array is {}",
        largest(&number_array)
    );
    println!(
        "The largest string in string_array is {}",
        largest(&string_array)
    );
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_index = 0;

    for index in 1..list.len() {
        if list[index] > list[largest_index] {
            largest_index = index;
        }
    }

    &list[largest_index]
}
