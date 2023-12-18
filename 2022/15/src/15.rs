use aoc::{Input, Parse};

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    fn distance(&self, other: &Pos) -> i64 {
        self.x.abs_diff(other.x) as i64 + self.y.abs_diff(other.y) as i64
    }

    fn tuning_frequency(&self) -> i64 {
        self.x * 4000000 + self.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    y_intercept: i64,
    increasing: bool,
}

impl Line {
    fn new(y_intercept: i64, increasing: bool) -> Self {
        Line {
            y_intercept,
            increasing,
        }
    }

    fn intersects(&self, other: &Line) -> Option<Pos> {
        if self.increasing == other.increasing {
            return None;
        }
        if self.y_intercept.abs_diff(other.y_intercept) % 2 == 1 {
            return None;
        }
        let c = (other.y_intercept - self.y_intercept) / 2;
        Some(Pos::new(
            if self.increasing { c } else { -c },
            other.y_intercept - c,
        ))
    }

    fn from(pos: &Pos, increasing: bool) -> Self {
        let mut y_intercept = pos.y;
        if increasing {
            y_intercept -= pos.x;
        } else {
            y_intercept += pos.x;
        }
        Line::new(y_intercept, increasing)
    }
}

pub struct Sensor {
    pos: Pos,
    beacon_distance: i64,
}

impl Sensor {
    fn new(pos: Pos, closest_beacon: Pos) -> Self {
        Sensor {
            pos,
            beacon_distance: pos.distance(&closest_beacon),
        }
    }

    fn get_x_skip_enter(&self, pos: &Pos) -> i64 {
        let y_diff = self.pos.y.abs_diff(pos.y) as i64;
        if self.beacon_distance < y_diff {
            i64::MAX
        } else {
            self.pos.x - self.beacon_distance + y_diff
        }
    }

    fn get_x_skip_exit(&self, pos: &Pos) -> i64 {
        self.pos.x + self.beacon_distance - self.pos.y.abs_diff(pos.y) as i64 + 1
    }

    fn is_in_distance(&self, pos: &Pos) -> bool {
        self.pos.distance(pos) <= self.beacon_distance
    }
}

fn part_1(input: Input) -> impl ToString {
    let mut sensors = vec![];

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;

    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    let mut max_distance = i64::MIN;

    for line in input {
        let [sx, sy, bx, by] = line.ints::<4, i64>();
        let sensor_pos = Pos::new(sx, sy);
        let beacon_pos = Pos::new(bx, by);

        min_x = min_x.min(sx).min(bx);
        min_y = min_y.min(sy).min(by);
        max_x = max_x.max(sx).max(bx);
        max_y = max_y.max(sy).max(by);

        max_distance = max_distance.max(sensor_pos.distance(&beacon_pos));
        sensors.push(Sensor::new(sensor_pos, beacon_pos));
    }

    let min_x = min_x - max_distance;
    let max_x = max_x + max_distance;
    let mut isnt_in = 0;

    let mut x = min_x;
    while x <= max_x {
        let pos = Pos::new(x, 2000000);

        let mut skip_x = x + 1;
        let mut in_range = false;

        for sensor in sensors.iter() {
            if sensor.is_in_distance(&pos) {
                skip_x = sensor.get_x_skip_exit(&pos);
                in_range = true;
                break;
            }
        }

        if !in_range {
            let mut enter_x = i64::MAX;
            for sensor in sensors.iter() {
                let enter = sensor.get_x_skip_enter(&pos);
                if enter >= skip_x {
                    enter_x = enter_x.min(enter);
                }
            }

            skip_x = enter_x;
        } else {
            isnt_in += skip_x - x;
        }
        x = skip_x;
    }

    isnt_in - 1
}

fn part_2(input: Input) -> impl ToString {
    let mut sensors = vec![];

    for line in input {
        let [sx, sy, bx, by] = line.ints::<4, i64>();
        let sensor_pos = Pos::new(sx, sy);
        let beacon_pos = Pos::new(bx, by);
        sensors.push(Sensor::new(sensor_pos, beacon_pos));
    }

    let mut last_line = None;
    for (i, sensor1) in sensors.iter().enumerate() {
        for sensor2 in sensors[i + 1..].iter() {
            let empty_space = sensor1.pos.distance(&sensor2.pos)
                - (sensor1.beacon_distance + sensor2.beacon_distance);
            if empty_space == 2 {
                let line = Line::from(
                    &Pos::new(
                        sensor1.pos.x,
                        if sensor1.pos.y > sensor2.pos.y {
                            sensor1.pos.y - sensor1.beacon_distance - 1
                        } else {
                            sensor1.pos.y + sensor1.beacon_distance + 1
                        },
                    ),
                    (sensor1.pos.x < sensor2.pos.x) ^ (sensor1.pos.y < sensor2.pos.y),
                );

                if let Some(last_line) = last_line {
                    return line.intersects(&last_line).unwrap().tuning_frequency();
                } else {
                    last_line = Some(line);
                }
            }
        }
    }

    unreachable!()
}
