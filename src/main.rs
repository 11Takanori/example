extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
struct People {
    last_name: String,
    first_name: String,
    year: usize,
}

fn main() {
    use csv::{Writer,Reader};

    let people = People {
        last_name: "Takahashi".to_owned(),
        first_name: "Hanako".to_owned(),
        year: 1992,
    };

    writer.encode(people).expect("CSV writer error");
    writer.flush().expect("Flush error");
}