
# ⚡ Lightning Nodes API

A **Rust-based** REST API to fetch, store, and expose public metrics from **Lightning Network** nodes. Fast, lightweight, and designed with concurrency and safety in mind.

---

## 🔧 Build Tools & Versions Used

- 🦀 **Rust** — `1.78.0`
- ⚙️ **Actix-web** — `4.x` (Web framework)
- 🗄️ **SQLite** — `3.x` (Database)
- 🔗 **SQLx** — `0.7` (Async ORM)
- 🔥 **Tokio** — `1.x` (Async runtime)
- 🌐 **Reqwest** — `0.12` (HTTP client)
- 🧠 **Serde / Serde JSON** — (Serialization)
- 🕰️ **Chrono** — (Datetime handling)
- 🔐 **dotenv** — (Environment management)
- 📝 **Log + Env Logger** — (Logging)

---

## 🚀 Steps to Run the App

1️⃣ **Clone the repository:**

```bash
git clone https://github.com/yourusername/lightning-nodes-api.git
cd lightning-nodes-api
```

2️⃣ **Set environment variables:**

Create a `.env` file:

```env
DATABASE_URL="sqlite://./lightning_nodes.db"
API_URL="https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"
SERVER_ADDR="0.0.0.0:8080"
RUST_LOG="info"
# Optional: IMPORT_INTERVAL_SECS=600
```

> 🗂️ The SQLite database will be created automatically on the first run.

3️⃣ **Build the project:**

```bash
cargo build --release
```

4️⃣ **Run the API:**

```bash
cargo run
```

5️⃣ **Test it:**

Health check:

```bash
curl http://localhost:8080/health
```

Get node data:

```bash
curl http://localhost:8080/nodes | jq
```

---

## 🎯 What Was the Reason for Your Focus?  
**What Problems Were You Trying to Solve?**

- Practice building a production-grade API in **Rust**, focusing on:
  - ✅ High performance
  - ✅ Safety (memory-safe, thread-safe)
  - ✅ Simplicity and clarity in architecture
- Learn how to integrate async Rust components (Actix-web, SQLx, Reqwest) together in a clean, maintainable way.
- Solve the problem of collecting Lightning Network node data and exposing it in a reliable, easy-to-consume API.

---

## ⏳ How Long Did You Spend on This Project?

Approximately **1 week** (around **32 hours** total), distributed between:

- 🏗️ Project setup and architecture
- 🔌 API implementation
- 🗄️ Database integration
- 🔄 Data fetching logic
- 🧪 Basic testing
- 📜 Writing documentation

---

## ⚖️ Did You Make Any Trade-offs for This Project?  
**What Would You Have Done Differently With More Time?**

- 🚫 Skipped authentication for admin endpoints (focus was on API fundamentals).
- 🚫 No pagination on `/nodes` — if the dataset grows, this would be necessary.
- 🚫 No Dockerfile or deployment pipeline — with more time, containerization and CI/CD would be next steps.
- 🔍 Limited error handling and observability — proper tracing and metrics (Prometheus, OpenTelemetry) would be essential in a production setup.

---

## 🧠 What Do You Think Is the Weakest Part of Your Project?

- ❌ Lack of **authentication** on admin routes (`/admin/import`).
- ❌ **Error handling** is basic; edge cases aren't fully covered.
- ❌ **Testing coverage** is minimal — focused only on simple endpoints like `/health`.
- ❌ **No frontend client** or API documentation (e.g., Swagger/OpenAPI).

---

## ℹ️ Is There Any Other Information You’d Like Us to Know?

- This project was developed as part of my learning journey with Rust, a language I'm currently studying to deepen my understanding of systems programming, performance, and safety.
- Throughout the project, I dedicated significant time to learning Rust's ecosystem, asynchronous programming, memory safety, and web development with Actix-web.
- The code was written with a focus on clarity, modularity, and maintainability, serving as both a functional project and a learning milestone.
- I'm highly motivated to continue improving my skills in backend development, particularly in building robust APIs and working with high-performance systems.
- I'm passionate about building efficient, reliable backend systems — especially APIs interacting with blockchain, financial, or real-time systems.
