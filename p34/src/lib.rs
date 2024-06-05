use std::ops::Add;

trait BigNumOps: Sized + Default + Copy + PartialOrd + From<bool> {
    fn add_overflow(self, rhs: Self) -> (Self, bool);
}

impl BigNumOps for u64 {
    fn add_overflow(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl BigNumOps for u32 {
    fn add_overflow(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct BigUint<T: BigNumOps, const N: usize> {
    limbs: [T; N],
}

impl<T: BigNumOps, const N: usize> Default for BigUint<T, N> {
    fn default() -> Self {
        BigUint {
            limbs: [T::default(); N],
        }
    }
}

impl<T: BigNumOps, const N: usize> Add for BigUint<T, N> {
    type Output = BigUint<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = BigUint::default();
        let mut carry: T = T::default();

        for i in 0..N {
            let (sum, overflow1) = self.limbs[i].add_overflow(rhs.limbs[i]);
            let (sum, overflow2) = sum.add_overflow(carry);
            out.limbs[i] = sum;
            carry = (overflow1 || overflow2).into();
        }
        if carry > T::default() {
            panic!("Overflow for when adding values of size {}.", N * 64);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type BU64 = BigUint<u64, 2>;
    type BU32 = BigUint<u32, 2>;

    #[test]
    fn test_bu64_add() {
        let u1 = BU64 { limbs: [0x1, 0x1] };
        let u2 = BU64 { limbs: [0x1, 0x0] };
        let u3 = BU64 { limbs: [0x2, 0x1] };
        assert_eq!(u1 + u2, u3);
    }

    #[test]
    fn test_bu64_add_overflow() {
        let u1 = BU64 {
            limbs: [0xFFFFFFFFFFFFFFFF, 0x0],
        };
        let u2 = BU64 { limbs: [0x1, 0x0] };
        let u3 = BU64 { limbs: [0x0, 0x1] };
        assert_eq!(u1 + u2, u3);
    }

    #[test]
    #[should_panic]
    fn test_bu64_add_overflow_panix() {
        let u1 = BU64 {
            limbs: [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF],
        };
        let u2 = BU64 { limbs: [0x1, 0x0] };
        let _ = u1 + u2;
    }

    #[test]
    fn test_bu32_add() {
        let u1 = BU32 { limbs: [0x1, 0x1] };
        let u2 = BU32 { limbs: [0x1, 0x0] };
        let u3 = BU32 { limbs: [0x2, 0x1] };
        assert_eq!(u1 + u2, u3);
    }

    #[test]
    fn test_bu32_add_overflow() {
        let u1 = BU32 {
            limbs: [0xFFFFFFFF, 0x0],
        };
        let u2 = BU32 { limbs: [0x1, 0x0] };
        let u3 = BU32 { limbs: [0x0, 0x1] };
        assert_eq!(u1 + u2, u3);
    }

    #[test]
    #[should_panic]
    fn test_bu32_add_overflow_panix() {
        let u1 = BU32 {
            limbs: [0xFFFFFFFF, 0xFFFFFFFF],
        };
        let u2 = BU32 { limbs: [0x1, 0x0] };
        let _ = u1 + u2;
    }
}
