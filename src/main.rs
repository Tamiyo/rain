mod cli;
mod interpreter;
mod io;
mod parser;

fn main() {
    let opt = crate::cli::parse_args();
    let input = opt.input;
    let pixels = match crate::io::read_image(&input) {
        Ok(p) => p,
        Err(e) => panic!("{:?}", e),
    };

    let instructions = crate::parser::parse(&pixels);
    crate::interpreter::interpret(&instructions);
}
