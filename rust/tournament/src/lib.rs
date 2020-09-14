use std::collections::BTreeMap;

pub fn tally(match_results: &str) -> String {
    let header = format!(
        "{0:<30} | {1:>2} | {2:>2} | {3:>2} | {4:>2} | {5:>2}",
        "Team", "MP", "W", "D", "L", "P"
    );
    let mut results = BTreeMap::new(); // Gives us sorted keys

    for result in match_results.lines() {
        let match_up: Vec<&str> = result.split(';').collect();
        match match_up[2] {
            "win" => {
                results
                    .entry(match_up[0])
                    .and_modify(|v: &mut (i32, i32, i32)| v.0 += 1)
                    .or_insert((1, 0, 0));
                results
                    .entry(match_up[1])
                    .and_modify(|v: &mut (i32, i32, i32)| v.2 += 1)
                    .or_insert((0, 0, 1));
            }
            "draw" => {
                results
                    .entry(match_up[0])
                    .and_modify(|v: &mut (i32, i32, i32)| v.1 += 1)
                    .or_insert((0, 1, 0));
                results
                    .entry(match_up[1])
                    .and_modify(|v: &mut (i32, i32, i32)| v.1 += 1)
                    .or_insert((0, 1, 0));
            }
            "loss" => {
                results
                    .entry(match_up[0])
                    .and_modify(|v: &mut (i32, i32, i32)| v.2 += 1)
                    .or_insert((0, 0, 1));
                results
                    .entry(match_up[1])
                    .and_modify(|v: &mut (i32, i32, i32)| v.0 += 1)
                    .or_insert((1, 0, 0));
            }
            _ => continue,
        }
    }

    let mut results: Vec<(&&str, &(i32, i32, i32))> = results.iter().collect();
    results.sort_by(|(_, &(wins1, draws1, _)), (_, &(wins2, draws2, _))| {
        (wins2 * 3 + draws2).cmp(&(wins1 * 3 + draws1))
    });

    std::iter::once(header)
        .chain(results.iter().map(|(&team, &(wins, draws, lost))| {
            format!(
                "{0:<30} | {1:>2} | {2:>2} | {3:>2} | {4:>2} | {5:>2}",
                team,
                wins + draws + lost,
                wins,
                draws,
                lost,
                (wins * 3) + draws
            )
        }))
        .collect::<Vec<String>>()
        .join("\n")
}
