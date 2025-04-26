use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn add() {
    let width = 8;
    let height = 4;
    let mut p = vec!["JM", "HH", "BP", "EB"];
    //let mut grid = vec![vec![0; width]; height];
    let mut a: Vec<Vec<String>> = vec![vec![String::from(""); width]; height];

    'outer: loop {

        for j in 0..width {
            //let s = format!("{}:{}", i + 1, j + 1);
            p.shuffle(&mut thread_rng());
            a[0][j] = p[0].to_string();
            a[1][j] = p[1].to_string();
            a[2][j] = p[2].to_string();
            a[3][j] = p[3].to_string();
        }


        // for k in 0..width {
        //     if a[0][k] == a[1][k] || a[1][k] == a[2][k] || a[2][k] == a[3][k] || a[0][k] == a[2][k] || a[1][k] == a[3][k] || a[0][k] == a[3][k] {
        //         continue;
        //     }
        //     else {
        //         break 'outer;
        //     }
        // }
        let mut row = 0;
        for l in 0..height {
            if a[l][0] != a[l][1] && a[l][2] != a[l][3] && a[l][4] != a[l][5] && a[l][6] != a[l][7] && a[l][1] != a[l][2] && a[l][3] != a[l][4] && a[l][5] != a[l][6] {
                row += 1;
            }

        }

        //each only does two
        for l in 0..height {
            let mut jm = 0;
            let mut hh = 0;
            let mut bp = 0;
            let mut ebh = 0;
            for w in 0..width {

                match a[l][w].as_str() {
                    "JM" => jm += 1,
                    "BP" => bp += 1,
                    "HH" => hh += 1,
                    "EB" => ebh += 1,
                    _ => (),
                }
            }
            if jm < 2 || hh < 2 || bp < 2 || ebh < 2 {
                continue 'outer;
            }
        }

        let mut jm = 0;
        let mut hh = 0;
        let mut bp = 0;
        let mut ebh = 0;
        for m in 0..height {
            for n in 0..width {
                if a[m][n] == "JM" {
                    jm += 1;
                }
                else if a[m][n] == "HH" {
                    hh += 1;
                }
                else if a[m][n] == "BP" {
                    bp += 1;
                }
                else if a[m][n] == "EB" {
                    ebh += 1;
                }
            }
        }
        if row == 4 && jm == 8 && hh == 8 && bp == 8 && ebh == 8 {
            break 'outer;
        }
       
    }



    println!("{:?}", a[0]);
    println!("{:?}", a[1]);
    println!("{:?}", a[2]);
    println!("{:?}", a[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        add();
        //assert_eq!(result, 4);
    }
}
