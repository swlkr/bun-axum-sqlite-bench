import { sqliteTable, text } from "drizzle-orm/sqlite-core";

export const messages = sqliteTable('messages', {
  msg: text("msg").notNull()
});