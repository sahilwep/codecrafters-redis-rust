# CodeCrafters Redis Clone in Rust

This repository contains my learning project, developed during my journey of learning Rust. It is a simplified clone of Redis, a popular in-memory data structure store.

## Overview

The project implements a basic Redis server that can handle a subset of Redis commands like `PING`, `ECHO`, `SET`, and `GET`. It listens for incoming TCP connections and processes commands in a format similar to the Redis protocol.

## Features

- **PING**: Responds with `PONG`.
- **ECHO**: Echoes back the provided message.
- **SET**: Stores a key-value pair.
- **GET**: Retrieves the value associated with a given key.


## Running redis:

1. Ensure you have `cargo` installed locally.
2. Run `./spawn_redis_server.sh` to run your Redis server, which is implemented
   in `src/main.rs`. This command compiles your Rust project, so it might be
   slow the first time you run it. Subsequent runs will be fast.
3. use `redis-cli` tool to intact.
