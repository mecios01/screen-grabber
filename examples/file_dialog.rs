use chrono::Utc;

fn main() {
    let path = std::env::current_dir().unwrap();
    let dt = Utc::now();

    let timestamp = dt.format("%Y%m%dT%H%M%S");
    let res = rfd::FileDialog::new()
        .set_file_name(format!("{}", timestamp))
        .add_filter("png", &["png"])
        .add_filter("jpg", &["jpg"])
        .add_filter("ico", &["ico"])
        .add_filter("gif", &["gif"])
        .set_directory(&path)
        .save_file();

    println!("The user choose: {:#?}", res);
}
