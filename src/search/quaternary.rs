pub fn quaternary_search<T>(
    list: &[T],
    left: usize,
    right: usize,
    target: &T,
) -> Result<usize, &'static str>
where
    T: PartialEq + std::cmp::PartialOrd + std::ops::Add,
{
    if list.is_empty() {
        return Err("The list is empty");
    }

    if left > right {
        return Err("The target could not be found");
    }
    let mid1 = left + (right - left) / 4;
    if list[mid1] == *target {
        return Ok(mid1);
    }
    let mid2 = left + 2 * (right - left) / 4;
    if list[mid2] == *target {
        return Ok(mid2);
    }
    let mid3 = left + 3 * (right - left) / 4;
    if list[mid3] == *target {
        return Ok(mid3);
    }

    if *target < list[mid1] {
        // Search first quarter: [left, mid1-1]
        if mid1 == 0 {
            Err("The target could not be found")
        } else {
            quaternary_search(list, left, mid1 - 1, target)
        }
    } else if *target < list[mid2] {
        // Search second quarter: [mid1+1, mid2-1]
        if mid2 == 0 {
            Err("The target could not be found")
        } else {
            quaternary_search(list, mid1 + 1, mid2 - 1, target)
        }
    } else if *target < list[mid3] {
        // Search third quarter: [mid2+1, mid3-1]
        if mid3 == 0 {
            Err("The target could not be found")
        } else {
            quaternary_search(list, mid2 + 1, mid3 - 1, target)
        }
    } else {
        // Search quaternaryth quarter: [mid3+1, right]
        quaternary_search(list, mid3 + 1, right, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternary_search_inbound_int() {
        let target = 3;
        let list = [0, 1, 2, 3];
        let index = quaternary_search(&list, 0, list.len() - 1, &target);
        assert_eq!(target, list[index.unwrap()]);
    }

    #[test]
    fn quaternary_search_inbound_float() {
        let target = 3.0;
        let list = [0.0, 1.0, 2.0, 3.0];
        let index = quaternary_search(&list, 0, list.len() - 1, &target);
        assert_eq!(target, list[index.unwrap()]);
    }
}
