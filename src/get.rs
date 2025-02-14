use {
    colored::Colorize,
    rpassword,
    std::io::{stdin, stdout, Error, Write},
    url::Url,
};

pub fn input<P: AsRef<str>>(prompt: P) -> Result<String, Error> {
    print!("{} {}", "[>]".bold().blue(), prompt.as_ref());
    stdout().flush()?;

    let mut input = String::new();
    let _ = stdin().read_line(&mut input);

    Ok(input.trim().to_owned())
}

pub fn agreement<P: AsRef<str>>(prompt: P) -> Result<bool, Error> {
    loop {
        let txt = input(prompt.as_ref())?;
        println!("{}", txt);
        match txt.as_str() {
            "y" | "Y" => return Ok(true),
            "n" | "N" => return Ok(false),
            _ => continue,
        }
    }
}

pub fn url<P: AsRef<str>>(prompt: P) -> Result<String, Error> {
    loop {
        match Url::parse(&input(&prompt)?).map_err(|_| ()) {
            Ok(url) => return Ok(url.to_string()),
            Err(_) => continue,
        };
    }
}

pub fn secret<P: AsRef<str>>(prompt: P) -> Result<String, Error> {
    let modified_prompt = format!("{} {}", "[>]".bold().blue(), prompt.as_ref());
    let input = rpassword::prompt_password(modified_prompt)?;
    Ok(input)
}
