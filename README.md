# CSV
Simplified CSV library to create and update CSV files. 

In order to keep the package light-weight, the default record is type `Vec<String>`.

Any data manipulation will need to be done outside of this crate. CSV is not meant to replace the functionality of a database.

## IMPORTANT
This library can only pars CSV's with a `,` delimiter. However, it can save CSV's with a custom delimiter if needed.

## Objective
Zero dependencies outside of the std library. 

## TODO
- *DUE 9/20/23*: Read values separated by "", handle new lines within "" as well... Checkout `./src/utils.rs`
- Read a CSV with any type of delimiter `utils::raw_csv_to_records`
- Create tests for different delimiter types, should probably restrict the characters used