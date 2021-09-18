use paperclip::actix::Apiv2Schema;
use paste::paste;
use serde::Deserialize;
use sqlx::Error;

pub(crate) mod entries;
pub(crate) mod meds;

/// Generates a default query type, with everything being optional, and accessor methods.
macro_rules! generate_default_query {
    (struct $query:ident {
        $(
            $name:ident: Option<$type:ty> => $default:expr
        ),*
    }) => {
        /// Simple query type that should work for most parameters.
        #[derive(Apiv2Schema, Deserialize)]
        struct $query {
            $(
                $name: Option<$type>
            ),+
        }

        paste! {
            impl $query {
                $(
                    #[doc = "Returns the stored value, or `default` if nothing stored"]
                    pub fn [<$name _or>](&self, default: $type) -> $type {
                        self.$name.unwrap_or(default)
                    }

                    #[doc = "Returns the stored value, or " $default " if nothing stored"]
                    pub fn [<$name _or_default>](&self) -> $type {
                        self.[<$name _or>]($default)
                    }
                )*
            }
        }
    };
}

generate_default_query! {
    struct DefaultQuery {
        count: Option<usize> => 100
    }
}

/// Simple function to log the correct response if a database query fails.
#[inline(always)]
fn log_error(err: Error) {
    match err {
        Error::Database(err) => {
            log::error!("Error returned from database: {}", err);
        }
        Error::Io(err) => {
            log::error!("IO error while communicating with database: {}", err);
        }
        Error::Tls(err) => {
            log::error!(
                "Unable to establish TLS connection while communicating with database: {}",
                err
            );
        }
        Error::Protocol(err) => {
            log::error!("Protocl error while communicating with database: {}", err);
        }
        Error::RowNotFound => {
            log::trace!("No rows returned from database query");
        }
        Error::TypeNotFound { type_name } => {
            log::error!("Unable to find type for database query: {}", type_name);
        }
        Error::ColumnIndexOutOfBounds { index, len } => {
            log::error!(
                "Column index out of bounds while performing database query: index {}; len {}",
                index,
                len
            );
        }
        Error::ColumnNotFound(column) => {
            log::error!("Column not found in database: {}", column);
        }
        Error::ColumnDecode { index, source } => {
            log::error!(
                "Unable to decode column in database: {}; source {}",
                index,
                source
            );
        }
        Error::Decode(err) => {
            log::error!("Unable to decode value from database: {}", err);
        }
        Error::PoolTimedOut => {
            log::warn!("No database pool connection available");
        }
        Error::PoolClosed => {
            log::warn!("Database pool closed while trying to acquire connection");
        }
        Error::WorkerCrashed => {
            log::error!("Background worker for database pool crashed");
        }
        err => {
            log::error!("Unknown error while performing database query: {}", err);
        }
    };
}

fn handle_entry<T>(entry: Result<T, Error>) -> Option<T> {
    match entry {
        Ok(entry) => Some(entry),
        Err(Error::Configuration(err)) => {
            panic!("FATAL: Issue with DB configuration: {}", err)
        }
        Err(err) => {
            log_error(err);
            None
        }
    }
}

/// Generates a database query using the given pool, query information (such as results limit), out type and the query itself.
#[macro_export]
macro_rules! query {
    (
        pool: $pool:expr,
        queries: $url_queries:expr,
        out: Vec<$out_struct:path>,
        $query:expr $(,$args:expr)*
    ) => {
        Json(sqlx::query_as::<_, $out_struct>($query)
            $(.bind($args))*
            .fetch($pool.as_ref())
            .filter_map(async move |entry| crate::routes::handle_entry(entry))
            .take($url_queries.count_or_default())
            .collect()
            .await)
    };
    (
        pool: $pool:expr,
        out: Option<$out_struct:path>,
        $query:expr $(,$args:expr)*
    ) => {
        Json(sqlx::query_as::<_, $out_struct>($query)
            $(.bind($args))*
            .fetch_one($pool.as_ref())
            .await
            .ok())
    };
}
