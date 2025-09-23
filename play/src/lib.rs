use std::collections::BTreeMap;

#[derive(Debug)]
struct Team<'a> {
    name: &'a str,
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

// fn add_to_scores(a: Points, b: Points) -> Points {
//     Points {
//         matches: 0, // matches calculated at end
//         wins: a.wins + b.wins,
//         draws: a.draws + b.draws,
//         losses: a.losses + b.losses,
//         points: 0, // points calculated at end
//     }
// }

pub fn tally(match_results: &str) -> String {
    let mut teams: BTreeMap<&str, Team> = BTreeMap::new();
    let mut scores: Vec<Team> = Vec::new();
    let mut table = String::new();

    for game in match_results.split('\n') {
        let s: Vec<&str> = game.split(';').collect();

        let ((a_win, a_draw, a_loss), (b_win, b_draw, b_loss)) = match s[2] {
            "win" => ((1, 0, 0), (0, 0, 1)),
            "loss" => ((0, 0, 1), (1, 0, 0)),
            _ => ((0, 1, 0), (0, 1, 0)),
        };

        teams
            .entry(s[0])
            .and_modify(|x| {
                x.wins += a_win;
                x.draws += a_draw;
                x.losses += a_loss
            })
            .or_insert(Team {
                name: s[0],
                matches: 0,
                wins: a_win,
                draws: a_draw,
                losses: a_loss,
                points: 0,
            });

        teams
            .entry(s[1])
            .and_modify(|x| {
                x.wins += b_win;
                x.draws += b_draw;
                x.losses += b_loss
            })
            .or_insert(Team {
                name: s[0],
                matches: 0,
                wins: b_win,
                draws: b_draw,
                losses: b_loss,
                points: 0,
            });
    }

    //println!("TEAMS\n{:?}\n", teams);

    let mut v:Vec<(&str, usize, usize, usize, usize, usize)> = Vec::new();

    for team in teams.iter_mut() {
        team.1.matches = team.1.wins + team.1.draws + team.1.losses;
        team.1.points = team.1.wins * 3 + team.1.draws;
        v.push((team.0, team.1.matches, team.1.wins, team.1.draws, team.1.losses, team.1.points));
        //println!("{:?}\n", team);
    }

    //println!("VEC\n{:?}", v);

    v.sort_by(|a, b| (b.5.cmp(&a.5)));

    //println!("VEC22222222222223\n{:?}", v);

    println!("Team                       | MP |  W |  D |  L |  P\n");
    for line in v {
        let ss = line.0.to_owned() +"         | " + 
                 &line.1.to_string() + " |  " + 
                 &line.2.to_string() + " |  " +
                 &line.3.to_string() + " |  " +
                 &line.4.to_string() + " |  " +
                 &line.5.to_string() + " |  " +
                 "\n";
        table.push_str(&ss);
    }

    println!("{table}");

    table

    //todo!()

    // Team                           | MP |  W |  D |  L |  P
    // Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
    // Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
    // Blithering Badgers             |  3 |  1 |  0 |  2 |  3
    // Courageous Californians        |  3 |  0 |  1 |  2 |  1
}
