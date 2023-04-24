use serde::{Deserialize, Serialize};
use std::{env, fs, path, process};

fn main() {
    let theme = theme();
    let classes = classes(theme);

    write_json(&classes);
    write_ts(&classes);
}

fn write_json(classes: &Classes) {
    let mut d = env::current_dir().unwrap();
    d.push("src/components/classes.json");
    let classes_json = serde_json::to_string_pretty(&classes).unwrap();
    fs::write(d, classes_json).unwrap();
}

fn write_ts(classes: &Classes) {
    let mut d = env::current_dir().unwrap();
    d.push("src/components/classes.ts");
    let classes_json = serde_json::to_string_pretty(&classes).unwrap();
    let ts = format!("export const classes = {classes_json}\n");
    fs::write(d, ts).unwrap();
}

fn classes(theme: Theme) -> Classes {
    Classes {
        card: card(&theme),
        link: link(),
        modal: modal(),
        sidebar: sidebar(&theme),
        header: header(&theme),
        button: button(&theme),
    }
}

fn theme() -> Theme {
    Theme {
        color: Color { primary: "blue" },
        border: Border {
            radius: "rounded-2xl",
        },
    }
}

fn link() -> String {
    "text-blue-500 hover:text-blue-600 hover:underline".to_owned()
}

struct Theme {
    color: Color,
    border: Border,
}

struct Color {
    primary: &'static str,
}

struct Border {
    radius: &'static str,
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

fn card(theme: &Theme) -> String {
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

fn sidebar(theme: &Theme) -> String {
    let primary = theme.color.primary;
    let base = "flex flex-col items-center border-r bg-green p-8 w-80";
    let border_color = format!("border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {border_color}")
}

fn header(theme: &Theme) -> String {
    let primary = theme.color.primary;
    let base = "flex items-center justify-between border-b py-8 px-16";
    let border_color = format!("border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {border_color}")
}

fn button(theme: &Theme) -> String {
    let primary = theme.color.primary;
    let radius = theme.border.radius;
    let base = "flex w-max space-x-1 py-4 px-6 text-white shadow-md hover:enabled:shadow-xl active:enabled:shadow-lg disabled:cursor-not-allowed disabled:bg-gray-400 disabled:shadow-none";
    let colors = format!("bg-{primary}-500 hover:enabled:bg-{primary}-600");
    let border = format!("{radius} border-{primary}-200 hover:border-{primary}-300");

    format!("{base} {colors} {border}")
}
