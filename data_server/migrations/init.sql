CREATE TABLE IF NOT EXISTS systems (
    id integer PRIMARY KEY,
    name varchar(40) UNIQUE
);

CREATE TABLE IF NOT EXISTS characters (
    id integer PRIMARY KEY,
    name varchar(40),
    system_id integer REFERENCES systems(id)
);

CREATE TABLE IF NOT EXISTS items (
    id integer PRIMARY KEY,
    name varchar(40),
    value integer,
    weight integer,
    system_id integer REFERENCES systems(id),
    details text
);
