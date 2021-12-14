pub mod utils{
    use std::io::{prelude::*, BufReader};
    use std::fs::File;

    pub fn read_buffer(filename: &str) -> BufReader<File> {
        let file = File::open(filename).unwrap();
        return BufReader::new(file);
    }

    pub fn read_nums(filename: &str) -> impl Iterator<Item=i32> + '_ {
        let lines = read_buffer(filename).lines();
        return lines.map(|x| x.unwrap().trim().parse::<i32>().unwrap());
    }

}
