use std::fs;
use std::io::Write;

pub trait Command {
    fn handle(&self) -> i32;
}

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        AddCommand { args }
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description_option = &self.args.get(2);
        if let Some(description) = description_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("Storage.txt")
                .expect("File not found");

            writeln!(file, "{description}").expect("Unable to write file");

            println!("Todo added");

            return 0;
        } else {
            println!("description is required");
            return 1;
        }
    }
}

pub struct ListCommand {}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let contents = fs::read_to_string("Storage.txt").expect("File error");
        println!("Listing Todos:");
        println!("{contents}");
        0
    }
}

// TESTS
#[cfg(test)]

mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn add_command() {
        // prepare
        let args = vec![
            "todo".to_string(),
            "other todo".to_string(),
            "final todo for test".to_string(),
        ];
        let command = AddCommand::new(args);

        // act
        let code = command.handle();

        // assert
        assert_eq!(0, code);
    }
    
        #[test]
    fn list_command() {
        // prepare
        let command = ListCommand::new();

        // act
        let code = command.handle();

        // assert
        assert_eq!(0, code);
    }
}
