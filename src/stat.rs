use colored::Colorize;

pub fn process<D: AsRef<str>>(data: D) {
    println!("{} {}", "[.]".bold().white(), data.as_ref());
}

pub fn success<D: AsRef<str>>(data: D) {
    println!("{} {}", "[+]".bold().green(), data.as_ref());
}

pub fn warning<D: AsRef<str>>(data: D) {
    println!("{} {}", "[!]".bold().black().on_yellow(), data.as_ref());
}

pub fn error<D: AsRef<str>>(data: D) {
    println!("{} {}", "[-]".bold().black().on_red(), data.as_ref());
}

pub fn fatal<D: AsRef<str>>(data: D) {
    println!(
        "{}{}",
        "[-] ".bold().black().on_red(),
        data.as_ref().black().on_red()
    );
}
