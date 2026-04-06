-- Your SQL goes here
CREATE TABLE "users"(
	"users_id" UUID PRIMARY KEY,
	"name" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"is_admin" BOOL NOT NULL,
	"is_deleted" BOOL NOT NULL
);

