# KeyStoreDB
**A small-time key-value based DB written in Rust.**

Aims to provide extremely fast DBaaS for your projects without the need to setup a 
service like PostgreSQL or MongoDB.
Built on top of [actix-web](https://actix.rs/) bacause of its speed and native multi-threading.
Uses a simple text-based querying system based on SQL, removing the need for a 
complicated client-side API (although some sort of client will be needed).

### Status - Work in Progress
Working on this whenever I can find time since the weird 
syntax of Rust makes me want to use it more.