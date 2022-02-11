use nipper::Document;
use reqwest::Url;

use crate::{search_book::SearchBook, utils::fetch_path};

pub async fn search_caimoge(search_key: &str) -> Result<Vec<SearchBook>, ()> {
    // let input_url = "https://www.caimoge.net/txt/69667.html";
    let sch = String::from("https://www.caimoge.net/search?searchkey=");
    let base = Url::parse("https://www.caimoge.net/search").unwrap();
    let response = fetch_path(&(sch + &search_key)).await;

    if let Ok(res) = response {
        if let Ok(data) = res.bytes().await {
            let mut search_books = Vec::new();
            for dl in Document::from(&String::from_utf8(data.to_vec()).unwrap())
                .select("#sitembox")
                .select("dl")
                .iter()
            {
                let link = base
                    .join(
                        &dl.select("dd:nth-child(2) > h3:nth-child(1) > a:nth-child(1)")
                            .attr_or("href", "nothing")
                            .to_string(),
                    )
                    .unwrap()
                    .to_string();

                search_books.push(SearchBook {
                    书名: dl
                        .select("dd:nth-child(2) > h3:nth-child(1) > a:nth-child(1)")
                        .text()
                        .to_string(),
                    作者: dl
                        .select("dd:nth-child(3) > span:nth-child(1) > a:nth-child(1)")
                        .text()
                        .to_string(),
                    状态: dl
                        .select("dd:nth-child(3) > span:nth-child(2)")
                        .text()
                        .to_string(),
                    分类: dl
                        .select("dd:nth-child(3) > span:nth-child(3)")
                        .text()
                        .to_string(),
                    字数: dl
                        .select("dd:nth-child(3) > span:nth-child(4)")
                        .text()
                        .to_string(),
                    简介: dl.select("dd:nth-child(4)").text().to_string(),
                    最新章节: dl
                        .select("dd:nth-child(5) > a:nth-child(1)")
                        .text()
                        .to_string(),
                    最近更新: dl
                        .select("dd:nth-child(5) > span:nth-child(2)")
                        .text()
                        .to_string(),
                    目录链接: link,
                });
            }
            Ok(search_books)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
