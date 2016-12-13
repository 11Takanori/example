extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
struct People {
    last_name: String,
    first_name: String,
    year: usize,
}

fn main() {
    // use csv::{Writer,Reader};

    let people = vec![
        ("Yamada", "Taro", 1989),
        ("Sato", "Takashi", 1990),
        ("Suzuki", "Emi", 1991),
    ];
    // let path = "sample.csv";
    let mut writer = csv::Writer::from_file("sample.csv").unwrap();

    // for row in people {
    //     writer.encode(row).expect("CSV writer error");
    // }

    let people = People {
        last_name: "Takahashi".to_owned(),
        first_name: "Hanako".to_owned(),
        year: 1992,
    };

    writer.encode(people).expect("CSV writer error");
    writer.flush().expect("Flush error");


}