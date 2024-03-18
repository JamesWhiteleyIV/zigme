use redis::{Client, Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};

/// Struct for holding a redis client connection
#[derive(Debug)]
pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    /// Create redis client instance  
    pub fn new(redis_uri: &str) -> Self {
        Self {
            client: Client::open(redis_uri).expect("Failed to open Redis connection"),
        }
    }

    /// Get redis client instance
    fn get_connection(&self) -> RedisResult<Connection> {
        // Get a connection from the client
        self.client.get_connection()
    }

    /// Get a value from redis db
    pub fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: FromRedisValue,
    {
        let mut con = self.get_connection()?;
        let value: Option<T> = con.get(key)?;
        Ok(value)
    }

    /// Set a value from redis db
    pub fn set<T>(&self, key: &str, value: T) -> RedisResult<()>
    where
        T: ToRedisArgs,
    {
        let mut con = self.get_connection()?;
        con.set(key, value)
    }

    /// Return a Vec<T> from redis db for given key
    pub fn get_list<T>(&self, list_key: &str) -> RedisResult<Vec<T>>
    where
        T: FromRedisValue,
    {
        let mut con = self.get_connection()?;
        let items: Vec<T> = con.lrange(list_key, 0, -1)?;
        RedisResult::Ok(items)
    }

    /// Function to push a new item onto the end of the list
    pub fn append_list<T>(&self, list_key: &str, item: T) -> RedisResult<()>
    where
        T: ToRedisArgs,
    {
        let mut con = self.get_connection()?;
        con.rpush(list_key, item)
    }

    /// Function to check the length of the list
    fn get_list_length(&self, list_key: &str) -> RedisResult<u64> {
        let mut con = self.get_connection()?;
        con.llen(list_key)
    }

    /// Function to remove the oldest item from the front of the list if it exceeds the maximum size
    pub fn remove_oldest_item(&self, list_key: &str, max_size: u64) -> RedisResult<()> {
        let mut con = self.get_connection()?;
        let length: u64 = self.get_list_length(list_key)?;

        if length > max_size {
            con.lpop(list_key, None)?;
        }

        RedisResult::Ok(())
    }
}
