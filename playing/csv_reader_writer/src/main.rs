use csv::{Writer, Reader};

fn main() {
    let mut csv = Reader::from_path("data.csv").unwrap_or_else(|_| {
        let mut wtr = Writer::from_path("data.csv").unwrap();
        let fruits = vec![
            ("Apple", 3, "New York", 1.2),
            ("Banana", 2, "Washington", 2.3),
            ("Cherry", 5, "Los Angeles", 3.4),
        ];
        wtr.write_record(&["Fruit", "Quantity", "City", "Price $"]).unwrap();
        fruits.iter().for_each(|(fruit, quantity, city, price)|
            wtr.write_record(&[fruit, quantity.to_string().as_str(), city, price.to_string().as_str()]).unwrap()
        );
        wtr.flush().unwrap();
        Reader::from_path("data.csv").unwrap()
    });

    let headers = &csv.headers().unwrap();
    headers.iter().for_each(|header| print!("{}, ", header));
    let records = &csv.records().map(|record| record.unwrap()).collect::<Vec<_>>();
    records.iter().for_each(|record| {
        println!();
        record.iter().for_each(|field| print!("{}, ", field));
    });
}
