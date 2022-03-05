use reqwest::Response;
use reqwest::{header::USER_AGENT, Client, Method};

const USER_AGENT_VAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36";

pub async fn fetch_path(path: &str) -> Result<Response, ()> {
    let request = Client::new()
        .request(Method::GET, path)
        .header(USER_AGENT, USER_AGENT_VAL)
        .build()
        .unwrap();

    match Client::new().execute(request).await {
        Ok(response) => Ok(response),
        Err(_) => {
            println!("Request get error!");
            Err(())
        }
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
