use std::io::{self, BufRead, Write};
use std::process;

#[derive(Debug)]
enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

impl MetaCommandResult {
    fn do_meta_command(line: &String) -> MetaCommandResult {
        match line.trim() {
            ".exit" => process::exit(0x0100),
            _ => MetaCommandResult::MetaCommandUnrecognizedCommand,
        }
    }
}

#[derive(Debug)]
enum StatementType {
    StatementInsert,
    StatementSelect,
}

#[derive(Debug)]
struct Statement {
    t: StatementType
}

#[derive(Debug)]
enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

impl PrepareResult {
    fn parepare_statement(line: &String, statement: &mut Statement) -> PrepareResult {
        match &line[0..6] {
            "insert" => {
                statement.t = StatementType::StatementInsert;
                return PrepareResult::PrepareSuccess;
            },
            "select" => {
                statement.t = StatementType::StatementSelect;
                return PrepareResult::PrepareSuccess;
            },
            _ => PrepareResult::PrepareUnrecognizedStatement,
        }
    }
}

fn execute_statement(statement: Statement) {
    match statement.t {
        StatementType::StatementInsert => println!("This is where we would do an insert."),
        StatementType::StatementSelect => println!("This is where we would do a select."),
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("db > ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        if line.chars().nth(0) == Some('.') {
            match MetaCommandResult::do_meta_command(&line) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command {}.", line.trim());
                    continue;
                },
            }
        }

        let mut statement = Statement{ t: StatementType::StatementSelect };
        match PrepareResult::parepare_statement(&line, &mut statement) {
            PrepareResult::PrepareSuccess => {
                execute_statement(statement);
                println!("Executed.");
            },
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}.", line.trim());
                continue;
            },
        }
    }
}
