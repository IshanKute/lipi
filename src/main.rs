use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::error::Error;

use crate::core::lexer::SourceLexer;
use crate::core::parser::FunctionParser;
use crate::core::interpreter::FunctionInterpreter;
use crate::lang::Language;
use crate::core::lexer::Lexer;
use crate::core::parser::Parser as LipiParser;
use crate::core::interpreter::Interpreter;

mod core;
mod lang;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Lipi program
    Run {
        /// The path to the .lp file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Language to use (English, Hindi, Marathi, Sanskrit)
        #[arg(short, long, default_value = "English")]
        lang: String,
    },
    /// Start REPL session
    Repl {
        /// Language to use (en, hi, mr, sa)
        #[arg(short, long, default_value = "English")]
        lang: String,
    },
}

fn execute_code(source: &str, language: Language) -> Result<(), Box<dyn Error>> {
    let mut lexer = SourceLexer::new(source, language);
    let mut parser = FunctionParser::new();
    let mut interpreter = FunctionInterpreter::new();

    let tokens = lexer.scan_tokens()?;
    let ast = parser.parse(tokens)?;
    interpreter.interpret(ast)?;

    Ok(())
}

fn run_file(file: &PathBuf, lang: &str) -> Result<(), Box<dyn Error>> {
    println!("Running file: {:?} with language: {:?}", file, lang);
    let content = std::fs::read_to_string(file)?;
    let language = Language::from_code(lang);
    execute_code(&content, language)
}

fn start_repl(lang: &str) -> Result<(), Box<dyn Error>> {
    let language = Language::from_code(lang);
    println!("Lipi REPL - Type 'exit' to quit");
    
    let mut input = String::new();
    loop {
        input.clear();
        print!(">> ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "exit" {
            break;
        }
        
        if let Err(e) = execute_code(input, language.clone()) {
            eprintln!("Error: {}", e);
        }
    }
    
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Run { file, lang }) => run_file(file, lang),
        Some(Commands::Repl { lang }) => start_repl(lang),
        None => {
            println!("No command provided. Try 'lipi --help'");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}