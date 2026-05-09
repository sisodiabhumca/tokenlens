// Apply cloud/db/schema.sql to $DATABASE_URL. Idempotent.
import { readFile } from "node:fs/promises";
import { Client } from "pg";

const url = process.env.DATABASE_URL;
if (!url) {
  console.error("DATABASE_URL not set");
  process.exit(1);
}
const sql = await readFile(new URL("../db/schema.sql", import.meta.url), "utf8");
const client = new Client({
  connectionString: url,
  ssl: process.env.DATABASE_SSL === "false" ? false : { rejectUnauthorized: false },
});
await client.connect();
await client.query(sql);
await client.end();
console.log("Schema applied.");
