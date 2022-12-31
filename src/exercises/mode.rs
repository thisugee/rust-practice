use std::collections::HashMap;

pub fn mode(numbers: Vec<i32>) -> i32 {
    let mut times: HashMap<i32, i32> = HashMap::new();

    for i in numbers {
        let count = times.entry(i).or_insert(0);
        *count += 1
    }

    let mut max: Option<i32> = None;
    for (key, value) in &times {
        match max {
            Some(max_key) => match times.get(&max_key) {
                Some(max_value) => {
                    if value > max_value {
                        max = Some(*key);
                    }
                }
                _ => (),
            },
            _ => max = Some(*key),
        }
    }

    match max {
        Some(value) => value,
        _ => 0,
    }
}

#[test]
fn mode_test() {
    assert_eq!(3, mode(vec![1, 3, 3, 6, 9, 26, 72]));
}
