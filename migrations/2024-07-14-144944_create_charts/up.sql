-- Your SQL goes here
CREATE TABLE charts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    full_degree FLOAT8 NOT NULL,
    norm_degree FLOAT8 NOT NULL,
    speed FLOAT8 NOT NULL,
    is_retro JSONB NOT NULL,
    sign VARCHAR NOT NULL,
    sign_lord VARCHAR NOT NULL,
    nakshatra VARCHAR NOT NULL,
    nakshatra_lord VARCHAR NOT NULL,
    nakshatra_pad INTEGER NOT NULL,
    house INTEGER NOT NULL,
    is_planet_set BOOLEAN NOT NULL,
    planet_awastha VARCHAR NOT NULL
);