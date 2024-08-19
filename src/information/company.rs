use super::duration::Duration;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Company {
    //公司信息
    pub name: String,
    pub duration: Duration,
}

impl Company {
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

impl Add for Company {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut ret = Self {
            name: self.name(),
            duration: update::renew_duration(self.duration, other.duration),
        };
        //这里是明显倾向于保留原本状态的
        if other.name.is_empty() || self.name == other.name {
            ret
        } else if self.name.is_empty() {
            ret.name = other.name;
            ret
        } else {
            panic!("{}", errors::panic_not_same())
        }
    }
}

impl AddAssign for Company {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Company {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Company"))
        )
    }
}

impl EqualStatement<String> for Company {
    fn statement(&self) -> &String {
        &self.name
    }
}

impl PartialEq for Company {
    fn eq(&self, other: &Self) -> bool {
        self.statement() == other.statement()
    }
}
