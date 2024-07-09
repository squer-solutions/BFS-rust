-- Your SQL goes here
ALTER TABLE "posts"
    ADD COLUMN "author_id" UUID NOT NULL;

ALTER TABLE "posts"
    ADD CONSTRAINT "posts_author_id_fkey" FOREIGN KEY ("author_id") REFERENCES "users" ("id") ON DELETE CASCADE;

