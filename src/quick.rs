#[allow(dead_code)]
pub fn quick_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    if list.len() < 2 {
        return;
    }

    quicksort(list, 0, list.len() as i32 - 1)
}

fn quicksort<T>(list: &mut [T], start: i32, end: i32)
where
    T: PartialOrd,
{
    match list.len() {
        1 => return,
        2 => {
            if list[0] > list[1] {
                list.swap(0, 1);
            }
            return;
        }
        _ => (),
    }

    if end < start {
        return;
    }

    let pivot_index = partition(list, start, end);

    quicksort(list, start, pivot_index - 1);
    quicksort(list, pivot_index + 1, end);
}

// TODO(dkg): fix this messy usize vs i32 business
fn partition<T>(list: &mut [T], start: i32, end: i32) -> i32
where
    T: PartialOrd,
{
    if list.len() == 2 {
        if list[0] > list[1] {
            list.swap(0, 1);
        }
        return 0;
    }

    let mut left = start;

    for j in start..end {
        if list[j as usize] < list[end as usize] {
            list.swap(left as usize, j as usize);
            left += 1;
        }
    }
    eprintln!("left {}", left);
    list.swap(left as usize, end as usize);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut input = vec![4i32, 10, 1, 2, 9, 25, 3, 11, 7, 24];

        quick_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 2, 3, 4, 7, 9, 10, 11, 24, 25]);
    }

    #[test]
    fn test_quick_sort_0_elements() {
        let mut input = vec![];

        quick_sort::<i32>(&mut input);

        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_quick_sort_1_element() {
        let mut input = vec![4i32];

        quick_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32]);
    }

    #[test]
    fn test_quick_sort_2_elements() {
        let mut input = vec![4i32, 1];

        quick_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 4]);
    }
}
