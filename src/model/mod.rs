pub mod user;
pub use self::user::*;

allow_tables_to_appear_in_same_query!(users,);
