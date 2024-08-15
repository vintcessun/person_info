use std::ops::{Add, AddAssign, Index, IndexMut};
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Display, Result};
use std::cmp::PartialEq;
use std::iter::{IntoIterator, Iterator};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job{//职位
    pub name: String,
    pub duration: Duration,
}

impl Job{
    pub fn new()->Self{
        Self{name:"".to_string(),duration:Duration::new()}
    }

    pub fn builder(name: String, duration: Duration)->Self{
        Self{name,duration}
    }

    pub fn is_empty(&self)->bool{
        self.name.is_empty() &&
        self.duration.is_empty()
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn duration(&self)->Duration{
        self.duration.clone()
    }
}

impl Add for Job{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.name == other.name{
            Self{
                name: self.name(),
                duration: update::renew_duration(self.duration, other.duration)
            }
        }
        else{
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for Job{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Job{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Job")))
    }
}

impl PartialEq for Job{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.duration == other.duration
    }
}

impl Default for Job{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct School{
    pub name: String,
    pub duration: Duration,
}

impl School{
    pub fn new()->Self{
        Self{name:"".to_string(),duration: Duration::new()}
    }

    pub fn builder(name: String, duration: Duration)->Self{
        Self{name,duration}
    }

    pub fn is_empty(&self)->bool{
        self.name.is_empty() &&
        self.duration.is_empty()
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn duration(&self)->Duration{
        self.duration.clone()
    }
}

impl Add for School{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.name == other.name{
            Self{
                name: self.name(),
                duration: update::renew_duration(self.duration, other.duration),
            }
        }
        else{
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for School{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for School{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("School")))
    }
}

impl PartialEq for School{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.duration == other.duration
    }
}

impl Default for School{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Education{//教育经历
    pub school: School,
    pub duration: Duration,
    pub job: Vec<Job>,
}

impl Education{
    pub fn new()->Self{
        Self{school:School::new(),duration:Duration::new(),job:Vec::with_capacity(8)}
    }

    pub fn builder(school:School, duration:Duration, job: Vec<Job>)->Self{
        Self{school,duration,job}
    }

    pub fn is_empty(&self)->bool{
        self.school.is_empty() &&
        self.duration.is_empty() &&
        self.job.is_empty()
    }

    pub fn school(&self)->School{
        self.school.clone()
    }

    pub fn duration(&self)->Duration{
        self.duration.clone()
    }

    pub fn job(&self)->Vec<Job>{
        self.job.clone()
    }
}

impl Add for Education{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            school: update::renew_school(self.school, other.school),
            duration: update::renew_duration(self.duration, other.duration),
            job: update::add_vec(self.job, other.job),
        }
    }
}

impl AddAssign for Education{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Education{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Education")))
    }
}

impl PartialEq for Education{
    fn eq(&self, other: &Self) -> bool {
        self.school == other.school &&
        self.duration == other.duration &&
        self.job == other.job
    }
}

impl Default for Education{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Company{//公司信息
    pub name: String,
    pub duration: Duration,
}

impl Company{
    pub fn new()->Self{
        Self{name:"".to_string(),duration:Duration::new()}
    }

    pub fn builder(name: String, duration: Duration)->Self{
        Self{name,duration}
    }

    pub fn is_empty(&self)->bool{
        self.name.is_empty() &&
        self.duration.is_empty()
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn duration(&self)->Duration{
        self.duration.clone()
    }
}

impl Add for Company{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.name == other.name{
            Self{
                name:self.name,
                duration:update::renew_duration(self.duration, other.duration)
            }
        }
        else{
            panic!("{}",errors::panic_not_same())
        }
    }
}

impl AddAssign for Company{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Company{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Company")))
    }
}

impl PartialEq for Company{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.duration == other.duration
    }
}

impl Default for Company{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Work{//工作经历
    pub company: Company,
    pub duration: Duration,
    pub job: Vec<Job>,
}

impl Work{
    pub fn new()->Self{
        Self{company:Company::new(), duration:Duration::new(), job:Vec::with_capacity(8)}
    }

