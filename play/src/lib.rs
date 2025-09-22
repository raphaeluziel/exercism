use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
struct Points {
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

fn add_to_scores(a: Points, b: Points) -> Points {
    Points {
        matches: 0, // matches calculated at end
        wins: a.wins + b.wins,
        draws: a.draws + b.draws,
        losses: a.losses + b.losses,
        points: 0, // points calculated at end
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: BTreeMap<&str, Points> = BTreeMap::new();
    let mut table = String::new();

    for game in match_results.split('\n') {
        let s: Vec<&str> = game.split(';').collect();

        let win = Points {
            matches: 0,
            wins: 1,
            draws: 0,
            losses: 0,
            points: 0,
        };
        let draw = Points {
            matches: 0,
            wins: 0,
            draws: 1,
            losses: 0,
            points: 0,
        };
        let loss = Points {
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 1,
            points: 0,
        };

        let (team_a_pts, team_b_pts) = match s[2] {
            "win" => (win, loss),
            "loss" => (loss, win),
            _ => (draw, draw),
        };

        teams
            .entry(s[0])
            .and_modify(|x| *x = add_to_scores(*x, team_a_pts))
            .or_insert(team_a_pts);
        teams
            .entry(s[1])
            .and_modify(|x| *x = add_to_scores(*x, team_b_pts))
            .or_insert(team_b_pts);
    }

    for mut team in teams {
        team.1.matches = team.1.wins + team.1.draws + team.1.losses;
        team.1.points = 3 * team.1.wins + team.1.draws;
        println!("team = {:?}", team);
    }

    let mut v: Vec<_> = teams
        .iter()
        .map(|x| {
            (
                x.0,
                x.1.matches,
                x.1.wins,
                x.1.draws,
                x.1.losses,
                x.1.points,
            )
        })
        .collect();

    v.sort();

    println!("{:?}", v);

    //println!("Team                           | MP |  W |  D |  L |  P\n");

    todo!()

    // Team                           | MP |  W |  D |  L |  P
    // Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
    // Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
    // Blithering Badgers             |  3 |  1 |  0 |  2 |  3
    // Courageous Californians        |  3 |  0 |  1 |  2 |  1
}
