use std::ops::RangeInclusive;

//target area: x=29..73, y=-248..-194
const X_RANGE: RangeInclusive<isize> = 29..=73;
const Y_RANGE: RangeInclusive<isize> = -248..=-194;
pub fn compute(_input: &str) -> (usize, usize) {
  let mut max_height = 0;
  let mut in_target_area = 0;
  for x in 7..=73 {
    for y in -248..=248 {
      let mut max_test_height = 0;
      let mut trajectory = (x, y);
      let mut position = (0, 0);
      let mut was_in_target_area = false;
      while !out_of_bounds(position) {
        position = (position.0 + trajectory.0, position.1 + trajectory.1);
        max_test_height = max_test_height.max(position.1);
        if trajectory.0 > 0 {
          trajectory.0 -= 1;
        }
        trajectory.1 -= 1;
        if is_within_target_area(position) {
          was_in_target_area = true;
        }
      }
      in_target_area += was_in_target_area as usize;
      if was_in_target_area && max_test_height > max_height {
        max_height = max_test_height;
      }
    }
  }
  (max_height as usize, in_target_area)
}

fn is_within_target_area((x, y): (isize, isize)) -> bool {
  X_RANGE.contains(&x) && Y_RANGE.contains(&y)
}

fn out_of_bounds((x, y): (isize, isize)) -> bool {
  x > *X_RANGE.end() || y < *Y_RANGE.start()
}
