# Degrees - Rust Implementation

This repository showcases a Rust implementation of the "Degrees of Separation" problem, inspired by CS50's Introduction to Artificial Intelligence course. This program efficiently determines the shortest path (measured in movie collaborations) between two actors within a given movie database (csv file).

## Performance Comparison: Rust vs Python

A key advantage of this implementation is its superior performance compared to an equivalent Python solution. Benchmarks reveal significant speedups achieved by the Rust version:

| Degrees | Python (s) | Rust (s) |
|---|---|---|
| 0 | 8.41 | 1.30 |
| 1 | 8.65 | 1.30 |
| 2 | 8.63 | 1.40 |
| 3 | 66.47 | 1.37 |
| 4 | 2845.22 | 3.39 |
| 5 | 4915.30 | 12.81 |

**Python takes 6.5x to 383.7x more time than Rust!** 

![Visualized Performance using Area chart](../images/TimeChart.png)

These results clearly demonstrate Rust's ability to deliver substantial performance gains, especially as problem complexity increases.

## Project Structure

The project is organized into three main modules:

*   **main.rs:** Handles argument parsing, data loading, user interaction, path finding, and output – the core program logic.
*   **graph.rs:** Defines data structures for actors and movies, and implements the graph algorithms for finding the shortest path between actors.
*   **utils.rs:** Provides utility functions for loading data from CSV files and handling user input.

## Running the Program

1. **Clone the Repository:**
```bash
git clone https://github.com/BhuvanSA/CS50AI-rs.git
cd CS50AI-rs/Week\ 0/degrees
```


2. **Build and Run:**
```bash
cargo run [directory]
```

The optional `directory` argument specifies the directory containing `people.csv`, `movies.csv`, and `stars.csv`. If omitted, it defaults to the "small" directory within the project.

**Example Usage:**
```
cargo run
Enter source actor name: Emma Watson
Enter target actor name: Jennifer Lawrence
3 degrees of separation.
1: Emma Watson and Matthew Broderick starred in The Tale of Despereaux
2: Matthew Broderick and Michelle Pfeiffer starred in Ladyhawke
3: Michelle Pfeiffer and Jennifer Lawrence starred in Mother!
```

## Implementation Details

*   **Data Structures:** The program uses `Actor` and `Movie` structs to represent actors and movies. Each actor stores a set of movies they've starred in, and each movie stores a set of actors who starred in it. 
*   **Graph Representation:** Relationships between actors and movies are represented using a graph where actors are nodes and movies are edges connecting them.
*   **Shortest Path Algorithm:** A breadth-first search (BFS) algorithm is implemented to efficiently find the shortest path between two actors within the graph.

## Potential Improvements

*   **Error Handling:** Implement more robust error handling, particularly for file operations and parsing, to improve program stability.
*   **Performance Optimizations:** Further optimize the BFS algorithm for handling larger datasets efficiently.
*   **User Interface:** Develop a more interactive and user-friendly interface for enhanced user experience.

## License

This project is licensed under the GNU General Public License. Please refer to the [LICENSE](../../LICENSE) file for specific terms and conditions. 
