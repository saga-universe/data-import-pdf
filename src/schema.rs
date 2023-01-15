// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    countries (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        code -> Char,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sagas (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        author -> Varchar,
        music -> Varchar,
        season -> Unsigned<Smallint>,
        creation_date -> Varchar,
        countryID -> Unsigned<Integer>,
        statusID -> Unsigned<Integer>,
        categoryID -> Unsigned<Integer>,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sagas_subcategories (id) {
        id -> Unsigned<Integer>,
        sagaID -> Unsigned<Integer>,
        subcategoryID -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    status (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subcategories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(sagas -> categories (categoryID));
diesel::joinable!(sagas -> countries (countryID));
diesel::joinable!(sagas -> status (statusID));
diesel::joinable!(sagas_subcategories -> sagas (sagaID));
diesel::joinable!(sagas_subcategories -> subcategories (subcategoryID));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    countries,
    sagas,
    sagas_subcategories,
    status,
    subcategories,
);
