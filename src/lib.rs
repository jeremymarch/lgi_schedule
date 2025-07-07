use jiff::{ToSpan, Zoned, civil::Weekday};
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
//use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

// enum SgiDays {
//     DayOne,
//     WeeksOneToThree,
//     WeeksFourToSix,
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LgiClass {
    pub title: String,
    pub instructor: String,
    pub handouts: Option<Vec<String>>,
}

pub struct Params<'a> {
    pub faculty: Vec<Vec<&'a str>>,
    pub start_date: &'a str,
    pub holidays: Vec<&'a str>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FacultyWeek {
    pub faculty: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Summer {
    #[serde(rename = "@startdate")]
    pub start_date: Zoned,
    #[serde(rename = "@holidays")]
    pub holidays: Vec<Zoned>,
    pub days: Vec<Day>, //Vec<Box<dyn SgiDay>>,
    pub faculty: Vec<FacultyWeek>,
}

impl Summer {
    pub fn to_xml(&self) -> String {
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 4);
        self.serialize(ser).unwrap();
        buffer
    }

    pub fn from_xml(s: String) -> Summer {
        from_str(&s).unwrap()
    }

    //week is the actual week, not zero-indexed
    pub fn get_seqs(&self, week: u32) -> Vec<(String, Vec<String>)> {
        let mut collector: HashMap<String, Vec<String>> = HashMap::new();

        //let mut holiday: Option<usize> = None;
        for day in self.days.iter() {
            if day.week == week {
                if let Some(_exam_fac) = day.exam.clone() {
                    // if let Some(fac_vector) = collector.get_mut(&exam_fac) {
                    //     fac_vector.push(String::from("Exam"));
                    // } else {
                    //     collector.insert(exam_fac, vec![String::from("Exam")]);
                    // }
                    continue;
                }

                //add "OFF" to each faculty for holidays
                if day.day == 0
                    && day.date.weekday() != Weekday::Saturday
                    && day.date.weekday() != Weekday::Sunday
                {
                    for f in self.faculty[week as usize - 1].faculty.clone() {
                        if let Some(fac_vector) = collector.get_mut(&f) {
                            fac_vector.push(String::from("HOL"));
                            fac_vector.push(String::from("HOL"));
                        } else {
                            collector.insert(f, vec![String::from("HOL"), String::from("HOL")]);
                        }
                    }
                }

                if let Some(exam_fac) = day.morning_optional.clone() {
                    if let Some(fac_vector) = collector.get_mut(&exam_fac) {
                        fac_vector.push(String::from("MO"));
                    } else {
                        collector.insert(exam_fac, vec![String::from("MO")]);
                    }
                }

                let drill1 = day.get_drill1();
                let fac_count = drill1.len();
                for (i, d1_fac) in drill1.clone().iter().enumerate() {
                    let group = match i {
                        0 => "E",
                        1 => {
                            if fac_count > 2 {
                                "F/G"
                            } else {
                                "F"
                            }
                        }
                        _ => "H",
                    };
                    if let Some(fac_vector) = collector.get_mut(d1_fac.as_str()) {
                        fac_vector.push(String::from(group));
                    } else {
                        collector.insert(d1_fac.to_owned(), vec![String::from(group)]);
                    }
                }

                //there are no morning optionals for week 1, so display
                // "OFF" for places where faculty member is off
                // drill 1 only because drill 2 has quiz
                if day.morning_optional.is_none() && !drill1.is_empty() {
                    let drill1_set: HashSet<_> = drill1.iter().cloned().collect();
                    let missing_in_drill1: Vec<_> = self.faculty[week as usize - 1]
                        .faculty
                        .iter()
                        .filter(|&x| !drill1_set.contains(x))
                        .cloned()
                        .collect();

                    for missing in missing_in_drill1 {
                        if let Some(fac_vector) = collector.get_mut(&missing) {
                            fac_vector.push(String::from("OFF"));
                        } else {
                            collector.insert(missing, vec![String::from("OFF")]);
                        }
                    }
                }

                if let Some(exam_fac) = day.quiz_grader.clone() {
                    if let Some(fac_vector) = collector.get_mut(&exam_fac) {
                        fac_vector.push(String::from("QUIZ"));
                    } else {
                        collector.insert(exam_fac, vec![String::from("QUIZ")]);
                    }
                }

                let drill2 = day.get_drill2();
                let fac_count = drill2.len();
                for (i, d2_fac) in drill2.clone().iter().enumerate() {
                    let group = match i {
                        0 => "E",
                        1 => {
                            if fac_count > 2 {
                                "F/G"
                            } else {
                                "F"
                            }
                        }
                        _ => "H",
                    };
                    if let Some(fac_vector) = collector.get_mut(d2_fac.as_str()) {
                        fac_vector.push(String::from(group));
                    } else {
                        collector.insert(d2_fac.to_owned(), vec![String::from(group)]);
                    }
                }
            }
        }

        let mut ret: Vec<(String, Vec<String>)> = vec![];
        for (key, value) in collector.into_iter() {
            ret.push((key, value));
        }
        ret.sort_by(|a, b| a.0.cmp(&b.0));

        ret
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Day {
    #[serde(rename = "@week")]
    pub week: u32,
    #[serde(rename = "@day")]
    pub day: u32,
    #[serde(rename = "@date")]
    pub date: Zoned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_one_lectures: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morning_optional: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiz_grader: Option<String>,
    #[serde(default)]
    pub drill1: Vec<String>,
    #[serde(default)]
    pub drill2: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noon_optional1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noon_optional2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noon_optional1_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noon_optional2_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lecture_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voc_notes: Option<String>,
    #[serde(default)]
    pub friday_review1: Vec<String>,
    #[serde(default)]
    pub friday_review2: Vec<String>,
    //pub sunday_review: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    #[serde(default)]
    pub test: Vec<LgiClass>,
}

pub trait SgiDay {
    fn get_drill1(&self) -> Vec<String>;
    fn get_drill2(&self) -> Vec<String>;
    fn get_stats(&self) -> Vec<(String, u32)>;
    fn validate(&self) -> bool;
}

impl SgiDay for Day {
    fn get_drill1(&self) -> Vec<String> {
        self.drill1.clone()
    }
    fn get_drill2(&self) -> Vec<String> {
        self.drill2.clone()
    }

    fn get_stats(&self) -> Vec<(String, u32)> {
        let mut fac_counts: HashMap<String, u32> = HashMap::new();

        if let Some(e) = self.exam.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.quiz_grader.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.morning_optional.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.noon_optional1.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.noon_optional2.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.lecture.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }
        if let Some(e) = self.voc_notes.as_ref() {
            match fac_counts.get(&e.to_owned()) {
                Some(&f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }

        for e in self.get_drill1() {
            match fac_counts.get(&e) {
                Some(f) => fac_counts.insert(e, f + 1),
                _ => fac_counts.insert(e, 1),
            };
        }

        for e in self.get_drill2() {
            match fac_counts.get(&e) {
                Some(f) => fac_counts.insert(e, f + 1),
                _ => fac_counts.insert(e, 1),
            };
        }

        for e in &self.friday_review1 {
            match fac_counts.get(&e.to_owned()) {
                Some(f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }

        for e in &self.friday_review2 {
            match fac_counts.get(&e.to_owned()) {
                Some(f) => fac_counts.insert(e.to_owned(), f + 1),
                _ => fac_counts.insert(e.to_owned(), 1),
            };
        }

        if let Some(d) = &self.day_one_lectures {
            for e in d {
                match fac_counts.get(&e.to_owned()) {
                    Some(f) => fac_counts.insert(e.to_owned(), f + 1),
                    _ => fac_counts.insert(e.to_owned(), 1),
                };
            }
        }

        let mut v: Vec<(String, u32)> = Vec::new();
        for (key, value) in fac_counts.into_iter() {
            v.push((key, value));
        }
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    fn validate(&self) -> bool {
        true
    }
}

fn make_faculty(vv: Vec<Vec<&str>>) -> Vec<FacultyWeek> {
    // .iter()
    // .map(|inner| inner.iter().map(|s| s.to_string()).collect())
    // .collect(),
    let mut ret: Vec<FacultyWeek> = vec![];
    for v in vv {
        ret.push(FacultyWeek {
            faculty: v.iter().map(|f| f.to_string()).collect(),
        });
    }
    ret
}

pub fn create_summer(params: &Params) -> Option<Summer> {
    let date_suffix = " 08:30[America/New_York]";

    //testxml();

    let mut holidays_zoned: Vec<Zoned> = vec![];
    for h in params.holidays.clone() {
        if let Ok(hz) = format!("{h}{date_suffix}").parse() {
            holidays_zoned.push(hz);
        }
    }

    let mut summer = Summer {
        start_date: format!("{}{date_suffix}", params.start_date)
            .parse()
            .unwrap(),
        holidays: holidays_zoned,
        days: vec![],
        faculty: make_faculty(params.faculty.clone()),
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

    let mut day_num = 1;
    let mut lecture_num: u32 = 0;
    let mut week_idx = 0;
    let mut faculty_len = params.faculty[week_idx].len();
    for d in 0..=70 {
        let is_exam = ((these_days.weekday() == Weekday::Tuesday
            && summer
                .holidays
                .contains(&these_days.checked_sub(one_day).unwrap()))
            || these_days.weekday() == Weekday::Monday)
            && day_num != 1
            && day_num != 24
            && day_num != 34
            && day_num != 44;

        let is_friday_review = (these_days.weekday() == Weekday::Thursday
            && summer
                .holidays
                .contains(&these_days.checked_add(one_day).unwrap()))
            || these_days.weekday() == Weekday::Friday
            || day_num == 27;

        if day_num == 1 {
            let day = Day {
                week: week_idx as u32 + 1,
                day: day_num,
                date: these_days.clone(),
                day_one_lectures: Some(vec![
                    #[allow(clippy::identity_op)]
                    params.faculty[week_idx][(d + 0) % faculty_len].to_string(),
                    params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                    params.faculty[week_idx][(d + 2) % faculty_len].to_string(),
                ]),
                exam: None,
                morning_optional: None,
                quiz_grader: None,
                drill1: vec![],
                drill2: vec![],
                noon_optional1: None,
                noon_optional2: None,
                noon_optional1_title: None,
                noon_optional2_title: None,
                lecture: None,
                lecture_title: None,
                voc_notes: None,
                friday_review1: vec![],
                friday_review2: vec![],
                other: None,
                test: vec![],
            };

            day_num += 1;
            summer.days.push(day); //Box::new(day));
        } else if these_days.weekday() == Weekday::Saturday
            || these_days.weekday() == Weekday::Sunday
            || summer.holidays.contains(&these_days)
        {
            let day = Day {
                week: week_idx as u32 + 1,
                day: 0,
                date: these_days.clone(),
                day_one_lectures: None,
                exam: None,
                morning_optional: None,
                quiz_grader: None,
                drill1: vec![],
                drill2: vec![],
                noon_optional1: None,
                noon_optional2: None,
                noon_optional1_title: None,
                noon_optional2_title: None,
                lecture: None,
                lecture_title: None,
                voc_notes: None,
                friday_review1: vec![],
                friday_review2: vec![],
                other: match these_days.weekday() {
                    Weekday::Saturday => Some(String::from("Rest and Study")),
                    Weekday::Sunday => Some(String::from("Review")),
                    _ => Some(String::from("Holiday, rest and study")),
                },
                test: vec![],
            };

            summer.days.push(day); //Box::new(day));
        } else {
            let day = Day {
                week: week_idx as u32 + 1,
                day: day_num,
                date: these_days.clone(),
                day_one_lectures: None,
                exam: if is_exam {
                    Some(String::from("JM"))
                } else {
                    None
                },
                morning_optional: if day_num < 6 || is_exam {
                    None
                } else if faculty_len > 3 {
                    Some(params.faculty[week_idx][(d + 2) % faculty_len].to_string())
                } else {
                    #[allow(clippy::identity_op)]
                    Some(params.faculty[week_idx][(d + 0) % faculty_len].to_string())
                },
                quiz_grader: if is_exam {
                    None
                } else if faculty_len > 3 {
                    Some(params.faculty[week_idx][(d + 3) % faculty_len].to_string())
                } else {
                    Some(params.faculty[week_idx][(d + 2) % faculty_len].to_string())
                },
                drill1: if is_exam {
                    vec![]
                } else if faculty_len > 3 {
                    vec![
                        params.faculty[week_idx][(d + 3) % faculty_len].to_string(),
                        #[allow(clippy::identity_op)]
                        params.faculty[week_idx][(d + 0) % faculty_len].to_string(),
                        params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                    ]
                } else {
                    vec![
                        params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                        params.faculty[week_idx][(d + 2) % faculty_len].to_string(),
                    ]
                },
                drill2: if is_exam {
                    vec![]
                } else if faculty_len > 3 {
                    vec![
                        params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                        params.faculty[week_idx][(d + 2) % faculty_len].to_string(),
                        #[allow(clippy::identity_op)]
                        params.faculty[week_idx][(d + 0) % faculty_len].to_string(),
                    ]
                } else {
                    vec![
                        #[allow(clippy::identity_op)]
                        params.faculty[week_idx][(d + 0) % faculty_len].to_string(),
                        params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                    ]
                },
                noon_optional1: if is_exam {
                    None
                } else if faculty_len > 3 {
                    #[allow(clippy::identity_op)]
                    Some(params.faculty[week_idx][(d + 3) % faculty_len].to_string())
                } else {
                    Some(params.faculty[week_idx][(d + 1) % faculty_len].to_string())
                },
                noon_optional2: if is_exam {
                    None
                } else {
                    Some(params.faculty[week_idx][(d + 2) % faculty_len].to_string())
                },
                noon_optional1_title: if is_exam {
                    None
                } else {
                    Some(String::from("Grammar"))
                },
                noon_optional2_title: if is_exam {
                    None
                } else {
                    Some(String::from("Sight"))
                },
                lecture: if is_friday_review {
                    None
                } else {
                    #[allow(clippy::identity_op)]
                    Some(params.faculty[week_idx][(d + 0) % faculty_len].to_string())
                },
                lecture_title: if is_friday_review {
                    None
                } else {
                    match day_num {
                        1 => Some(String::from("Lecture on Accents")),
                        2..27 => Some(format!(
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
                        )),
                        _ => None,
                    }
                },
                voc_notes: if is_friday_review {
                    None
                } else {
                    Some(params.faculty[week_idx][(d + 1) % faculty_len].to_string())
                },
                friday_review1: if is_friday_review {
                    vec![
                        params.faculty[week_idx][(d + 1) % faculty_len].to_string(),
                        #[allow(clippy::identity_op)]
                        params.faculty[week_idx][(d + 0) % faculty_len].to_string(),
                    ]
                } else {
                    vec![]
                },
                friday_review2: if is_friday_review {
                    vec![
                        if faculty_len > 3 {
                            params.faculty[week_idx][(d + 3) % faculty_len].to_string()
                        } else {
                            #[allow(clippy::identity_op)]
                            params.faculty[week_idx][(d + 0) % faculty_len].to_string()
                        },
                        params.faculty[week_idx][(d + 2) % faculty_len].to_string(),
                    ]
                } else {
                    vec![]
                },
                other: None,
                test: vec![LgiClass {
                    title: String::from(""),
                    instructor: String::from(""),
                    handouts: Some(vec![String::from("")]),
                }],
            };
            day_num += 1;
            summer.days.push(day); //Box::new(day));
        }

        these_days = these_days.checked_add(one_day).unwrap();
        if these_days.weekday() == Weekday::Monday {
            week_idx += 1;
            faculty_len = params.faculty[week_idx].len();
        }
    }

    Some(summer)
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
    fn test_xml_roundtrip() {
        let start_date = "2025-06-09";
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

        let p = Params {
            faculty,
            start_date,
            holidays,
        };

        let s = create_summer(&p).unwrap();
        let xml = s.to_xml();
        //println!("{xml}");

        let s2 = Summer::from_xml(xml);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_get_seqs() {
        let start_date = "2025-06-09";
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

        let p = Params {
            faculty,
            start_date,
            holidays,
        };

        let s = create_summer(&p).unwrap();
        let seq = s.get_seqs(4);
        println!("seq: {seq:?}");
    }
}
