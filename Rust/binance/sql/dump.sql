CREATE TABLE pair (
    id serial PRIMARY KEY,
    symbol text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TABLE rate (
    id serial PRIMARY KEY,
    pair_id int REFERENCES pair(id),
    rate real NOT NULL,
    created_at timestamp WITH TIME ZONE DEFAULT now() NOT NULL
);
