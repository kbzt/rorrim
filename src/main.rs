use self::cli::{MIRROR_NUM, MIRROR_URL};
use self::request::{Mirror, Request};
use chrono::prelude::*;
use std::{fs, io::Write, path::PathBuf};

mod cli;
mod request;

fn write_file(path: &PathBuf, mirrors: &Vec<&Mirror>) {
    let date: DateTime<Local> = Local::now();

    let mut file = match fs::File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("Unable to open file {}: {}", path.display(), why),
    };

    file.write_all(format!("# generated by rorrim \n# {}\n\n", date).as_bytes())
        .unwrap();
    mirrors.iter().for_each(|mirror| {
        file.write_all(format!("{}\n", mirror.url).as_bytes())
            .unwrap()
    });
}

fn main() {
    let args = <cli::Args as clap::Parser>::parse();

    // let req = reqwest::blocking::get(MIRROR_URL).unwrap();
    // let var: Request = req.json().unwrap();

    let req = fs::read_to_string("sample.json").unwrap();
    let var: Request = serde_json::from_str(&req).unwrap();

    let mirrors: Vec<&Mirror> = var
        .urls
        .iter()
        .filter(|x| {
            args.country.contains(&x.country)
                && args
                    .protocol
                    .iter()
                    .map(|x| x.to_str())
                    .any(|n| n == x.protocol)
                && args.no_iso
                && args.no_ipv4
                && args.no_ipv6
        })
        .take({
            match args.number {
                Some(num) => num.into(),
                None => MIRROR_NUM.into(),
            }
        })
        .collect();

    match args.save {
        Some(path) => write_file(&path, &mirrors),
        None => mirrors.iter().for_each(|mirror| println!("{}", mirror.url)),
    }
}
