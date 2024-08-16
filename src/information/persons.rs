use std::ops::{Add, AddAssign, Index, IndexMut};
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use std::iter::{IntoIterator, Iterator};
use super::person::Person;
use crate::process::*;

#[derive(Debug, Clone)]
pub struct Persons{//众人信息
    persons: Vec<Person>,
}

impl Persons{
    pub fn new()->Self{
        Self{persons:vec![]}
    }

    pub fn with_capacity(len: usize)->Self{
        Self{persons:Vec::with_capacity(len)}
    }

    pub fn contains(&self, index: &str)->bool{
        let mut ret = false;
        for person in &self.persons{
            if person.name == index{
                ret = true;
            }
        }
        ret
    }

    pub fn builder(persons: Vec<Person>)->Self{
        Self{persons}
    }

    pub fn to_vec(&self)->Vec<Person>{
        self.persons.clone()
    }

    pub fn persons(&self)->Vec<Person>{
        self.to_vec()
    }

    pub fn len(&self)->usize{
        self.persons.len()
    }

    pub fn push(&mut self, person: Person){
        self.persons.push(person)
    }

    pub fn extend(&mut self, other: Persons){
        self.persons = update::add_vec(self.persons(), other.persons);
    }

    pub fn pop(&mut self)->Option<Person>{
        self.persons.pop()
    }

    pub fn capacity(&self)->usize{
        self.persons.capacity()
    }

    pub fn reserve(&mut self, additional: usize){
        self.persons.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize){
        self.persons.reserve_exact(additional)
    }

    pub fn try_reserve(&mut self, additional: usize)->std::result::Result<(), std::collections::TryReserveError>{
        self.persons.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize)->std::result::Result<(), std::collections::TryReserveError>{
        self.persons.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self){
        self.persons.shrink_to_fit()
    }

    pub fn shrink_to(&mut self, min_capacity: usize){
        self.persons.shrink_to(min_capacity)
    }

    pub fn into_boxed_slice(self)->Box<[Person]>{
        self.persons.into_boxed_slice()
    }

    pub fn truncate(&mut self, len: usize){
        self.persons.truncate(len)
    }

    pub fn as_slice(&self)->&[Person]{
        self.persons.as_slice()
    }

    pub fn as_mut_slice(&mut self)->&mut [Person]{
        self.persons.as_mut_slice()
    }

    pub fn as_ptr(&self)->*const Person{
        self.persons.as_ptr()
    }

    pub fn as_mut_ptr(&mut self)->*mut Person{
        self.persons.as_mut_ptr()
    }

    pub fn swap_remove(&mut self, index: usize)->Person{
        self.persons.swap_remove(index)
    }

    pub fn insert(&mut self, index: usize, element: Person){
        self.persons.insert(index, element)
    }

    pub fn remove(&mut self, index: usize) -> Person{
        self.persons.remove(index)
    }

    pub fn append(&mut self, other: &mut Self){
        self.persons.append(&mut other.persons)
    }

    pub fn clear(&mut self){
        self.persons.clear()
    }

    pub fn is_empty(&self)->bool{
        self.persons.is_empty()
    }

    pub fn split_off(&mut self, at: usize)->Vec<Person>{
        self.persons.split_off(at)
    }

    pub fn leak<'a>(self) -> &'a mut [Person]{
        self.persons.leak()
    }

    pub fn spare_capacity_mut(&mut self)->&mut [std::mem::MaybeUninit<Person>]{
        self.persons.spare_capacity_mut()
    }

    pub fn resize(&mut self, new_len: usize, value: Person){
        self.persons.resize(new_len, value)
    }

    pub fn extend_from_slice(&mut self, other: &[Person]){
        self.persons.extend_from_slice(other)
    }
}

impl Add for Persons{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{persons:update::add_vec(self.persons, other.persons)}
    }
}

impl AddAssign for Persons{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other
    }
}

impl Index<usize> for Persons{
    type Output = Person;

    fn index(&self, index: usize) -> &Self::Output{
        &self.persons[index]
    }
}

impl Index<&str> for Persons{
    type Output = Person;

    fn index(&self, index: &str) -> &Self::Output {
        for person in &self.persons{
            if person.name == index{
                return person;
            }
        }
        panic!("{}", errors::panic_not_found());
    }
}

impl Index<String> for Persons{
    type Output = Person;

    fn index(&self, index: String) -> &Self::Output {
        &self[index.as_str()]
    }
}

impl IndexMut<usize> for Persons{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.persons[index]
    }
}

impl IndexMut<&str> for Persons{
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        let mut date = false;
        let mut target = 0;
        for i in 0..self.len(){
            if self[i].name == index{
                date = true;
                target = i;
            }
        }

        if !date{
            self.push(Person::new(index.to_string()));
            target = self.len()-1;
        }

        &mut self[target]
    }
}

impl IndexMut<String> for Persons{
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        &mut self[index.as_str()]
    }
}

impl Display for Persons{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Persons")))
    }
}

impl PartialEq for Persons{
    fn eq(&self, other: &Self) -> bool {
        self.persons == other.persons
    }
}

impl IntoIterator for Persons{
    type Item = Person;
    type IntoIter = IteratorPersons;

    fn into_iter(self) -> Self::IntoIter {
        IteratorPersons::new(self)
    }
}

impl Default for Persons{
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Persons{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.persons.len()))?;
        for person in &self.persons{
            seq.serialize_element(person)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Persons{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        deserializer.deserialize_seq(PersonsVisitor)
    }
}

pub struct PersonsVisitor;

impl<'de> Visitor<'de> for PersonsVisitor{
    type Value = Persons;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("The Persons is not the correct form")
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>, {
        let mut persons = Persons::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(data) = seq.next_element()?{
            persons.push(data);
        }
        Ok(persons)
    }
}

pub struct IteratorPersons{//众人信息的迭代器
    count: usize,
    persons: Persons,
}

impl IteratorPersons{
    fn new(persons: Persons)->Self{
        Self{count:0,persons}
    }

    pub fn len(&self)->usize{
        self.persons.len()
    }

    pub fn is_empty(&self)->bool{
        self.len() == 0
    }
}

impl Index<usize> for IteratorPersons{
    type Output = Person;

    fn index(&self, index: usize) -> &Self::Output{
        &self.persons[index]
    }
}

impl Index<&str> for IteratorPersons{
    type Output = Person;

    fn index(&self, index: &str) -> &Self::Output {
        &self.persons[index]
    }
}

impl Index<String> for IteratorPersons{
    type Output = Person;

    fn index(&self, index: String) -> &Self::Output {
        &self.persons[index]
    }
}

impl IndexMut<usize> for IteratorPersons{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.persons[index]
    }
}

impl IndexMut<&str> for IteratorPersons{
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut self.persons[index]
    }
}

impl IndexMut<String> for IteratorPersons{
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        &mut self.persons[index]
    }
}

impl Iterator for IteratorPersons{
    type Item = Person;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.count < self.len(){
            Some(self.persons[self.count].clone())
        }
        else{
            None
        };
        self.count += 1;
        ret
    }
}