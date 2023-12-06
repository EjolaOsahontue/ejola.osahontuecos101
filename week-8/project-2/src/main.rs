use std::io;

struct Sibling {
    name: String,
    age: u32,
    marital_status: MaritalStatus,
    details: Option<SiblingDetails>,
}

struct SiblingDetails {
    university: Option<String>,
    course: Option<String>,
    offspring: Option<u32>,
    city: Option<String>,
    secondary_school: Option<String>,
    current_class: Option<String>,
}

enum MaritalStatus {
    Single,
    Married,
}

fn main() {
    // Input the number of siblings
    let num_siblings = get_user_input("Enter the number of siblings: ");

    // Initialize an empty array of siblings
    let mut siblings: Vec<Sibling> = Vec::new();

    // Iterate for each sibling
    for i in 1..=num_siblings {
        println!("Enter details for Sibling #{}:", i);

        // Input name and age
        let name = get_user_input("Enter name: ");
        let age = get_user_input("Enter age: ");

        // Input marital status
        let marital_status = if age > 18 {
            match get_user_input("Is the sibling married? (1 for Yes, 2 for No): ") {
                1 => MaritalStatus::Married,
                _ => MaritalStatus::Single,
            }
        } else {
            MaritalStatus::Single
        };

        // Initialize sibling details
        let details = match marital_status {
            MaritalStatus::Single => Some(SiblingDetails {
                university: if get_user_input("Is the sibling a student? (1 for Yes, 2 for No): ") == 1 {
                    Some(get_user_input("Enter university: "))
                } else {
                    None
                },
                course: if let Some(_) = details.university {
                    Some(get_user_input("Enter course of study: "))
                } else {
                    None
                },
                offspring: None,
                city: None,
                secondary_school: None,
                current_class: None,
            }),
            MaritalStatus::Married => Some(SiblingDetails {
                university: None,
                course: None,
                offspring: Some(get_user_input("Enter number of offspring: ")),
                city: Some(get_user_input("Enter city the family lives in: ")),
                secondary_school: None,
                current_class: None,
            }),
        };

        // Create sibling struct and push to the array
        let sibling = Sibling {
            name,
            age,
            marital_status,
            details,
        };
        siblings.push(sibling);
    }

    // Display sibling data
    println!("Sibling Data:");
    for sibling in &siblings {
        println!("Name: {}", sibling.name);
        println!("Age: {}", sibling.age);

        match &sibling.marital_status {
            MaritalStatus::Single => {
                if let Some(details) = &sibling.details {
                    if let Some(university) = &details.university {
                        println!("University: {}", university);
                    }
                    if let Some(course) = &details.course {
                        println!("Course: {}", course);
                    }
                }
            }
            MaritalStatus::Married => {
                if let Some(details) = &sibling.details {
                    if let Some(offspring) = &details.offspring {
                        println!("Number of Offspring: {}", offspring);
                    }
                    if let Some(city) = &details.city {
                        println!("City: {}", city);
                    }
                }
            }
        }
        println!("----------------------");
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn get_user_input_u32(prompt: &str) -> u32 {
    loop {
        match get_user_input(prompt).parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
