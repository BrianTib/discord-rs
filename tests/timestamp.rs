use discord_rs::structs::timestamp::Timestamp;

#[test]
fn test_string_to_timestamp() {
    let string_timestamp: Timestamp = String::from("2023-11-21T14:52:38.313Z").into();
    let number_timestamp = Timestamp::Number(1700578358);
    // Assert that both compile to the same value
    assert_eq!(string_timestamp, number_timestamp);
}