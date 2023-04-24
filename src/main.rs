use clap::{command, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{env, fs};

fn main() {
    let cli: Cli = Cli::parse();
    match cli.out {
        Out::Json => {
            write_json();
        }
        Out::Ts => {
            write_ts();
        }
        Out::Js => {
            write_js();
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specify the desired output file format
    #[arg(value_enum, default_value_t=Out::Json)]
    out: Out,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Out {
    /// creates classes.json
    Json,
    /// creates classes.ts
    Ts,
    /// creates classes.js
    Js,
}

fn write_json() {
    let classes = classes(theme());
    let mut d = env::current_dir().unwrap();
    d.push("src/themewind/classes.json");
    let classes_json = serde_json::to_string_pretty(&classes).unwrap();
    fs::write(d, classes_json).unwrap();
}

fn write_ts() {
    let classes = classes(theme());
    let mut d = env::current_dir().unwrap();
    d.push("src/themewind/classes.ts");
    let classes_json = serde_json::to_string_pretty(&classes).unwrap();
    let ts = format!("export const classes = {classes_json}\n");
    fs::write(d, ts).unwrap();
}

// This is essentially the _exact_ same as write_ts
// A refactor is imminent.
fn write_js() {
    let classes = classes(theme());
    let mut d = env::current_dir().unwrap();
    d.push("src/themewind/classes.js");
    let classes_json = serde_json::to_string_pretty(&classes).unwrap();
    let ts = format!("export const classes = {classes_json}\n");
    fs::write(d, ts).unwrap();
}

fn classes(theme: Theme) -> Classes {
    Classes {
        card: card(theme.clone()),
        link: link(),
        modal: modal(),
        sidebar: sidebar(theme.clone()),
        header: header(theme.clone()),
        button: button(theme.clone()),
    }
}

fn theme() -> Theme {
    let mut d = env::current_dir().unwrap();
    d.push("src/themewind/theme.json");
    let content = fs::read_to_string(d).unwrap();
    let theme: Theme = serde_json::from_str(&content).unwrap();
    theme
    // Theme {
    //     color: Color { primary: "blue" },
    //     border: Border {
    //         radius: "rounded-2xl",
    //     },
    // }
}

fn link() -> String {
    "text-blue-500 hover:text-blue-600 hover:underline".to_owned()
}

#[derive(Serialize, Deserialize, Clone)]
struct Theme {
    color: Color,
    border: Border,
}

#[derive(Serialize, Deserialize, Clone)]
struct Color {
    primary: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Border {
    radius: String,
}

#[derive(Serialize, Deserialize)]
struct Classes {
    card: String,
    link: String,
    modal: String,
    sidebar: String,
    header: String,
    button: String,
}

fn card(theme: Theme) -> String {
    let primary = theme.color.primary;
    let radius = theme.border.radius;

    let base = "m-4 overflow-hidden border p-8 shadow-sm bg-white";
    let border_colors = format!("border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {radius} {border_colors}")
}

fn modal() -> String {
    "fixed top-0 bottom-0 left-0 right-0 z-20 flex items-center justify-center bg-black/30"
        .to_owned()
}

fn sidebar(theme: Theme) -> String {
    let primary = theme.color.primary;
    let base = "flex flex-col items-center border-r bg-green p-8 w-80";
    let border_color = format!("border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {border_color}")
}

fn header(theme: Theme) -> String {
    let primary = theme.color.primary;
    let base = "flex items-center justify-between border-b py-8 px-16";
    let border_color = format!("border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {border_color}")
}

fn button(theme: Theme) -> String {
    let primary = theme.color.primary;
    let radius = theme.border.radius;
    let base = "flex w-max space-x-1 py-4 px-6 text-white shadow-md hover:enabled:shadow-xl active:enabled:shadow-lg disabled:cursor-not-allowed disabled:bg-gray-400 disabled:shadow-none";
    let colors = format!("bg-{primary}-500 hover:enabled:bg-{primary}-600");
    let border = format!("{radius} border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {colors} {border}")
}
