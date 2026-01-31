CREATE EXTENSION OF NOT EXISTS "uuid-ossp";

CREATE TABLE 
    IF NOT EXISTS tasks (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        crated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
    );