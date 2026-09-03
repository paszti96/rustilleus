//! Rust's alternatives to classes and implementation inheritance.
//!
//! A `struct` stores state, an `impl` block adds associated functions and methods,
//! and traits define shared behavior. Rust favors composition over inheritance.

pub fn run() {
    println!("3. STRUCTS, TRAITS, AND OOP");

    let teacher = Teacher::new("Grace", "Computer Science");
    let mut student = Student::new("Linus");
    student.enroll("Systems Programming");

    // Composition: Course contains a Teacher instead of inheriting from it.
    let course = Course::new("Practical Rust", teacher.clone());
    println!("   class-like struct + methods: {}", course.summary());

    // Trait objects provide runtime polymorphism (dynamic dispatch).
    let people: Vec<Box<dyn Person>> = vec![Box::new(teacher), Box::new(student)];
    for person in &people {
        println!("   dynamic dispatch: {}", person.describe());
    }

    // A generic function uses static dispatch and is normally monomorphized.
    let guest = Teacher::new("Barbara", "Compilers");
    println!("   static dispatch: {}\n", introduce(&guest));
}

/// Similar to a small class: fields hold state and `impl` defines behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teacher {
    name: String,
    subject: String,
}

impl Teacher {
    /// An associated constructor; Rust has no special `new` keyword.
    pub fn new(name: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            subject: subject.into(),
        }
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    name: String,
    courses: Vec<String>,
}

impl Student {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            courses: Vec::new(),
        }
    }

    pub fn enroll(&mut self, course: impl Into<String>) {
        self.courses.push(course.into());
    }

    pub fn course_count(&self) -> usize {
        self.courses.len()
    }
}

/// A trait is a shared behavioral contract, somewhat like an interface.
pub trait Person {
    fn name(&self) -> &str;
    fn role(&self) -> &'static str;

    /// Traits may supply default behavior, but do not inherit fields.
    fn describe(&self) -> String {
        format!("{} is a {}", self.name(), self.role())
    }
}

impl Person for Teacher {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &'static str {
        "teacher"
    }
}

impl Person for Student {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &'static str {
        "student"
    }

    fn describe(&self) -> String {
        format!(
            "{} is a student taking {} course(s)",
            self.name,
            self.course_count()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    title: String,
    instructor: Teacher,
}

impl Course {
    pub fn new(title: impl Into<String>, instructor: Teacher) -> Self {
        Self {
            title: title.into(),
            instructor,
        }
    }

    pub fn summary(&self) -> String {
        format!("{} is taught by {}", self.title, self.instructor.name())
    }
}

pub fn introduce(person: &impl Person) -> String {
    format!("Welcome, {}!", person.name())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods_manage_encapsulated_state() {
        let mut student = Student::new("Ada");
        student.enroll("Rust 101");
        assert_eq!(student.course_count(), 1);
        assert_eq!(student.describe(), "Ada is a student taking 1 course(s)");
    }

    #[test]
    fn trait_objects_hold_different_concrete_types() {
        let people: Vec<Box<dyn Person>> = vec![
            Box::new(Student::new("A")),
            Box::new(Teacher::new("B", "Rust")),
        ];
        let roles: Vec<&str> = people.iter().map(|person| person.role()).collect();
        assert_eq!(roles, ["student", "teacher"]);
    }
}
