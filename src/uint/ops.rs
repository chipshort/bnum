use super::{BUint, ExpType};
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::impl_ops;
use crate::digit::Digit;

impl<const N: usize> const Add<Digit> for BUint<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Digit) -> Self {
        let mut out = Self::ZERO;
        let result = self.digits[0].carrying_add(rhs, false);
        out.digits[0] = result.0;
        let mut carry = result.1;
        let mut i = 1;
        while i < N {
            let result = self.digits[0].carrying_add(0, carry);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        out
    }
}

impl<const N: usize> const BitAnd for BUint<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] & rhs.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> const BitOr for BUint<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] | rhs.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> const BitXor for BUint<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] ^ rhs.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> const Div for BUint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

impl<const N: usize> const Div<Digit> for BUint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Digit) -> Self {
        self.div_rem_digit(rhs).0
    }
}

impl<const N: usize> const Rem<Digit> for BUint<N> {
    type Output = Digit;

    #[inline]
    fn rem(self, rhs: Digit) -> Digit {
        self.div_rem_digit(rhs).1
    }
}

impl<const N: usize> const Not for BUint<N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = !self.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> const Rem for BUint<N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}

impl_ops!(BUint);

#[cfg(test)]
mod tests {
    use crate::types::U128;

    #[test]
    fn bitand() {
        let a = 934539445645648753475987u128;
        let b = 9384592074589749679475697u128;
        assert_eq!(U128::from(a) & U128::from(b), U128::from(a & b));
    }
    #[test]
    fn bitor() {
        let a = 345797465893865897346983548797u128;
        let b = 23496529846782457694586979779465u128;
        assert_eq!(U128::from(a) | U128::from(b), U128::from(a | b));
    }
    #[test]
    fn bitxor() {
        let a = 1873649845684389645897456757697889u128;
        let b = 2384689734763458437865873468485789u128;
        assert_eq!(U128::from(a) ^ U128::from(b), U128::from(a ^ b));
    }
    #[test]
    fn not() {
        let a = 2903646984856974586794084057698457689u128;
        assert_eq!(!U128::from(a), U128::from(!a));
    }
}