    pub fn build(company: Company, duration: Duration, job: Vec<Job>)->Self{
        Self{company,duration,job}
    }

    pub fn is_empty(&self)->bool{
        self.company.is_empty() &&
        self.duration.is_empty() &&
        self.job.is_empty()
    }

    pub fn company(&self)->Company{
        self.company.clone()
    }

    pub fn duration(&self)->Duration{
        self.duration.clone()
    }

    pub fn job(&self)->Vec<Job>{
        self.job.clone()
    }
}

impl Add for Work{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self{
            company: update::renew_company(self.company, other.company),
            duration: update::renew_duration(self.duration, other.duration),
            job: update::add_vec(self.job, other.job),
        }
    }
}

impl AddAssign for Work{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Work{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Company")))
    }
}

impl PartialEq for Work{
    fn eq(&self, other: &Self) -> bool {
        self.company == other.company &&
        self.duration == other.duration &&
        self.job == other.job
    }
}

impl Default for Work{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relation{//亲属关系
    pub name: String,
    pub phone: Vec<String>,
    pub from: String,
    pub sex: String,
    pub qq: Vec<String>,
    pub wechat: Vec<String>,
    pub relate_name: String,
    pub education: Vec<Education>,
    pub work: Vec<Work>,
}

impl Relation{
    pub fn new(name: String)->Self{
        Self{name, phone:Vec::with_capacity(2),from:"".to_string(),sex:"".to_string(),qq:Vec::with_capacity(2),wechat:Vec::with_capacity(2),
        relate_name: "".to_string(), education:Vec::with_capacity(8), work:Vec::with_capacity(8)}
    }

    pub fn builder(name: String, phone: Vec<String>, from: String, sex: String, qq: Vec<String>, wechat: Vec<String>, relate_name: String,
        education: Vec<Education>,work: Vec<Work>,)->Self{
        Self{name,phone,from,sex,qq,wechat,relate_name,education,work}
    }

    pub fn is_empty(&self)->bool{
        self.name.is_empty() &&
        self.phone.is_empty() &&
        self.from.is_empty() &&
        self.sex.is_empty() &&
        self.qq.is_empty() &&
        self.wechat.is_empty() &&
        self.relate_name.is_empty() &&
        self.education.is_empty() &&
        self.work.is_empty()
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn phone(&self)->Vec<String>{
        self.phone.clone()
    }

    pub fn from(&self)->String{
        self.from.clone()
    }

    pub fn sex(&self)->String{
        self.sex.clone()
    }

    pub fn qq(&self)->Vec<String>{
        self.qq.clone()
    }

    pub fn wechat(&self)->Vec<String>{
        self.wechat.clone()
    }

    pub fn relate_name(&self)->String{
        self.relate_name.clone()
    }
}

impl Add for Relation{
    type Output = Self;

