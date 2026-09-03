//! # Lesson 3: Object-oriented ideas in Rust
//!
//! Object-oriented programming (OOP) groups data with behavior. Rust supports
//! this goal but deliberately has no classes or implementation inheritance.
//! Instead, it separates the job into smaller tools:
//!
//! - A `struct` stores an object's data, also called its **state**.
//! - An `impl` block adds constructors and methods to a type.
//! - A `trait` describes behavior that several types can share.
//! - **Composition** builds a larger type by placing smaller types inside it.
//!
//! "Polymorphism" means writing code that works with more than one concrete type.
//! Rust offers compile-time polymorphism with generics and runtime polymorphism
//! with trait objects such as `Box<dyn Person>`.

/// Runs and prints the object-oriented examples.
pub fn run() {
    println!("3. STRUCTS, TRAITS, AND OOP");

    let teacher = Teacher::new("Grace", "Computer Science");
    let mut student = Student::new("Linus");
    student.enroll("Systems Programming");

    // Composition: Course owns a Teacher as one of its fields. This makes the
    // relationship clear: a course *has an* instructor. There is no confusing
    // claim that one kind of object "is a" different kind of object.
    let course = Course::new("Practical Rust", teacher.clone());
    println!("   class-like struct + methods: {}", course.summary());

    // Trait objects provide runtime polymorphism, also called dynamic dispatch.
    // `Box` stores each value on the heap. `dyn Person` means the exact type may
    // differ, but it promises to implement Person. This lets one Vec contain a
    // Teacher and a Student even though they have different fields and sizes.
    let people: Vec<Box<dyn Person>> = vec![Box::new(teacher), Box::new(student)];
    for person in &people {
        println!("   dynamic dispatch: {}", person.describe());
    }

    // A generic function uses static dispatch. The compiler knows `guest` is a
    // Teacher and creates efficient machine code for that exact type. This process
    // is called monomorphization, but the simple idea is "specialize at compile time."
    let guest = Teacher::new("Barbara", "Compilers");
    println!("   static dispatch: {}\n", introduce(&guest));
}

/// Stores the state that belongs to a teacher.
///
/// This is the closest Rust equivalent to the data part of a small class. Fields
/// are private because they lack `pub`, so outside code must use our methods. That
/// control over access is called **encapsulation**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teacher {
    name: String,
    subject: String,
}

impl Teacher {
    /// Constructs a new Teacher.
    ///
    /// Rust has no special constructor keyword. `new` is simply a common function
    /// name. `impl Into<String>` accepts a String or a string slice and converts it
    /// to an owned String, which makes this function convenient for callers.
    pub fn new(name: impl Into<String>, subject: impl Into<String>) -> Self {
        // `Self` means Teacher inside `impl Teacher`. The field-init shorthand
        // `name: name.into()` converts the argument before storing it.
        Self {
            name: name.into(),
            subject: subject.into(),
        }
    }

    /// Borrows the teacher's subject without exposing the private String itself.
    pub fn subject(&self) -> &str {
        &self.subject
    }
}

/// Stores a student's name and growable list of course names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    name: String,
    courses: Vec<String>,
}

impl Student {
    /// Constructs a Student whose course list starts empty.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            courses: Vec::new(),
        }
    }

    /// Adds a course through a mutable borrow of the Student.
    pub fn enroll(&mut self, course: impl Into<String>) {
        self.courses.push(course.into());
    }

    /// Reports the number of enrolled courses without changing the Student.
    pub fn course_count(&self) -> usize {
        self.courses.len()
    }
}

/// Behavior shared by every type that represents a person.
///
/// A trait is similar to an interface: it states what an implementing type must
/// be able to do. Unlike class inheritance, it does not copy fields from a parent.
pub trait Person {
    /// Returns a borrowed view of the person's name.
    fn name(&self) -> &str;

    /// Returns a role label built into the program.
    fn role(&self) -> &'static str;

    /// Builds a description using the two required methods above.
    ///
    /// This is a default method: implementing types receive it automatically but
    /// may replace it with more specific behavior, as Student does below.
    fn describe(&self) -> String {
        format!("{} is a {}", self.name(), self.role())
    }
}

// `impl Person for Teacher` fulfills the Person contract for Teacher. Rust does
// not change Teacher's fields; it simply declares that Teacher has this behavior.
impl Person for Teacher {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &'static str {
        "teacher"
    }
}

// Student implements the same trait, allowing Student and Teacher to be treated
// alike by code that only needs Person behavior.
impl Person for Student {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &'static str {
        "student"
    }

    fn describe(&self) -> String {
        // Student replaces the default method so its description can include
        // information that exists only on Student.
        format!(
            "{} is a student taking {} course(s)",
            self.name,
            self.course_count()
        )
    }
}

/// Demonstrates composition by owning a Teacher inside a Course.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    title: String,
    instructor: Teacher,
}

impl Course {
    /// Constructs a Course and moves the instructor into it.
    pub fn new(title: impl Into<String>, instructor: Teacher) -> Self {
        Self {
            title: title.into(),
            instructor,
        }
    }

    /// Combines data from the Course and its composed Teacher.
    pub fn summary(&self) -> String {
        format!("{} is taught by {}", self.title, self.instructor.name())
    }
}

/// Greets any concrete type that implements Person.
///
/// `&impl Person` is a concise generic parameter. This uses static dispatch: the
/// compiler knows the exact type for each call and checks its trait implementation.
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
        // Every Box has the same outside type (`Box<dyn Person>`) even though its
        // inside value differs. Calling `role` chooses the correct implementation
        // at runtime.
        let people: Vec<Box<dyn Person>> = vec![
            Box::new(Student::new("A")),
            Box::new(Teacher::new("B", "Rust")),
        ];
        let roles: Vec<&str> = people.iter().map(|person| person.role()).collect();
        assert_eq!(roles, ["student", "teacher"]);
    }
}
