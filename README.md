# CSV
Simplified CSV library to create and update CSV files. 

In order to keep the package light-weight, the default record is type `Vec<String>`.

Any data manipulation will need to be done outside of this crate. CSV is not meant to replace the functionality of a database.

## Objective
Zero dependencies outside of the std library.