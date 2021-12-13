use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn compute(input: &str) -> (usize, usize) {
  let mut caves = HashMap::new();
  for l in input.lines() {
    let link = l.split('-').collect::<Vec<_>>();
    assert_eq!(2, link.len());
    match caves.entry(link[0].to_string()) {
      Entry::Vacant(v) => {
        v.insert(vec![link[1].to_string()]);
      }
      Entry::Occupied(o) => {
        o.into_mut().push(link[1].to_string());
      }
    }
    match caves.entry(link[1].to_string()) {
      Entry::Vacant(v) => {
        v.insert(vec![link[0].to_string()]);
      }
      Entry::Occupied(o) => {
        o.into_mut().push(link[0].to_string());
      }
    }
  }
  let paths_1 = step_1("start", &caves, vec!["start".into()]);
  let paths_2 = step_2("start", &caves, vec!["start".into()]);
  // dbg!(&paths_2);
  (paths_1.len(), paths_2.len())
}

fn step_1(
  cavern: &str,
  caves: &HashMap<String, Vec<String>>,
  path: Vec<String>,
) -> Vec<Vec<String>> {
  let mut paths = vec![];
  for c in caves.get(cavern).unwrap() {
    if c == "end" {
      paths.push({
        let mut end = path.clone();
        end.push(c.clone());
        end
      });
      continue;
    }
    if path
      .iter()
      .any(|p| p == c && p.chars().next().unwrap().is_ascii_lowercase())
    {
      continue;
    }
    let mut clone = path.clone();
    clone.push(c.to_string());
    paths.extend_from_slice(&step_1(c, caves, clone));
  }
  paths
}

fn step_2(
  cavern: &str,
  caves: &HashMap<String, Vec<String>>,
  path: Vec<String>,
) -> Vec<Vec<String>> {
  // println!("cavern: {}, prev_paths: {:?}", cavern, path);
  let mut paths = vec![];
  for c in caves.get(cavern).unwrap() {
    if c == "end" {
      paths.push({
        let mut end = path.clone();
        end.push(c.clone());
        end
      });
      continue;
    }
    if c == "start" {
      continue;
    }
    if !can_visit_again(c, &path) {
      continue;
    }
    let mut clone = path.clone();
    clone.push(c.to_string());
    paths.extend_from_slice(&step_2(c, caves, clone));
  }
  paths
}

fn can_visit_again(cave: &str, caves: &[String]) -> bool {
  // println!("cave: {}, caves: {:?}", cave, caves);
  let mut set = HashSet::new();
  let mut two = false;
  for cave in caves {
    if cave.chars().next().unwrap().is_ascii_lowercase() {
      if !set.insert(cave.as_str()) {
        if two {
          return false;
        }
        two = true;
      }
    }
  }
  if set.contains(cave) {
    if two {
      return false;
    }
  }
  true
}
