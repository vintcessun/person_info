use super::duration::Duration;
use super::job::Job;
use super::school::School;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Education {
    //教育经历
    pub school: School,
    pub duration: Duration,
    pub job: Vec<Job>,
}

impl Education {
    pub fn new() -> Self {
        Self {
            school: School::new(),
            duration: Duration::new(),
            job: Vec::with_capacity(8),
        }
    }

    pub fn builder(school: School, duration: Duration, job: Vec<Job>) -> Self {
        Self {
            school,
            duration,
            job,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.school.is_empty() && self.duration.is_empty() && self.job.is_empty()
    }

    pub fn school(&self) -> School {
        self.school.clone()
    }

    pub fn duration(&self) -> Duration {
        self.duration.clone()
    }

    pub fn job(&self) -> Vec<Job> {
        self.job.clone()
    }
}

impl Add for Education {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            school: update::renew_school(self.school, other.school),
            duration: update::renew_duration(self.duration, other.duration),
            job: update::add_vec_renew(self.job, other.job),
        }
    }
}

impl AddAssign for Education {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Education {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Education"))
        )
    }
}

impl PartialEq for Education {
    fn eq(&self, other: &Self) -> bool {
        self.school == other.school
    }
}
