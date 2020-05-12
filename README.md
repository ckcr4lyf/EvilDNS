# EvilDNS

This is a "prank" DNS server that repsonds to every query with `127.0.0.1`

The server currently does **not** handle AAAA queries and such, and has a naive method of replying. It was created for the purpose of illustrating socket programming in Rust, as well as a focus on the DNS protocol and structuring a repsonse. 

Use it at your own risk.