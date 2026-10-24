import fs from "node:fs";
const open = fs.readFileSync("../openapi/purpose-lattice.yaml", "utf8");
for (const p of [
  "/covenants",
  "/covenants/{id}/evaluate",
  "/covenants/{id}/audit",
])
  if (!open.includes(p)) throw new Error("missing " + p);
console.log("contract drift: pass");
