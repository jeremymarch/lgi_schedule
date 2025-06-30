use jiff::{civil::Weekday, ToSpan, Zoned};
use serde::{Deserialize, Serialize};
//use rand::seq::SliceRandom;
//use rand::thread_rng;

enum SgiDays {
    DayOne,
    WeeksOneToThree,
    WeeksFourToSix,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    pub exam: Option<String>,
    pub day: u32,
    pub date: Zoned,
    pub morning_optional: Option<String>,
    pub quiz_grader: String,
    pub drill1: Vec<String>,
    pub drill2: Vec<String>,
    pub noon_optional1: String,
    pub noon_optional2: Option<String>,
    pub noon_optional1_title: String,
    pub noon_optional2_title: Option<String>,
    pub lecture: String,
    pub lecture_title: String,
    pub voc_notes: String,
    pub friday_review1: Vec<String>,
    pub friday_review2: Vec<String>,
}

pub trait SgiDay {
    fn get_drills(&self);
}

impl SgiDay for Day {
    fn get_drills(&self) {}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Summer {
    pub start_date: Zoned,
    pub holidays: Vec<Zoned>,
    pub days_array: Vec<Day>, //Vec<Box<dyn SgiDay>>,
}

pub fn create_summer(
    start_date: &str,
    holidays: Vec<&str>,
    faculty: Vec<Vec<&str>>,
) -> Option<Summer> {
    let date_suffix = " 08:30[America/New_York]";

    let mut holidays_zoned: Vec<Zoned> = vec![];
    for h in holidays {
        if let Ok(hz) = format!("{}{}", h, date_suffix).parse() {
            holidays_zoned.push(hz);
        }
    }

    let mut summer = Summer {
        start_date: format!("{}{}", start_date, date_suffix).parse().unwrap(),
        holidays: holidays_zoned,
        days_array: vec![],
    };

    if summer.start_date.weekday() != Weekday::Monday {
        return None;
    }

    let mut these_days = summer.start_date.clone();
    let one_day = 1.day();

    // Drills:
    // 1    3
    // 2    4
    // 3    2
    let mut faculty_len = faculty[0].len();
    let mut day_num = 1;
    let mut lecture_num: u32 = 0;
    let mut week_idx = 0;
    for d in 0..=70 {
        if these_days.weekday() == Weekday::Saturday
            || these_days.weekday() == Weekday::Sunday
            || summer.holidays.contains(&these_days)
        {
            let day = Day {
                exam: None,
                day: 0,
                date: these_days.clone(),
                morning_optional: None,
                quiz_grader: String::from(""),
                drill1: vec![],
                drill2: vec![],
                noon_optional1: String::from(""),
                noon_optional2: None,
                noon_optional1_title: String::from(""),
                noon_optional2_title: None,
                lecture: String::from(""),
                lecture_title: String::from(""),
                voc_notes: String::from(""),
                friday_review1: vec![],
                friday_review2: vec![],
            };

            summer.days_array.push(day); //Box::new(day));
        } else {
            let day = Day {
                exam: if these_days.weekday() == Weekday::Monday {
                    Some(String::from("JM"))
                } else {
                    None
                },
                day: day_num,
                date: these_days.clone(),
                morning_optional: Some(faculty[0][(d + 3) % faculty_len].to_string()),
                quiz_grader: faculty[0][(d + 0) % faculty_len].to_string(),
                drill1: if day_num < 15 {
                    vec![
                        faculty[week_idx][(d + 0) % faculty_len].to_string(),
                        faculty[week_idx][(d + 1) % faculty_len].to_string(),
                        faculty[week_idx][(d + 2) % faculty_len].to_string(),
                    ]
                } else {
                    vec![
                        faculty[week_idx][(d + 0) % faculty_len].to_string(),
                        faculty[week_idx][(d + 1) % faculty_len].to_string(),
                    ]
                },
                drill2: if day_num < 15 {
                    vec![
                        faculty[week_idx][(d + 2) % faculty_len].to_string(),
                        faculty[week_idx][(d + 3) % faculty_len].to_string(),
                        faculty[week_idx][(d + 1) % faculty_len].to_string(),
                    ]
                } else {
                    vec![
                        faculty[week_idx][(d + 1) % faculty_len].to_string(),
                        faculty[week_idx][(d + 2) % faculty_len].to_string(),
                    ]
                },
                noon_optional1: faculty[0][(d + 2) % faculty_len].to_string(),
                noon_optional2: Some(faculty[0][(d + 3) % faculty_len].to_string()),
                noon_optional1_title: String::from(""),
                noon_optional2_title: None,
                lecture: faculty[0][(d + 0) % faculty_len].to_string(),
                lecture_title: match day_num {
                    1 => String::from("Lecture on Accents"),
                    2..28 => format!(
                        "Lecture on Unit {}",
                        if (these_days.weekday() == Weekday::Thursday
                            && summer
                                .holidays
                                .contains(&these_days.checked_add(one_day).unwrap()))
                            || these_days.weekday() == Weekday::Friday
                        {
                            0
                        } else {
                            lecture_num += 1;
                            lecture_num
                        }
                    ),
                    _ => String::from(""),
                },
                voc_notes: faculty[0][(d + 1) % faculty_len].to_string(),
                friday_review1: if (these_days.weekday() == Weekday::Thursday
                    && summer
                        .holidays
                        .contains(&these_days.checked_add(one_day).unwrap()))
                    || these_days.weekday() == Weekday::Friday
                {
                    vec![
                        faculty[0][(d + 0) % faculty_len].to_string(),
                        faculty[0][(d + 1) % faculty_len].to_string(),
                    ]
                } else {
                    vec![]
                },
                friday_review2: if (these_days.weekday() == Weekday::Thursday
                    && summer
                        .holidays
                        .contains(&these_days.checked_add(one_day).unwrap()))
                    || these_days.weekday() == Weekday::Friday
                {
                    vec![
                        faculty[0][(d + 2) % faculty_len].to_string(),
                        if day_num < 15 {
                            faculty[0][(d + 3) % faculty_len].to_string()
                        } else {
                            faculty[0][(d + 0) % faculty_len].to_string()
                        },
                    ]
                } else {
                    vec![]
                },
            };
            day_num += 1;
            summer.days_array.push(day); //Box::new(day));
        }

        these_days = these_days.checked_add(one_day).unwrap();
        if these_days.weekday() == Weekday::Monday {
            week_idx += 1;
            faculty_len = faculty[week_idx].len();
        }
    }

    Some(summer)
}

pub fn t1() {
    let groups = vec!["E", "F/G", "H"];
    let faculty = vec!["JM", "HH", "BP", "EBH"];
    let days = 4;
    let hours = 2;

    let mut drill_hours: Vec<Vec<String>> = vec![];
    for n in 0..(days * hours) {
        let mut gr: Vec<String> = vec![];
        for g in &groups {
            gr = vec![];
            for f in &faculty {
                //print!(format!(" {} ", f));
                gr.push(f.to_string());
            }
        }
        drill_hours.push(gr);
    }

    for (i, n) in drill_hours.iter().enumerate() {
        for (j, g) in groups.iter().enumerate() {
            println!("{} - {}", g, n[j]);
        }
    }
}

/*
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
            if a[l][0] != a[l][1]
                && a[l][2] != a[l][3]
                && a[l][4] != a[l][5]
                && a[l][6] != a[l][7]
                && a[l][1] != a[l][2]
                && a[l][3] != a[l][4]
                && a[l][5] != a[l][6]
            {
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
                } else if a[m][n] == "HH" {
                    hh += 1;
                } else if a[m][n] == "BP" {
                    bp += 1;
                } else if a[m][n] == "EB" {
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
*/

struct Params<'a> {
    faculty: &'a [String],
    groups: &'a [String],
    state: Zoned,
    holidays: &'a [Zoned],
    lecture_assignments: &'a [String],
}

