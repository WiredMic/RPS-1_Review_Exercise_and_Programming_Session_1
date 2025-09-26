// fn instertion_sort(A, left, right) {
//     i ← left + 1
//     while i <= right
//         j ← i
//         while j > left and A[j-1] > A[j]
//             swap A[j] and A[j-1]
//             j ← j - 1
//         end while
//         i ← i + 1
//     end while
// }

pub fn insertion_sort<T>(array: &mut [T], left: usize, right: usize)
where
    T: PartialEq + std::cmp::PartialOrd,
{
    let mut i = left + 1;
    while i <= right {
        let mut j = i;
        while j > left && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod insertion_sort {
    use super::*;

    #[test]
    fn sort_array() {
        let mut array = [2, 3, 1];
        let len = array.len();
        insertion_sort(&mut array, 0, len - 1);
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn sort_part_array() {
        let mut array: [u8; 5] = [22, 2, 3, 1, 0];
        let len = array.len();
        insertion_sort(&mut array, 1, len - 2);
        assert_eq!(array, [22, 1, 2, 3, 0]);
    }
}
