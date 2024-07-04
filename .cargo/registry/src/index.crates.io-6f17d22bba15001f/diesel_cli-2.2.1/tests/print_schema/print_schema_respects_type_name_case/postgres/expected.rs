// @generated automatically by Diesel CLI.

/// A module containing custom SQL type definitions
///
/// (Automatically generated by Diesel.)
pub mod sql_types {
    /// The `MyType` SQL type
    ///
    /// (Automatically generated by Diesel.)
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "MyType"))]
    pub struct MyType;

    /// The `MyType2` SQL type
    ///
    /// (Automatically generated by Diesel.)
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "MyType2"))]
    pub struct MyType2;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MyType;
    use super::sql_types::MyType2;

    /// Representation of the `custom_types` table.
    ///
    /// (Automatically generated by Diesel.)
    custom_types (id) {
        /// The `id` column of the `custom_types` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `custom_enum` column of the `custom_types` table.
        ///
        /// Its SQL type is `MyType`.
        ///
        /// (Automatically generated by Diesel.)
        custom_enum -> MyType,
        /// The `custom_enum_nullable` column of the `custom_types` table.
        ///
        /// Its SQL type is `Nullable<MyType>`.
        ///
        /// (Automatically generated by Diesel.)
        custom_enum_nullable -> Nullable<MyType>,
        /// The `custom_enum2` column of the `custom_types` table.
        ///
        /// Its SQL type is `MyType2`.
        ///
        /// (Automatically generated by Diesel.)
        custom_enum2 -> MyType2,
    }
}
