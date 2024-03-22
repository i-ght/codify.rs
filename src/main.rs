use std::{collections::BTreeMap, error::Error, fs::File, io::BufReader, str::FromStr};
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
type DayBookEntries = BTreeMap<EntryYear, EntryMonths>;

fn proc_record(
    record: Result<StringRecord, csv::Error>,
    book_of_days: &mut BTreeMap<NaiveDate, Vec<DayBookEntry>>
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
        book_of_days
            .entry(date)
            .or_insert(Vec::with_capacity(8));
    entries.push(entry);

    Result::Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut book_of_days =
        BTreeMap::<NaiveDate, Vec<DayBookEntry>>::new();

    let path = "data/book_of_days.csv";
    let file = File::open(path)?;
    let reader =
        BufReader::<File>::new(file);
    let mut rdr =
        csv::Reader::from_reader(reader);

    for record in rdr.records() {
        match proc_record(record, &mut book_of_days) {
            Err(e) => panic!("{:#?}", e),
            Ok(()) => ()
        };
    }

    let mut book_of_fate =
        DayBookEntries::new();

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
    
    println!("{:#?}", book_of_fate[&2024]);

    for (year, months) in book_of_fate {
        for (month, days) in months {
            for (day, entries) in days {

            }
        }
    }
        
    Result::Ok(())
}
