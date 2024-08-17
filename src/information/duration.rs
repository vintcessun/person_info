use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Duration {
    //时期
    pub start: String,
    pub end: String,
}

impl Duration {
    pub fn new() -> Self {
        Self {
            start: "".to_string(),
            end: "".to_string(),
        }
    }

    pub fn builder(start: String, end: String) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start.is_empty() && self.end.is_empty()
    }

    pub fn start(&self) -> String {
        self.start.clone()
    }

    pub fn end(&self) -> String {
        self.end.clone()
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            start: update::renew_tag(self.start, other.start),
            end: update::renew_tag(self.end, other.end),
        }
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Duration"))
        )
    }
}
