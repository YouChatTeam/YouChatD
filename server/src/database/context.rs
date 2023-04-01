use redis::{Client as RedisClient, Commands};
use cassandra_cpp::{Cluster, Session};

#[derive(Clone)]
pub struct Context {
    redis: RedisClient,
    cassandra: Session,
}

impl Context {
    pub fn new(redis_url: &str, cassandra_url: &str) -> Self {
        // not thread safe so we will only have the client here
        let redis_client = RedisClient::open(redis_url).unwrap();

        // thread safe so we can open the connection and maintain it
        let mut cassandra_cluster = Cluster::default();
        cassandra_cluster.set_contact_points(cassandra_url).unwrap();
        let cassandra_session = cassandra_cluster.connect().unwrap();

        // store the two database in the context
        Context {
            redis: redis_client,
            cassandra: cassandra_session,
        }
    }

    // create a new connection to redis using the client
    pub fn get_redis_connection(&self) -> redis::Connection {
        self.redis.get_connection().unwrap()
    }

    pub fn get_cassandra_session(&self) -> &Session {
        &self.cassandra
    }
}