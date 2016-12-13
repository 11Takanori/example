extern crate csv;

fn main() {
    use csv::{Writer, Reader};

    // let people = vec![
    //     ("Yamada", "Taro", 1989),
    //     ("Sato", "Takashi", 1990),
    //     ("Suzuki", "Emi", 1991),
    // ];
    // let path = "sample.csv";
    // let mut writer = Writer::from_file(path).unwrap();

    // for row in people {
    //     writer.encode(row).expect("CSV writer error");
    // }

    let mut reader = Reader::from_file("sample.csv").unwrap().has_headers(false);
    for row in reader.decode() {
        let row: (String, String, usize) = row.unwrap();
        println!("{:?}", row);
    }
}