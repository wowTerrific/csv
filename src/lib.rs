use std::collections::HashMap;

pub mod errors;

/// The CSV struct is used to store and manipulate date within memory
/// before saving/writing to an external file
#[derive(Debug)]
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
    
    /// Creates a blank CSV Instance with a desired file path
    pub fn new(path: &str) -> CSV {
        todo!();
    }

    pub fn new_with_data(path: &str, data: Vec<Vec<String>>) -> CSV {
        CSV {
            path: path.to_string(),
            data,
        }
    }

    pub fn new_from_file(path: &str) -> Result<CSV, errors::Error> {
        todo!();
    }


    pub fn get_last_record(&self) -> Result<&Vec<String>, errors::Error> {
        let last_line = &self.data[&self.data.len() - 1];
        if last_line.len() == 0 {
            return Err(errors::Error::DataNotFound);
        }
        Ok(last_line)
    }


    pub fn get_headers(&self) -> Result<HashMap<usize, &String>, errors::Error> {
        let first_line = &self.data[0];
        if first_line.len() == 0 {
            return Err(errors::Error::DataNotFound);
        }
        let mut map: HashMap<usize, &String> = HashMap::new();
        for (i, v) in first_line.into_iter().enumerate() {
            map.insert(i, v);
        }
        Ok(map)
    }


    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn record_len(&self) -> usize {
        self.data[0].len()
    }

    pub fn insert_one(&mut self, data: Vec<String>) {
        self.data.push(data);
    }

    pub fn insert_multi(&mut self, data: Vec<Vec<String>>) {
        for x in data {
            self.data.push(x);
        }
    }

    pub fn save(&self) -> Result<(), errors::Error> {
        // writes to filepath
        todo!();
    }

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
