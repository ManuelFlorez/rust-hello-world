fn bubble_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    let mut swapped;

    loop {
        swapped = false;

        for i in 0 .. len -1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn main() {    
    let mut nums = vec![5, 3, 1, 4, 2];
    println!("Before sorting: {:?}", nums);

    bubble_sort(&mut nums);
    println!("After sorting: {:?}", nums);
}
