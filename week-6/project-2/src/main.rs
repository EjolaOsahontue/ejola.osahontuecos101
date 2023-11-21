use std::io;

struct Researcher {
    name: String,
    papers_published: u32,
}

impl Researcher {
    fn new(name: &str, papers_published: u32) -> Researcher {
        Researcher {
            name: name.to_string(),
            papers_published,
        }
    }

    fn calculate_incentive(&self) -> u32 {
        match self.papers_published {
            3..=5 => 500_000,
            6..=9 => 800_000,
            10..=u32::MAX => 1_000_000,
            _ => 100_000,
        }
    }

    fn display_incentive(&self) {
        let incentive = self.calculate_incentive();
        println!("Name: {}", self.name);
        println!("Papers Published: {}", self.papers_published);
        println!("Incentive: N{}", incentive);
    }
}

fn main() {
    let mut researchers = Vec::new();

    // Accept input for 500 researchers
    for _ in 0..500 {
        let name = get_input("Enter researcher's name: ");
        let papers_published: u32 = get_input("Enter the number of papers published: ")
            .trim()
            .parse()
            .expect("Invalid input. Please enter a valid number.");

        let researcher = Researcher::new(&name, papers_published);
        researchers.push(researcher);
    }

    // Display incentive for each researcher
    for researcher in &researchers {
        researcher.display_incentive();
        println!("------------------------");
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}