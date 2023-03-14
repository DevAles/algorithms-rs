fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }

    println!("{:?}", arr);
}

fn main() {
    let mut arr = [1, 3, 2, 9, 4, 92, 43, 58, 21];
    bubble_sort(&mut arr);
}
