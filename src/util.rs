pub fn load_file(day: i32, part: i32, runtest: bool) -> String {
    let teststr;
    if runtest {
        teststr = "test_"
    } else {
        teststr = ""
    }

    let path = std::format!("data/day{day}/{teststr}input{part}.txt");
    println!("loading data from '{}'", path);
    std::fs::read_to_string(path).unwrap()
}