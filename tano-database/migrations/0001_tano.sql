CREATE TABLE IF NOT EXISTS "providers" ("id" TEXT, PRIMARY KEY ("id"));

CREATE TABLE IF NOT EXISTS "songs" (
  "id" INTEGER,
  "title" TEXT NOT NULL,
  "provider_id" TEXT NOT NULL,
  "path" TEXT NOT NULL UNIQUE,
  PRIMARY KEY ("id" AUTOINCREMENT),
  FOREIGN KEY ("provider_id") REFERENCES "providers" ("id")
);
