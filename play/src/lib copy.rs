use std::collections::BTreeMap;

// Tuple is (Matches Played [MP], Wins [W], Draws [D], Losses [L], Points[P])
type WinLossDraw = (usize, usize, usize, usize, usize); // (MP, W, D, L, P)

fn add_to_scores(a: WinLossDraw, b: WinLossDraw) -> WinLossDraw {
    // Do not add matches played or points (both will be calculated at the end)
    (0, a.1 + b.1, a.2 + b.2, a.3 + b.3, 0)
}

pub fn tally(match_results: &str) -> String {

    let mut teams:BTreeMap<&str, WinLossDraw> = BTreeMap::new();
    let mut table = String::new();

    for game in match_results.split('\n') {
        
        let s:Vec<&str> = game.split(';').collect();

        let pts = match s[2] {
            "win"  => ((0, 1, 0, 0, 0), (0, 0, 1, 0, 0)),
            "loss" => ((0, 0, 1, 0, 0), (0, 1, 0, 0, 0)),
            _      => ((0, 0, 0, 1, 0), (0, 0, 0, 1, 0)),
        };

        teams.entry(s[0]).and_modify(|x| *x = add_to_scores(*x, pts.0)).or_insert(pts.0);
        teams.entry(s[1]).and_modify(|x| *x = add_to_scores(*x, pts.1)).or_insert(pts.1);

    }

    for team in teams {
        
        println!("team = {:?}", team);
    }

    //println!("Team                           | MP |  W |  D |  L |  P\n");

    todo!()

    // Team                           | MP |  W |  D |  L |  P
    // Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
    // Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
    // Blithering Badgers             |  3 |  1 |  0 |  2 |  3
    // Courageous Californians        |  3 |  0 |  1 |  2 |  1

}
