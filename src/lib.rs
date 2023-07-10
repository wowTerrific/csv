pub mod errors;

/// The CSV struct is used to store and manipulate date within memory
/// before saving/writing to an external file
pub struct CSV {
    path: String,
    data: Vec<Vec<String>>
}

pub fn read(path: &str) -> Result<CSV, errors::Error> {
    let csv = CSV::new(path);
    todo!();
}

pub fn write(path: &str, data: Vec<Vec<String>>) -> Result<(), errors::Error> {
    todo!();
}

pub fn append(path: &str, data: Vec<Vec<String>>) -> Result<(), errors::Error> {
    todo!();
}



impl CSV {
    
    // used when creating a new CSV
    pub fn new(path: &str) -> CSV {
        todo!();
    }

    pub fn new_with_data(path: &str, data: Vec<Vec<String>>) -> CSV {
        CSV {
            path: path.to_string(),
            data,
        }
    }

    pub fn get_last_record(&self) -> Result<Vec<String>, errors::Error> {
        todo!();
    }

    pub fn get_headers(&self) -> Result<Vec<String>, errors::Error> {
        todo!();
    }

    pub fn len(&self) -> Result<u32, errors::Error> {
        todo!();
    }

    // would it be better to add "insert line" methods and "save" to write to file?

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
