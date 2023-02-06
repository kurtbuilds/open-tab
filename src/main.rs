use std::path::PathBuf;
use std::process::{Command, Stdio};
use clap::{Parser, Subcommand, ValueEnum, Args};

const SCRIPT_DIR: &'static str = env!("SCRIPT_DIR");

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    url: String,
}

pub struct Tab {
    title: String,
    window: usize,
    tab: usize,
}

fn parse_tabs(list_chrome_tabs_output: &str) -> Vec<Tab> {
    let mut tabs = Vec::new();
    for line in list_chrome_tabs_output.lines() {
        let mut parts = line.splitn(3, ',');
        let window: usize = parts.next().unwrap().parse().unwrap();
        let tab: usize = parts.next().unwrap().parse().unwrap();
        let title = parts.next().unwrap().to_string();
        tabs.push(Tab { title, window, tab });
    }
    tabs
}

fn main() {
    let cli = Cli::parse();

    let chrome_tabs = Command::new("osascript")
        .arg(PathBuf::from(SCRIPT_DIR).join("list_chrome_tabs.scpt"))
        .stdout(Stdio::piped())
        .output()
        .expect("list_chrome_tabs.scpt failed to start.");
    let tabs = parse_tabs(&String::from_utf8(chrome_tabs.stdout).unwrap());
    if let Some(t) = tabs.iter().find(|t| t.title == cli.url) {
        Command::new("osascript")
            .arg(PathBuf::from(SCRIPT_DIR).join("activate_chrome_tab.scpt"))
            .arg(t.window.to_string())
            .arg(t.tab.to_string())
            .output()
            .expect("activate_chrome_tab.scpt failed to start.");
    } else {
        Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(cli.url)
            .output()
            .expect("open failed to start.");
    }
}