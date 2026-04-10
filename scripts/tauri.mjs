import { spawn } from "node:child_process";

const args = process.argv.slice(2);
const env = { ...process.env };

if (process.platform === "linux" && args.includes("build") && !env.NO_STRIP) {
  env.NO_STRIP = "true";
}

const child = spawn("tauri", args, {
  stdio: "inherit",
  env,
  shell: process.platform === "win32",
});

child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});

child.on("error", (error) => {
  throw error;
});
