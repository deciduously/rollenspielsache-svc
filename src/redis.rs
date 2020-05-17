//! Working with Redis datatypes.

use lazy_static::lazy_static;
use r2d2_redis::redis::ToRedisArgs;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env::var;

/*
PUT THIS SOMEWHERE
impl RedisInterface for Roll {
    fn to_redis_hash(&self) -> RedisHash {
        let mut hash = RedisHash::new();
        hash.add_pair("die", &format!("{}", self.die));
        hash.add_pair("repeat", &format!("{}", self.repeat));
        hash.add_pair("modifiers", &format!("{}", self.modifiers)); // TODO this should really become a List
        hash
    }
}
*/

// This isn't really used at all.

pub type Pool = r2d2::Pool<RedisConnectionManager>;

struct Connection {
    port: i16,
    host: String,
}

impl Connection {
    fn new() -> Self {
        let mut ret = Self::default();
        // Check if REDIS_HOST and/or REDIS_PORT are set and use those instead
        if let Ok(hostname) = var("REDIS_HOST") {
            ret.host = hostname;
        }
        if let Ok(portnum) = var("REDIS_PORT") {
            ret.port = portnum.parse::<i16>().unwrap();
        }
        ret
    }

    fn url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }

    fn establish(&self) -> Pool {
        let manager = RedisConnectionManager::new(self.url()).unwrap();
        r2d2::Pool::builder().build(manager).unwrap()
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            port: 6379,
            host: "localhost".into(),
        }
    }
}

lazy_static! {
    pub static ref POOL: Pool = Connection::new().establish();
}

/// These items can use the Redis connection direction
trait RedisType {
    fn add_to_redis<K: ToRedisArgs>(&self, key: K, pool: Pool);
}

/// A single key-value Redis Hash mapping
pub struct RedisHashPair(String, String);

impl From<RedisHashPair> for (String, String) {
    fn from(r: RedisHashPair) -> Self {
        (r.0, r.1)
    }
}

/// A representation of a Rust object's data as RedisHashPairs
#[derive(Default)]
pub struct RedisHash(Vec<RedisHashPair>);

impl RedisHash {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_pair(&mut self, key: &str, value: &str) {
        self.0.push(RedisHashPair(key.into(), value.into()))
    }
}
/*
impl RedisType for RedisHash {
    fn add_to_redis<K: ToRedisArgs>(&self, key: K, pool: Pool) {
        let conn = pool.get().unwrap();
        let pairs = self
            .0
            .iter()
            .map(|&el| <(String, String)>::from(el))
            .collect::<Vec<(String, String)>>();
        conn.hset_multiple(key, &pairs);
    }
}
*/

/// Objects must implement this trait to work with the data store
pub trait RedisInterface {
    fn to_redis_hash(&self) -> RedisHash;
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::ops::DerefMut;

    #[test]
    fn test_connect_to_redis() {
        let mut conn = POOL.get().unwrap();
        let reply = redis::cmd("PING")
            .query::<String>(conn.deref_mut())
            .unwrap();
        assert_eq!("PONG", reply)
    }
}
