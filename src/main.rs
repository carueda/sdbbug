use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db("test").await?;
    db.query("\
DEFINE TABLE A;
DEFINE TABLE B;
DEFINE TABLE R TYPE RELATION FROM A TO B;")
        .await
        .expect("schema creation should work");

    Ok(())
}
