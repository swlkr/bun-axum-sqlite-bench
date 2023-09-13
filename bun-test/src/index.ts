import Database from "bun:sqlite";
import { Elysia } from "elysia";
import { drizzle } from 'drizzle-orm/bun-sqlite';
import { migrate } from "drizzle-orm/bun-sqlite/migrator";
import { messages } from "./schema";
import { sql } from "drizzle-orm";

const client = new Database("db.sqlite3");
const db = drizzle(client)
let _ = await migrate(db, { migrationsFolder: "drizzle" });
 
const rows = await db.select({total: sql`count(*)`}).from(messages);
if(rows[0].total === 0) {
  for (var i = 0; i < 1000; i++) {
    await db.insert(messages).values({msg: i.toString()});
  }
}

const query = db.select({ msg: messages.msg }).from(messages).limit(100).prepare();

async function root() {
  let rows = await query.execute();
  return rows;
}

const router = new Elysia().get("/", root);
const app = router.listen(9001);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
