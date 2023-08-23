struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    // Adding students to the vector
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });

    // Add more students here...

    // Accessing a student by index with error handling
    let index = 2; // Change this to the desired index
    match students.get(index) {
        Some(student) => {
            println!("Student ID: {}", student.id);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone Number: {}", student.phno);
        }
        None => {
            println!("Student not found at index {}", index);
        }
    }
}
