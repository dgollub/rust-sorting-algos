#[allow(dead_code)]
pub fn selection_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    for i in 0..list.len() {
        let mut min = i;
        for j in i + 1..list.len() {
            if list[j] < list[min] {
                min = j
            }
        }
        if min != i {
            list.swap(i, min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut input = vec![4i32, 10, 1, 2, 9, 25, 3, 11, 7, 24];

        selection_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 2, 3, 4, 7, 9, 10, 11, 24, 25]);
    }

    #[test]
    fn test_selection_sort_0_elements() {
        let mut input = vec![];

        selection_sort::<i32>(&mut input);

        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_selection_sort_1_element() {
        let mut input = vec![4i32];

        selection_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32]);
    }

    #[test]
    fn test_selection_sort_2_elements() {
        let mut input = vec![4i32, 1];

        selection_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 4]);
    }
}
