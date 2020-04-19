#[allow(dead_code)]
fn fizzbuzz_step(i: usize) -> String {
    match (i % 3, i % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (_, 0) => String::from("buzz"),
        (0, _) => String::from("fizz"),
        (_, _) => format!("{}", i),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_step_returns_one() {
        assert_eq!(fizzbuzz_step(1), "1");
    }

    #[test]
    fn fizzbuzz_step_for_multiple_of_3() {
        for x in (3..=12).step_by(3) {
            assert_eq!(fizzbuzz_step(x), "fizz");
        }
    }

    #[test]
    fn fizbuzz_step_formultiple_of_5() {
        assert_eq!(fizzbuzz_step(5), "buzz");
        assert_eq!(fizzbuzz_step(10), "buzz");
    }

    #[test]
    fn fizzbuzz_step_for_both_3_and_5() {
        assert_eq!(fizzbuzz_step(15), "fizzbuzz");
    }
}