fn make_schedule(params: Params) {
    let groups = vec!["E", "F/G", "H"];
    let faculty = vec!["JM", "HH", "BP", "EBH"];
    let days = 4;
    let hours = 2;

    let hours: Vec<String> = vec![];
    let num = days * 2 * groups.len();
    //for h in num {}
}

pub fn get_weekday(w: Weekday) -> String {
    match w {
        Weekday::Monday => String::from("Monday"),
        Weekday::Tuesday => String::from("Tuesday"),
        Weekday::Wednesday => String::from("Wednesday"),
        Weekday::Thursday => String::from("Thursday"),
        Weekday::Friday => String::from("Friday"),
        Weekday::Saturday => String::from("Saturday"),
        Weekday::Sunday => String::from("Sunday"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_schedule() {
        let start = "2025-06-09";
        let holidays = vec!["2025-06-19", "2025-07-04"];
        let faculty = vec![
            vec!["BP", "JM", "HH", "EBH"],
            vec!["BP", "JM", "HH", "EBH"],
            vec!["BP", "JM", "HH", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
            vec!["BP", "JM", "EBH"],
        ];
        let s = create_summer(start, holidays, faculty).unwrap();
        for a in s.days_array {
            println!("{} {}", a.day, get_weekday(a.date.weekday()));
            println!("     {}    {}", a.drill1[0], a.drill2[0]);
            println!("     {}    {}", a.drill1[1], a.drill2[1]);
            println!("     {}    {}", a.drill1[2], a.drill2[2])
        }
    }

    #[test]
    fn it_works() {
        //add();
        //assert_eq!(result, 4);
    }

    #[test]
    fn test1() {
        t1();
        //assert_eq!(result, 4);
    }
}
