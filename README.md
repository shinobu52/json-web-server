## 実践Rust入門

11章の『Webアプリケーション, データベース接続』の写経 <https://github.com/ghmagazine/rustbook>

### 補足

- actix-web
  - 書籍が2019年のもので、その3年間で仕様が大きく変わっているため、調べながら対応
  - ログはactix-webが提供するLogger Middlewareを使用
  - **Multipartに関しては調査中のため未実装**

- Clap
  - かなりよくできているCLI開発用クレート
Arg(Builder)とParser(derive)の２種類の実装方法がとれるが、
今回は書籍とは別のParser方式で実装

- Diesel
  - 大きく変わっていないし、実装しやすかったが、cliをM1 Macに入れるのに少し苦労した

``` shell:Postgres
brew install postgresql
brew install libpq
cargo install diesel_cli --no-default-features --features postgres
```

``` shell:MySQL
brew install mysql-client
cargo install diesel_cli --no-default-features --features mysql
```

<https://stackoverflow.com/questions/70383711/problem-trying-to-install-diesel-mac-air-m1>

- serde
  - deriveを使うように実装

- reqwest
  - 非同期対応しているため、`Client::new()`ではなく、`blocking::Client::new()`で同期のインスタンスを生成する必要がある
