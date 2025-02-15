struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以在这里实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 测试创建正确尺寸的 Rectangle
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);  // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width() {
        // 测试负数宽度导致 panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_height() {
        // 测试负数高度导致 panic
        let _rect = Rectangle::new(10, -10);
    }

    // 新增：同时为负数的情况
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width_and_height() {
        let _rect = Rectangle::new(-10, -10);
    }

    // 新增：零宽度的情况
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn zero_width() {
        let _rect = Rectangle::new(0, 10);
    }

    // 新增：零高度的情况
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn zero_height() {
        let _rect = Rectangle::new(10, 0);
    }

    // 新增：宽度和高度都为零
    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn zero_width_and_height() {
        let _rect = Rectangle::new(0, 0);
    }
}
