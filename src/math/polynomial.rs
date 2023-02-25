use core::fmt;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};
use Vec;
// use crate::complex::Complex;
use conv::prelude::*;
use itertools::Itertools;
use num_traits::{pow::Pow, One, Zero};
use std::default::Default;

pub trait PolynomialOperationTypes {}

impl PolynomialOperationTypes for u8 {}
impl PolynomialOperationTypes for u16 {}
impl PolynomialOperationTypes for u32 {}
impl PolynomialOperationTypes for usize {}
impl PolynomialOperationTypes for u64 {}
impl PolynomialOperationTypes for u128 {}
impl PolynomialOperationTypes for i8 {}
impl PolynomialOperationTypes for i16 {}
impl PolynomialOperationTypes for i32 {}
impl PolynomialOperationTypes for i64 {}
impl PolynomialOperationTypes for i128 {}
impl PolynomialOperationTypes for f32 {}
impl PolynomialOperationTypes for f64 {}
// impl PolynomialOperationTypes for Complex {}

#[macro_export]
macro_rules! x {
    () => {
        Polynomial {
            poly: vec![0.0, 1.0],
            deg: 1,
        }
    };
}

#[macro_export]
macro_rules! pn {
    (+$($coef:exprx^($exp:expr))) => {
        Polynomial {
            poly: vec![0.0, 1.0],
            deg: 1,
        }
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T: PolynomialOperationTypes>
where
    T: Copy,
{
    pub poly: Vec<T>,
    pub deg: u32,
}

impl<'b, T: PolynomialOperationTypes + fmt::Display + Copy + num_traits::Zero + std::cmp::PartialEq> Polynomial<T>
where
    T: Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Pow<T, Output = T>
        + ValueFrom<u32>
        + Copy,
{
    pub fn new() -> Self {
        Self {
            poly: Vec::<T>::new(),
            deg: 0_u32,
        }
    }

    pub fn p_add(self, other: &'b Polynomial<T>) -> Polynomial<T> {
        Polynomial {
            poly: self
                .poly
                .iter()
                .zip_longest(other.poly.iter())
                .map(|c| match c {
                    itertools::EitherOrBoth::Both(l, r) => *l + *r,
                    itertools::EitherOrBoth::Left(l) => *l,
                    itertools::EitherOrBoth::Right(r) => *r,
                })
                .collect::<Vec<T>>(),
            deg: if self.deg > other.deg {
                self.deg
            } else {
                other.deg
            },
        }
    }

    pub fn evaluate(&self, _x: T) -> T
    where
        T: ValueFrom<usize>,
    {
        self.poly
            .iter()
            .enumerate()
            .map(|(index, item)| -> T { *item * _x.pow(index.value_into().unwrap()) })
            .reduce(|accumulator, val| accumulator + val)
            .unwrap()
    }

    pub fn derivative(&self) -> Self {
        Self {
            poly: (1_u32..self.poly.len() as u32)
                .into_iter()
                .map(|c| -> T { T::value_from(c).unwrap() * self.poly[c as usize] })
                .collect::<Vec<T>>(),
            deg: self.deg - 1,
        }
    }

    pub fn to_string(&self) -> String {
        self.poly
            .iter()
            .enumerate()
            .filter(|(index, x)| T::value_from(0).unwrap() != **x)
            .map(|(index, val)| -> String {
                match index {
                    0 => format!("{}", val),
                    1 => format!("{}x", val),
                    _ => format!("{}x^({})", val, index)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ")
    }
}

impl<'b, T: fmt::Display + Copy + ValueFrom<T> + Zero + PartialEq> Display for Polynomial<T>
where
    T: PolynomialOperationTypes + conv::ValueFrom<T> + conv::ValueInto<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_output = self
            .poly
            .iter()
            .enumerate()
            .map(|(index, val)| -> String {
                match index {
                    0 => format!("{}", val),
                    1 => format!("{}x", val),
                    _ => format!("{}x^({})", val, self.poly.len() - index - 1)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");
        f.write_str(format!("Poly({})", str_output).as_str())
    }
}

impl<'a, 'b, T: Copy + Add<Output = T>> Add<&'b Polynomial<T>> for &'a Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn add(self, other: &'b Polynomial<T>) -> Polynomial<T> {
        Polynomial {
            poly: self
                .poly
                .iter()
                .zip_longest(other.poly.iter())
                .map(|c| -> T {
                    match c {
                        itertools::EitherOrBoth::Both(l, r) => *l + *r,
                        itertools::EitherOrBoth::Left(l) => *l,
                        itertools::EitherOrBoth::Right(r) => *r,
                    }
                })
                .collect::<Vec<T>>(),
            deg: if self.deg > other.deg {
                self.deg
            } else {
                other.deg
            },
        }
    }
}

impl<'a, T: Copy + Add<Output = T> + AddAssign> Add<T> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn add(self, other: T) -> Polynomial<T> {
        let mut new_polynomial = Polynomial {
            poly: self.poly,
            deg: self.deg,
        };
        new_polynomial.poly[0] += other;
        new_polynomial
    }
}

impl<'b, T: Copy + AddAssign> AddAssign<&'b Polynomial<T>> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    fn add_assign(&mut self, rhs: &'b Polynomial<T>) {
        if rhs.poly.len() > self.poly.len() {
            self.poly.extend(rhs.poly.iter().skip(self.poly.len()));
        }
        if rhs.poly.len() < self.poly.len() {
            self.poly[..rhs.poly.len()]
                .iter_mut()
                .enumerate()
                .for_each(|(index, elem)| *elem += rhs.poly[index]);
        }
    }
}

impl<'a, T: Copy + AddAssign> AddAssign<T> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    fn add_assign(&mut self, rhs: T) {
        self.poly[0] += rhs;
    }
}

