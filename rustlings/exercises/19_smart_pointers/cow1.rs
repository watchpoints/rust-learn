// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
fn reference_no_mutation() {
    let vec = vec![0, 1, 2];
    let mut input = Cow::from(&vec);
    abs_all(&mut input);
    assert!(matches!(input, Cow::Borrowed(_)));  // ✅ 未修改，仍然是 Borrowed
}

#[test]
fn owned_no_mutation() {
    let vec = vec![0, 1, 2];
    let mut input = Cow::from(vec);
    abs_all(&mut input);
    assert!(matches!(input, Cow::Owned(_)));  // ✅ 已拥有，不变
}

#[test]
fn owned_mutation() {
    let vec = vec![-1, 0, 1];
    let mut input = Cow::from(vec);
    abs_all(&mut input);
    assert!(matches!(input, Cow::Owned(_)));  // ✅ 需要修改，仍然是 Owned
}

}
