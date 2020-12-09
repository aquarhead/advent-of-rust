use std::collections::VecDeque;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/9.txt").unwrap();
  let numbers = content.lines().map(|x| x.parse::<i64>().unwrap());

  {
    // part 1
    let mut validators = VecDeque::with_capacity(25);
    for n in numbers {
      if validators.len() < 25 {
        validators.push_back(n);
      } else {
        if xmas_validate(&validators, n) {
          validators.pop_front();
          validators.push_back(n);
        } else {
          println!("First invalid number: {}", n);
          break;
        }
      }
    }
  }
}

fn xmas_validate(validators: &VecDeque<i64>, num: i64) -> bool {
  validators.iter().enumerate().any(|(idx, val)| {
    let other = num - val;
    validators.iter().position(|x| *x == other).map_or(false, |p| p != idx)
      || validators.iter().rposition(|x| *x == other).map_or(false, |p| p != idx)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::VecDeque;

  #[test]
  fn test1() {
    let mut validators = VecDeque::new();
    validators.push_back(1);
    validators.push_back(2);
    validators.push_back(3);
    validators.push_back(4);
    validators.push_back(5);

    assert_eq!(true, xmas_validate(&validators, 9));
    assert_eq!(false, xmas_validate(&validators, 10));
  }
}
