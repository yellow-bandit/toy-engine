use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use csv::{ReaderBuilder, Trim, Writer};
use tempfile::tempfile;
use toy_engine::Engine;

#[test]
fn test_example_no_whitespace() {
    assert_input_output("inputs/example_no_whitespace.csv", "outputs/example.csv");
}

#[test]
fn test_example_whitespace() {
    assert_input_output("inputs/example_whitespace.csv", "outputs/example.csv");
}

#[test]
fn test_example_no_zeroes() {
    assert_input_output("inputs/example_no_zeroes.csv", "outputs/example.csv");
}

#[test]
fn test_big_number() {
    assert_input_output("inputs/big_number.csv", "outputs/big_number.csv");
}

#[test]
fn test_example_trailing_comma() {
    assert_input_output("inputs/example_trailing_comma.csv", "outputs/example.csv");
}

#[test]
fn test_empty_input() {
    assert_input_output("inputs/empty.csv", "outputs/empty.csv");
}

#[test]
fn test_wrong_header() {
    let mut engine = Engine::default();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path("inputs/wrong_header.csv")
        .unwrap();
    assert!(engine.load_from_reader(reader).is_err());
}

fn assert_input_output(input_path: &str, output_path: &str) {
    let mut engine = Engine::default();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(input_path)
        .unwrap();
    engine.load_from_reader(reader).unwrap();

    let mut tmpfile = tempfile().unwrap();
    let writer = Writer::from_writer(&mut tmpfile);
    engine.dump_accounts(writer).unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    assert_files_equal(&mut tmpfile, &mut File::open(output_path).unwrap());
}

fn assert_files_equal(f1: &mut File, f2: &mut File) {
    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    f1.read_to_string(&mut buffer1).unwrap();
    f2.read_to_string(&mut buffer2).unwrap();

    assert_eq!(buffer1.trim(), buffer2.trim());
}
