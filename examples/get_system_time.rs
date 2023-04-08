use reqwest::blocking::Client;

fn main() {
    let response = Client::new()
        .get("http://127.0.0.1:8000/time")
        .header("Authorization", "WVdCXmcO07VdKX8GA")
        .send();

    match response {
        Ok(r) => {
            let status_code = r.status().as_u16();
            match status_code {
                401 => eprintln!("The auth key is wrong!"),
                200 => println!("{}", r.text().unwrap()),
                _ => eprintln!("Unknown error. The status code is {}.", status_code),
            }
        },
        Err(_) => {
            eprintln!("Please run api.rs first. Use `cargo run --example get_system_time_api`.")
        },
    }
}
