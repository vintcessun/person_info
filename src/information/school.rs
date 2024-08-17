use super::duration::Duration;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct School {
    pub name: String,
    pub duration: Duration,
}

impl School {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            duration: Duration::new(),
        }
    }

    pub fn builder(name: String, duration: Duration) -> Self {
        Self { name, duration }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.duration.is_empty()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn duration(&self) -> Duration {
        self.duration.clone()
    }
}

impl Add for School {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.name == other.name {
            Self {
                name: self.name(),
                duration: update::renew_duration(self.duration, other.duration),
            }
        } else {
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for School {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for School {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("School"))
        )
    }
}
