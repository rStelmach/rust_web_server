// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Int4,
        name -> Varchar,
        full_degree -> Float8,
        norm_degree -> Float8,
        speed -> Float8,
        is_retro -> Jsonb,
        sign -> Varchar,
        sign_lord -> Varchar,
        nakshatra -> Varchar,
        nakshatra_lord -> Varchar,
        nakshatra_pad -> Int4,
        house -> Int4,
        is_planet_set -> Bool,
        planet_awastha -> Varchar,
        chart_name -> Varchar,
    }
}
