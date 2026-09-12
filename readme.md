# rust와 sqlite를 사용해서 만드는 RESTful server

기본 코드는 
https://www.rustfinity.com/blog/create-high-performance-rest-api-with-rust
에서 가져옴.

1. docker가 필요없도록 PostgreSQL 를 sqlite로 바꿈

2. 코드에 오타들을 수정함. (링크의 코드에는 오타가 있음)

3. axum path extract syntax가 예전 버전? 이라 최신 버전으로 변경함.

4. .env 파일에 DATABASE_URL=sqlite://data.db 추가


## 프로젝트 설정을 위한 명령어들.

    cargo new rust-axum-rest-api
    cd rust-axum-rest-api
    cargo add sqlx --features runtime-tokio,tls-native-tls,sqlite
    cargo install sqlx-cli --no-default-features --features native-tls,sqlite
    sqlx database create
    sqlx migrate add create_users_table
    <!-- sqlx migrate add create_posts_table -->
    sqlx migrate add create_documents_table
    sqlx migrate run
    cargo add tokio -F full
    cargo add dotenvy
    cargo add axum serde tracing tracing_subscriber --features serde/derive
    cargo add serde_json