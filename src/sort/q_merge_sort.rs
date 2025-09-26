// fn Q-mergesort(A, left, right) {
//    # test for empty array and length 1
//    if left >= right {
//        return break;
//    }
//
//    length = right - left + 1;

//    # find quaters
//    q1 = ⌊left + 1 * (length) / 4⌋
//    q2 = ⌊left + 2 * (length) / 4⌋
//    q3 = ⌊left + 3 * (length) / 4⌋

//    # quick sort inplace
//    Q-MERGESORT(A, left, q1-1 )
//    Q-MERGESORT(A, q1, q2 -1)
//    Q-MERGESORT(A, q2, q3 - 1)
//    Q-MERGESORT(A, q3, right)

//    # Sort four of less elements inplace
//    # Choose a sorting algorithm (any)
//    insertion_sort(A, left, right);
// }

// use super::insertion_sort;

pub fn q_merge_sort<T>(list: &mut [T], left: usize, right: usize)
where
    T: PartialEq + std::cmp::PartialOrd + Clone,
{
    // Handle the case where left > right (invalid range)
    if left >= right {
        return;
    }

    let length = right - left + 1;

    // Calculate quarter points more carefully
    let q1 = left + (1 * length / 4);
    let q2 = left + (2 * length / 4);
    let q3 = left + (3 * length) / 4;

    // Recursive calls with proper bounds checking
    if q1 > left {
        q_merge_sort(list, left, q1 - 1);
    }
    if q2 > q1 {
        q_merge_sort(list, q1, q2 - 1);
    }
    if q3 > q2 {
        q_merge_sort(list, q2, q3 - 1);
    }
    if right >= q3 {
        q_merge_sort(list, q3, right);
    }

    merge_four_quarters(list, left, q1, q2, q3, right);
    // insertion_sort(list, left, right);
}

fn merge_four_quarters<T>(list: &mut [T], start: usize, q1: usize, q2: usize, q3: usize, end: usize)
where
    T: PartialEq + std::cmp::PartialOrd + Clone,
{
    // Create temporary storage for the merge
    let mut temp = Vec::with_capacity(end - start + 1);

    // Indices for each quarter
    let mut i1 = start; // First quarter: [start, q1)
    let mut i2 = q1; // Second quarter: [q1, q2)
    let mut i3 = q2; // Third quarter: [q2, q3)
    let mut i4 = q3; // Fourth quarter: [q3, end+1)

    // Merge all four quarters
    while i1 < q1 || i2 < q2 || i3 < q3 || i4 <= end {
        let mut min_val = None;
        let mut min_source = 0;

        // Find the minimum among the current elements of all quarters
        if i1 < q1 {
            min_val = Some(&list[i1]);
            min_source = 1;
        }

        if i2 < q2 && (min_val.is_none() || &list[i2] < min_val.unwrap()) {
            min_val = Some(&list[i2]);
            min_source = 2;
        }

        if i3 < q3 && (min_val.is_none() || &list[i3] < min_val.unwrap()) {
            min_val = Some(&list[i3]);
            min_source = 3;
        }

        if i4 <= end && (min_val.is_none() || &list[i4] < min_val.unwrap()) {
            // min_val = Some(&list[i4]);
            min_source = 4;
        }

        // Move the minimum element to temp and advance the corresponding index
        match min_source {
            1 => {
                temp.push(list[i1].clone());
                i1 += 1;
            }
            2 => {
                temp.push(list[i2].clone());
                i2 += 1;
            }
            3 => {
                temp.push(list[i3].clone());
                i3 += 1;
            }
            4 => {
                temp.push(list[i4].clone());
                i4 += 1;
            }
            _ => unreachable!(),
        }
    }

    // Copy the merged result back to the original array
    for (i, item) in temp.into_iter().enumerate() {
        list[start + i] = item;
    }
}

#[cfg(test)]
mod q_merge_sort {
    use super::*;

    #[test]
    fn sort_array() {
        let mut array = [0, 7, 6, 1, 4, 5];
        let length = array.len();
        q_merge_sort(&mut array, 0, length - 1);
        assert_eq!(array, [0, 1, 4, 5, 6, 7]);
    }

    #[test]
    fn sort_empty_array() {
        let mut array: [i32; 0] = [];
        if !array.is_empty() {
            let length = array.len();
            q_merge_sort(&mut array, 0, length - 1);
        }
        assert_eq!(array, []);
    }

    #[test]
    fn sort_single_element() {
        let mut array = [42];
        q_merge_sort(&mut array, 0, 0);
        assert_eq!(array, [42]);
    }

    #[test]
    fn sort_small_array() {
        let mut array = [3, 1, 2];
        let length = array.len();
        q_merge_sort(&mut array, 0, length - 1);
        assert_eq!(array, [1, 2, 3]);
    }
}
