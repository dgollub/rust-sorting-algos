/**
 * Gnome Sort is based on the technique used by the standard Dutch Garden Gnome
 * (Du.: tuinkabouter).
 *
 * Here is how a garden gnome sorts a line of flower pots.
 *
 * Basically, he looks at the flower pot next to him and the previous one;
 * if they are in the right order he steps one pot forward, otherwise, he swaps them
 * and steps one pot backward.
 *
 * Boundary conditions: if there is no previous pot, he steps forwards;
 * if there is no pot next to him, he is done.
 *
 * — "Gnome Sort - The Simplest Sort Algorithm". Dickgrune.com
 */
#[allow(dead_code)]
pub fn gnome_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let mut n = 0;
    while n < list.len() {
        if n == 0 || list[n] > list[n - 1] {
            n += 1;
        } else {
            list.swap(n, n - 1);
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnome_sort() {
        let mut input = vec![4i32, 10, 1, 2, 9, 25, 3, 11, 7, 24];

        gnome_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 2, 3, 4, 7, 9, 10, 11, 24, 25]);
    }

    #[test]
    fn test_gnome_sort_0_elements() {
        let mut input = vec![];

        gnome_sort::<i32>(&mut input);

        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_gnome_sort_1_element() {
        let mut input = vec![4i32];

        gnome_sort::<i32>(&mut input);

        assert_eq!(input, vec![4i32]);
    }

    #[test]
    fn test_gnome_sort_2_elements() {
        let mut input = vec![4i32, 1];

        gnome_sort::<i32>(&mut input);

        assert_eq!(input, vec![1i32, 4]);
    }
}
