use std::collections::HashMap;

pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let reference = self.students.entry(grade).or_default();
        reference.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.keys().cloned().collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut students = self.students.get(&grade).cloned()?;
        students.sort();
        Some(students)
    }
}
