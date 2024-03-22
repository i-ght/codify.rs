use std::{collections::BTreeMap, error::Error, fs::File, io::{BufReader, Write}, str::FromStr};
use chrono::{Datelike, NaiveDate};
use csv::StringRecord;

#[derive(Debug, Clone)]
struct DayBookEntry {
    date: NaiveDate,
    name: String,
    content: String,
    tags: Vec<String>
}

type EntryDay = i32;
type EntryYear = i32;
type EntryMonth = i32;
type Entries = Vec<DayBookEntry>;
type EntryDays = BTreeMap<EntryDay, Entries>;
type EntryMonths = BTreeMap<EntryMonth, EntryDays>;
type DayBook = BTreeMap<EntryYear, EntryMonths>;

fn proc_record(
    record: Result<StringRecord, csv::Error>,
    days: &mut BTreeMap<NaiveDate, Vec<DayBookEntry>>
) -> Result<(), Box<dyn Error>> {
    let entry = record?;
    let date_str = 
        entry
            .get(0)
            .expect("date.");
    let date = NaiveDate::from_str(date_str)?;
    let name = 
        entry
            .get(1)
            .expect("name");
    let tags =
        entry
            .get(2)
            .expect("tags");
    let tags: Vec<String> =
        tags
            .split(",")
            .map(String::from)
            .collect();
        
    let content =
        entry
            .get(3)
            .expect("content");

    let entry = 
        DayBookEntry {
            date,
            name: String::from(name),
            content: String::from(content),
            tags
        };
    
    let entries =
        days
            .entry(date)
            .or_insert(Vec::with_capacity(8));
    entries.push(entry);

    Result::Ok(())
}

fn construct_book_of_fate() -> Result<DayBook, Box<dyn Error>> {
    let mut book_of_days =
        BTreeMap::<NaiveDate, Vec<DayBookEntry>>::new();

    let path = "data/book_of_days.csv";
    let file = File::open(path)?;
    let reader =
        BufReader::<File>::new(file);
    let mut rdr =
        csv::Reader::from_reader(reader);

    for record in rdr.records() {
        proc_record(record, &mut book_of_days)?;
    }

    let mut book_of_fate =
        DayBook::new();

    for (date, entries) in book_of_days {
        let (_ce, year) = date.year_ce();
        let month = (date.month0() + 1) as i32;
        let year = year as i32;
        let day = (date.day0() + 1) as i32;

        let months = 
            book_of_fate
                .entry(year)
                .or_insert_with(BTreeMap::new);

        let days =
            months
                .entry(month)
                .or_insert_with(BTreeMap::new);

        let _entries = 
            days
                .entry(day)
                .or_insert(entries.to_vec());
    }

    Result::Ok(book_of_fate)    
}

fn month_number_to_name(month: u32) -> Option<&'static str> {
    match month {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
}

fn adoc(
    title: &str,
    book_of_fate: &DayBook
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut adoc: Vec<String> = Vec::with_capacity(8);

    let title = format!("= {}", title);

    let header = vec! [
        &title[..],
        ":toc: left",
        ":toclevels: 4",
        ""
    ].join("\n");

    adoc.push(header);

    for (year, months) in book_of_fate {
        let year_fmt = format!("== {}", year);
        adoc.push(year_fmt);
        adoc.push(String::from(""));
    
        for (month, days) in months {
            let date = NaiveDate::from_ymd_opt(0, *month as u32, 1).unwrap();
            let month_name = month_number_to_name(date.month0()+1).unwrap();
            let month_fmt = format!("=== {} {}", year, month_name);
            adoc.push(month_fmt);
            adoc.push(String::from(""));

            for (day, entries) in days {
                let day_fmt = format!("==== {} {}", month_name, day);
                adoc.push(day_fmt);
                adoc.push(String::from(""));

                for entry in entries {

                    let name_fmt = format!("===== {}", entry.name);
                    adoc.push(name_fmt);
                    adoc.push(String::from(""));
                    
                    let content = entry.content.clone().trim().to_string();
                    let mut code_lines: Vec<String> =
                        content
                            .split("\n")
                            .map(|line| format!("{} +", line))
                            .collect();
                    let i = code_lines.len() - 1;
                    code_lines[i] = code_lines[i].trim_end_matches(" +").to_string();
                    for code_line in code_lines {
                        adoc.push(code_line);
                    }
                    adoc.push(String::from(""));
                }
            }
        }
    }
    

    Result::Ok(adoc)
}

fn main() -> Result<(), Box<dyn Error>> {
    let book_of_fate =
        construct_book_of_fate()?;
    let memory_map = adoc("Memory Map", &book_of_fate)?;
    let mut codex = File::create("codex.adoc")?;
    for memory in memory_map {
        codex.write_all(memory.as_bytes())?;
        codex.write_all(b"\n")?;
    }
    codex.flush()?;

    Result::Ok(())
}
