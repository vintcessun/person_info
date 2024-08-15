use std::ops::{Add, AddAssign};
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use super::contact::Contact;
use crate::process::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonId{//个人隐私信息
    pub id: String,
    pub drive_id: String,
    pub contact: Contact,
}

impl PersonId{
    pub fn new()->Self{
        Self{id:"".to_string(), drive_id:"".to_string(),contact:Contact::new()}
    }

    pub fn build(id: String, drive_id:String, contact: Contact)->Self{
        Self{id,drive_id,contact}
    }

    pub fn is_empty(&self)->bool{
        self.id.is_empty() &&
        self.drive_id.is_empty()
    }

    pub fn id(&self)->String{
        self.id.clone()
    }

    pub fn drive_id(&self)->String{
        self.drive_id.clone()
    }

    pub fn contact(&self)->Contact{
        self.contact.clone()
    }
}

impl Add for PersonId{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            id: update::renew_tag(self.id , other.id),
            drive_id: update::renew_tag(self.drive_id, other.drive_id),
            contact: update::renew_contact(self.contact, other.contact),
        }
    }
}

impl AddAssign for PersonId{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for PersonId{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("PersonId")))
    }
}

impl PartialEq for PersonId{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.drive_id == other.drive_id
    }
}

impl Default for PersonId{
    fn default() -> Self {
        Self::new()
    }
}