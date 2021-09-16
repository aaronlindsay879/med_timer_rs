use paperclip::actix::Apiv2Schema;
use paste::paste;
use serde::Deserialize;
use sqlx::{query::QueryAs, Sqlite};

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

/// Simple alias to make it simpler to pass partial queries around.
type Query<'a, T> = QueryAs<'a, Sqlite, T, sqlx::sqlite::SqliteArguments<'a>>;

/// Generates functions which fetch a response from the database, returning a specific type.
///
/// This _should_ be done with generics instead, but issues with resolving lifetimes made that infeasible.
#[macro_export]
macro_rules! generate_response_functions {
    ($($name:ident<$type:ty>),+) => {
        use futures::StreamExt;
        $(
            /// Generates a vector of results from the given query and database pool.
            /// If results are found, simply return those results.
            /// Otherwise serve empty json.
            async fn $name(query: crate::routes::Query<'_, $type>, limit: usize, db_pool: &sqlx::SqlitePool) -> Vec<$type> {
                query
                    .fetch(db_pool)
                    .filter_map(async move |entry| entry.ok())
                    .take(limit)
                    .collect()
                    .await
            }
        )+
    };
}
