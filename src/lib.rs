use std::io::Write;
use std::process::{Command, Stdio};

struct Tab {
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

pub fn open_tab(url: &str) {
    let mut child = Command::new("osascript")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Applescript list_chrome_tabs failed to start.");

    child.stdin.as_mut().unwrap().write_all(include_bytes!("applescript/list_chrome_tabs.applescript")).unwrap();
    let chrome_tabs = child.wait_with_output().unwrap();
    let tabs = parse_tabs(&String::from_utf8(chrome_tabs.stdout).unwrap());
    if let Some(t) = tabs.iter().find(|t| t.title == url) {
        let mut child = Command::new("osascript")
            .stdin(Stdio::piped())
            .arg("-")
            .arg(t.window.to_string())
            .arg(t.tab.to_string())
            .spawn()
            .expect("Applescript activate_chrome_tab failed to start.");
        child.stdin.as_mut().unwrap().write_all(include_bytes!("applescript/activate_chrome_tab.applescript")).unwrap();
        child.wait().unwrap();
    } else {
        Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(url)
            .output()
            .expect("open failed to start.");
    }
}