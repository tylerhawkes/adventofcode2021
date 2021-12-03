const WIDTH: usize = 12;
pub fn compute(input: &str) -> (usize, usize) {
  let mut ones = [0_u16; WIDTH];
  let mut count = 0;
  input.lines().for_each(|l| {
    count += 1;
    l.chars().enumerate().for_each(|(i, c)| {
      if c == '1' {
        ones[i] += 1;
      }
    })
  });
  let cutoff = count / 2;

  let mut gamma = 0_u16;
  for (i, &one) in ones.iter().rev().enumerate() {
    if one > cutoff {
      gamma |= 1 << i;
    }
  }
  dbg!(gamma);
  (
    gamma as usize * ((!gamma << (16 - WIDTH)) >> (16 - WIDTH)) as usize,
    compute_oxygen_generator_and_co2_scrubber(input),
  )
}

fn compute_oxygen_generator_and_co2_scrubber(input: &str) -> usize {
  let input = input
    .lines()
    .map(|l| {
      let mut x = 0_u16;
      for (i, c) in l.chars().enumerate() {
        if c == '1' {
          x |= 1 << (l.len() - i - 1)
        }
      }
      x
    })
    .collect::<Vec<_>>();

  let mut oxygen_generator = input.clone();
  for i in (0..WIDTH).rev() {
    let and_value = 1 << i;
    let total = oxygen_generator.len();
    let ones = oxygen_generator.iter().filter(|&&x| x & and_value > 0).count();
    // oxygen_generator.iter().for_each(|x|println!("{:12b}", x));
    // println!("i: {}, total: {}, ones: {}", i, total, ones);
    if ones >= total - ones {
      // println!("one");
      oxygen_generator = oxygen_generator.into_iter().filter(|x| *x & and_value > 0).collect();
    } else {
      // println!("zero");
      oxygen_generator = oxygen_generator.into_iter().filter(|x| *x & and_value == 0).collect();
    }
    if oxygen_generator.len() == 1 {
      break;
    }
  }

  let mut co2_scrubber = input.clone();
  for i in (0..WIDTH).rev() {
    let and_value = 1 << i;
    let total = co2_scrubber.len();
    let ones = co2_scrubber.iter().filter(|&&x| x & and_value > 0).count();
    // println!("i: {}, total: {}, ones: {}", i, total, ones);
    if ones < total - ones {
      co2_scrubber = co2_scrubber.into_iter().filter(|x| *x & and_value > 0).collect();
    } else {
      co2_scrubber = co2_scrubber.into_iter().filter(|x| *x & and_value == 0).collect();
    }
    if co2_scrubber.len() == 1 {
      break;
    }
  }

  println!(
    "oxygen_generator: {:12b}, co2_scrubber: {:12b}",
    oxygen_generator[0], co2_scrubber[0]
  );
  println!("oxygen_generator: {:?}, co2_scrubber: {:?}", oxygen_generator, co2_scrubber);
  oxygen_generator[0] as usize * co2_scrubber[0] as usize
}