    fn add(self, other: Self)->Self{
        if self.name == other.name{
            Self { name: self.name,
                phone: update::add_vec(self.phone, other.phone),
                from: update::renew_tag(self.from, other.from),
                sex: update::renew_tag(self.sex, other.sex),
                qq: update::add_vec(self.qq, other.qq),
                wechat: update::add_vec(self.wechat, other.wechat),
                relate_name: update::renew_tag(self.relate_name, other.relate_name),
                education: update::add_vec(self.education, other.education),
                work: update::add_vec(self.work, other.work),
            }
        }
        else{
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for Relation{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Relation{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Relation")))
    }
}

impl PartialEq for Relation{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.phone == other.phone &&
        self.from == other.from &&
        self.sex == other.sex &&
        self.qq == other.qq &&
        self.wechat == other.wechat &&
        self.relate_name == other.relate_name
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person{//个人信息
    pub name: String,
    pub phone: Vec<String>,
    pub category: String,
    pub from: String,
    pub sex: String,
    pub qq: Vec<String>,
    pub wechat: Vec<String>,
    pub class: String,
    pub class_qq: Vec<String>,
    pub relations: Vec<Relation>,
    pub education: Vec<Education>,
    pub work: Vec<Work>,
    pub grade: String,
    pub school: School,
}

impl Person{
    pub fn new(name: String)->Self{
        Person{name,phone:Vec::with_capacity(2),category:"".to_string(),from:"".to_string(),sex:"".to_string(),qq:Vec::with_capacity(2),
            wechat:Vec::with_capacity(2),class:"".to_string(),class_qq:Vec::with_capacity(2),relations:Vec::with_capacity(4),
            education:Vec::with_capacity(8), work:Vec::with_capacity(8), grade:"".to_string(), school:School::new()}
    }

    pub fn builder(name: String, phone: Vec<String>, category: String, from: String, sex: String, qq: Vec<String>, wechat: Vec<String>,
        class: String, class_qq: Vec<String>, relations: Vec<Relation>, education: Vec<Education>, work: Vec<Work>, grade: String,
        school: School)->Self{
        Self{name,phone,category,from,sex,qq,wechat,class,class_qq,relations,education,work,grade,school}
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn phone(&self)->Vec<String>{
        self.phone.clone()
    }

    pub fn category(&self)->String{
        self.category.clone()
    }

    pub fn from(&self)->String{
        self.from.clone()
    }

    pub fn sex(&self)->String{
        self.sex.clone()
    }

    pub fn qq(&self)->Vec<String>{
        self.qq.clone()
    }

    pub fn wechat(&self)->Vec<String>{
        self.wechat.clone()
    }

    pub fn class(&self)->String{
        self.class.clone()
    }

    pub fn class_qq(&self)->Vec<String>{
        self.class_qq.clone()
    }

    pub fn relations(&self)->Vec<Relation>{
        self.relations.clone()
    }
}

impl Add for Person{
    type Output = Self;

    fn add(self, other: Self)->Self{
        if self.name == other.name{
            Self { name: self.name,
                phone: update::add_vec(self.phone, other.phone),
                category: update::renew_tag(self.category, other.category),
                from: update::renew_tag(self.from, other.from),
                sex: update::renew_tag(self.sex, other.sex),
                qq: update::add_vec(self.qq, other.qq),
                wechat: update::add_vec(self.wechat, other.wechat),
                class: update::renew_tag(self.class, other.class),
                relations: update::add_vec(self.relations, other.relations),
                class_qq: update::add_vec(self.class_qq, other.class_qq),
                education: update::add_vec(self.education, other.education),
                work: update::add_vec(self.work, other.work),
                grade: update::renew_tag(self.grade, other.grade),
                school: update::renew_school(self.school, other.school),
            }
        }
        else{
            panic!("{}", errors::panic_not_same());
        }
    }
}

impl AddAssign for Person{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Display for Person{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or(errors::display_error("Person")))
    }
}

impl PartialEq for Person{
    fn eq(&self, other: &Person) -> bool{
        self.name == other.name && 
        self.phone == other.phone &&
        self.category == other.category &&
        self.from == other.from &&
        self.sex == other.sex &&
        self.qq == other.qq &&
        self.wechat == other.wechat &&
        self.class == other.class &&
        self.relations == other.relations &&
        self.class_qq == other.class_qq
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

mod update{
    use super::*;

    pub fn add_vec<T>(vec1:Vec<T>, vec2:Vec<T>)->Vec<T>{
        let mut ret = Vec::with_capacity(vec1.len()+vec2.len());
        ret.extend(vec1);
        ret.extend(vec2);
        ret
    }

    pub fn renew_tag(old_tag: String, new_tag: String)->String{
        if new_tag.is_empty(){
            old_tag
        }
        else{
            new_tag
        }
    }

    pub fn renew_school(old_tag: School, new_tag: School)->School{
        old_tag + new_tag
    }

    pub fn renew_duration(old_tag: Duration, new_tag: Duration)->Duration{
        old_tag + new_tag
    }

    pub fn renew_company(old_tag: Company, new_tag: Company)->Company{
        old_tag + new_tag
    }
}

mod errors{
    pub fn panic_not_same()->&'static str{
        "You cannot add two different name"
    }

    pub fn panic_not_found()->&'static str{
        "There is not name found"
    }

    pub fn display_error(t: &str)->String{
        format!("Some error(serde_json_error) happened in the Display of {}",t)
    }
}