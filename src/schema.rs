// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Int4,
        chart_name -> Varchar,
        chart_data -> Jsonb,
        day -> Int4,
        month -> Int4,
        year -> Int4,
        hour -> Int4,
        min -> Int4,
        lat -> Float8,
        lon -> Float8,
        tzone -> Float8,
    }
}
