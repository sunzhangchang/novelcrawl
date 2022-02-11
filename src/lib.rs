#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::io::Write;

mod search;
mod search_book;
mod utils;

use search::caimoge::search_caimoge;
use search_book::SearchBook;
use utils::fetch_path;

async fn download_caimoge(input_url: &str) -> Result<Vec<u8>, ()> {
    println!("{}", input_url);
    let html_id = input_url
        .trim()
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap();

    let novel_url =
        String::from("https://down.caimoge.net/modules/article/txtarticle.php?id=") + html_id;

    let response = fetch_path(&novel_url).await;

    if let Ok(res) = response {
        match res.bytes().await {
            Ok(data_buf) => {
                let data = data_buf.to_vec();
                Ok(data)
            }
            Err(_) => {
                println!("Read response bytes error!");
                Err(())
            }
        }
    } else {
        Err(())
    }
}

#[napi]
pub async fn rs_search(search_key: String) -> Option<Vec<SearchBook>> {
    let mut search_books = Vec::new();

    if let Ok(data) = search_caimoge(&search_key).await {
        search_books.extend(data)
    }

    if search_books.len() > 0 {
        Some(search_books)
    } else {
        None
    }
}

#[napi]
pub async fn rs_download(download_url: String, download_path: String) -> Option<()> {
    let data = if let Ok(res) = download_caimoge(&download_url).await {
        res
    } else {
        return None;
    };

    let mut pth = download_path;

    if !&pth.ends_with(".txt") {
        pth += ".txt"
    }

    let mut f = std::fs::File::create(pth).unwrap();
    f.write(&data).unwrap();
    Some(())
}
