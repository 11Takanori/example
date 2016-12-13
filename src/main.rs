extern crate csv;

fn main() {
    use csv::Writer;

    let people = vec![
        ("Yamada", "Taro", 1989),
        ("Sato", "Takashi", 1990),
        ("Suzuki", "Emi", 1991),
    ];
    let path = "sample.csv";
    let mut writer = Writer::from_file(path).unwrap();

    for row in people {
        writer.encode(row).expect("CSV writer error");
    }
}