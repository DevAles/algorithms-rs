pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }

    println!("{:?}", arr);
}

pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut min_index = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

fn main() {
    println!(
        "Please run \"cargo test\" to run all algorithms or \
             \"cargo test <name-of-algorithm>_test\" \
             to run a specific algorithm \
             (example: cargo test bubble_sort_test)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const RIGHT_ARR: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    #[test]
    fn bubble_sort_test() {
        let mut arr = [1, 3, 2, 4, 6, 5, 9, 7, 8];
        bubble_sort(&mut arr);

        assert_eq!(arr, RIGHT_ARR);
    }
}
