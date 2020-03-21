use std::fmt::Display;
use std::cmp::PartialOrd;

fn arr_to_string<T: Display>(slice: &[T]) -> String {
    let mut s = String::from("[");
    if s.len() > 0 {
        s.push_str(&slice[0].to_string());
        let tail = &slice[1..];
        for i in tail {
            s.push_str(",");
            s.push_str(&i.to_string());
        }
    }
    s.push_str("]");
    return s;
}


// O(n^2)
fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..(a.len() -1) {
            if a[i] > a[i+1] {
                a.swap(i, i+1);
                swapped = true;
            }
        }
    }
}


fn min_index<T: PartialOrd>(a: &[T], start_index: usize) -> usize {
    let mut min: usize = start_index;
    for i in (start_index + 1)..(a.len()) {
        if a[i] < a[min] {
            min = i;
        }
    }
    return min;
}


// O(n^2)
fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() {
        let min = min_index(&a, i);
        a.swap(i, min);
    }
}


// O(n^2)
fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut loc = 0;
        while loc < i && a[loc] < key {
            // Walk through the sub-array until we get to the
            // a[loc] >= key. We then know to insert key at loc
            loc += 1;
        }

        // Key wants to be inserted at loc
        // Need to move everything from loc up to i-1 up by one
        for j in (loc..i).rev() {
            a[j+1] = a[j];
        }

        a[loc] = key;
    }
}


fn print_array<T: Display>(slice: &[T]) {
    println!("{}", &arr_to_string(slice));
}


fn main() {
    let mut arr1 = [5, 4, 3, 2, 1, 10];
    println!("Unsorted:");
    print_array(&arr1);
    selection_sort(&mut arr1);
    println!("Selection sorted:");
    print_array(&arr1);
}
