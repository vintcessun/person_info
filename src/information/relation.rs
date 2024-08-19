use super::education::Education;
use super::personid::PersonId;
use super::work::Work;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relation {
    //亲属关系
    pub name: String,
    pub from: String,
    pub sex: String,
    pub relate_name: String,
    pub education: Vec<Education>,
    pub work: Vec<Work>,
    pub id: PersonId,
}

impl Relation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            from: "".to_string(),
            sex: "".to_string(),
            relate_name: "".to_string(),
            education: Vec::with_capacity(8),
            work: Vec::with_capacity(8),
            id: PersonId::new(),
        }
    }

    pub fn builder(
        name: String,
        from: String,
        sex: String,
        relate_name: String,
        education: Vec<Education>,
        work: Vec<Work>,
        id: PersonId,
    ) -> Self {
        Self {
            name,
            from,
            sex,
            relate_name,
            education,
            work,
            id,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.from.is_empty()
            && self.sex.is_empty()
            && self.relate_name.is_empty()
            && self.education.is_empty()
            && self.work.is_empty()
            && self.id.is_empty()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn sex(&self) -> String {
        self.sex.clone()
    }

    pub fn relate_name(&self) -> String {
        self.relate_name.clone()
    }

    pub fn education(&self) -> Vec<Education> {
        self.education.clone()
    }

    pub fn work(&self) -> Vec<Work> {
        self.work.clone()
    }

    pub fn id(&self) -> PersonId {
        self.id.clone()
    }
}

impl Add for Relation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut ret = Self {
            name: other.name(),
            from: update::renew_tag(self.from, other.from),
            sex: update::renew_tag(self.sex, other.sex),
            relate_name: update::renew_tag(self.relate_name, other.relate_name),
            education: update::add_vec_renew(self.education, other.education),
            work: update::add_vec_renew(self.work, other.work),
            id: update::renew_personid(self.id, other.id),
        };
        //这里是明显倾向于保留原本状态的
        if other.name.is_empty() || self.name == other.name {
            ret
        } else if self.name.is_empty() {
            ret.name = other.name;
            ret
        } else {
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for Relation {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Relation"))
        )
    }
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
