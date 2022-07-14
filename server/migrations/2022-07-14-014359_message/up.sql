-- Your SQL goes here
CREATE TABLE "messages" (
	"id"	INTEGER NOT NULL,
	"from"	INTEGER NOT NULL,
	"to"	INTEGER NOT NULL,
	"message"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("from") REFERENCES "user"("id"),
	FOREIGN KEY("to") REFERENCES "user"("id")
);
