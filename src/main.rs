use error::{display::display_error, AstryxResult};
use repl;
use structopt::StructOpt;

mod build;
mod render;
mod server;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
struct Opt {
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Repl,
    /// start a server
    Serve {
        /// Input file
        file: Option<String>,
        port: Option<u32>,
    },
    /// build the project
    Build {
        /// Input file
        file: Option<String>,
    },
    New,
}

pub fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}

/// run cli commands
fn run() -> Result<String, String> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Serve { file, port } => {
            let path = &file.unwrap_or(String::from("site.astryx"));

            server::start(path.into(), port.unwrap_or(8888)).map_err(|e| display_error(&e, path))
        }
        Command::Build { file } => {
            let path = &file.unwrap_or(String::from("site.astryx"));
            let file = std::fs::read_to_string(&path).expect(&format!("could not open {}", path));

            println!("building: {}\n", &path);
            build::build(&file, &path).map_err(|e| display_error(&e, path))
        }
        Command::New => new_project().map_err(|e| format!("error creating new project: {:?}", e)),
        Command::Repl => {
            repl::run();
            Ok(())
        }
    }
    .map(|_| "\ndone.".to_string())
}

/// set up a new project in the current directory
fn new_project<'a>() -> AstryxResult<()> {
    unimplemented!()
}
