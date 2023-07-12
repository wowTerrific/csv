use csv::*;

mod setup;

#[test]
fn create_csv_instance_from_file() {
    let csv = CSV::new_from_file("./tests/test_data/instance_test.csv")
        .unwrap_or_else(|e| {
            panic!("{:?}: Check the test_data folder for instance_test.csv!", e);
        });

    let expected_data: Vec<Record> = vec![
        vec!["header 1".to_string(), "header 2".to_string(), "header 3".to_string()],
        vec!["value 1".to_string(), "value 2".to_string(), "value 3".to_string()],
        vec!["value 4".to_string(), "value 5".to_string(), "value 6".to_string()],
    ];

    assert_eq!(expected_data, csv.data);
    assert_eq!("./tests/test_data/instance_test.csv", csv.path);
}

#[test]
fn create_edit_csv_file_from_instance() {
    setup::remove_extra_csvs();

    let test_string = String::from("Test manipulation");

    {
        let mut csv = CSV::new_from_file("./tests/test_data/instance_test.csv")
            .unwrap_or_else(|e| {
                panic!("{:?}: Check the test_data folder for instance_test.csv!", e);
            });

        // manipulation testing
        csv.path = "./tests/test_data/save_test.csv";
        csv.data[0][0] = test_string.clone();

        if let Err(e) = csv.save() {
            panic!("{:?}: save method failed!", e);
        };

        assert!(match csv.check_state() {
            &SaveState::Saved => true,
            &SaveState::Unsaved => false,
        });
    }
    

    {
        let new_csv = CSV::new_from_file("./tests/test_data/save_test.csv")
        .unwrap_or_else(|e| {
            panic!("{:?}: save and read failed!", e)
        });

        assert_eq!(new_csv.data[0][0], test_string);
        assert!(match new_csv.check_state() {
            &SaveState::Saved => false,
            &SaveState::Unsaved => true,
        });   
    }

    setup::remove_extra_csvs();
}