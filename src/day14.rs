use std::collections::HashMap;

pub fn compute(input: &str) -> (usize, usize) {
  let mut split = input.split("\n\n");
  let polymers_orig = split.next().unwrap().chars().collect::<Vec<_>>();
  let template = split
    .next()
    .unwrap()
    .lines()
    .map(|l| {
      let s = l.split(" -> ").collect::<Vec<_>>();
      let c = s[0].chars().collect::<Vec<_>>();
      ((c[0], c[1]), s[1].chars().next().unwrap())
    })
    .collect::<HashMap<_, _>>();

  let mut polymers = polymers_orig.clone();
  for _ in 0..10 {
    let last = polymers.last().copied().unwrap();
    polymers = polymers
      .windows(2)
      .flat_map(|w| [w[0], template.get(&(w[0], w[1])).copied().unwrap()])
      .collect();
    polymers.push(last);
  }
  let mut counts = HashMap::new();
  polymers.iter().for_each(|c| {
    *counts.entry(c).or_insert(0_usize) += 1;
  });
  let mut counts = counts.into_iter().collect::<Vec<_>>();
  counts.sort_unstable_by_key(|x| x.1);
  // dbg!(&counts);
  let mut polymers = HashMap::new();
  for w in polymers_orig.windows(2) {
    *polymers.entry((w[0], w[1])).or_insert(0_usize) += 1;
  }

  for _ in 0..40 {
    let mut new_polymers = HashMap::with_capacity(polymers.len() * 2);
    for (&p, &c) in polymers.iter() {
      let n = template.get(&p).copied().unwrap();
      *new_polymers.entry((p.0, n)).or_insert(0_usize) += c;
      *new_polymers.entry((n, p.1)).or_insert(0_usize) += c;
    }
    polymers = new_polymers;
  }

  let mut counts_2 = HashMap::new();
  polymers.iter().for_each(|(&(l, r), &c)| {
    *counts_2.entry(l).or_insert(0_usize) += c;
    *counts_2.entry(r).or_insert(0_usize) += c;
  });
  *counts_2.get_mut(polymers_orig.first().unwrap()).unwrap() += 1;
  *counts_2.get_mut(polymers_orig.last().unwrap()).unwrap() += 1;
  let mut counts_2 = counts_2
    .into_iter()
    .map(|c| (c.0, c.1 / 2))
    .collect::<Vec<_>>();
  counts_2.sort_unstable_by_key(|x| x.1);
  // dbg!(&counts_2);
  (
    counts.last().unwrap().1 - counts[0].1,
    counts_2.last().unwrap().1 - counts_2[0].1,
  )
}
