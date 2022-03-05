#![deny(clippy::all)]

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::{io::Write, path::PathBuf};

mod search;
mod search_book;
mod utils;

use search::caimoge::search_caimoge;
use search_book::SearchBook;
use utils::fetch_path;

async fn download_caimoge(input_url: &str) -> Result<Vec<u8>, ()> {
    // println!("{}", input_url);
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

#[wasm_bindgen]
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

#[wasm_bindgen]
pub async fn rs_download(url: String, dir: String, name: String) -> Option<()> {
    let data = if let Ok(res) = download_caimoge(&url).await {
        res
    } else {
        return None;
    };

    // let mut dir = dir;

    // if !&dir.ends_with("/") {
    //     dir += "/";
    // }

    // println!("{}", &dir);

    Some((||
    {
        let mut ptt = PathBuf::from(&dir);
        ptt.push(&name);
        ptt.set_extension("txt");

        let pth = ptt.as_path();

        // println!("--  {:?}", pth);

        std::fs::create_dir_all(&dir).unwrap();

        let mut f = std::fs::File::create(pth).unwrap();
        f.write(&data).unwrap();
    })())
}
