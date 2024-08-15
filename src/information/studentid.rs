use std::ops::{Add, AddAssign};
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use super::contact::Contact;
use super::class::Class;
use crate::process::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StudentId{//学生隐私信息
    pub id: String,
    pub student_id: String,
    pub drive_id: String,
    pub contact: Contact,
    pub class: Class,
}

impl StudentId{
    pub fn new()->Self{
        Self{id:"".to_string(), student_id:"".to_string(), drive_id:"".to_string(),contact:Contact::new(),class:Class::new()}
    }

    pub fn build(id: String, student_id:String, drive_id:String, contact: Contact, class: Class)->Self{
        Self{id,student_id,drive_id,contact,class}
    }

    pub fn is_empty(&self)->bool{
        self.id.is_empty() &&
        self.student_id.is_empty() &&
        self.drive_id.is_empty() &&
        self.contact.is_empty() &&
        self.class.is_empty()
    }

    pub fn id(&self)->String{
        self.id.clone()
    }

    pub fn student_id(&self)->String{
        self.student_id.clone()
    }

    pub fn drive_id(&self)->String{
        self.drive_id.clone()
    }

    pub fn contact(&self)->Contact{
        self.contact.clone()
    }

    pub fn class(&self)->Class{
        self.class.clone()
    }
}

impl Add for StudentId{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            id: update::renew_tag(self.id , other.id),
            student_id: update::renew_tag(self.student_id, other.student_id),
            drive_id: update::renew_tag(self.drive_id, other.drive_id),
            contact: update::renew_contact(self.contact, other.contact),
            class: update::renew_class(self.class, other.class),
        }
    }
}

impl AddAssign for StudentId{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for StudentId{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("StudentId")))
    }
}

impl PartialEq for StudentId{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.student_id == other.student_id &&
        self.drive_id == other.drive_id
    }
}

impl Default for StudentId{
    fn default() -> Self {
        Self::new()
    }
}