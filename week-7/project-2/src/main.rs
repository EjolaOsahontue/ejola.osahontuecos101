use std::io;

// Struct to represent a Sibling
struct Sibling {
    name: String,
    age: u32,
    is_married: Option<bool>,
    is_student: Option<bool>,
    university: Option<String>,
    course_of_study: Option<String>,
    offspring: Option<bool>,
    city: Option<String>,
    waec_status: Option<bool>,
    secondary_school: Option<String>,
    current_class: Option<String>,
}

impl Sibling {
    // Function to create a new Sibling instance
    fn new(name: String, age: u32) -> Sibling {
        Sibling {
            name,
            age,
            is_married: None,
            is_student: None,
            university: None,
            course_of_study: None,
            offspring: None,
            city: None,
            waec_status: None,
            secondary_school: None,
            current_class: None,
        }
    }
}

fn main() {
    // Get the number of siblings
    println!("Enter the number of siblings: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_siblings: usize = input.trim().parse().expect("Invalid input");

    // Create an array to store sibling details
    let mut siblings: Vec<Sibling> = Vec::new();

    // Iterate for each sibling
    for i in 0..num_siblings {
        println!("Enter details for sibling #{}:", i + 1);

        // Get sibling's name
        println!("Enter name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");

        // Get sibling's age
        println!("Enter age: ");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read input");
        let age: u32 = age_str.trim().parse().expect("Invalid input");

        // Create a new Sibling instance
        let mut sibling = Sibling::new(name.trim().to_string(), age);

        // Process based on age
        if age > 18 {
            // Sibling is 18 or older
            println!("Is the sibling married? (yes/no): ");
            let mut is_married = String::new();
            io::stdin().read_line(&mut is_married).expect("Failed to read input");
            let is_married = is_married.trim().to_lowercase() == "yes";

            sibling.is_married = Some(is_married);

            if is_married {
                // Sibling is married
                println!("Any offspring? (yes/no): ");
                let mut has_offspring = String::new();
                io::stdin().read_line(&mut has_offspring).expect("Failed to read input");
                let has_offspring = has_offspring.trim().to_lowercase() == "yes";

                sibling.offspring = Some(has_offspring);

                if has_offspring {
                    // Sibling has offspring
                    println!("Enter city where the family lives: ");
                    let mut city = String::new();
                    io::stdin().read_line(&mut city).expect("Failed to read input");
                    sibling.city = Some(city.trim().to_string());
                }
            } else {
                // Sibling is not married
                println!("Is the sibling a student? (yes/no): ");
                let mut is_student = String::new();
                io::stdin().read_line(&mut is_student).expect("Failed to read input");
                let is_student = is_student.trim().to_lowercase() == "yes";

                sibling.is_student = Some(is_student);

                if is_student {
                    // Sibling is a student
                    println!("Enter university: ");
                    let mut university = String::new();
                    io::stdin().read_line(&mut university).expect("Failed to read input");
                    sibling.university = Some(university.trim().to_string());

                    println!("Enter course of study: ");
                    let mut course_of_study = String::new();
                    io::stdin().read_line(&mut course_of_study).expect("Failed to read input");
                    sibling.course_of_study = Some(course_of_study.trim().to_string());
                }
            }
        } else {
            // Sibling is less than 18
            println!("Is the sibling's WAEC status 'yes'? (yes/no): ");
            let mut waec_status = String::new();
            io::stdin().read_line(&mut waec_status).expect("Failed to read input");
            let waec_status = waec_status.trim().to_lowercase() == "yes";

            sibling.waec_status = Some(waec_status);

            if !waec_status {
                // Sibling did not take WAEC
                println!("Enter current class level: ");
                let mut current_class = String::new();
                io::stdin().read_line(&mut current_class).expect("Failed to read input");
                sibling.current_class = Some(current_class.trim().to_string());
            } else {
                // Sibling took WAEC
                println!("Enter secondary school attended: ");
                let mut secondary_school = String::new();
                io::stdin().read_line(&mut secondary_school).expect("Failed to read input");
                sibling.secondary_school = Some(secondary_school.trim().to_string());
            }
        }

        // Add the sibling to the array
        siblings.push(sibling);
    }

    // Display the data of all siblings
    println!("Details of all siblings:");

    for (i, sibling) in siblings.iter().enumerate() {
        println!("Sibling #{}", i + 1);
        println!("Name: {}", sibling.name);
        println!("Age: {}", sibling.age);

        if let Some(is_married) = sibling.is_married {
            if is_married {
                println!("Marital Status: Married");
                if let Some(has_offspring) = sibling.offspring {
                    if has_offspring {
                        println!("Offspring: Yes");
                        if let Some(city) = &sibling.city {
                            println!("City: {}", city);
                        }
                    }
                }
            } else {
                println!("Marital Status: Single");
                if let Some(is_student) = sibling.is_student {
                    if is_student {
                        println!("Student: Yes");
                        if let Some(university) = &sibling.university {
                            println!("University: {}", university);
                        }
                        if let Some(course_of_study) = &sibling.course_of_study {
                            println!("Course of Study: {}", course_of_study);
                        }
                    }
                }
            }
        } else {
            println!("Marital Status: Not specified");
        }

        if let Some(waec_status) = sibling.waec_status {
            if waec_status {
                if let Some(secondary_school) = &sibling.secondary_school {
                    println!("Secondary School Attended: {}", secondary_school);
                }
            } else {
                if let Some(current_class) = &sibling.current_class {
                    println!("Current Class Level: {}", current_class);
                }
            }
        } else {
            println!("WAEC Status: Not specified");
        }

        println!("------------------------");
    }
}
