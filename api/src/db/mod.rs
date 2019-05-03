use actix_web::dev::Payload;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{Error, FromRequest, HttpRequest};
pub use diesel::connection::Connection as DieselConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::ops::Deref;

pub mod models;
pub mod schema;

/// The raw database connection type. Aliased so it's easy to switch when using, eg, an sqlite backend.
pub type DbConnection = PgConnection;

/// Convenient type alias for the postgres database pool so we don't have to type this out.
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

/// Type alias for the pooled connection.
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

/// Initializes a new connection pool for the database at `url`.
pub fn init_pool<S>(url: S) -> Result<Pool, r2d2::PoolError>
where
    S: Into<String>,
{
    let manager = ConnectionManager::<DbConnection>::new(url);

    r2d2::Pool::builder().build(manager)
}

/// Request guard type for handing out db connections from the pool.
pub struct Connection(pub PooledConnection);

/// Custom guard implementation so routes can grab a database connection easily.
///
/// Attempts to retrieve a single connection from the database pool;
/// if no pool is online, fails with `InternalServerError`.
/// If no connections are available, fails with `ServiceUnavailable`.
impl FromRequest for Connection {
    type Config = ();
    type Error = Error;
    type Future = Result<Self, Error>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // retrieve the database connection from Rocket's managed data
        let pool = req.app_data::<Pool>().unwrap();

        match pool.get() {
            // .get() a connection from the pool
            Ok(conn) => Ok(Connection(conn)),
            Err(_) => {
                Err(InternalError::new(&"no db connection", StatusCode::SERVICE_UNAVAILABLE).into())
            }
        }
    }
}

/// A convenient way to use a `&db::Connection` as a `&DbConnection`.
///
/// Just allows deref-ing the inner `PooledConnection`.
impl Deref for Connection {
    type Target = DbConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Connection> for Connection {
    fn as_ref(&self) -> &Connection {
        &self
    }
}
