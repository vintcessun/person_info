use std::ops::{Add, AddAssign};
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use crate::process::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Duration{//时期
    pub start: String,
    pub end: String,
}

impl Duration{
    pub fn new()->Self{
        Self{start:"".to_string(), end: "".to_string()}
    }

    pub fn builder(start: String, end: String)->Self{
        Self{start,end}
    }

    pub fn is_empty(&self)->bool{
        self.start.is_empty() && self.end.is_empty()
    }

    pub fn start(&self)->String{
        self.start.clone()
    }

    pub fn end(&self)->String{
        self.end.clone()
    }
}

impl Add for Duration{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            start: update::renew_tag(self.start, other.start),
            end: update::renew_tag(self.end, other.end)
        }
    }
}

impl AddAssign for Duration{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Duration{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Duration")))
    }
}

impl PartialEq for Duration{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start &&
        self.end == other.end
    }
}

impl Default for Duration{
    fn default() -> Self {
        Self::new()
    }
}