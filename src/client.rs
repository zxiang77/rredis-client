use tokio_codec;
use connection::{ConnectionInfo, Connection, IntoConnectionInfo, connect};
use types::{RedisResult};
#[derive(Debug, Clone)]
pub struct Client {
    connection_info: ConnectionInfo,
}

/// example usage:
///  let client = redis::Client::open("redis://127.0.0.1/").unwrap();
///  let con = client.get_connection().unwrap();

impl Client {
    /// alternatively use IntoConnectionInfo
    pub fn open<T: IntoConnectionInfo>(url: T) -> RedisResult<Client> {
        Ok(Client {
            connection_info: url.into_connection_info()?,
        })
    }

    pub fn get_connection(&self) -> RedisResult<Connection> {
        // with connection info try to reach out to redis
        Ok(connect(&self.connection_info)?)
    }

}
