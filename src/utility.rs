use std::fs::File;
use std::io::Read;

use anyhow::Result;
use ndarray::Array2;

/// Read problem input from file
pub fn input_from_file(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents.trim().to_owned())
}

pub fn plot(array: &Array2<u32>) -> String {
    (0..array.shape()[0])
        .flat_map(|i| {
            (0..array.shape()[1])
                .map(move |j| if array[(i, j)] == 0 { ' ' } else { '█' })
                .chain(vec!['\n'].into_iter())
        })
        .collect()
}
