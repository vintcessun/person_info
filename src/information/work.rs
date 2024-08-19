use super::company::Company;
use super::duration::Duration;
use super::job::Job;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Work {
    //工作经历
    pub company: Company,
    pub duration: Duration,
    pub job: Vec<Job>,
}

impl Work {
    pub fn new() -> Self {
        Self {
            company: Company::new(),
            duration: Duration::new(),
            job: Vec::with_capacity(8),
        }
    }

    pub fn build(company: Company, duration: Duration, job: Vec<Job>) -> Self {
        Self {
            company,
            duration,
            job,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.company.is_empty() && self.duration.is_empty() && self.job.is_empty()
    }

    pub fn company(&self) -> Company {
        self.company.clone()
    }

    pub fn duration(&self) -> Duration {
        self.duration.clone()
    }

    pub fn job(&self) -> Vec<Job> {
        self.job.clone()
    }
}

impl Add for Work {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            company: update::renew_company(self.company, other.company),
            duration: update::renew_duration(self.duration, other.duration),
            job: update::add_vec_renew(self.job, other.job),
        }
    }
}

impl AddAssign for Work {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Work {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Work"))
        )
    }
}

impl PartialEq for Work {
    fn eq(&self, other: &Self) -> bool {
        self.company == other.company
    }
}
