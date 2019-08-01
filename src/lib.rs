trait AtCollection<T>: IntoIterator
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

pub struct AtLeastOne<T> {
    raw: Vec<T>,
}

impl<T> AtLeastOne<T> {
    pub fn new(a: T) -> AtLeastOne<T> {
        AtLeastOne::new_and(a, Vec::new())
    }

    pub fn new_and(a: T, mut v: Vec<T>) -> AtLeastOne<T> {
        v.insert(0, a);
        AtLeastOne { raw: v }
    }
}

impl<T> AtCollection<T> for AtLeastOne<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

pub struct AtLeastTwo<T> {
    raw: Vec<T>,
}

impl<T> AtLeastTwo<T> {
    pub fn new(a: T, b: T) -> AtLeastTwo<T> {
        AtLeastTwo::new_and(a, b, Vec::new())
    }

    pub fn new_and(a: T, b: T, mut v: Vec<T>) -> AtLeastTwo<T> {
        v.insert(0, a);
        v.insert(1, b);
        AtLeastTwo { raw: v }
    }
}

impl<T> AtCollection<T> for AtLeastTwo<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

pub struct AtLeastThree<T> {
    raw: Vec<T>,
}

impl<T> AtLeastThree<T> {
    pub fn new(a: T, b: T) -> AtLeastThree<T> {
        AtLeastThree::new_and(a, b, Vec::new())
    }

    pub fn new_and(a: T, b: T, mut v: Vec<T>) -> AtLeastThree<T> {
        v.insert(0, a);
        v.insert(1, b);
        AtLeastThree { raw: v }
    }
}

impl<T> AtCollection<T> for AtLeastThree<T> {
    fn as_slice(&self) -> &[T] {
        self.raw.as_slice()
    }

    fn as_vec(self) -> Vec<T> {
        self.raw
    }
}

pub struct AtMostOne<T> {
    raw: Vec<T>,
}

impl<T> AtMostOne<T> {
    pub fn new(a: T) -> AtMostOne {
        let raw = !vec[a];
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
    pub fn new(a: T) -> AtMostTwo {
        let raw = !vec[a];
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
    pub fn new(a: T) -> AtMostThree {
        let raw = !vec[a];
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

        for i in alt {
            assert_eq!(i, 1);
        }
    }

}
