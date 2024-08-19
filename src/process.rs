use crate::information::*;

pub mod update {
    use std::ops::AddAssign;

    use super::*;

    pub fn add_vec(vec1: Vec<String>, vec2: Vec<String>) -> Vec<String> {
        let mut ret = Vec::with_capacity(vec1.len() + vec2.len());
        ret.extend(vec1);
        for per in vec2 {
            if !ret.contains(&per) {
                ret.push(per);
            }
        }
        ret
    }

    pub fn add_vec_renew<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T>
    where
        T: PartialEq + AddAssign,
    {
        let capacity = vec1.len() + vec2.len();
        let mut ret = Vec::with_capacity(capacity);
        ret.extend(vec1);
        for per in vec2 {
            let mut pos: i32 = -1;
            for (i, each) in ret.iter().enumerate() {
                if &per == each {
                    pos = i as i32;
                }
            }
            if pos == -1 {
                ret.push(per);
            } else {
                ret[pos as usize] += per;
            }
        }
        ret
    }

    pub fn renew_tag(old_tag: String, new_tag: String) -> String {
        if new_tag.is_empty() {
            old_tag
        } else {
            new_tag
        }
    }

    pub fn renew_school(old_tag: School, new_tag: School) -> School {
        old_tag + new_tag
    }

    pub fn renew_duration(old_tag: Duration, new_tag: Duration) -> Duration {
        old_tag + new_tag
    }

    pub fn renew_company(old_tag: Company, new_tag: Company) -> Company {
        old_tag + new_tag
    }

    pub fn renew_personid(old_tag: PersonId, new_tag: PersonId) -> PersonId {
        old_tag + new_tag
    }

    pub fn renew_studentid(old_tag: StudentId, new_tag: StudentId) -> StudentId {
        old_tag + new_tag
    }

    pub fn renew_contact(old_tag: Contact, new_tag: Contact) -> Contact {
        old_tag + new_tag
    }

    pub fn renew_class(old_tag: Class, new_tag: Class) -> Class {
        old_tag + new_tag
    }
}

pub mod errors {
    pub fn panic_not_same() -> &'static str {
        "You cannot add two different name"
    }

    pub fn panic_not_found() -> &'static str {
        "There is not name found"
    }

    pub fn display_error(t: &str) -> String {
        format!(
            "Some error(serde_json_error) happened in the Display of {}",
            t
        )
    }
}
