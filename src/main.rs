use clap::Parser;
use std::fs;
use std::io::Write;

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
    if cli.command == "create" {
        if cli.args.len() == 1 {
            let project_name = cli.args[0].clone();
            fs::create_dir(&project_name).unwrap();
            let mut readme_file = fs::File::create(format!("{}/README.md", project_name)).unwrap();  
            let mut python_file = fs::File::create(format!("{}/main.py", project_name)).unwrap(); 
            let mut toml_file = fs::File::create(format!("{}/py.toml", project_name)).unwrap();
            readme_file.write_all(format!("# {}\n", project_name).as_bytes()).unwrap();
            python_file.write_all(b"").unwrap();
            toml_file.write_all(b"").unwrap();

            println!("Using project_name for env");
            return;
        } else if cli.args.len() == 2 {
            let project_name = cli.args[0].clone();
            fs::create_dir(&project_name).unwrap();
            let mut readme_file = fs::File::create(format!("{}/README.md", project_name)).unwrap();   
            let mut python_file = fs::File::create(format!("{}/main.py", project_name)).unwrap(); 
            let mut toml_file = fs::File::create(format!("{}/py.toml", project_name)).unwrap();
            python_file.write_all(b"").unwrap();
            toml_file.write_all(b"").unwrap(); 
            readme_file.write_all(format!("# {}\n", project_name).as_bytes()).unwrap();

            println!("Using custom name for env");
            return;
        } 
    }
    println!("Command: {:?}, args: {:?}", cli.command, cli.args);   
}
