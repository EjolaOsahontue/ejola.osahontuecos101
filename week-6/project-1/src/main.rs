use std::io;

struct Candidate {
    name: String,
    email: String,
    department: String,
    state_of_origin: String,
    is_class_rep: bool,
    level: u32,
    cgpa: f64,
}

impl Candidate {
    fn new(
        name: &str,
        email: &str,
        department: &str,
        state_of_origin: &str,
        is_class_rep: bool,
        level: u32,
        cgpa: f64,
    ) -> Candidate {
        Candidate {
            name: name.to_string(),
            email: email.to_string(),
            department: department.to_string(),
            state_of_origin: state_of_origin.to_string(),
            is_class_rep,
            level,
            cgpa,
        }
    }

    fn is_eligible(&self) -> bool {
        if self.is_class_rep && self.level > 1 && self.cgpa > 4.0 {
            return true;
        }
        false
    }

    fn display_details(&self) {
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Department: {}", self.department);
        println!("State of Origin: {}", self.state_of_origin);

        if self.is_eligible() {
            println!("You can vote");
        } else {
            println!("Sorry, you are not eligible to vote");
        }
    }
}

fn main() {
    // Input candidate details
    let candidate = Candidate::new("John Doe", "john@example.com", "Computer Science", "Lagos", true, 2, 4.5);

    // Display details for the candidate
    candidate.display_details();
}
