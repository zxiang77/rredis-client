use std::net::IpAddr;
use std::path::PathBuf;
use crate::types::RedisResult;
use url;
use std::io::BufReader;
use std::net::TcpStream;
use tokio_uds::UnixStream;
use std::collections::hash_map::Values;
use std::time::Duration;
use std::intrinsics::uninit;
use futures::failed;

/// maintain the connection status, Info.

/// Accept either Tcp connection addr as a string for ip addr and u16 for port number
/// Or A Unix addr as a path
pub enum ConnectionAddr {
    Tcp(String, u16),
    Unix(PathBuf),
}

impl ConnectionAddr {
    pub fn is_supported(&self) -> bool {
        match *self { // the value of the self
            ConnectionAddr::Tcp(_, _) => true,
            #[cfg(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets"))]
            ConnectionAddr::Unix(_) => true,
            #[cfg(not(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets")))]
            ConnectionAddr::Unix(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub addr: Box<ConnectionAddr>,

    /// optional db info
    pub db: Option<i64>,
    pub pw: Option<String>,
}

pub trait IntoConnectionInfo {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo>;
}

impl IntoConnectionInfo for ConnectionInfo {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        Ok(self)
    }
}

impl<'a> IntoConnectionInfo for &'a str {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        match parse_redis_url(self) {
            Ok(u) => u.into_connection_info(),
            Err(_) => fail!((ErrorKind::InvalidClientConfig, "Redis URL did not parse")),
        }
    }
}

pub fn parse_redis_url(input: &str) -> Result<url::Url, ()> {
    match url::Url::parse(input) {
        Ok(result) => match result.scheme() {
            "redis" | "redis+unix" | "unix" => Ok(result),
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub enum ActualConnection {
    Tcp(TcpConnection),
    #[cfg(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets"))]
    Unix(UnixConnection),
}

impl ActualConnection {
    pub fn new(addr: &ConnectionAddr) -> RedisResult<ActualConnection> {
        Ok(
            match addr {
                ConnectionAddr::Tcp(ref addr, ref port) => {
                    let host: &str = &*addr; // ???
                    let tcp = TcpStream::connect((host, *port))?;
                    let buffered = BufReader::new(tcp);
                    ActualConnection::Tcp(TcpConnection{reader: buffered, open: true})
                },
                #[cfg(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets"))]
                ConnectionAddr::Unix(pbuf) => ActualConnection::Unix(UnixConnection{
                    open: true,
                    sock: BufReader::new(UnixStream::connect(pbuf)?),
                }),
                #[cfg(not(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets")))]
                ConnectionAddr::Unix(pbuf) => {
                    fail!((
                        "Wrong config" // !ErrorKind
                    ))
                }
            }
        )
    }

    pub fn send_bytes(&mut self, bytes: &[u8]) -> RedisResult<Value> {
        unimplemented!()
    }

    pub fn read_response(&mut self) -> RedisResult<Value> {
        unimplemented!()
    }

    //?? timeout not part of attr
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> RedisResult<bool> {
        unimplemented!()
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> RedisResult<bool> {
        unimplemented!()
    }

    pub fn is_open(&self) -> bool {
        match *self {
            ActualConnection::Tcp(TcpConnection{open, ..}) => open,
            #[cfg(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets"))]
            ActualConnection::Unix(UnixConnection{open, ..}) => open,
        }
    }


}

struct TcpConnection {
    reader: BufReader<TcpStream>,
    open: bool,
}

#[cfg(any(feature = "with-unix-sockets", feature = "with-system-unix-sockets"))]
struct UnixConnection {
    sock: BufferReader<UnixStream>,
    open: bool,
}

pub fn connect(ci: &ConnectionInfo) -> RedisResult<Connection> {
    let con = ActualConnection::new(&ci.addr)?;
    let rv = Connection {

    }; // 0
}

pub struct Connection {

}
