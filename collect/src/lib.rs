pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {  
        let mut swapped = false;
        for j in 0..len - i - 1 { // Last i elements are already sorted
            // Traverse the array from 0 to len-i-1
            // Swap if the element found is greater than the next element
            // If the current element is greater than the next element, swap them
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped { // If no two elements were swapped in the inner loop, then break
            break;
        }
    }
}