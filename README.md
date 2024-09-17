# Zero Knowledge Proofs in Rust

## Udemy Course
This repo documents my progress, notes and exercises to learn the concepts of "Zero Knowledge Proof" following this [Udemy course](https://www.udemy.com/course/zero-knowledge-proofs-in-rust/?couponCode=ST11MT91624A) by Guido Giuntoli.

## Content

This course has the following chapters and su-sections
1. Course Description
    * Course Description
    * Introduction to Modular Arithmetics
    * Quiz 1: Modular Arithmetics
    * Groups
    * Generators
    * Discrete Logarithm Problem
    * Chaum-Pedersen ZKP Protocol
    * Quiz: Chaum-Pedersen ZKP Protocol
    * Toy Example
    * Assignment: Importance of Good Random Number Generators
2. ZKP Protocol in Rust
    * Install Rust
    * Design Schema
    * Coding Exponentiate, Solve & Verify
    * Unit Test: Toy Example
    * Random Number Generator (RNG)
    * Refactoring: Add a ZKP struct
    * 1024-bit Unit Test
3. Building a gRPC server
    * gRPC Server Design
    * Writting and Compiling a Protobuf File with Tonic
    * Client/Server gRPC Protocol
    * Creating Server & Client Executables
    * Run the Tonic Server
    * Process Register Requests
    * Process Challenge Request
    * Process Solution Request
    * Build the Client: Create Register Request
    * Build the Client: Create Authentication Requests
    * Possible Code Improvements as Homework
4. Dockerizing the Application
    * Introduction to Docker
    * Writting a Dockerfile and `docker-compose.yaml`
    * Running the server and client in the Docker container