impl<'b, T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq> Mul<&'b Polynomial<T>>
    for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: &'b Polynomial<T>) -> Self::Output {
        let mut new_poly: Polynomial<T> = Polynomial {
            poly: vec![Default::default(); self.deg as usize + rhs.deg as usize + 1],
            deg: self.poly.len() as u32 + rhs.poly.len() as u32 - 2_u32,
        };

        for first_index in 0..self.poly.len() {
            if self.poly[first_index] != Default::default() {
                for second_index in 0..rhs.poly.len() {
                    new_poly.poly[first_index + second_index] +=
                        self.poly[first_index] * rhs.poly[second_index];
                }
            }
        }

        new_poly
    }
}

impl<'b, T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq> Mul<T> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Polynomial {
            poly: self.poly.into_iter().map(|x| x * rhs).collect::<Vec<T>>(),
            deg: self.deg,
        }
    }
}

impl<'b, T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq> MulAssign<&'b Polynomial<T>>
    for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    fn mul_assign(&mut self, rhs: &'b Polynomial<T>) {
        let mut new_poly: Vec<T> =
            vec![Default::default(); self.deg as usize + rhs.deg as usize + 1];

        for first_index in 0..self.poly.len() {
            if self.poly[first_index] != Default::default() {
                for second_index in 0..rhs.poly.len() {
                    new_poly[first_index + second_index] +=
                        self.poly[first_index] * rhs.poly[second_index];
                }
            }
        }

        *self = Polynomial {
            poly: new_poly,
            deg: (self.poly.len() + rhs.poly.len() - 1) as u32,
        }
    }
}

impl<'b, T: Copy + SubAssign + Neg<Output = T>> SubAssign<&'b Polynomial<T>> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    fn sub_assign(&mut self, rhs: &'b Polynomial<T>) {
        if rhs.poly.len() > self.poly.len() {
            self.poly.extend(
                rhs.poly
                    .iter()
                    .skip(self.poly.len())
                    .map(|item| -> T { -*item }),
            );
        }
        if rhs.poly.len() < self.poly.len() {
            self.poly[..rhs.poly.len()]
                .iter_mut()
                .enumerate()
                .for_each(|(index, elem)| *elem -= rhs.poly[index]);
        }
    }
}

impl<'b, T: Copy + Sub<Output = T> + Neg<Output = T>> Sub<&'b Polynomial<T>> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn sub(self, other: &'b Polynomial<T>) -> Polynomial<T> {
        Polynomial {
            poly: self
                .poly
                .iter()
                .zip_longest(other.poly.iter())
                .map(|c| match c {
                    itertools::EitherOrBoth::Both(l, r) => *l - *r,
                    itertools::EitherOrBoth::Left(l) => *l,
                    itertools::EitherOrBoth::Right(r) => -(*r),
                })
                .collect::<Vec<T>>(),
            deg: if self.deg > other.deg {
                self.deg
            } else {
                other.deg
            },
        }
    }
}

