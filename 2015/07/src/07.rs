use aoc::Parse;
use parse_display::FromStr;
use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

#[derive(FromStr, PartialEq, Debug)]
enum Value {
    #[display("{0}")]
    Int(u16),
    #[from_str(regex = "(?<0>[a-z]+)")]
    Wire(String),
}

#[derive(FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("{0} -> {1}")]
    Load(Value, String),
    #[display("NOT {0} -> {1}")]
    Not(Value, String),

    #[display("{0} LSHIFT {1} -> {2}")]
    Lshift(Value, Value, String),
    #[display("{0} RSHIFT {1} -> {2}")]
    Rshift(Value, Value, String),

    #[display("{0} AND {1} -> {2}")]
    And(Value, Value, String),
    #[display("{0} OR {1} -> {2}")]
    Or(Value, Value, String),
}

fn part_1(input: aoc::Input) -> impl ToString {
    let instructions = input
        .lines()
        .map(|i| i.parse_uw::<Instruction>())
        .collect::<Vec<Instruction>>();

    let mut wires: FxHashMap<String, u16> = FxHashMap::default();
    while !wires.contains_key("a") {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Load(v, w) => match v {
                    Value::Int(v) => {
                        wires.insert(w.clone(), *v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), *v);
                        }
                    }
                },
                Instruction::Not(v, w) => match v {
                    Value::Int(v) => {
                        wires.insert(w.clone(), !v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), !v);
                        }
                    }
                },

                Instruction::Lshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 << v2);
                }
                Instruction::Rshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 >> v2);
                }

                Instruction::And(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 & v2);
                }
                Instruction::Or(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 | v2);
                }
            }
        }
    }

    *wires.get("a").unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let instructions = input
        .lines()
        .map(|i| i.parse_uw::<Instruction>())
        .collect::<Vec<Instruction>>();

    let mut wires: FxHashMap<String, u16> = FxHashMap::default();
    while !wires.contains_key("a") {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Load(v, w) => match v {
                    Value::Int(v) => {
                        wires.insert(w.clone(), *v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), *v);
                        }
                    }
                },
                Instruction::Not(v, w) => match v {
                    Value::Int(v) => {
                        wires.insert(w.clone(), !v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), !v);
                        }
                    }
                },

                Instruction::Lshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 << v2);
                }
                Instruction::Rshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 >> v2);
                }

                Instruction::And(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 & v2);
                }
                Instruction::Or(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 | v2);
                }
            }
        }
    }

    let a = *wires.get("a").unwrap();
    let mut wires: FxHashMap<String, u16> = FxHashMap::default();
    wires.insert("b".to_owned(), a);

    while !wires.contains_key("a") {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Load(v, w) => match v {
                    Value::Int(v) => {
                        if w == "b" {
                            continue;
                        }
                        wires.insert(w.clone(), *v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), *v);
                        }
                    }
                },
                Instruction::Not(v, w) => match v {
                    Value::Int(v) => {
                        wires.insert(w.clone(), !v);
                    }
                    Value::Wire(v) => {
                        if let Some(v) = wires.get(v) {
                            wires.insert(w.clone(), !v);
                        }
                    }
                },

                Instruction::Lshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 << v2);
                }
                Instruction::Rshift(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 >> v2);
                }

                Instruction::And(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 & v2);
                }
                Instruction::Or(v1, v2, w) => {
                    let v1 = match v1 {
                        Value::Int(v1) => v1,
                        Value::Wire(v1) => {
                            if let Some(v1) = wires.get(v1) {
                                v1
                            } else {
                                continue;
                            }
                        }
                    };
                    let v2 = match v2 {
                        Value::Int(v2) => v2,
                        Value::Wire(v2) => {
                            if let Some(v2) = wires.get(v2) {
                                v2
                            } else {
                                continue;
                            }
                        }
                    };

                    wires.insert(w.clone(), v1 | v2);
                }
            }
        }
    }

    *wires.get("a").unwrap()
}
