-- This file should undo anything in `up.sql`
ALTER TABLE "posts" DROP CONSTRAINT "posts_author_id_fkey";

ALTER TABLE "posts" DROP COLUMN "author_id";


