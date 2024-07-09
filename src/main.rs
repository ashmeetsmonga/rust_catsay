use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    message: String,

    #[clap(short = 'd', long = "dead")]
    ///Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    let eye = if options.dead {"x"} else {"o"};

    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("   /\\_/\\");
    println!("  ( {eye} {eye} )", eye=eye.red().bold());
    println!("  =( I )=")
}
