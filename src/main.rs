use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.args.is_empty() {
        println!("No arguments provided.");
        return;
    }
    if cli.command.is_empty() {
        println!("No command provided.");
        return;
    }
    
    if cli.args.len() == 1 {
        println!("Using project_name for env");
        return;
    } else if cli.args.len() == 2 {
        println!("Using custom name for env");
    } 

    println!("Command: {:?}, args: {:?}", cli.command, cli.args);   
}
