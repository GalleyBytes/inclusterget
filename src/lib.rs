use std::{env, fs};

pub fn env_with_default(key: String, default: String) -> Result<String, ()> {
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(_) => {
            if default == "" {
                Err(())
            } else {
                Ok(default)
            }
        }
    }
}

#[tokio::main]
pub async fn get(
    group: String,
    kind: String,
    namespace: String,
    resource: String,
) -> Result<String, reqwest::Error> {
    let host = env_with_default(
        String::from("KUBERNETES_SERVICE_HOST"),
        String::from("kubernetes.default.svc"),
    )
    .expect("KUBERNETES_SERVICE_HOST is not set");
    let crtfile = env_with_default(
        String::from("CERTFILE"),
        String::from("/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"),
    )
    .expect("CERTFILE is not set");
    let token_file = env_with_default(
        String::from("TOKENFILE"),
        String::from("/var/run/secrets/kubernetes.io/serviceaccount/token"),
    )
    .expect("TOKENFILE is not set");

    let apis = if group.contains("/") {
        String::from("apis")
    } else {
        String::from("api")
    };
    let url = format!(
        "https://{}/{}/{}/namespaces/{}/{}/{}",
        host, apis, group, namespace, kind, resource
    );
    println!("{}", url);

    fs::metadata(&crtfile).expect(&crtfile);
    let crtdata = fs::read(crtfile).unwrap();
    let ca_cert = reqwest::tls::Certificate::from_pem(&crtdata).expect("Not a valid cert");

    let client = reqwest::Client::builder()
        .add_root_certificate(ca_cert)
        .build()
        .unwrap();

    fs::metadata(&token_file).expect(&token_file);
    let mut bearer = Vec::from("Bearer ".as_bytes());
    let mut tokendata = fs::read(token_file).unwrap();
    bearer.append(&mut tokendata);
    let token = reqwest::header::HeaderValue::from_bytes(&bearer).unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", token);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