impl<T: Copy + Sub<Output = T> + Neg<Output = T>> Sub for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn sub(self, other: Polynomial<T>) -> Polynomial<T> {
        Polynomial {
            poly: self
                .poly
                .iter()
                .zip_longest(other.poly.iter())
                .map(|c| match c {
                    itertools::EitherOrBoth::Both(l, r) => *l - *r,
                    itertools::EitherOrBoth::Left(l) => *l,
                    itertools::EitherOrBoth::Right(r) => -(*r),
                })
                .collect::<Vec<T>>(),
            deg: if self.deg > other.deg {
                self.deg
            } else {
                other.deg
            },
        }
    }
}

impl<T: Copy + Sub<Output = T> + Neg<Output = T> + SubAssign> Sub<T> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn sub(self, rhs: T) -> Polynomial<T> {
        let mut p = Polynomial {
            poly: self.poly.clone(),
            deg: self.deg,
        };
        p.poly[0] -= rhs.value_into().unwrap();
        p
    }
}


impl<T: Copy + Div<Output = T>> Div<T> for Polynomial<T>
where
    T: PolynomialOperationTypes,
{
    type Output = Polynomial<T>;
    fn div(self, rhs: T) -> Polynomial<T> {
        Polynomial {
            poly: self
                .poly
                .iter()
                .map(|c| *c / rhs)
                .collect::<Vec<T>>(),
            deg: self.deg,
        }
    }
}

macro_rules! define_commutative_operators {
    ($type: ident) => {
        impl<'b, T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq + Sized>
            Mul<&'b Polynomial<T>> for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn mul(self, rhs: &'b Polynomial<T>) -> Self::Output {
                Polynomial {
                    poly: rhs
                        .poly
                        .iter()
                        .map(|x| *x * (self.value_into().unwrap()))
                        .collect::<Vec<T>>(),
                    deg: rhs.deg,
                }
            }
        }

        impl<T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq + Sized> Mul<Polynomial<T>>
            for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn mul(self, rhs: Polynomial<T>) -> Self::Output {
                Polynomial {
                    poly: rhs
                        .poly
                        .into_iter()
                        .map(|x| x * (self.value_into().unwrap()))
                        .collect::<Vec<T>>(),
                    deg: rhs.deg,
                }
            }
        }

        /////////////////////////////////////////////////////////////////////////////

        impl<'b, T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq + Sized>
            Add<&'b Polynomial<T>> for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn add(self, rhs: &'b Polynomial<T>) -> Self::Output {
                let mut value = Polynomial {
                    poly: rhs.poly.iter().map(|x| *x).collect::<Vec<T>>(),
                    deg: rhs.deg,
                };
                value.poly[0] += (self.value_into().unwrap());
                value
            }
        }

        impl<T: Copy + Mul<Output = T> + AddAssign + Default + PartialEq + Sized> Add<Polynomial<T>>
            for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn add(self, rhs: Polynomial<T>) -> Self::Output {
                let mut value = Polynomial {
                    poly: rhs.poly.into_iter().map(|x| x).collect::<Vec<T>>(),
                    deg: rhs.deg,
                };
                value.poly[0] += self.value_into().unwrap();
                value
            }
        }

        ///////////////////////////////////////////////////////////////////////////////////////////////

        impl<'b, T: Copy + Mul<Output = T> + SubAssign + Default + PartialEq + Sized>
            Sub<&'b Polynomial<T>> for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn sub(self, rhs: &'b Polynomial<T>) -> Self::Output {
                let mut value = Polynomial {
                    poly: rhs.poly.iter().map(|x| *x).collect::<Vec<T>>(),
                    deg: rhs.deg,
                };
                value.poly[0] -= self.value_into().unwrap();
                value
            }
        }

        impl<T: Copy + Mul<Output = T> + SubAssign + Default + PartialEq + Sized> Sub<Polynomial<T>>
            for $type
        where
            T: PolynomialOperationTypes + ValueFrom<<T>::Output> + conv::ValueFrom<$type>,
        {
            type Output = Polynomial<T>;

            fn sub(self, rhs: Polynomial<T>) -> Self::Output {
                let mut value = Polynomial {
                    poly: rhs.poly.into_iter().map(|x| x).collect::<Vec<T>>(),
                    deg: rhs.deg,
                };
                value.poly[0] -= self.value_into().unwrap();
                value
            }
        }
    };
}

define_commutative_operators!(f64);
define_commutative_operators!(f32);
define_commutative_operators!(i128);
define_commutative_operators!(i64);
define_commutative_operators!(i32);
define_commutative_operators!(i16);
define_commutative_operators!(i8);
define_commutative_operators!(u64);
define_commutative_operators!(u32);
define_commutative_operators!(u16);
define_commutative_operators!(u8);
define_commutative_operators!(usize);
define_commutative_operators!(isize);
