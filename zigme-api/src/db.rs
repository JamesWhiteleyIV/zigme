use redis::{Client, Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};

/// Struct for holding a redis client connection
pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    /// Create redis client instance  
    pub async fn new(redis_uri: &str) -> Self {
        Self {
            client: Client::open(redis_uri).expect("Failed to open Redis connection"),
        }
    }

    // Get redis client instance
    async fn get_connection(&self) -> RedisResult<Connection> {
        // Get a connection from the client
        self.client.get_connection()
    }

    /// Get a value from redis db
    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: FromRedisValue,
    {
        let mut con = self.get_connection().await?;
        let value: Option<T> = con.get(key)?;
        Ok(value)
    }

    /// Set a value from redis db
    pub async fn set<T>(&self, key: &str, value: T) -> RedisResult<()>
    where
        T: ToRedisArgs,
    {
        let mut con = self.get_connection().await?;
        con.set(key, value)
    }
}
