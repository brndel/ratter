CREATE TABLE "sessions" (
    "token" BLOB NOT NULL,
    "user_id" BLOB,
    "created_at" TEXT NOT NULL,
    "valid_until" TEXT NOT NULL,
    PRIMARY KEY ("token")
);
-- #[toasty::breakpoint]
CREATE TABLE "users" (
    "id" BLOB NOT NULL,
    "name" TEXT NOT NULL,
    "role" TEXT NOT NULL CHECK ("role" IN ('admin', 'user')),
    "salt" BLOB NOT NULL,
    "password" BLOB NOT NULL,
    "created_at" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE UNIQUE INDEX "index_users_by_name" ON "users" ("name");
