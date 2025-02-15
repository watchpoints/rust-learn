use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

// 用迭代器实现 `count_for`
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&&v| v == value).count()
}

// 用迭代器实现 `count_collection_for`
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().flat_map(|map| map.values()).filter(|&&v| v == value).count()
}

fn main() {
    let mut map = HashMap::new();
    map.insert("exercise1".to_string(), Progress::Complete);
    map.insert("exercise2".to_string(), Progress::None);
    
    println!("{}", count_iterator(&map, Progress::Complete)); // 输出: 1
}
