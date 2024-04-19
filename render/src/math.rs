use std::ops::Div;
use std::ops::Sub;

pub fn slope<T>(y2: T, y1: T, height: T) -> T
    where T: Sub<Output=T> + Div<Output=T>, <T as Sub>::Output: Div<T>
{
    (y2 - y1) / height
}
