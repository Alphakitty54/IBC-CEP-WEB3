// Online Rust compiler to run Rust program online
// Print "Hello World!" message

struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    // Add students to the vector
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });
    // Add more students here...

    // Display details of all students
    for student in &students {
        println!("Student ID: {}", student.id);
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone Number: {}", student.phno);
        println!();
    }
    
    
    students.push(Student {
    name: String::from("Bob"),
    email: String::from("bob@example.com"),
    phno: String::from("987-654-3210"),
    id: 2,
});

students.push(Student {
    name: String::from("Charlie"),
    email: String::from("charlie@example.com"),
    phno: String::from("555-123-4567"),
    id: 3,
});

let index_to_access = 5; // Index is out of bounds
match students.get(index_to_access) {
    Some(student) => {
        println!("Student ID: {}", student.id);
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone Number: {}", student.phno);
    }
    None => {
        eprintln!("Error: Student not found at index {}", index_to_access);
    }
}

}
