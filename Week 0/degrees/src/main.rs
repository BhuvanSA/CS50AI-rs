mod graph;
mod utils;

fn main() {
    
    println!("Loading Data...");
    let graph = utils::load_data("large");
    println!("Data Loaded");

    // Take source name as input
    // println!("Enter source actor name:");
    // let source_name: String = utils::read_input();
    let source_name: String = String::from("Leonardo DiCaprio");

    // Take target name as input
    // println!("Enter target actor name:");
    // let target_name: String = utils::read_input();
    let target_name: String = String::from("Leonardo DiCaprio");

    // Find the ID of the source actor
    let source_id = graph.person_id_for_name(&source_name).unwrap();
    // Find the ID of the target actor
    let target_id = graph.person_id_for_name(&target_name).unwrap();
    
    let result = graph.shortest_path(source_id, target_id);

    // If a path is found, print the path
    match result {
        Some(path) => {
            println!("{} degrees of separation.", path.len());
            for (i, (movie_id, actor_id)) in path.iter().enumerate() {
                let actor = graph.actors.get(actor_id).unwrap();
                let movie = graph.movies.get(movie_id).unwrap();
                if i == 0 {
                    println!("{} and {} starred in {}", source_name, actor.name, movie.title);
                } else {
                    let prev_actor_id = path[i - 1].1.clone();
                    let prev_actor = graph.actors.get(&prev_actor_id).unwrap();
                    println!("{} and {} starred in {}", prev_actor.name, actor.name, movie.title);
                }
            }
        }
        None => println!("No path found."),
    }

    // Does passing around graph cost extra memory or time?
    // No, it doesn't. The graph is a reference to the data, so it doesn't take up extra memory.

    // What are the benefits of using a struct to represent the graph?
    // Using a struct to represent the graph allows us to encapsulate the data and methods related to the graph in one place. This makes the code more organized and easier to understand.

    // Can parallelizing the shortest path algorithm improve performance?
    // Yes, parallelizing the shortest path algorithm can improve performance by distributing the work across multiple threads or processes. This can reduce the time taken to find the shortest path between two actors.
    // However, parallelizing the algorithm may introduce additional complexity and overhead, so it's important to carefully consider the trade-offs before parallelizing the algorithm.

    // Can we start reading the data and processing it in parallel?
    // Yes, we can start reading the data and processing it in parallel by using multiple threads or processes. This can help improve performance by overlapping the I/O operations with the processing of the data.
    // For example, we can read the data from the files in parallel and then construct the graph once all the data has been read. This can help reduce the overall time taken to load the data and start processing it.



}