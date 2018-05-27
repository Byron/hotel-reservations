## Goal

Solve the hotel reservations problem the way I would like it, in Rust.
Try to be as idiomatic as possible, and consider the answer the main user
value to provide. Thus that and only that MUST be tested, anything else 
is 'extra', like actually testing for a few things the user can run into.

## Getting the answers

Run `make answers` if your `rust` installation is at least at v1.26.
If you have no `rust` but `docker`, run `make answers-in-docker`.

In any case, you can run all `make` targets using docker via `make interactive-developer-environment-in-docker`.
Please be warned that initial compilation takes a while.

## Features

* [x] shows correct answers
* [x] fully streaming with minimal state
* [x] strict overflow checking for all computations
* [x] support for profiling
* [x] support for benchmarking
* [x] support for linting
* [x] interactive developer environment in docker

## Benchmark Results

The Rust implementation comes in at 216 lines, with a binary sized at 780kb. It runs the benchmark
in 54ms.
