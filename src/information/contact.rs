use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Contact {
    //联系方式
    pub phone: Vec<String>,
    pub qq: Vec<String>,
    pub wechat: Vec<String>,
}

impl Contact {
    pub fn new() -> Self {
        Self {
            phone: Vec::with_capacity(4),
            qq: Vec::with_capacity(4),
            wechat: Vec::with_capacity(4),
        }
    }

    pub fn build(phone: Vec<String>, qq: Vec<String>, wechat: Vec<String>) -> Self {
        Self { phone, qq, wechat }
    }

    pub fn is_empty(&self) -> bool {
        self.phone.is_empty() && self.qq.is_empty() && self.wechat.is_empty()
    }

    pub fn phone(&self) -> Vec<String> {
        self.phone.clone()
    }

    pub fn qq(&self) -> Vec<String> {
        self.qq.clone()
    }

    pub fn wechat(&self) -> Vec<String> {
        self.wechat.clone()
    }
}

impl Add for Contact {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            phone: update::add_vec(self.phone, other.phone),
            qq: update::add_vec(self.qq, other.qq),
            wechat: update::add_vec(self.wechat, other.wechat),
        }
    }
}

impl AddAssign for Contact {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Contact"))
        )
    }
}
