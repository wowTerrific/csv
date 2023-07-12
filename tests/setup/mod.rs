use std::fs;

pub fn remove_extra_csvs() {
    fs::remove_file("./tests/test_data/save_test.csv").unwrap_or(());
}