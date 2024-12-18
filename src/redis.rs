use crate::scrapy::Novel;
use bb8_redis::{bb8, RedisConnectionManager};
use color_eyre::{eyre::Ok, Result};
use redis::AsyncCommands;
type RedisPool = bb8::Pool<RedisConnectionManager>;

async fn get_redis_conn() -> redis::aio::MultiplexedConnection {
    let client = redis::Client::open(REDIS_URL).unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}

pub async fn save_to_redis(novels: &Vec<Novel>) -> Result<()> {
    let pool = get_pool().await;
    let mut conn = pool.get().await.unwrap();

    for novel in novels {
        let key = format!("novel:{}", &novel.title);
        let novel_json = serde_json::to_string(&novel).unwrap();

        conn.set(&key, novel_json).await?;
    }
    Ok(())
}

async fn get_from_redis(title: &str) -> Option<Novel> {
    let mut pool = get_pool().await;
    let mut conn = pool.get().await.unwrap();
    let key = format!("novel:{}", title);

    let novel_json: Option<String> = conn.get(&key).await.ok();
    novel_json.and_then(|json| serde_json::from_str(&json).ok())
}

async fn batch_save_to_redis(novels: &[Novel]) -> Result<()> {
    let mut conn = get_redis_conn().await;
    let mut pipe = redis::pipe();

    for novel in novels {
        let key = format!("novel:{}", &novel.title);
        let novel_json = serde_json::to_string(&novel).unwrap();
        pipe.set(&key, novel_json);
    }

    pipe.exec_async(&mut conn).await?;
    Ok(())
}
static REDIS_URL: &str = "redis://:000415@192.168.0.49:6379/1";
async fn create_pool() -> RedisPool {
    let manager = RedisConnectionManager::new(REDIS_URL).unwrap();
    bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap()
}
use tokio::sync::OnceCell;

static REDIS_POOL: OnceCell<RedisPool> = OnceCell::const_new();

// 获取连接池的函数
async fn get_pool() -> &'static RedisPool {
    REDIS_POOL
        .get_or_init(|| async { create_pool().await })
        .await
}

async fn test_read_redis() {
    if let Some(novel) = get_from_redis("").await {
        println!("novel: {}", novel.title);
    } else {
        println!("not found");
    }
}
