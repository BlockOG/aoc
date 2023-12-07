use aoc::Parse;

aoc::parts!(1, 2);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Part1Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

const PART_1_MAP: [Part1Card; 256] = [
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::Two,
    Part1Card::Three,
    Part1Card::Four,
    Part1Card::Five,
    Part1Card::Six,
    Part1Card::Seven,
    Part1Card::Eight,
    Part1Card::Nine,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::J,
    Part1Card::K,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::Q,
    Part1Card::A,
    Part1Card::A,
    Part1Card::T,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
    Part1Card::A,
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Part1Hand {
    hand_type: HandType,
    cards: [Part1Card; 5],
}

impl Part1Hand {
    fn new(cards: [Part1Card; 5]) -> Self {
        let mut card_count = [0; 13];
        for &card in cards.iter() {
            card_count[card as usize] += 1;
        }

        let mut hand_type = HandType::High;
        for i in card_count {
            if i == 5 {
                hand_type = HandType::Five;
                break;
            } else if i == 4 {
                hand_type = HandType::Four;
                break;
            } else if i == 3 {
                if hand_type == HandType::High {
                    hand_type = HandType::Three;
                } else if hand_type == HandType::One {
                    hand_type = HandType::Full;
                    break;
                }
            } else if i == 2 {
                if hand_type == HandType::High {
                    hand_type = HandType::One;
                } else if hand_type == HandType::One {
                    hand_type = HandType::Two;
                    break;
                } else if hand_type == HandType::Three {
                    hand_type = HandType::Full;
                    break;
                }
            }
        }

        Self { hand_type, cards }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Part2Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

const PART_2_MAP: [Part2Card; 256] = [
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::Two,
    Part2Card::Three,
    Part2Card::Four,
    Part2Card::Five,
    Part2Card::Six,
    Part2Card::Seven,
    Part2Card::Eight,
    Part2Card::Nine,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::J,
    Part2Card::K,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::Q,
    Part2Card::A,
    Part2Card::A,
    Part2Card::T,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
    Part2Card::A,
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Part2Hand {
    hand_type: HandType,
    cards: [Part2Card; 5],
}

impl Part2Hand {
    fn new(cards: [Part2Card; 5]) -> Self {
        let mut card_count = [0; 13];
        for &card in cards.iter() {
            card_count[card as usize] += 1;
        }

        let mut hand_type = HandType::High;
        for i in card_count.into_iter().take(12) {
            if i == 5 {
                hand_type = HandType::Five;
                break;
            } else if i == 4 {
                hand_type = HandType::Four;
                break;
            } else if i == 3 {
                if hand_type == HandType::High {
                    hand_type = HandType::Three;
                } else if hand_type == HandType::One {
                    hand_type = HandType::Full;
                    break;
                }
            } else if i == 2 {
                if hand_type == HandType::High {
                    hand_type = HandType::One;
                } else if hand_type == HandType::One {
                    hand_type = HandType::Two;
                    break;
                } else if hand_type == HandType::Three {
                    hand_type = HandType::Full;
                    break;
                }
            }
        }

        if card_count[12] == 5 || card_count[12] == 4 {
            hand_type = HandType::Five;
        } else if card_count[12] == 3 {
            hand_type = match hand_type {
                HandType::One => HandType::Five,
                HandType::High => HandType::Four,
                _ => unreachable!(),
            };
        } else if card_count[12] == 2 {
            hand_type = match hand_type {
                HandType::Three => HandType::Five,
                HandType::One => HandType::Four,
                HandType::High => HandType::Three,
                _ => unreachable!(),
            };
        } else if card_count[12] == 1 {
            hand_type = match hand_type {
                HandType::Four => HandType::Five,
                HandType::Three => HandType::Four,
                HandType::Two => HandType::Full,
                HandType::One => HandType::Three,
                HandType::High => HandType::One,
                _ => unreachable!(),
            };
        }

        Self { hand_type, cards }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    calc(input, |i| {
        (
            Part1Hand::new([
                PART_1_MAP[i.idx(0) as usize],
                PART_1_MAP[i.idx(1) as usize],
                PART_1_MAP[i.idx(2) as usize],
                PART_1_MAP[i.idx(3) as usize],
                PART_1_MAP[i.idx(4) as usize],
            ]),
            i[6..].parse_uw(),
        )
    })
}

fn part_2(input: aoc::Input) -> impl ToString {
    calc(input, |i| {
        (
            Part2Hand::new([
                PART_2_MAP[i.idx(0) as usize],
                PART_2_MAP[i.idx(1) as usize],
                PART_2_MAP[i.idx(2) as usize],
                PART_2_MAP[i.idx(3) as usize],
                PART_2_MAP[i.idx(4) as usize],
            ]),
            i[6..].parse_uw(),
        )
    })
}

fn calc<T>(input: aoc::Input, parse: fn(&str) -> (T, u32)) -> u32
where
    T: Ord,
{
    let mut hands = input.lines().map(parse).collect::<Vec<_>>();
    hands.sort_unstable();
    let n = hands.len();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (n - i) as u32 * bid)
        .sum()
}
