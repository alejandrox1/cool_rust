fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 61, 100, -1];
    let result = largest(&number_list);
    println!("The largest number in {:?} is {}", number_list, result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_generic(&number_list);
    println!("The largest number in {:?} is {}", number_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic(&char_list);
    println!("The largest char in {:?} is {}", char_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic_ref(&char_list);
    println!("The largest char in {:?} is {}", char_list, result);
}
