fn main() {
    let lines: Vec<String> = std::io::stdin().lines().collect::<Result<_, _>>().unwrap();
    let shard_count = lines.get(0).unwrap().parse().unwrap();
    let addresses = lines[1..lines.len()].to_vec();
    let client = dianadb::Client::connect(shard_count, &addresses).unwrap();
    let statement = "SELECT column_name FROM table_name".to_string();
    let table = client.run_statement(&statement).unwrap();
    table.print();
}
