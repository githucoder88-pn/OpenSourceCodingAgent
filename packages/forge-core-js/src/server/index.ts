import { ForgeServer } from "./forge-server.js";

const port = parseInt(process.env.FORGE_PORT || "3000", 10);
const host = process.env.FORGE_HOST || "0.0.0.0";

const server = new ForgeServer();
server.start(port, host).catch(console.error);

process.on("SIGINT", () => {
  console.log("Shutting down...");
  server.stop();
  process.exit(0);
});
