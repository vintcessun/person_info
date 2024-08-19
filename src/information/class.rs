use super::school::School;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Class {
    //班级信息
    pub class: String,
    pub class_qq: Vec<String>,
    pub grade: String,
    pub school: School,
}

impl Class {
    pub fn new() -> Self {
        Self {
            class: "".to_string(),
            class_qq: Vec::with_capacity(4),
            grade: "".to_string(),
            school: School::new(),
        }
    }

    pub fn build(class: String, class_qq: Vec<String>, grade: String, school: School) -> Self {
        Self {
            class,
            class_qq,
            grade,
            school,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.class.is_empty()
            && self.class_qq.is_empty()
            && self.grade.is_empty()
            && self.school.is_empty()
    }

    pub fn class(&self) -> String {
        self.class.clone()
    }

    pub fn class_qq(&self) -> Vec<String> {
        self.class_qq.clone()
    }

    pub fn grade(&self) -> String {
        self.grade.clone()
    }

    pub fn school(&self) -> School {
        self.school.clone()
    }
}

impl Add for Class {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut ret = Self {
            class: self.class(),
            class_qq: update::add_vec(self.class_qq, other.class_qq),
            grade: update::renew_tag(self.grade, other.grade),
            school: update::renew_school(self.school, other.school),
        };
        //这里是明显倾向于保留原本状态的
        if other.class.is_empty() || self.class == other.class {
            ret
        } else if self.class.is_empty() {
            ret.class = other.class;
            ret
        } else {
            panic!("{}", errors::panic_not_same())
        }
    }
}

impl AddAssign for Class {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Class"))
        )
    }
}

impl EqualStatement<String> for Class {
    fn statement(&self) -> &String {
        &self.class
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.statement() == other.statement()
    }
}
