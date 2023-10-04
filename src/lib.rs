use std::time::Duration;
use std::{env, fs};

/// make a get request to incluster api. Err returns a (String, bool) as a message and err_code respectively.
#[tokio::main]
pub async fn get(
    group: String,
    kind: String,
    namespace: String,
    resource: String,
) -> Result<String, (String, u8)> {
    let default_host = "kubernetes.default.svc";
    let default_cert_file = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt";
    let default_token_file = "/var/run/secrets/kubernetes.io/serviceaccount/token";
    let host = env::var("KUBERNETES_SERVICE_HOST").unwrap_or(String::from(default_host));
    let cert_file = env::var("CERTFILE").unwrap_or(String::from(default_cert_file));
    let token_file = env::var("TOKENFILE").unwrap_or(String::from(default_token_file));
    let apis = if group.contains("/") {
        String::from("apis")
    } else {
        String::from("api")
    };
    let url = format!(
        "https://{}/{}/{}/namespaces/{}/{}/{}",
        host, apis, group, namespace, kind, resource
    );
    // println!("{}", url);
    let mut bearer = Vec::from("Bearer ".as_bytes());

    let stat = fs::metadata(&cert_file);
    if stat.is_err() {
        return Err((
            format!("{}: {}", cert_file, stat.unwrap_err().to_string()),
            2,
        ));
    }
    let crtdata = fs::read(&cert_file).unwrap();
    let ca_cert_valid = reqwest::tls::Certificate::from_pem(&crtdata);
    if ca_cert_valid.is_err() {
        return Err((
            format!(
                "{}: Not a valid cert: {}",
                cert_file,
                ca_cert_valid.unwrap_err().to_string()
            ),
            3,
        ));
    }
    let ca_cert = ca_cert_valid.unwrap();

    let client = reqwest::Client::builder()
        .add_root_certificate(ca_cert)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let stat = fs::metadata(&token_file);
    if stat.is_err() {
        return Err((
            format!("{}: {}", token_file, stat.unwrap_err().to_string()),
            4,
        ));
    }

    let read_tokendata = fs::read(&token_file);
    if read_tokendata.is_err() {
        return Err((
            format!(
                "{}: {}",
                token_file,
                read_tokendata.unwrap_err().to_string()
            ),
            5,
        ));
    }
    let mut tokendata = read_tokendata.unwrap();

    bearer.append(&mut tokendata);

    let token_header_value = reqwest::header::HeaderValue::from_bytes(&bearer);
    if token_header_value.is_err() {
        return Err((
            format!(
                "{}: {}",
                token_file,
                token_header_value.unwrap_err().to_string()
            ),
            6,
        ));
    }
    let token = token_header_value.unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", token);

    let response: Result<reqwest::Response, reqwest::Error> =
        client.get(url).headers(headers).send().await;
    let body: Result<String, String> = match response {
        Ok(r) => {
            let text: Result<String, reqwest::Error> = r.text().await;
            match text {
                Ok(s) => Ok(s),
                Err(e) => Err(format!("{}", e.to_string())),
            }
        }
        Err(e) => Err(format!("{}", e.to_string())),
    };

    match body {
        Ok(b) => Ok(b.to_string()),
        Err(e) => Err((e, 1)),
    }
}
