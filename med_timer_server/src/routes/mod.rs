use paperclip::actix::Apiv2Schema;
use paste::paste;
use serde::Deserialize;

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
            .filter_map(async move |entry| entry.ok())
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
