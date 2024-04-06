## Python Wrapper for NetRust

[NetRust](https://github.com/mateoneira/netrust) - is a simple graph library built from the ground-up using Rust.

Benchmarks

**BFS**: on barabasi-alberto graph with n=100,000 and m=3
| Library   | Runtime (ms) |
|-----------|--------------|
| NetRust   | 54.1ms |
| NetworkX  | 351ms |
| igraph    | 15.5ms |

**Dijkstra single source shortest path**: on barabasi-alberto graph with n=100,000 and m=3 and random weights between 0-100
| Library   | Runtime (ms) |
|-----------|--------------|
| NetRust   | 320ms |
| NetworkX  | 1.73s |
| igraph    | 158ms |
