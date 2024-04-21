use std::ops::Div;
use std::ops::Sub;

pub fn slope<T>(y2: T, y1: T, height: T) -> T
    where T: Sub<Output=T> + Div<Output=T>, <T as Sub>::Output: Div<T>
{
    (y2 - y1) / height
}

#[cfg(test)]
mod tests {
    #[test]
    fn slope() {
        let r = super::slope(10, 6, 2);
        assert_eq!(2, r);

        let r = super::slope(12.0, 2.0, 2.0);
        assert_eq!(5.0, r);
    }
}
