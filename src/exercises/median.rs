pub fn median(mut numbers: Vec<i32>) -> i32 {
    let mut median = 0;
    numbers.sort();
    let size = numbers.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = numbers.get((even / 2) - 1);
            let snd_med = numbers.get(even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => median = (fst + snd) / 2,
                _ => (),
            }
        }
        odd => {
            let med = numbers.get(odd / 2);
            match med {
                Some(med) => median = med / 1,
                _ => (),
            }
        }
    }

    median
}

#[test]
fn median_test() {
    assert_eq!(6, median(vec![1, 3, 3, 6, 9, 26, 72]));
}
