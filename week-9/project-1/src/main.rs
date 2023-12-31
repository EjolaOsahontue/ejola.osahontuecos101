use std::io::Write;

fn main() {
    let table_header = ["Lager", "Stout", "Non-Alcoholic"];
    let table_data = [
        ["33 Export", "Legend", "Maltina"],
        ["Desperados", "Turbo", "King Amstel Malta"],
        ["Goldberg", "Williams", "Malta Gold"],
        ["Gulder", "-", "Fayrouz"],
        ["Heineken", "-", "-"],
        ["Star", "-", "-"],
    ];

    if let Err(err) = create_and_write_file("nigerian_breweries_categories.txt", &table_header, &table_data) {
        eprintln!("Error: {}", err);
    } else {
        println!("File created successfully.");
    }
}

fn create_and_write_file(file_path: &str, table_header: &[&str], table_data: &[ [&str; 3]; 6]) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_path)?;

    file.write_all(table_header.join("\t").as_bytes())?;
    file.write_all("\n".as_bytes())?;

    for row in table_data {
        file.write_all(row.join("\t").as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }

    println!("Data written to file.");
    Ok(())
}
