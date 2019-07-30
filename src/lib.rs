pub struct AtLeastTwo<T> {
    raw: Vec<T>
}

impl <T> AtLeastTwo<T> {
    pub fn new(a: T, b: T) -> AtLeastTwo<T> {
        AtLeastTwo::new_and(a, b, Vec::new())
    }

    pub fn new_and(a:T, b: T, mut v:Vec<T>) -> AtLeastTwo<T> {
        v.insert(0, a);
        v.insert(1, b);
        AtLeastTwo {
            raw: v
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.raw.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.raw.into_iter()
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


}
