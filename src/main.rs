use clap::Parser;
use colored::{ColoredString, Colorize};
use degcolor::Color;

#[derive(Parser)]
#[command(
    name = "degcolor",
    version,
    about = "Terminal color toolkit for Linux shells",
    long_about = "Degcolor is a Rust-powered CLI for generating,
    transforming, and visualizing colors directly inside the Linux terminal."
)]
struct Cli {
    /// Generates a random color in RGB.
    #[arg(short = 'c', long)]
    color: bool,

    /// Ensures RGB output format.
    #[arg(long)]
    rgb: bool,

    /// Ensures HEX output format.
    #[arg(long)]
    hex: bool,

    /// Ensures HSL output format.
    #[arg(long)]
    hsl: bool,

    /// Remove the context, use optionally.
    #[arg(long)]
    context: bool,

    /// Returns the opposite color of the input.
    #[arg(short = 'r', long, value_name = "COLOR")]
    reverse: Option<String>,

    /// It combines two colors.
    #[arg(short = 'u', long, num_args = 2, value_names = ["COLOR1", "COLOR2"])]
    union: Option<Vec<String>>,

    /// Returns the nearest random inverse color of the input.
    #[arg(short = 'm', long, value_name = "COLOR")]
    magic: Option<String>,

    /// Returns the complementary color of the endpoint.
    #[arg(short = 'e', long, value_name = "COLOR")]
    extreme: Option<String>,

    /// Returns to the display of the color selected in the input.
    #[arg(short = 's', long, value_name = "COLOR")]
    show: Option<String>,
}

/// Conventional types of color formats for user output.
enum FormatColor {
    Rgb,
    Hex,
    Hsl,
}

impl FormatColor {
    /// The instance for selecting the user's preferred color receives
    /// the command structure for the terminal as a parameter.
    pub fn new(cli: &Cli) -> Self {
        if cli.hex {
            Self::Hex
        } else if cli.hsl {
            Self::Hsl
        } else {
            Self::Rgb
        }
    }
}

/// Ensures the standard output desired by the user.
fn display_color(c: &Color, f: FormatColor, context: bool) {
    let formatted: String = match f {
        FormatColor::Rgb => c.to_rgb(),
        FormatColor::Hex => c.to_hex(),
        FormatColor::Hsl => c.to_hsl(),
    };

    if !context {
        let width: usize = "hsl(360, 100%, 100%)".len();
        let color_block: ColoredString = "    ".on_truecolor(c.red, c.green, c.blue);
        let padded: String = format!("{:^width$}", formatted, width = width);

        println!("+------+{}+", "-".repeat(width + 2));
        println!("| {} | {} |", color_block, padded.white());
        println!("+------+{}+", "-".repeat(width + 2));
    } else {
        println!("{}", formatted);
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    if cli.color {
        let c: Color = Color::random();
        display_color(&c, FormatColor::new(&cli), cli.context);
    }

    if let Some(input) = &cli.reverse {
        match Color::from_str(input) {
            Some(c) => {
                let c: Color = c.reverse();
                display_color(&c, FormatColor::new(&cli), cli.context);
            }
            None => println!(
                "Format value in command --reverse invalid!
                Use: rgb(255, 0, 0) or #FF0000"
            ),
        }
    }

    if let Some(colors) = &cli.union {
        let color_one: Option<Color> = Color::from_str(&colors[0]);
        let color_two: Option<Color> = Color::from_str(&colors[1]);

        if color_one.is_some() & color_two.is_some() {
            let color_one: Color = color_one.unwrap();
            let color_two: Color = color_two.unwrap();

            display_color(
                &color_one.join(&color_two),
                FormatColor::new(&cli),
                cli.context,
            );
        } else {
            println!(
                "Format value in command --union invalid!
                Use: --union \"#FF0000\" \"rgb(0, 0, 0)\""
            )
        }
    }

    if let Some(input) = &cli.magic {
        match Color::from_str(&input) {
            Some(c) => {
                let color_magic: &Color = &c.random_magic();
                let format_color: FormatColor = FormatColor::new(&cli);
                display_color(color_magic, format_color, cli.context);
            }
            None => {
                println!(
                    "Format value in command --magic invalid!
                    Use: rgb(255, 0, 0) or #FF0000"
                )
            }
        }
    }

    if let Some(input) = &cli.extreme {
        match Color::from_str(&input) {
            Some(c) => {
                let color_extreme: &Color = &c.complementary();
                let format_color: FormatColor = FormatColor::new(&cli);
                display_color(color_extreme, format_color, cli.context);
            }
            None => {
                println!(
                    "Format value in command --extreme invalid!
                    Use: rgb(255, 0, 0) or #FF0000"
                )
            }
        }
    }

    if let Some(input) = &cli.show {
        match Color::from_str(&input) {
            Some(c) => {
                let format_color: FormatColor = FormatColor::new(&cli);
                display_color(&c, format_color, cli.context);
            }
            None => {
                println!(
                    "Format value in command --extreme invalid!
                    Use: rgb(255, 0, 0) or #FF0000"
                )
            }
        }
    }
}
