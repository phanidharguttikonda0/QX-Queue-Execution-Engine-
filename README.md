# 🦀 RustQ — Lightweight Message Queue + Lambda Executor

RustQ is a lightweight, self-hosted **message queue** built in **Rust** with async execution support.  
It lets you create multiple queues, enqueue messages dynamically, and automatically trigger
**lambda-like async functions** whenever new messages arrive.

Think of it as a **minimal AWS SQS + Lambda**, built purely in Rust for developers who want full control,
performance, and zero cloud costs.

---

## 🚀 Features

- ✅ Multiple dynamic queues
- ✅ Background workers for each queue
- ✅ Async message execution
- ✅ Retry mechanism for failed jobs
- ✅ Configurable retry limits & wait durations
- ✅ Simple to host on any EC2, VPS, or bare-metal system

---

## 🧠 Architecture Overview

Each queue runs in its **own async worker loop**.  
Messages are added dynamically and processed concurrently.

### Workers continuously:
1. Fetch messages from queues
2. Execute a user-defined async lambda
3. Retry failed messages up to N times
4. Sleep when idle