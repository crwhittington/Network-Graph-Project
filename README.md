# Network Graph Analysis with Kosaraju’s Algorithm

This project applies **graph theory** and **Rust programming** to analyze the structure of large directed networks. Using the **Kosaraju algorithm**, it identifies **Strongly Connected Components (SCCs)** in two real-world datasets:

- **Stanford.edu webpages** (nodes = pages, edges = hyperlinks)  
- **Google webpages** (nodes = pages, edges = hyperlinks)  

---

## Project Overview

Built a **Rust-based pipeline** with modular design:

- `graph`: Constructs directed graphs from input files  
- `kosaraju`: Implements Kosaraju’s algorithm to detect SCCs  
- `components`: Provides analytical tools (`find_significant_components`, `find_large_components`) to evaluate SCCs  
- `main`: Applies all modules to Stanford and Google datasets  

**Network Connectivity Results**:
- **Stanford graph**: 281,904 nodes, 21,411 SCCs, 2 significant SCCs (largest = 77.35% of nodes)  
- **Google graph**: 916,428 nodes, 203,180 SCCs, 1 significant SCC (largest = 65.53% of nodes)  

Stanford webpages are more densely interconnected, while Google pages are more fragmented.

---

## Key Findings

- **Stanford.edu** network is highly cohesive: nearly 80% of webpages are mutually reachable.  
- **Google** network, while larger, has more isolated nodes and fewer large SCCs.  
- Indicates thematic cohesion at Stanford vs. content diversity across Google.  

---

## Testing

Implemented **six Rust unit tests** to validate:

- Graph construction (e.g., handling empty files)  
- Kosaraju algorithm correctness  
- Component analysis functions  

Five tests pass normally; one test intentionally panics (to verify error handling).  

Run tests with:
```bash
cargo test --test test

