pub mod io {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::sync::Mutex;

    use chrono::{DateTime, Local};
    use hashbrown::HashMap;
    use rayon::prelude::*;
    use regex::Regex;

    use crate::IP_REGEX;

    pub fn get_file_buffer(file: &str) -> BufReader<File> {
        let file = File::open(file).expect(format!("The file {} does not exist or is not readable by the current user", file).as_str());
        BufReader::new(file)
    }

    pub fn fill_ip(ips: &mut HashMap<String, u32>, line: String) {
        if let Some(ip) = Regex::new(IP_REGEX).unwrap().find(&line) {
            let ip = ip.as_str().to_string();
            ips.entry(ip).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    pub fn fill_ips(ips: &Mutex<HashMap<String, u32>>, buffer: BufReader<File>) {
        buffer.lines().par_bridge().for_each(|line| {
            let line = line.unwrap();
            let mut ips = ips.lock().unwrap();
            fill_ip(&mut ips, line);
        });
    }

    fn format_filename(filename: &str) -> String {
        let now: DateTime<Local> = Local::now();
        now.format(filename).to_string()
    }

    pub fn ip_writer(filename: &str, ips: &Vec<(String, u32)>, extra: impl Fn(String) -> ()) {
        let mut output = File::create(format_filename(filename)).unwrap();
        writeln!(output, "Count\tName").unwrap();
        writeln!(output, "-----\t----").unwrap();

        for (ip, count) in ips.iter() {
            let data = format!("{:>5}\t{}", count, ip);
            extra(data.clone());
            writeln!(output, "{}", data).unwrap();
        }
    }
}

pub mod iter {
    use hashbrown::HashMap;

    pub fn get_top_ips(ips: &HashMap<String, u32>, top: usize) -> Vec<(String, u32)> {
        let mut top_ips = ips.iter().map(|(ip, count)| (ip, *count)).collect::<Vec<_>>();
        top_ips.sort_by(|a, b| b.1.cmp(&a.1));
        top_ips.iter().take(top).map(|(ip, count)| (ip.to_string(), *count)).collect()
    }
}

pub mod web {
    use reqwest::blocking;
    use serde_json::Value;

    pub fn get_country(ip: &str) -> String {
        let country: Value = blocking::get(format!("http://ip-api.com/json/{}", ip)).unwrap().json().unwrap();
        country.get("country").unwrap().to_string()
    }
}

