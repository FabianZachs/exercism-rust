use std::collections::HashMap;

const POINTS_FOR_WIN: u32 = 3;
const POINTS_FOR_LOSS: u32 = 0;
const POINTS_FOR_DRAW: u32 = 1;

#[derive(Default, Debug, Eq)]
struct Team {
    name: String,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl std::cmp::Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.points != other.points {
            self.points.cmp(&other.points)
        } else {
            other.name.cmp(&self.name)
        }
    }
}

impl std::cmp::PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Team {
    fn eq(&self, _other: &Self) -> bool {
        false // we will never have a equivalent
    }
}

impl Team {
    fn loss(&mut self) {
        self.losses += 1;
        self.points += POINTS_FOR_LOSS;
    }

    fn win(&mut self) {
        self.wins += 1;
        self.points += POINTS_FOR_WIN;
    }

    fn draw(&mut self) {
        self.draws += 1;
        self.points += POINTS_FOR_DRAW;
    }
}
// implement identifier for a team in the hashset

fn gather_data(data: &str) -> HashMap<String, Team> {
    let mut map = HashMap::new();
    if data.is_empty() {
        return map;
    }

    // split lines (i.e. matches)
    let matches: Vec<&str> = data.split("\n").collect();

    for result in matches {
        let match_result: Vec<&str> = result.split(";").collect();
        insert_data(&mut map, &match_result); // NOTE could move in match_result
    }

    map
}

fn insert_data(map: &mut HashMap<String, Team>, match_result: &[&str]) {
    let (t1, t2, result) = (match_result[0], match_result[1], match_result[2]);
    let t1 = map.entry(String::from(t1)).or_insert(Team {
        name: String::from(t1),
        ..Default::default()
    });
    match result {
        "win" => t1.win(),
        "loss" => t1.loss(),
        "draw" => t1.draw(),
        _ => panic!("Unexpected match result"),
    };

    let t2 = map.entry(String::from(t2)).or_insert(Team {
        name: String::from(t2),
        ..Default::default()
    });
    match result {
        "win" => t2.loss(),
        "loss" => t2.win(),
        "draw" => t2.draw(),
        _ => panic!("Unexpected match result"),
    };
}

pub fn tally(match_results: &str) -> String {
    let data = gather_data(match_results);
    let mut data: Vec<&Team> = data.values().collect();

    data.sort_by(|a, b| b.cmp(&a));

    let mut result = String::new();
    result.push_str(&format!(
        "Team{:<WIDTH$} | MP |  W |  D |  L |  P",
        " ",
        WIDTH = 26
    )); // WIDTH$ looks for width in arguments, :< left aligned 0th arg

    for team in data {
        result.push_str(&format!(
            "\n{:<WIDTH$} |  {} |  {} |  {} |  {} |  {}",
            team.name,
            team.wins + team.draws + team.losses,
            team.wins,
            team.draws,
            team.losses,
            team.points,
            WIDTH = 30
        ));
    }

    result
}
