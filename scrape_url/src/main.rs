use std::fs;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let url = &args[1];
  let output = &args[2];

  println!("you input the url is  {}", url);
  println!("you input the file name is {}", output);

  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);

  for arg in std::env::args() {
    println!("{}", arg);
  }
}