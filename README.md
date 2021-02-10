# Storinator DB
**An experimental key-value database written in Rust.**

![issues](https://img.shields.io/github/issues/zrthxn/storinator)
![commits](https://img.shields.io/github/last-commit/zrthxn/storinator)
![wip](https://img.shields.io/badge/dev-active-blue)

Aims to provide extremely fast DBaaS without the need to setup a service like AWS RDS or Mongo Atlas.
Built on top of [actix-web](https://actix.rs/) bacause of its speed and native multi-threading.
Uses a simple text-based querying system based on SQL, removing the need for a 
complicated client-side API (although some sort of client will be needed).

### Motivation
Whenever I start developing a webapp, I need some or the other key-value storage since
I tend to stay away from SQL. For that you have to either use a service like Firebase or Atlas
or have a self-deployed DB on a VPS. Both of these options introduce complexity and need management.
This project is meant to be a Dockerizable app which you I just throw onto my server 
and use simple plaintext over HTTP to query. It will also hopefully be very fast.
**This is still only an experiment and won't be anywhere as good as actual DBs.**

### Status [Work in Progress]
Working on this whenever I can find time since the weird 
syntax of Rust makes me want to use it more.

**Checklist**
- [x] Tokenizer
- [x] Query Parser
- [x] Sequence Builder
- [x] DB Interactions
- [x] HTTP Server
- [ ] HTTP Routes
- [ ] Reliability Testing
- [ ] Query Benchmarking