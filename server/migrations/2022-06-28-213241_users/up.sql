-- Your SQL goes here
CREATE TABLE "users" (
"id"	INTEGER NOT NULL,
"username"	TEXT NOT NULL UNIQUE,
"hashed_password"	TEXT NOT NULL,
PRIMARY KEY("id" AUTOINCREMENT)
);
