use std::cmp::min;

pub fn f1(tuple: &mut (u32, u32), second: bool) -> &mut u32 {
    if second {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}

pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    let mut n = min(slice.len(), n);
    if n == 0 {
        n = 1;
    }
    &mut slice[n - 1]
}

pub fn f3(slice: &[u32], n: usize) -> &u32 {
    let mut n = min(slice.len(), n);
    if n == 0 {
        n = 1;
    }
    &slice[slice.len() - n]
}

pub fn f4(slice: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let n = slice.len() / 4;
    (
        &slice[0..n],
        &slice[n..2 * n],
        &slice[2 * n..3 * n],
        &slice[3 * n..slice.len()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let mut tuple1 = (1u32, 2u32);
        assert_eq!(f1(&mut tuple1, false), &mut 1);
        assert_eq!(f1(&mut tuple1, true), &mut 2);
    }

    #[test]
    fn test_f2() {
        let mut a = [1, 2, 3, 4, 5];
        assert_eq!(f2(&mut a, 0), &mut 1);
        assert_eq!(f2(&mut a, 1), &mut 1);
        assert_eq!(f2(&mut a, 2), &mut 2);
        assert_eq!(f2(&mut a, 3), &mut 3);
        assert_eq!(f2(&mut a, 4), &mut 4);
        assert_eq!(f2(&mut a, 5), &mut 5);
        assert_eq!(f2(&mut a, 6), &mut 5);
    }

    #[test]
    fn test_f3() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(f3(&a, 0), &5);
        assert_eq!(f3(&a, 1), &5);
        assert_eq!(f3(&a, 2), &4);
        assert_eq!(f3(&a, 3), &3);
        assert_eq!(f3(&a, 4), &2);
        assert_eq!(f3(&a, 5), &1);
        assert_eq!(f3(&a, 6), &1);
    }

    #[test]
    fn test_4() {
        let a = [1, 2, 3, 4, 5];
        let res = f4(&a);
        assert_eq!(res.0, &[1]);
        assert_eq!(res.1, &[2]);
        assert_eq!(res.2, &[3]);
        assert_eq!(res.3, &[4, 5]);
    }
}
