pub fn binary_search<T>(list: &[T], target: &T) -> Result<usize, &'static str>
where
    T: PartialEq + std::cmp::PartialOrd + std::ops::Add,
{
    if list.is_empty() {
        return Err("The list is empty");
    }

    let mut left = 0;
    let mut right = list.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if list[mid] == *target {
            return Ok(mid);
        }
        if list[mid] < *target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    Err("The target could not be found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_inbound_int() {
        let target = 3;
        let list = [0, 1, 2, 3];
        let index = binary_search(&list, &target);
        assert_eq!(target, list[index.unwrap()]);
    }

    #[test]
    fn binary_search_inbound_float() {
        let target = 3.0;
        let list = [0.0, 1.0, 2.0, 3.0];
        let index = binary_search(&list, &target);
        assert_eq!(target, list[index.unwrap()]);
    }

    #[test]
    fn binary_search_not_found() {
        let target = 5;
        let list = [0, 1, 2, 3];
        let index = binary_search(&list, &target);
        assert_eq!(Err("The target could not be found"), index);
    }

    #[test]
    fn binary_search_empty_list() {
        let target = 1;
        let list: [i32; 0] = [];
        let index = binary_search(&list, &target);
        assert_eq!(Err("The list is empty"), index);
    }

    #[test]
    fn binary_search_single_element_found() {
        let target = 42;
        let list = [42];
        let index = binary_search(&list, &target);
        assert_eq!(Ok(0), index);
    }
}
