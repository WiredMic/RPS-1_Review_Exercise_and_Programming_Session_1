pub fn linear_search<T: PartialEq>(list: &[T], target: &T) -> Result<usize, &'static str> {
    if list.is_empty() {
        return Err("The list is empty");
    }

    for (index, item) in list.iter().enumerate() {
        if item == target {
            return Ok(index);
        }
    }
    Err("The target could not be found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linaer_search_inbound_int() {
        let target = 3;
        let list = [0, 1, 2, 3];
        let index = linear_search(&list, &target);
        assert_eq!(target, list[index.unwrap()]);
    }

    #[test]
    fn linear_search_inbound_float() {
        let target = 3.0;
        let list = [0.0, 1.0, 2.0, 3.0];
        let index = linear_search(&list, &target);
        assert_eq!(target, list[index.unwrap()]);
    }
}
