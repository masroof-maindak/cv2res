use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CV {
    name: String,
    phone: String,
    email: String,
    site: String,
    github: String,
    linkedin: String,
    institutes: Option<Vec<Education>>,
    workplaces: Option<Vec<Work>>,
    projects: Option<Vec<Proj>>,
    small_projects: Option<Vec<SmallProj>>,
    hobby_projects: Option<Vec<HobbyProj>>,
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
    title: String,
    url: String,
    stack: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallProj {
    title: String,
    url: String,
    desc: Vec<String>,
}

type HobbyProj = SmallProj;
