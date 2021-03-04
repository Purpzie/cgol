//! Contains macros used to implement operators and traits on [`Cell`](super::Cell).

macro_rules! integers {
    ($($i:ty)*) => {$(
        impl From<$i> for Cell {
            fn from(i: $i) -> Cell {
                if i > 0 { Alive } else { Dead }
            }
        }

        impl From<Cell> for $i {
            fn from(c: Cell) -> $i {
                c as $i
            }
        }

        impl_ops!($i);
    )*};
}

macro_rules! impl_ops {
    ($i:ty) => {
        impl_ops! {
            $i,
            + Add add += AddAssign add_assign
            - Sub sub -= SubAssign sub_assign
            * Mul mul *= MulAssign mul_assign
            / Div div /= DivAssign div_assign
            % Rem rem %= RemAssign rem_assign
            & BitAnd bitand &= BitAndAssign bitand_assign
            | BitOr bitor |= BitOrAssign bitor_assign
            ^ BitXor bitxor ^= BitXorAssign bitxor_assign
            << Shl shl <<= ShlAssign shl_assign
            >> Shr shr >>= ShrAssign shr_assign
        }
    };

    (
        $i:ty,
        $(
            $op:tt
            $trait:ident
            $method:ident
            $op_assign:tt
            $trait_assign:ident
            $method_assign:ident
        )*
    ) => {$(
        impl $trait<Cell> for $i {
            type Output = $i;

            #[inline]
            fn $method(self, c: Cell) -> $i {
                self $op (c as $i)
            }
        }

        forward_binop!(impl $trait::$method<Cell> for $i);

        impl $trait_assign<Cell> for $i {
            #[inline]
            fn $method_assign(&mut self, c: Cell) {
                *self $op_assign (c as $i);
            }
        }

        forward_assign!(impl $trait_assign::$method_assign<Cell> for $i);
    )*};
}

// op &T
macro_rules! forward_unop {
    (impl $trait:ident::$method:ident for $ty:ty) => {
        impl $trait for &$ty {
            type Output = <$ty as $trait>::Output;

            #[inline]
            fn $method(self) -> Self::Output {
                $trait::$method(*self)
            }
        }
    };
}

// T op &U, &T op U, &T op &U
macro_rules! forward_binop {
    (impl $trait:ident::$method:ident<$u:ty> for $t:ty) => {
        impl $trait<&$u> for $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> Self::Output {
                $trait::$method(self, *other)
            }
        }

        impl $trait<$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> Self::Output {
                $trait::$method(*self, other)
            }
        }

        impl $trait<&$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> Self::Output {
                $trait::$method(*self, *other)
            }
        }
    };
}

// T op &U
macro_rules! forward_assign {
    (impl $trait:ident::$method:ident<$u:ty> for $t:ty) => {
        impl $trait<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $trait::$method(self, *other);
            }
        }
    };
}
