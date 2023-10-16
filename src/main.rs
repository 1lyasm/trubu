fn main() {
    let shard_count = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut addresses = Vec::new();
    std::io::stdin()
        .lines()
        .for_each(|line| addresses.push(line.unwrap()));
    let client = dianadb::Client::connect(shard_count, &addresses).unwrap();
    let mut statement = "SELECT column_name FROM table_name".to_string();
    let table = client.run_statement(&statement).unwrap();
    table.print();
}
