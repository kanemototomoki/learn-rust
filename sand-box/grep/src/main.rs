use rayon::prelude::*;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
    #[structopt(short = "-n", long, help = "Prints line numbers")]
    line_number: bool,
}

impl GrepArgs {
    fn grep(&self, content: &str, file_name: &str) {
        let mut line_number = 1;
        for line in content.lines() {
            if line.contains(self.pattern.as_str()) {
                let prefix_line_number = if self.line_number {
                    format!("{}:", line_number)
                } else {
                    format!("")
                };
                let prefix_file_name = if self.path.len() > 1 {
                    format!("{}:", file_name)
                } else {
                    format!("")
                };
                println!("{}{}{}", prefix_file_name, prefix_line_number, line)
            }
            line_number += 1;
        }
    }

    fn run(&self) {
        self.path
            .par_iter()
            .for_each(|file| match read_to_string(file) {
                Ok(content) => self.grep(&content, &file),
                Err(reason) => println!("{}", reason),
            });
    }
}

fn main() {
    let grep = GrepArgs::from_args();
    grep.run();
}
