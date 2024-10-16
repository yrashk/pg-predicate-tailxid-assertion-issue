use std::time::Instant;
use tokio_postgres::{NoTls, Error, GenericClient};
use uuid::Uuid;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use tokio_postgres::IsolationLevel::{Serializable};
use futures::future::join_all;
use tokio::task;


#[tokio::main]
async fn main() {
    let mut cfg = Config::new();
    cfg.url = std::env::var("PG_URL").ok();
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.builder(NoTls).unwrap().max_size(15).build().unwrap();

    const BATCH_SIZE: usize = 1000;

    {
        let client = pool.get().await.unwrap();
        _ = client.execute("create table if not exists tab (i int)", &[]).await.unwrap();
    }

    for batch in (0..100_000).step_by(BATCH_SIZE) {
        let tasks: Vec<_> = (0..10_000)
            .map(|i| {
                let pool = pool.clone();
                task::spawn(async move {
                    let mut client = pool.get().await.unwrap();
                    let stmt = client.prepare_cached("insert into tab default values").await?;
                    let txn = client.build_transaction().isolation_level(Serializable).start().await?;
                    txn.execute(&stmt, &[]).await?;

                    txn.commit().await?;
                    Ok::<(), Error>(())
                })
            })
            .collect();

        // Wait for all tasks to complete
        if join_all(tasks).await.iter().any(Result::is_err) {
            return;
        }
        println!("inserted {}", batch);
    }
}
