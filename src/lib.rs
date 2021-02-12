#[allow(dead_code)]
fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let mut n = list.len();
    if n < 1 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 1..n {
            // n not included in range
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }

        n -= 1;

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut input = vec![4i32, 10, 1, 2, 9, 25, 3, 11, 7, 24];

        bubble_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 2, 3, 4, 7, 9, 10, 11, 24, 25]);
    }

    #[test]
    fn test_bubble_sort_0_elements() {
        let mut input = vec![];

        bubble_sort::<i32>(&mut input);

        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_bubble_sort_1_element() {
        let mut input = vec![4i32];

        bubble_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32]);
    }

    #[test]
    fn test_bubble_sort_2_elements() {
        let mut input = vec![4i32, 10];

        bubble_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32, 10]);
    }
}
