# Time Capsule

This project is a learning repository for exploring the [Axum](https://github.com/tokio-rs/axum) web framework in Rust. Axum is a powerful and ergonomic framework built on top of [Tokio](https://tokio.rs/) and [Hyper](https://hyper.rs/), designed for building fast and reliable web applications.

## Available Endpoints

The following endpoints are available in this API:

- `GET /health` - returns server health.
- `GET /capsule` - Returns the current server time.
- `POST /capsule` - Accepts a JSON payload to store a time capsule entry.
- `GET /capsule/:public_id` - Retrieves a specific time capsule entry by its public_id.

## Features

- Hands-on examples to understand Axum's core concepts.
- Demonstrates routing, middleware, error handling, and more.
- Focused on learning and experimentation.

## Prerequisites

- Rust installed (use [rustup](https://rustup.rs/)).
- Basic understanding of Rust programming.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/ap211unitech/time-capsule.git
   cd time-capsule
   ```

2. Run the examples:

   ```bash
   cargo run
   ```

3. Create `.env` file:

   ```
   SERVER_ADDRESS=127.0.0.1:8000
   MONGODB_URL=mongodb.....
   ```

4. Open your browser and navigate to the specified URL (e.g., `127.0.0.1:8000`).

## Learning Goals

- Understand Axum's routing system.
- Learn how to use extractors and middleware.
- Explore integration with databases and async tasks.
