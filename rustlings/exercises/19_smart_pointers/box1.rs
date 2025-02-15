#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>), // 使用 `Box` 解决递归问题
    Nil,
}

// 创建一个空的 `List`
fn create_empty_list() -> List {
    List::Nil
}

// 创建一个非空的 `List`
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!("This is a non-empty cons list: {:?}", create_non_empty_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
