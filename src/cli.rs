pub fn get_io_paths() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();

    // args.iter().for_each(|x| println!("{:?}", x));
    // 0th arg will generally be the executable path
    // 1st arg will be csv name/path
    // TODO probably a better way to ensure this
    let input_path = args
        .get(1)
        .unwrap_or_else(|| {
            panic!("input path was not provided");
        })
        .clone();
    println!("Input path: {:?}", input_path);
    let output_path = args
        .get(2)
        .unwrap_or_else(|| {
            panic!("Output path was not provided");
        })
        .clone();
    (input_path, output_path)
}
