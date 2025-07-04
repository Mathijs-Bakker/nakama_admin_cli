# ⚙️ Nakama Admin CLI

A flexible, lightweight command-line tool written in **Rust** for invoking **server-to-server (S2S)** admin-only RPCs on a [Nakama](https://heroiclabs.com/nakama/) server.

Use it to:
- Toggle maintenance mode
- Set the required client version
- Call any of your own custom admin RPCs

It opens a pre-filled **JSON template in your editor** so you can easily modify and send it.

---

## 🌟 Features

- 🔑 Authenticates using Nakama's HTTP key (S2S)
- 🧩 Works with **any server-side RPC**
- 📝 Opens a JSON template in your `$EDITOR` for editing before sending
- ✅ Includes shortcut commands for common admin RPCs
- 📁 Supports `.env` configuration
- 🧼 JSON output for automation or scripting

---

## 📋 Requirements

- [Rust](https://rust-lang.org/tools/install) (1.70+ recommended)
- A running Nakama server (v3.x+)
- A valid Nakama **HTTP key** for S2S auth
- Your own custom server-side admin RPCs registered on the Nakama backend

---

## 🛡️ Server-to-Server (S2S) RPCs

This CLI is meant for **server-to-server** use only. It authenticates using the HTTP key (`Authorization: Bearer <key>`) and never logs in as a user.

These admin RPCs should be protected like so (TypeScript example):

```ts
    function rpcAdminSetMaintenanceMode(ctx: nkruntime.Context, logger: nkruntime.Logger): string {
        if (ctx.userId) {
            logger.warn("Attempted access from user session.");
            throw new Error("This RPC is admin-only and must be called server-to-server.");
        }

        // Continue with logic...
    }
```
Your RPCs should **reject calls from clients or the Nakama Console**, which typically use a user session.

---

## 🔧 Installation

### 📦 Build from source
```bash
    git clone https://github.com/Mathijs-Bakker/nakama_admin_cli.git
    cd nakama_admin_cli
    cargo build --release
```
Install globally:
```bash
    cp target/release/nakama_admin_cli /usr/local/bin/
```
---

## ⚙️ Environment Configuration

You can store your Nakama config in a `.env` file:
```dotenv
    NAKAMA_URL=http://localhost:7350
    NAKAMA_HTTP_KEY=defaulthttpkey
```
Copy the example:
```bash
    cp .env.example .env
```
Now you can run the CLI without flags:
```bash
    nakama_admin_cli set-maintenance
```
You can override the values at any time:
```bash
    nakama_admin_cli --server http://localhost:7350 --key "overridekey" set-client-version
```
---

## 🚀 Usage
```bash
    nakama_admin_cli <command> [options]
```
When you run a command, it:
1. Loads a JSON template into your default system editor (`$EDITOR`)
2. You edit and save the file
3. The payload is POSTed to your Nakama RPC

---

## 📦 Available Commands

### 🔧 Set Maintenance Mode
```bash
    nakama_admin_cli set-maintenance
```
Opens:
```json
    {
        "enabled": "false",
        "message": "🚧 Maintenance Mode Activated! Hang tight—we’ll be back online shortly!",
        "reason": "Bugfix"
    }
```
### 📱 Set Required Client Version
```bash
    nakama_admin_cli set-client-version
```
Opens:
```json
    {
      "version": "0.0.0"
    }
```
### 🎯 Call Any Admin RPC
```bash
    nakama_admin_cli call Admin_MyTool
```
Opens:
```json
    {
      "your": "payload"
    }
```
You can also pipe JSON via `stdin`:
```bash
    echo '{"foo": "bar"}' | nakama_admin_cli call Admin_CustomRPC
```
---

## 📄 Output

Success:
```json
    {
      "message": "Maintenance mode enabled"
    }
```
Errors:
```bash
    [ERROR] Invalid JSON payload.
```
---

## 🧪 Example Admin RPCs

In your Nakama backend (TypeScript or Go), register RPCs like:
```typescript
    initializer.registerRpc('admin_set-maintenance-mode', rpcAdminSetMaintenanceMode);
    initializer.registerRpc('admin_set-required-client-version', rpcAdminSetRequiredClientVersion);
```
These are **example RPCs** — you can create and call any admin-only endpoint you need.

---

## 🤝 Contributing

PRs are welcome!

1. Fork the repo  
2. Create a branch: `git checkout -b feature/my-feature`  
3. Commit and push  
4. Open a pull request

---

## 📃 License

MIT License — see [LICENSE](https://github.com/Mathijs-Bakker/nakama_admin_cli/blob/master/LICENSE).

---

## 💬 Feedback or Questions?

Open an [issue](https://github.com/Mathijs-Bakker/nakama_admin_cli/issues).
