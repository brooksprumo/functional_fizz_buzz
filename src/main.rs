//! Based on https://youtu.be/rCKPgu4DvcE

type Number = u32;
type Label = String;

#[derive(Debug, Clone, Eq, PartialEq)]
enum FizzBuzzResult {
    Unhandled(Number),
    Handled(Label),
}

impl FizzBuzzResult {
    fn if_unhandled_do(self, f: impl Fn(Number) -> Self) -> Self {
        match self {
            FizzBuzzResult::Handled(_) => self,
            FizzBuzzResult::Unhandled(x) => f(x),
        }
    }

    fn last_step(self) -> Label {
        match self {
            FizzBuzzResult::Unhandled(x) => x.to_string(),
            FizzBuzzResult::Handled(s) => s,
        }
    }
}

fn is_divisible_by(x: Number, div: Number) -> bool {
    x % div == 0
}

fn handle(x: Number, div: Number, label: Label) -> FizzBuzzResult {
    if is_divisible_by(x, div) {
        FizzBuzzResult::Handled(label)
    } else {
        FizzBuzzResult::Unhandled(x)
    }
}

fn fizz_buzz(x: Number) -> (Number, Label) {
    let handle_3 = |x| handle(x, 3, "Fizz".to_string());
    let handle_5 = |x| handle(x, 5, "Buzz".to_string());
    let handle_15 = |x| handle(x, 15, "FizzBuzz".to_string());

    let s = handle_15(x)
        .if_unhandled_do(handle_5)
        .if_unhandled_do(handle_3)
        .last_step();

    (x, s)
}

fn main() {
    (1..100).map(fizz_buzz).for_each(|(x, s)| {
        println!("{}: {}", x, s);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by() {
        assert!(is_divisible_by(30, 15));
        assert!(is_divisible_by(30, 5));
        assert!(is_divisible_by(30, 3));

        assert!(!is_divisible_by(97, 15));
        assert!(!is_divisible_by(97, 5));
        assert!(!is_divisible_by(97, 3));
    }

    #[test]
    fn test_handle() {
        let label = String::default();
        assert_eq!(
            handle(45, 15, label.clone()),
            FizzBuzzResult::Handled(label.clone())
        );
        assert_eq!(handle(66, 17, label), FizzBuzzResult::Unhandled(66));
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(3), (3, "Fizz".to_string()));
        assert_eq!(fizz_buzz(5), (5, "Buzz".to_string()));
        assert_eq!(fizz_buzz(15), (15, "FizzBuzz".to_string()));
        assert_eq!(fizz_buzz(22), (22, "22".to_string()));
    }
}
