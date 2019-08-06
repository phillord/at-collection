use failure::bail;
use failure::Error;
use std::convert::TryFrom;
use std::ops::Deref;

pub trait AtCollection<T>
where
    Self: Sized,
{
    fn as_slice(&self) -> &[T];

    fn as_vec(self) -> Vec<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.as_vec().into_iter()
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.as_slice().iter()
    }
}

macro_rules! atleast {
    ($name:ident, $n:literal, $($cons:ident),*) => {

        #[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name<T> {
            raw: Vec<T>,
        }

        impl<T> $name<T> {
            pub fn new(
                $($cons: T),*
            ) -> $name<T> {
                $name::new_and($($cons),*, Vec::new())
            }

            pub fn new_and($($cons: T),*, mut v: Vec<T>) -> $name<T> {
                let mut raw = vec![$($cons),*];
                raw.append(&mut v);
                $name { raw }
            }
        }

        impl<T> AtCollection<T> for $name<T> {
            fn as_slice(&self) -> &[T] {
                self.raw.as_slice()
            }

            fn as_vec(self) -> Vec<T> {
                self.raw
            }
        }
        impl<T> Deref for $name<T> {
            type Target = [T];

            fn deref(&self) -> &Self::Target {
                &self.as_slice()
            }
        }

        impl<T> TryFrom<Vec<T>> for $name<T> {
            type Error = Error;

            fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
                if value.len() < $n {
                    bail!("$name requires at least $n values")
                } else {
                    Ok($name { raw: value })
                }
            }
        }

    };
}

atleast! {AtLeastOne, 1, a}
atleast! {AtLeastTwo, 2, a, b}
atleast! {AtLeastThree, 3, a, b, c}
atleast! {AtLeastFour, 4, a, b, c, d}

impl<T> AtLeastOne<T> {
    pub fn one(&self) -> Result<&T, Error> {
        match self.raw.len() {
            1 => Ok(&self.raw[0]),
            _ => Err(format_err!("AtLeastOne has more than one element")),
        }
    }

    pub fn to_one(&self) -> &T {
        self.one().unwrap()
    }
}

#[macro_export]
macro_rules! atl1 {
    ($($cons:expr),*) => {
        AtLeastOne::try_from(vec![$($cons),*])
    }
}

#[macro_export]
macro_rules! atl2 {
    ($($cons:expr),*) => {
        $crate::AtLeastTwo::try_from(vec![$($cons),*])
    }
}

#[macro_export]
macro_rules! atl3 {
    ($($cons:expr),*) => {
        AtLeastThree::try_from(vec![$($cons),*])
    }
}

#[macro_export]
macro_rules! atl4 {
    ($($cons:expr),*) => {
        AtLeastFour::try_from(vec![$($cons),*])
    }
}

pub struct AtMostOne<T> {
    raw: Vec<T>,
}

impl<T> AtMostOne<T> {
    pub fn new(a: T) -> AtMostOne<T> {
        let raw = vec![a];
        AtMostOne { raw }
    }
}

impl<T> AtCollection<T> for AtMostOne<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

pub struct AtMostTwo<T> {
    raw: Vec<T>,
}

impl<T> AtMostTwo<T> {
    pub fn new(a: T) -> AtMostTwo<T> {
        let raw = vec![a];
        AtMostTwo { raw }
    }
}

impl<T> AtCollection<T> for AtMostTwo<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

pub struct AtMostThree<T> {
    raw: Vec<T>,
}

impl<T> AtMostThree<T> {
    pub fn new(a: T) -> AtMostThree<T> {
        let raw = vec![a];
        AtMostThree { raw }
    }
}

impl<T> AtCollection<T> for AtMostThree<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let alt = AtLeastTwo::new(1, 2);

        let mut i = alt.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
    }

    #[test]
    fn into_iter() {
        let alt = AtLeastTwo::new(1, 2);

        let mut i = alt.into_iter();
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
    }

    #[test]
    fn for_iter() {
        let alt = AtLeastOne::new(1);

        for i in alt.into_iter() {
            assert_eq!(i, 1);
        }
    }

    #[test]
    fn length() {
        let alt = AtLeastTwo::new(1, 2);

        assert_eq!(alt.len(), 2);
    }

    #[test]
    fn try_from() {
        assert!(AtLeastThree::try_from(vec![1, 2, 3]).is_ok());
        assert!(AtLeastThree::try_from(vec![1, 2]).is_err());

        assert!(AtLeastTwo::try_from(vec![1, 2, 3]).is_ok());
        assert!(AtLeastTwo::try_from(vec![1]).is_err());
    }

    #[test]
    fn at_least_one_macro() {
        let atl1 = atl1![1].unwrap();
        assert_eq!(1, atl1.len());

        let atl1 = atl1![1, 2, 3].unwrap();
        assert_eq!(3, atl1.len());
    }

    #[test]
    fn at_least_two_macro() {
        let atl2 = atl2![1];
        assert!(atl2.is_err());
    }

    #[test]
    fn at_least_two_expr() {
        let atl2 = atl2![1 + 2, 3 + 4];
        assert_eq!(atl2.unwrap().len(), 2);
    }

    #[test]
    fn to_one() {
        let ato1 = atl1![1].unwrap();
        let ato2 = atl1![1, 2].unwrap();

        assert!(ato1.one().is_ok());
        assert!(ato2.one().is_err());

        assert_eq!(ato1.to_one(), &1);
    }
}
