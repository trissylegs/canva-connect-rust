```rust,skt-connect

fn main() {{
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()    
        .build()
        .unwrap()
        .block_on(async {{
          {}
        }})
}}
```
