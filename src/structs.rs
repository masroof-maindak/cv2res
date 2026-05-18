use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CV {
    name: String,
    phone: String,
    email: String,
    site: String,
    github: String,
    linkedin: String,
    edu: Option<Vec<Education>>,
    wrk: Option<Vec<Work>>,
    projs: Option<Vec<Proj>>,
    small_projs: Option<Vec<SmallProj>>,
    hobby_projs: Option<Vec<HobbyProj>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    _start_date: Date,
    _end_date: Date,
    _institute: String,
    _location: String,
    _degree: String,
    _major: String,
    _desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    _start_date: Date,
    _end_date: Date,
    _title: String,
    _company: String,
    _desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    _year: usize,
    _month: usize,
    _day: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proj {
    _title: String,
    _url: String,
    _stack: String,
    _desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallProj {
    _title: String,
    _url: String,
    _desc: Vec<String>,
}

type HobbyProj = SmallProj;
