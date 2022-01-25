#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::io::Write;

use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};
use futures::executor::block_on;
use regex::Regex;
use reqwest::{header::USER_AGENT, Client, Method};

const USER_AGENT_VAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36";

#[tokio::main]
async fn fetch_path(path: &str) -> (String, Vec<u8>) {
  let mut data = (String::new(), vec![]);

  let request = reqwest::Client::new()
    .request(Method::GET, path)
    .header(USER_AGENT, USER_AGENT_VAL)
    .build()
    .unwrap();

  match block_on(Client::new().execute(request)) {
    Ok(response) => {
      let header = response
        .headers()
        .get("content-disposition")
        .unwrap()
        .clone();
      let name = header.as_bytes();
      let name = GBK.decode(name, DecoderTrap::Strict).unwrap();
      let name = name.split("filename=").last().unwrap();

      match block_on(response.bytes()) {
        Ok(data_buf) => {
          data = (String::from(name), data_buf.to_vec());
        }
        Err(_) => {
          println!("Read response bytes error!");
        }
      };
    }
    Err(_) => {
      println!("Request get error!");
    }
  }
  data
}

#[napi]
pub fn crawl(input_url: String) -> String {
  // let input_url = "https://www.caimoge.net/txt/69667.html";
  let re = Regex::new(r"((http(s)?:)?//)?www.caimoge.net/txt/[1-9]+.html").unwrap();
  let url = input_url.trim();

  if !re.is_match_at(url, 0) {
    return String::from("目前只支持采墨阁(www.caimoge.net)!");
  }

  let html_id = url.split('/').last().unwrap().split('.').next().unwrap();
  let novel_url =
    String::from("https://down.caimoge.net/modules/article/txtarticle.php?id=") + html_id;

  let (name, data) = fetch_path(&novel_url);
  let name = name.split("[www.caimoge.net]").next().unwrap();
  let mut f = std::fs::File::create(String::from(name) + ".txt").unwrap();
  f.write(&data).unwrap();

  String::from("Ok")
}
