use persist::PersistDb;
use toasty_cli::ToastyCli;




#[tokio::main]
async fn main() {
    let db = PersistDb::create_db(":memory:").await.unwrap();
    ToastyCli::new(db).parse_and_run().await.unwrap();
}