# inclusterget

A basic http client for rust to GET resources in k8s when running in-cluster.

## Usage

Add the module as a Cargo.toml dependency:

```toml
[dependencies]
inclusterget = { path = "../inclusterget" }
```

or

```toml
[dependencies]
inclusterget = { git = "https://github.com/galleybytes/inclusterget.git" }
```

Example code:

```rust
use inclusterget;

fn main() {
    let group = inclusterget::env_with_default(String::from("GROUP"), String::from(""))
        .expect("GROUP is not set");
    let kind = inclusterget::env_with_default(String::from("KIND"), String::from(""))
        .expect("KIND is not set");
    let namespace = inclusterget::env_with_default(String::from("NAMESPACE"), String::from(""))
        .expect("NAMESPACE is not set");
    let resource = inclusterget::env_with_default(String::from("RESOURCE"), String::from(""))
        .expect("RESOURCE is not set");

    let body =
        inclusterget::get(group, kind, namespace, resource).expect("Failure getting resource");
    println!("Response: {}", body);
}

```

The response is a String that can then be parsed.
