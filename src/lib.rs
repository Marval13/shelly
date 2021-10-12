#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author)]
/// shelly is a small and bad clone of cowsay written in Rust.
/// It contains turtles!
///
/// You can use your own template for Shelly.
/// The placeholders for eyes and mouth in the templates are respectively `{e}` and `{m}`.
/// See the default templates for examples.
pub struct Opt {
    #[structopt(default_value = "Hi, I'm Shelly!")]
    /// What does Shelly say?
    message: Vec<String>,

    #[structopt(short = "d", long = "dead")]
    /// Dead Shelly
    dead: bool,

    #[structopt(short = "s", long = "smile")]
    /// Makes Shelly smile :) (only for small Shelly)
    smile: bool,

    #[structopt(short = "b", long = "big")]
    /// Big Shelly
    big: bool,

    #[structopt(short = "e", long = "eye")]
    /// Eye character
    eye: Option<char>,

    #[structopt(short = "m", long = "mouth")]
    /// Mouth character
    mouth: Option<char>,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Loads a custom picture of Shelly
    shellyfile: Option<std::path::PathBuf>,
}

pub fn say(opt: Opt) -> Result<String, Box<dyn std::error::Error>> {
    let template = if let Some(path) = opt.shellyfile {
        std::fs::read_to_string(path)?
    } else if opt.big {
        std::fs::read_to_string("./templates/bigshelly.txt")?
    } else {
        std::fs::read_to_string("./templates/shelly.txt")?
    }
    .replace("\r\n", "\n");

    let message = opt.message.join(" ");

    let mouth = if let Some(m) = opt.mouth {
        m
    } else if opt.smile {
        '/'
    } else {
        '_'
    };

    let eye = if let Some(e) = opt.eye {
        e
    } else if opt.dead {
        'x'
    } else {
        'o'
    };

    let shelly = template
        .replace("{m}", &mouth.to_string())
        .replace("{e}", &eye.to_string());

    Ok(format!("{}\n{}", message, shelly))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_say() {
        let opt = Opt {
            message: vec!["Hello,".to_string(), "World!".to_string()],
            dead: false,
            smile: false,
            eye: None,
            mouth: None,
            shellyfile: None,
            big: false,
        };

        assert_eq!(
            say(opt).unwrap(),
            r"Hello, World!
  \
 ____    _____
| o  |  /     \  
|____ \|       |_
     \_|_______|/
       |_|| |_||
"
        )
    }

    #[test]
    fn dead_say() {
        let opt = Opt {
            message: vec!["uh".to_string(), "im".to_string(), "dead".to_string()],
            dead: true,
            smile: false,
            eye: None,
            mouth: None,
            shellyfile: None,
            big: false,
        };

        assert_eq!(
            say(opt).unwrap(),
            r"uh im dead
  \
 ____    _____
| x  |  /     \  
|____ \|       |_
     \_|_______|/
       |_|| |_||
"
        )
    }
}
