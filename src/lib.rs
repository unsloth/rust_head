use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Print the first 10 lines of each FILE to standard output.
With more than 1 FILE, precede each with a header giving the file name."
)]
struct Cli {
    #[arg(default_values = vec!["-"], help = "File(s) to read")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Print the first LINES number of lines"
    )]
    lines: u32,

    #[arg(
        short = 'c',
        long,
        conflicts_with = "lines",
        help = "Print the first BYTES number of bytes"
    )]
    bytes: Option<u32>,
}

pub fn run() -> MyResult<()> {
    let cli = Cli::parse();

    // We check if there's multiple files because the output includes a title
    // for each file only if there are multiple. We have to do this before we
    // iterate across cli.files
    let multiple_files = cli.files.len() > 1;

    // There needs to be a newline between every file output so we add one at
    // the start of each new file, but not the first file. So we check for it.
    let mut first_file = true;
    for filename in cli.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                // Check if we should add a title and/or a space. Note we only
                // do this if the file exists.
                let space = if first_file { "" } else { "\n" };
                if multiple_files {
                    println!("{}==> {} <==", space, filename);
                    first_file = false;
                }

                // Check if user input argument for bytes
                if let Some(num_bytes) = cli.bytes {
                    // Reads each byte until the user defined limit or until EOF
                    // and stores them in a vec
                    let mut buf: Vec<u8> = vec![0; num_bytes as usize];
                    let n = file.read(&mut buf)?;
                    // We use the from_utf8_lossy method to mimic the behavior
                    // of head when dealing with multibyte characters
                    let buf = String::from_utf8_lossy(&buf[..n]);
                    print!("{}", buf);
                } else {
                    // Prints each line. Specifically uses read_line method to
                    // account for different line endings
                    for _ in 0..cli.lines {
                        let mut buf = String::new();
                        file.read_line(&mut buf)?;
                        print!("{}", buf);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
