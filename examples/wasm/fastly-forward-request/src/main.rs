use fastly::{
    http::{Request, Response, StatusCode},
    Backend, Error,
};
use std::str::FromStr;
use std::time::Duration;
use url::Url;

// Entry point—Fastly gives us a request, we return a response.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Grab the raw URL and pull out whatever’s after "url=".
    let raw_url = req.get_url();
    let binding = raw_url.to_string();
    let forward_url = binding.split("url=").nth(1).expect("Expect url");

    // Parse it. If it’s bad, send a 400 with an error.
    let parsed_url = match Url::from_str(forward_url) {
        Ok(url) => url,
        Err(e) => {
            return Ok(Response::from_status(StatusCode::BAD_REQUEST)
                .with_body(format!("Invalid URL: {}", e)))
        }
    };

    // Get the host and set up a backend with timeouts and SSL.
    let host = parsed_url.host().expect("Expected host").to_string();
    let backend_name = host.replace('.', "_");
    let backend = Backend::builder(backend_name.as_str(), host.as_str())
        .override_host(host.as_str())
        .connect_timeout(Duration::from_secs(1))
        .first_byte_timeout(Duration::from_secs(15))
        .between_bytes_timeout(Duration::from_secs(10))
        .enable_ssl()
        .finish()?;

    // Make a new request with the original method and forward URL.
    let new_req = Request::new(req.get_method(), forward_url);

    // Send it to the backend and return the response.
    Ok(new_req.send(backend)?)
}
