extern crate csv;
extern crate rustc_serialize;
extern crate hyper;

#[derive(RustcEncodable)]
struct People {
    last_name: String,
    first_name: String,
    year: usize,
}

fn main() {
    let people = People {
        last_name: "Takahashi".to_owned(),
        first_name: "Hanako".to_owned(),
        year: 1992,
    };
    let mut writer = csv::Writer::from_file("sample.csv").unwrap();

    writer.encode(people).expect("CSV writer error");
}
