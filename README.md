# rust-anyhow-sample

[このPR](https://github.com/rust-lang/cargo/pull/7776)で、Cargoも`failure`から[anyhow](https://docs.rs/anyhow/1.0.26/anyhow/)に移行したらしい。
まずは`anyhow`を試してみる。

## 使い方

`use anyhow::Result`で使う。

```rust
extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let cm = get_cluster_info().unwrap();
    println!("{:?}", cm);
}
```

### 正常系

利用するjson

```json
{
  "name": "cluster A",
  "group": 1
}
```

実行結果

```sh
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/myanyhow`
ClusterMap { name: "cluster A", group: 1 }
```

### `cluster.json`がない場合

```sh
❯ cargo run
   Compiling myanyhow v0.1.0 (/Users/cipepser/.go/src/github.com/cipepser/rust-anyhow-sample/myanyhow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/myanyhow`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: No such file or directory (os error 2)', src/libcore/result.rs:1165:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

### jsonの型が違う場合

`group`をstringにしてみる。

```json
{
  "name": "cluster A",
  "group": "1"
}
```

```sh
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/myanyhow`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: invalid type: string "1", expected i32 at line 3 column 14', src/libcore/result.rs:1165:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

## context

`main`も`panic`させずに、`err`を表示させるようにした。

```rust
extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")
        .context("failed to read config file")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let _ = match get_cluster_info() {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}
```

### `cluster.json`がない場合

```sh
❯ cargo run
failed to read config file

Caused by:
    No such file or directory (os error 2)
```

## with_context

```rust
extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info(path: &str) -> Result<ClusterMap> {
    let config = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path))?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let _ = match get_cluster_info("cluster.json") {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}
```

### `cluster.json`がない場合

```sh
❯ cargo run
failed to read config file: cluster.json

Caused by:
    No such file or directory (os error 2)
```


## References
- [anyhow \- Rust](https://docs.rs/anyhow/1.0.26/anyhow/)
- [Migrate from the \`failure\` crate to \`anyhow\` by alexcrichton · Pull Request \#7776 · rust\-lang/cargo](https://github.com/rust-lang/cargo/pull/7776)
- [RustのSerdeの簡単な紹介 \- Qiita](https://qiita.com/garkimasera/items/0442ee896403c6b78fb2)