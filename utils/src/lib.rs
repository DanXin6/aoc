
pub fn file_reader(path: &str) -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open(path).unwrap();
    return std::io::BufReader::new(file);
}