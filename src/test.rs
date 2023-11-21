use discord_rs::structs::timestamp::Timestamp;

pub fn main() {
    let string_timestamp: Timestamp = String::from("2023-11-21T14:52:38.313Z").into();
    let number_timestamp = Timestamp::Number(1700578358);

    println!("{} {}", string_timestamp, number_timestamp);
}