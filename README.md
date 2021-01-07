# KeyStoreDB
A small-time key-value based DB written in Rust. 
Aims to provide extremely fast DBaaS for your projects without the need to setup a 
service like PostgreSQL or MongoDB.

Built on top of actix-web bacause of its speed and native multi-threading.
Uses a simple text-based querying system based on SQL, removing the need for a 
client-side API.