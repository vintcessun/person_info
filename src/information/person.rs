use super::education::Education;
use super::relation::Relation;
use super::studentid::StudentId;
use super::work::Work;
use crate::process::*;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Person {
    //个人信息
    pub name: String,
    pub category: String,
    pub from: String,
    pub sex: String,
    pub relations: Vec<Relation>,
    pub education: Vec<Education>,
    pub work: Vec<Work>,
    pub id: StudentId,
}

impl Person {
    pub fn new(name: String) -> Self {
        Person {
            name,
            category: "".to_string(),
            from: "".to_string(),
            sex: "".to_string(),
            relations: Vec::with_capacity(4),
            education: Vec::with_capacity(8),
            work: Vec::with_capacity(8),
            id: StudentId::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn builder(
        name: String,
        category: String,
        from: String,
        sex: String,
        relations: Vec<Relation>,
        education: Vec<Education>,
        work: Vec<Work>,
        id: StudentId,
    ) -> Self {
        Self {
            name,
            category,
            from,
            sex,
            relations,
            education,
            work,
            id,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn category(&self) -> String {
        self.category.clone()
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn sex(&self) -> String {
        self.sex.clone()
    }

    pub fn relations(&self) -> Vec<Relation> {
        self.relations.clone()
    }

    pub fn education(&self) -> Vec<Education> {
        self.education.clone()
    }

    pub fn work(&self) -> Vec<Work> {
        self.work.clone()
    }

    pub fn id(&self) -> StudentId {
        self.id.clone()
    }
}

impl Add for Person {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut ret = Self {
            name: self.name(),
            category: update::renew_tag(self.category, other.category),
            from: update::renew_tag(self.from, other.from),
            sex: update::renew_tag(self.sex, other.sex),
            relations: update::add_vec_renew(self.relations, other.relations),
            education: update::add_vec_renew(self.education, other.education),
            work: update::add_vec_renew(self.work, other.work),
            id: update::renew_studentid(self.id, other.id),
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

impl AddAssign for Person {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(errors::display_error("Person"))
        )
    }
}

impl EqualStatement<String> for Person {
    fn statement(&self) -> &String {
        &self.name
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.statement() == other.statement()
    }
}
