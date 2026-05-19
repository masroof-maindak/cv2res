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
    start_date: Date,
    end_date: Date,
    institute: String,
    location: String,
    degree: String,
    major: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    start_date: Date,
    end_date: Date,
    title: String,
    company: String,
    desc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
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
