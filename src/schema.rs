// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Int4,
        chart_name -> Varchar,
        chart_data -> Jsonb,
    }
}
