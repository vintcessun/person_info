use std::ops::{Add, AddAssign};
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use super::school::School;
use crate::process::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Class{//班级信息
    pub class: String,
    pub class_qq: Vec<String>,
    pub grade: String,
    pub school: School,
}

impl Class{
    pub fn new()->Self{
        Self{class:"".to_string(), class_qq:Vec::with_capacity(4), grade:"".to_string(), school: School::new()}
    }

    pub fn build(class:String, class_qq:Vec<String>, grade: String, school:School)->Self{
        Self{class,class_qq,grade,school}
    }

    pub fn is_empty(&self)->bool{
        self.class.is_empty() &&
        self.class_qq.is_empty() &&
        self.grade.is_empty() &&
        self.school.is_empty()
    }

    pub fn class(&self)->String{
        self.class.clone()
    }

    pub fn class_qq(&self)->Vec<String>{
        self.class_qq.clone()
    }

    pub fn grade(&self)->String{
        self.grade.clone()
    }

    pub fn school(&self)->School{
        self.school.clone()
    }
}

impl Add for Class{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.class == other.class{
            Self{
                class: self.class,
                class_qq: update::add_vec(self.class_qq, other.class_qq),
                grade: update::renew_tag(self.grade, other.grade),
                school: update::renew_school(self.school, other.school),
            }
        }
        else{
            panic!("{}", errors::panic_not_same())
        }
    }
}

impl AddAssign for Class{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Class{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Class")))
    }
}

impl PartialEq for Class{
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class &&
        self.class_qq == other.class_qq &&
        self.grade == other.grade &&
        self.school == other.school
    }
}

impl Default for Class{
    fn default() -> Self {
        Self::new()
    }
}