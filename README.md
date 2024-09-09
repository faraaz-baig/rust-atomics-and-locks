# Learning Rust Concurrency and Synchronization

This repository contains my personal explorations and code examples as I delve into advanced concurrency and synchronization concepts in Rust.


## Purpose

The primary goal of this project is to deepen my understanding of Rust's concurrency features, with a focus on:

- Thread parking and unparking
- Condition variables and wait methods
- Mutex and other synchronization primitives
- Advanced asynchronous programming concepts

## File Descriptions

- `lib.rs`: Contains shared utilities and functions used across the project.
- `main.rs`: The entry point of the application, used for running and testing different concurrency scenarios.
- `park_and_unpark_threads.rs`: Explores the concept of parking and unparking threads for efficient waiting.
- `wait_method.rs`: Implements and examines the use of condition variables and wait methods for thread synchronization.

## Key Concepts Explored

1. **Thread Parking**: Understanding how to efficiently pause thread execution when no work is available.
2. **Unparking Threads**: Exploring mechanisms to resume parked threads when new work arrives.
3. **Condition Variables**: Implementing and using condition variables for thread synchronization.
4. **Mutex Guards**: Utilizing mutex guards for safe concurrent access to shared resources.
5. **Wait Methods**: Examining the implementation and usage of wait methods on mutex guards.

## Learning Resources

I'm using the following resources in my learning journey:

1. "Rust Atomics and Locks" by Mara Bos
2. The Rustonomicon
3. Rust Async Book
4. Rust standard library documentation


## Future Goals

- Implement a simple async runtime from scratch
- Explore lock-free data structures
- Add examples demonstrating the use of `Arc` with these synchronization primitives

## Disclaimer

The code in this repository is experimental and part of my learning process. It may contain errors or non-optimal implementations as I work through these concepts.

---

Last updated: 9th Sept 2024
