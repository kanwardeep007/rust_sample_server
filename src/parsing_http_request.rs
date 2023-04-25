// Import the Pest library
use pest::{iterators::Pair, Parser};

// Define the grammar for parsing HTTP requests
#[derive(Parser)]
#[grammar = "http_request.pest"]
struct HttpRequestParser;

pub struct HttpParsedRequest<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub uri: &'a str,
    pub version: &'a str,
}

// Define a function that takes an HTTP request as a string and returns a result
pub fn parse_http_request(http_request: &str) -> Result<HttpParsedRequest, String> {
    // Use the Pest library to parse the HTTP request
    let pairs =
        HttpRequestParser::parse(Rule::request, http_request).map_err(|e| format!("{}", e))?;

    let mut parsed_request = HttpParsedRequest {
        path: Default::default(),
        method: Default::default(),
        uri: Default::default(),
        version: Default::default(),
    };
    // Iterate over the pairs returned by the parser
    for pair in pairs {
        match pair.as_rule() {
            Rule::request => {
                // The path of the HTTP request
                parsed_request.path = pair.as_str();
                println!("Path: {}", parsed_request.path);
                let mut inner = pair.into_inner();
                parsed_request.method = inner.next().unwrap().as_str();
                println!("method: {}", parsed_request.method);
                parsed_request.uri = inner.next().unwrap().as_str();
                println!("uri: {}", parsed_request.uri);
                parsed_request.version = inner.next().unwrap().as_str();
                println!("version: {}", parsed_request.version);
            }
            _ => unreachable!(),
        }
    }

    Ok(parsed_request)
}
