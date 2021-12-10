#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
  Open(Brace),
  Close(Brace),
}

impl Direction {
  fn opposite(self) -> Self {
    match self {
      Self::Open(b) => Self::Close(b),
      Self::Close(b) => Self::Open(b),
    }
  }
  fn inner(self) -> Brace {
    match self {
      Self::Open(b) | Self::Close(b) => b,
    }
  }
}

impl From<char> for Direction {
  fn from(c: char) -> Self {
    match c {
      '{' => Self::Open(Brace::Curly),
      '}' => Self::Close(Brace::Curly),
      '(' => Self::Open(Brace::Paren),
      ')' => Self::Close(Brace::Paren),
      '[' => Self::Open(Brace::Square),
      ']' => Self::Close(Brace::Square),
      '<' => Self::Open(Brace::Angle),
      '>' => Self::Close(Brace::Angle),
      _ => unreachable!(),
    }
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Brace {
  Curly,
  Angle,
  Square,
  Paren,
}

impl Brace {
  fn syntax_error_score(self) -> usize {
    match self {
      Self::Curly => 1197,
      Self::Angle => 25137,
      Self::Square => 57,
      Self::Paren => 3,
    }
  }
  fn completion_score(self) -> usize {
    match self {
      Self::Curly => 3,
      Self::Angle => 4,
      Self::Square => 2,
      Self::Paren => 1,
    }
  }
}

pub fn compute(input: &str) -> (usize, usize) {
  let mut syntax_error_score = 0;
  let mut incomplete = Vec::new();
  'outer: for l in input.lines() {
    let mut order = Vec::new();
    for d in l.chars().map(Direction::from) {
      match d {
        Direction::Open(_) => order.push(d),
        Direction::Close(b) => {
          let last = order.pop().unwrap();
          if last.opposite() != d {
            // corrupted
            syntax_error_score += b.syntax_error_score();
            continue 'outer;
          }
        }
      }
    }
    let completion_score = order
      .iter()
      .rev()
      .copied()
      .fold(0, |s, d| s * 5 + d.inner().completion_score());
    incomplete.push(completion_score)
  }
  incomplete.sort();
  (syntax_error_score, incomplete[incomplete.len() / 2])
}
