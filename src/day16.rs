use std::collections::VecDeque;

pub fn compute(input: &str) -> (usize, usize) {
  let mut bits = input
    .as_bytes()
    .iter()
    .copied()
    .flat_map(|b| {
      let x = (b as char).to_digit(16).unwrap() as u8;
      (0..4).rev().map(move |i| (x & (1 << i)) == (1 << i))
    })
    .collect::<VecDeque<_>>();
  let packet = packet_parser(&mut bits);
  // dbg!(packet);
  (packet.sum_versions(), packet.value())
}

#[derive(Debug)]
struct Packet {
  version: u8,
  type_id: TypeId,
}

impl Packet {
  fn sum_versions(&self) -> usize {
    let inner = if let TypeId::Operator(_, p) = &self.type_id {
      p.iter().map(|p| p.sum_versions()).sum::<usize>()
    } else {
      0
    };
    inner + self.version as usize
  }
  fn value(&self) -> usize {
    match &self.type_id {
      TypeId::Literal(u) => *u,
      TypeId::Operator(u, v) if *u == 0 => v.iter().map(|p| p.value()).sum::<usize>(),
      TypeId::Operator(u, v) if *u == 1 => v.iter().map(|p| p.value()).product::<usize>(),
      TypeId::Operator(u, v) if *u == 2 => v.iter().map(|p| p.value()).min().unwrap(),
      TypeId::Operator(u, v) if *u == 3 => v.iter().map(|p| p.value()).max().unwrap(),
      TypeId::Operator(u, v) if *u == 5 => (v[0].value() > v[1].value()) as usize,
      TypeId::Operator(u, v) if *u == 6 => (v[0].value() < v[1].value()) as usize,
      TypeId::Operator(u, v) if *u == 7 => (v[0].value() == v[1].value()) as usize,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
enum TypeId {
  Operator(u8, Vec<Packet>),
  Literal(usize),
}

fn packet_parser(bits: &mut VecDeque<bool>) -> Packet {
  let version = take_n_bits_to_u16::<3>(bits) as u8;
  let type_id = take_n_bits_to_u16::<3>(bits) as u8;
  // dbg!((version, type_id));
  let type_id = if type_id == 4 {
    TypeId::Literal(get_literal(bits))
  } else {
    let packets = if bits.pop_front().unwrap() {
      // take bits as number of packets
      let packet_count = take_n_bits_to_u16::<11>(bits);
      (0..packet_count)
        .map(|_| packet_parser(bits))
        .collect::<Vec<_>>()
    } else {
      // take 15 bits as number of bits
      let bit_count = take_n_bits_to_u16::<15>(bits);
      // println!("bit_count: {}", bit_count);
      let mut new_bits = (0..bit_count)
        .map(|_| bits.pop_front().unwrap())
        .collect::<VecDeque<_>>();
      let mut packets = Vec::new();
      while !new_bits.is_empty() && new_bits.iter().any(|b| *b) {
        packets.push(packet_parser(&mut new_bits));
      }
      packets
    };
    TypeId::Operator(type_id, packets)
  };
  Packet { version, type_id }
}

fn take_n_bits_to_u16<const N: u8>(bits: &mut VecDeque<bool>) -> u16 {
  assert!(N < 16);
  let mut u = 0;
  for _ in 0..N {
    let o = bits.pop_front().unwrap() as u16;
    // println!("pop: {}", o);
    u = (u << 1) | o;
    // println!("u: {}", u);
  }
  // println!("n: {}, val: {}", N, u);
  u
}

fn take_5_to_u8(bits: &mut VecDeque<bool>) -> (bool, u8) {
  (
    bits.pop_front().unwrap(),
    take_n_bits_to_u16::<4>(bits) as u8,
  )
}

fn get_literal(bits: &mut VecDeque<bool>) -> usize {
  let mut v = 0;
  loop {
    let (c, n) = take_5_to_u8(bits);
    v = (v << 4) | n as usize;
    if !c {
      break;
    }
  }
  v
}
