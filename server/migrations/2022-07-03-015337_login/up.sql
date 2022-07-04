-- Your SQL goes here
CREATE TABLE "logins" (
"id"	INTEGER NOT NULL,
"username"	TEXT NOT NULL UNIQUE,
"token"	TEXT NOT NULL,
PRIMARY KEY("id" AUTOINCREMENT)
);
