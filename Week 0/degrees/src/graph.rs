use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct People {
    pub name: String,
    pub birth: u32,
    pub movies: HashSet<u32>,
}

#[derive(Debug, Clone)]
pub struct Movie {
    pub title: String,
    pub year: u16,
    pub stars: HashSet<u32>,
}


#[derive(Debug, Clone)]
pub struct Graph {
    pub actors: HashMap<u32, People>,
    pub movies: HashMap<u32, Movie>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub state: u32,
    pub parent: Option<Box<Node>>,
    pub action: u32,
}

impl Node {
    pub fn new(state: u32, parent: Option<Box<Node>>, action: u32) -> Self {
        Node { state, parent, action }
    }
}

impl Graph {
    pub fn new(actors: HashMap<u32, People>, movies: HashMap<u32, Movie>) -> Self {
        Graph { actors, movies }
    }

    pub fn shortest_path(&self, source: u32, target: u32) -> Option<Vec<(u32, u32)>> {
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();
        let mut path_map  = Vec::new();

        frontier.push_back(Node::new(source, None, 0));

        while let Some(mut node) = frontier.pop_front() {
            if node.state == target {
                while let Some(parent) = node.parent {
                    path_map.push((node.action, node.state));
                    node = *parent
                }
                path_map.reverse();
                return Some(path_map)
            }

            let neighbors = self.neighbors_for_person(node.state);
            visited.insert(node.state);

            for (movie_id, actor_id) in neighbors.iter() {
                if !visited.contains(actor_id) {
                    frontier.push_back(Node::new(*actor_id, Some(Box::new(node.clone())), *movie_id))
                }
            }

        }
        None

    }


    pub fn neighbors_for_person(&self, actor_id: u32) -> HashSet<(u32, u32)> {
        let mut neighbors = HashSet::new();
        if let Some(actor) = self.actors.get(&actor_id) {
            for movie_id in &actor.movies {
                if let Some(movie) = self.movies.get(movie_id) {
                    for co_star_id in &movie.stars {
                        neighbors.insert((*movie_id, *co_star_id));
                    }
                }
            }
        }
        neighbors
    }

    pub fn person_id_for_name(&self, name: &str) -> Option<u32> {
        let mut potential_ids: Vec<_> = self.actors.iter()
            .filter(|(_, actor)| actor.name.to_lowercase() == name.to_lowercase())
            .map(|(id, _)| *id)
            .collect();

        match potential_ids.len() {
            0 => None,
            1 => Some(potential_ids.pop().unwrap()),
            _ => {
                println!("Multiple actors found with name '{}':", name);
                for id in potential_ids {
                    let actor = self.actors.get(&id).unwrap();
                    println!("{}", actor.name);
                }
                None
            }
        }
    }
}