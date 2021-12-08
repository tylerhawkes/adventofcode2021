use std::convert::TryFrom;

pub fn compute(input: &str) -> (usize, usize) {
  let notes = input
    .lines()
    .map(|s| {
      let mut split = s.split(" | ");
      let lights = split.next().unwrap().split_whitespace().collect::<Vec<_>>();
      let output = split.next().unwrap().split_whitespace().collect::<Vec<_>>();
      (
        <[_; 10]>::try_from(lights).unwrap().map(string_to_u8),
        <[_; 4]>::try_from(output).unwrap().map(string_to_u8),
      )
    })
    .collect::<Vec<([u8; 10], [u8; 4])>>();

  let occurrences = notes
    .iter()
    .map(|(_, output)| {
      output
        .iter()
        .filter(|x| match x.count_ones() {
          2 | 3 | 4 | 7 => true,
          _ => false,
        })
        .count()
    })
    .sum::<usize>();

  let sum = notes.iter().copied().map(lights_to_output).sum::<usize>();
  (occurrences, sum)
}

fn string_to_u8(s: &str) -> u8 {
  s.chars().fold(0, |u, c| u | 1 << (c as u8 - b'a'))
}

//  aaaa   0000
// b    c 1    2
// b    c 1    2
//  dddd   3333
// e    f 4    5
// e    f 4    5
//  gggg   6666

fn lights_to_output((lights, output): ([u8; 10], [u8; 4])) -> usize {
  let mut light = [0x7f_u8; 7];
  let mut light_mapping = [u8::MAX; 10];
  let mut iters = 0;
  // println!("new note");
  while light.into_iter().any(|x| x.count_ones() > 1) && iters < 4 {
    iters += 1;
    for l in lights {
      match l.count_ones() {
        2 => {
          light_mapping[1] = l;
          light[2] &= l;
          light[5] &= l;
        }
        3 => {
          light_mapping[7] = l;
          light[0] &= l;
          light[2] &= l;
          light[5] &= l;
        }
        4 => {
          light_mapping[4] = l;
          light[1] &= l;
          light[2] &= l;
          light[3] &= l;
          light[5] &= l;
        }
        5 => {
          // 2, 3, 5 have these in common so they can be set
          light[0] &= l;
          light[3] &= l;
          light[6] &= l;
          if light[2].count_ones() <= 2 && light[5].count_ones() <= 2 {
            if l & light[2] == light[2] && l & light[5] == light[5] {
              light_mapping[3] = l;
            } else if (l & light_mapping[4]).count_ones() == 3 {
              light_mapping[5] = l;
            } else if (l & light_mapping[4]).count_ones() == 2 {
              light_mapping[2] = l;
            }
          }
        }
        6 => {
          light[0] &= l;
          light[1] &= l;
          light[5] &= l;
          light[6] &= l;
          if light[2].count_ones() <= 2 && light[5].count_ones() <= 2 {
            if l & light[2] == light[2] && l & light[5] == light[5] {
              // light is 9 or 0
            } else {
              // light is a 6
              light[3] &= l;
              light[4] &= l;
            }
          }
        }
        7 => light_mapping[8] = l,
        _ => unreachable!(),
      }
    }
    // dbg!(light);
    for (i, l) in light.into_iter().enumerate() {
      if l.count_ones() == 1 {
        for (j, x) in light.iter_mut().enumerate() {
          if i != j {
            *x = *x & !l;
          }
        }
      }
    }
    // dbg!(light_mapping);
    // dbg!(&light);
  }
  if iters == 4 {
    panic!();
  }
  light_mapping[0] = light[0] | light[1] | light[2] | light[4] | light[5] | light[6];
  light_mapping[1] = light[2] | light[5];
  light_mapping[2] = light[0] | light[2] | light[3] | light[4] | light[6];
  light_mapping[3] = light[0] | light[2] | light[3] | light[5] | light[6];
  light_mapping[4] = light[1] | light[2] | light[3] | light[5];
  light_mapping[5] = light[0] | light[1] | light[3] | light[5] | light[6];
  light_mapping[6] = light[0] | light[1] | light[3] | light[4] | light[5] | light[6];
  light_mapping[7] = light[0] | light[2] | light[5];
  light_mapping[9] = light[0] | light[1] | light[2] | light[3] | light[5] | light[6];

  output
    .into_iter()
    .rev()
    .enumerate()
    .map(|(c, o)| light_mapping.iter().position(|l| *l == o).unwrap() * 10_usize.pow(c as u32))
    .sum()
}
