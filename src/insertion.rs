#[allow(dead_code)]
pub fn insertion_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let mut i = 1;
    while i < list.len() {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j, j - 1);
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut input = vec![4i32, 10, 1, 2, 9, 25, 3, 11, 7, 24];

        insertion_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 2, 3, 4, 7, 9, 10, 11, 24, 25]);
    }

    #[test]
    fn test_insertion_sort_0_elements() {
        let mut input = vec![];

        insertion_sort::<i32>(&mut input);

        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_insertion_sort_1_element() {
        let mut input = vec![4i32];

        insertion_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32]);
    }

    #[test]
    fn test_insertion_sort_2_elements() {
        let mut input = vec![4i32, 1];

        insertion_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 4]);
    }
}
