use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

// Main
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|
        {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );

    run(config).unwrap_or_else(|err|
    {
        eprintln!("File could not be opened: {err}");
        process::exit(1);
    }); 
    
}
// Main end

fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let file_data = fs::read_to_string(config.file_path)?;
    let query = config.query;
    let result = if config.ignore_case{
        search_case_insensitive(query, &file_data)
    }
    else{
        search(query, &file_data)
    };

    for line in result{
        println!("{line}");
    }

    Ok(())
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case : bool
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() == 1 {
            return Err("Query and File Path not included");
        } else if args.len() == 2 {
            return Err("File Path not included");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case})
    }
}
