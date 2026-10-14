CREATE TABLE handbooks (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source     TEXT NOT NULL,        -- fannie | freddie | fha | va | usda
    product    TEXT,                 -- 30yr_fixed, 15yr_fixed, 7_1_arm, ...
    screen     TEXT,                 -- income, assets, property, terms, ...
    title      TEXT NOT NULL,
    content    TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON handbooks(source);
CREATE INDEX ON handbooks(product);
CREATE INDEX ON handbooks(screen);