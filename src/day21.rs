use std::collections::HashMap;

pub fn compute(_input: &str) -> (usize, usize) {
  let positions = [1 - 1, 10 - 1];
  let mut game_state = GameState::<1000> {
    scores: [0; 2],
    positions,
  };
  let mut die = 0;
  let mut rolls = 0;
  for i in (0..2).cycle() {
    let mut moves: usize = 0;
    for _ in 0..3 {
      die += 1;
      if die > 100 {
        die -= 100;
      }
      moves += die;
      rolls += 1;
    }
    if game_state.step(i, moves) {
      break;
    }
    println!(
      "player: {}, position: {}, score: {}, moves: {}, rolls: {}",
      i + 1,
      game_state.positions[i] + 1,
      game_state.scores[i],
      moves,
      rolls,
    );
  }
  // 428736
  (
    rolls * game_state.scores[0].min(game_state.scores[1]),
    part_two(positions),
  )
}

fn part_two(positions: [usize; 2]) -> usize {
  let probs = [(3, 1_usize), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
  let mut player_wins = [0_usize; 2];
  let mut player_universe_scores = HashMap::new();
  for (sum, count) in probs {
    let mut game_state = GameState::<21> {
      scores: [0, 0],
      positions,
    };
    game_state.step(0, sum);
    player_universe_scores.insert(game_state, count);
  }
  for i in (0..2).cycle().skip(1) {
    if player_universe_scores.is_empty() {
      break;
    }
    let mut new = HashMap::with_capacity(player_universe_scores.len() * 2);
    std::mem::swap(&mut new, &mut player_universe_scores);
    for (game_state, count_1) in new {
      for (sum, count_2) in probs {
        let mut gs = game_state;
        if gs.step(i, sum) {
          player_wins[i] += count_1 * count_2;
        } else {
          *player_universe_scores.entry(gs).or_default() += count_1 * count_2;
        }
      }
    }
  }
  dbg!(player_wins);
  player_wins[0].max(player_wins[1])
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct GameState<const LIMIT: usize> {
  scores: [usize; 2],
  positions: [usize; 2],
}

impl<const LIMIT: usize> GameState<LIMIT> {
  fn step(&mut self, player: usize, die_sum: usize) -> bool {
    self.positions[player] = (self.positions[player] + die_sum) % 10;
    self.scores[player] += self.positions[player] as usize + 1;
    self.scores[player] >= LIMIT
  }
}
