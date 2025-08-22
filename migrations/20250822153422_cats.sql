CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE eye_color AS ENUM ('Blue', 'Brown');

CREATE TABLE cats (
    name TEXT NOT NULL,
    cool_cat_club_id UUID NOT NULL,
    age SMALLINT NOT NULL,
    eye_color eye_color NOT NULL
);

