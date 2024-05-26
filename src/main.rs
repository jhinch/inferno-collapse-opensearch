use inferno::collapse::Collapse;

mod collapse;
mod profile;
#[derive(clap::Parser)]
struct Cli {
    input_file: Option<String>,
}

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let mut folder= collapse::Folder::default();
    folder.collapse_file_to_stdout(cli.input_file.as_ref()).unwrap();
}
