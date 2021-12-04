use std::str::FromStr;

#[derive(Debug)]
struct BingoBoard {
  // x is inner, y is outer
  // This means that rows are together
  values: [[u8; 5]; 5],
  // bitmap of filled bits
  filled: u32,
}

impl FromStr for BingoBoard {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut values = [[u8::MAX; 5]; 5];
    s.lines().enumerate().for_each(|(row, l)| {
      for (i, n) in l.split_whitespace().enumerate() {
        values[row][i] = n.parse().unwrap();
      }
    });
    Ok(Self { values, filled: 0 })
  }
}

impl BingoBoard {
  const FILLED_ROW: u32 = 0b11111;
  const FILLED_COLUMN: u32 = 0b100001000010000100001;
  fn is_complete(&self) -> bool {
    for i in 0..5 {
      let row = Self::FILLED_ROW << (i * 5);
      let column = Self::FILLED_COLUMN << i;
      if self.filled & row == row || self.filled & column == column {
        return true;
      }
    }
    false
  }
  fn values(&self) -> impl Iterator<Item = u8> + 'static {
    self
      .values
      .clone()
      .into_iter()
      .flat_map(|row| row.into_iter())
  }
  fn unfilled_values(&self) -> impl Iterator<Item = u8> + '_ {
    self
      .values()
      .enumerate()
      .filter_map(|(i, v)| (self.filled & 1 << i == 0).then(|| v))
  }
  fn fill_value(&mut self, value: u8) {
    for (i, v) in self.values().enumerate() {
      if v == value {
        self.filled |= 1 << i;
      }
    }
  }
}

pub fn compute(input: &str) -> (usize, usize) {
  let mut boards = input.split("\n\n");
  let random_numbers = boards.next().unwrap();
  let random_numbers = random_numbers
    .split(',')
    .map(|s| s.parse::<u8>().unwrap())
    .collect::<Vec<_>>();
  let mut boards = boards
    .map(BingoBoard::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
  let mut first_score = 0;
  let mut last_score = 0;
  for n in random_numbers.iter().copied() {
    boards.iter_mut().for_each(|b| b.fill_value(n));
    if first_score == 0 {
      if let Some(b) = boards.iter().find(|b| b.is_complete()) {
        first_score = b.unfilled_values().map(|u| u as usize).sum::<usize>() * n as usize;
      }
    }
    if boards.len() == 1 && boards[0].is_complete() {
      last_score = boards[0]
        .unfilled_values()
        .map(|u| u as usize)
        .sum::<usize>()
        * n as usize;
      break;
    }
    boards = boards.into_iter().filter(|b| !b.is_complete()).collect();
  }
  (first_score, last_score)
}
