# ⚙️ Nakama Admin CLI

A flexible, lightweight command-line tool for invoking **server-to-server** RPCs on a Nakama server.

---

## 🚀 What It Solves

Nakama’s server-to-server (S2S) RPC endpoints — authenticated via an HTTP key — are powerful tools for managing and testing server-side logic. But they’re **not accessible through Nakama’s web Console**, which makes them hard to use during development or admin operations.

You *can* use `cURL` or `Postman`, but crafting the JSON payloads manually is tedious, error-prone, and often disconnected from your real development workflow. Sure, you could build an online admin panel, but sometimes all you need is a **handy, smart CLI tool** — something you can run directly in your terminal, edit the request, and fire it off. That’s what **Nakama Admin CLI** is built for.

---

## 🌟 Features

- 🔑 Authenticates using Nakama's HTTP key (S2S)
- 🧩 Works with any server-side RPC
- 📝 Opens a JSON template in your `$EDITOR` before sending
- ✅ Includes shortcut commands for common admin RPCs
- 📁 Supports `.env` configuration
- 🧼 Outputs JSON for automation or scripting

