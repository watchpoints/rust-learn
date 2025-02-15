// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).
// TODO: Add the `AsRef` trait appropriately as a trait bound.
use std::ops::Mul;
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
    //trait 定义了一种行为，任何实现该 trait 的类型都必须提供该行为。
    //AsRef<T> 是 Rust 标准库中的一个 trait，用于将一个类型转换为对 T 的引用
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<U>, U: Mul<Output = U> + Copy>(arg: &mut T) {
    let value = arg.as_mut();
    *value = (*value) * (*value);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
