// fn merge_sort(A, left, right) {
//   if left < right{
//     middle = (left + right) / 2;
//     merge_sort(A, left, middle - 1);
//     merge_sort(A,middle,right);
//     merge(A,left, right);
//   }
// }

// use super::insertion_sort;

pub fn merge_sort<T>(array: &mut [T], left: usize, right: usize)
where
    T: PartialEq + std::cmp::PartialOrd + Clone,
{
    if left < right {
        let middle = (left + right) / 2;

        merge_sort(array, left, middle);
        merge_sort(array, middle + 1, right);
        // insertion_sort(array, left, right);
        merge(array, left, middle, right);
    }
}

fn merge<T>(array: &mut [T], left: usize, middle: usize, right: usize)
where
    T: PartialEq + std::cmp::PartialOrd + Clone,
{
    // Create temporary arrays for the two halves
    let left_half: Vec<T> = array[left..=middle].to_vec();
    let right_half: Vec<T> = array[middle + 1..=right].to_vec();

    let mut i = 0; // Index for left_half
    let mut j = 0; // Index for right_half
    let mut k = left; // Index for merged array

    // Merge the two halves back into array[left..=right]
    while i < left_half.len() && j < right_half.len() {
        if left_half[i] <= right_half[j] {
            array[k] = left_half[i].clone();
            i += 1;
        } else {
            array[k] = right_half[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements from left_half, if any
    while i < left_half.len() {
        array[k] = left_half[i].clone();
        i += 1;
        k += 1;
    }

    // Copy remaining elements from right_half, if any
    while j < right_half.len() {
        array[k] = right_half[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_sort {
    use super::*;

    #[test]
    fn sort_array() {
        let mut array = [0, 7, 6, 1, 4, 5];
        let length = array.len();
        merge_sort(&mut array, 0, length - 1);
        assert_eq!(array, [0, 1, 4, 5, 6, 7]);
    }

    #[test]
    fn sort_empty_array() {
        let mut array: [i32; 0] = [];
        if !array.is_empty() {
            let length = array.len();
            merge_sort(&mut array, 0, length - 1);
        }
        assert_eq!(array, []);
    }

    #[test]
    fn sort_single_element() {
        let mut array = [42];
        merge_sort(&mut array, 0, 0);
        assert_eq!(array, [42]);
    }

    #[test]
    fn sort_small_array() {
        let mut array = [3, 1, 2];
        let length = array.len();
        merge_sort(&mut array, 0, length - 1);
        assert_eq!(array, [1, 2, 3]);
    }
}
