use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn compute(input: &str) -> (usize, usize) {
  let mut numbers = input
    .lines()
    .map(|l| l.parse::<SnailNumber>().unwrap())
    .collect::<Vec<_>>();
  let mut max_magnitude = 0;
  for x in &numbers {
    for y in &numbers {
      let magnitude = x.clone().add(y.clone()).magnitude();
      if magnitude > max_magnitude {
        max_magnitude = magnitude;
      }
    }
  }
  let start = numbers.remove(0);
  let f = numbers.into_iter().fold(start, |l, r| l.add(r));

  // println!("{}", &f);
  (f.magnitude(), max_magnitude)
}

#[derive(Debug, Clone)]
enum SnailNumber {
  Scalar(u8),
  Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl Display for SnailNumber {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Scalar(u) => write!(f, "{}", u),
      Self::Pair(l, r) => {
        write!(f, "[")?;
        l.fmt(f)?;
        write!(f, ",")?;
        r.fmt(f)?;
        write!(f, "]")
      }
    }
  }
}

impl FromStr for SnailNumber {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    Ok(Self::parse(&mut chars))
  }
}

impl SnailNumber {
  fn parse(c: &mut impl Iterator<Item = char>) -> Self {
    let left = match c.next().unwrap() {
      '[' => Self::parse(c),
      c if c.is_digit(10) => Self::Scalar(c as u8 - b'0'),
      _ => unreachable!(),
    };
    match c.next() {
      Some(c) => assert_eq!(c, ','),
      None => return left,
    }
    let right = match c.next().unwrap() {
      '[' => Self::parse(c),
      c if c.is_digit(10) => Self::Scalar(c as u8 - b'0'),
      _ => unreachable!(),
    };
    assert_eq!(']', c.next().unwrap());
    Self::Pair(Box::new(left), Box::new(right))
  }
  // None means success
  fn try_add_left(&mut self, v: u8) -> Option<u8> {
    match self {
      Self::Scalar(x) => {
        *x += v;
        None
      }
      Self::Pair(l, _) => l.try_add_left(v),
    }
  }
  fn try_add_right(&mut self, v: u8) -> Option<u8> {
    match self {
      Self::Scalar(x) => {
        *x += v;
        None
      }
      Self::Pair(_, r) => r.try_add_right(v),
    }
  }
  fn explode(&mut self, level: u8) -> (bool, Option<u8>, Option<u8>) {
    match self {
      Self::Pair(l, r) if level > 4 => {
        // match &mut **l {
        //   Self::Scalar(_) => {}
        //   Self::Pair(l, r) => match (&mut **l, &mut **r) {
        //     (SnailNumber::Scalar(x1), SnailNumber::Scalar(x2)) => {
        //       return (true, Some(*x1), Some(*x2))
        //     }
        //     (l, r) => return l.try_explode(r, level),
        //   },
        // }
        // match &mut **r {
        //   Self::Scalar(_) => {}
        //   Self::Pair(l, r) => match (&mut **l, &mut **r) {
        //     (SnailNumber::Scalar(x1), SnailNumber::Scalar(x2)) => {
        //       return (true, Some(*x1), Some(*x2))
        //     }
        //     (l, r) => return l.try_explode(r, level),
        //   },
        // }
        match (&mut **l, &mut **r) {
          (Self::Scalar(x1), Self::Scalar(x2)) => (true, Some(*x1), Some(*x2)),
          (l, r) => l.try_explode(r, level + 1),
        }
      }
      Self::Pair(l, r) => return l.try_explode(r, level),
      Self::Scalar(_) => return (false, None, None),
    }
  }
  fn try_explode(&mut self, right: &mut Self, level: u8) -> (bool, Option<u8>, Option<u8>) {
    match self.explode(level + 1) {
      (true, Some(x1), Some(x2)) => {
        *self = Self::Scalar(0);
        return (true, Some(x1), right.try_add_left(x2));
      }
      (true, None, Some(x2)) => return (true, None, right.try_add_left(x2)),
      (true, Some(x1), None) => return (true, Some(x1), None),
      (true, None, None) => return (true, None, None),
      (false, None, None) => {}
      _ => unreachable!(),
    }
    match right.explode(level + 1) {
      (true, Some(x1), Some(x2)) => {
        *right = Self::Scalar(0);
        return (true, self.try_add_right(x1), Some(x2));
      }
      (true, None, Some(x2)) => return (true, None, Some(x2)),
      (true, Some(x1), None) => return (true, self.try_add_right(x1), None),
      (true, None, None) => return (true, None, None),
      (false, None, None) => {}
      _ => unreachable!(),
    }
    (false, None, None)
  }
  fn split(&mut self) -> bool {
    match self {
      Self::Scalar(v) => {
        let v = *v;
        if v > 9 {
          *self = Self::Pair(
            Box::new(Self::Scalar(v / 2)),
            Box::new(Self::Scalar((v / 2) + (v & 1))),
          );
          true
        } else {
          false
        }
      }
      Self::Pair(l, r) => {
        if l.split() {
          return true;
        }
        r.split()
      }
    }
  }
  fn add(self, r: Self) -> Self {
    let mut new = Self::Pair(Box::new(self), Box::new(r));
    let mut c = true;
    // println!("adding   {}", new);
    while c {
      let (explode, _, _) = new.explode(1);
      if explode {
        c = true;
        // println!("exploded {}", new);
        continue;
      }
      c = new.split();
      if c {
        // println!("split    {}", new);
      }
    }
    // println!("added    {}", new);
    new
  }
  fn magnitude(&self) -> usize {
    match self {
      Self::Scalar(u) => *u as usize,
      Self::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
    }
  }
}
