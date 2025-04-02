use std::collections::HashMap;
use std::collections::HashSet; 
use std::vec::Vec; 

#[derive(Debug)]
#[derive(Default)]
struct Graph<T> {
	n: usize, 
	m: usize,
	vertices: HashSet<T>,   
	edges: HashMap<T, HashSet<T>> 
}

impl<T: std::hash::Hash + Eq + Copy + std::default::Default> Graph<T> {
	fn new() -> Self {
		 Graph::<T> {
			n:0, 
			m:0,
			..Default::default()  
		}
	}
	fn contains_vertex(&self, v:T) -> bool {
		self.vertices.contains(&v)
	}
	fn contains_edge(&self, v1:T, v2:T) -> bool {
		assert!(self.contains_vertex(v1),"Can't add edge; Vertex not present!");	
		assert!(self.contains_vertex(v2),"Can't add edge; Vertex not present!");	
		self.edges.get(&v1).unwrap().contains(&v2)
	}
	fn get_neighbors(&self, v:T) -> &HashSet<T> {
		assert!(self.contains_vertex(v),"Vertex missing in graph!");
		self.edges.get(&v).unwrap()
	}
	fn add_vertex(&mut self, v:T) {
		assert!(!self.contains_vertex(v),"Vertex to add already present!");
		self.vertices.insert(v);
		self.edges.insert(v,HashSet::new()); 
		self.n += 1
	}
	fn add_edge (&mut self, v1:T, v2:T) {
		assert!(!self.contains_edge(v1,v2),"Edge to add already present!");
		// can safely assume v1 and v2
		self.edges.get_mut(&v1).unwrap().insert(v2);	
		self.edges.get_mut(&v2).unwrap().insert(v1);
		self.m+=1;
	} 
	fn depth_first_search(&self, node: T) -> Vec<T> {
		let mut visited: Vec<T> = Vec::new(); 
		let mut to_visit: Vec<T>  = Vec::new();
		let mut neighbors: &HashSet<T>;
		
		to_visit.push(node);
		let mut cur: T; 
		
		while !to_visit.is_empty() {
			cur = to_visit.pop().expect("Depth First Search Loop"); 
			visited.push(cur); 

			neighbors = self.get_neighbors(cur);
			for neighbor in neighbors {
				if !visited.contains(neighbor) {
					to_visit.push(*neighbor);
				}
			}
		}
		return visited; 
	}
}

fn main() {
	let mut graph = Graph::<i32>::new();	
	println!("graph: {graph:?}");
	
	for v in 0..32 {
		graph.add_vertex(v);
	}
	println!("graph: {graph:?}");	

	graph.add_edge(1,2);
	graph.add_edge(1,3); 
	graph.add_edge(2,5);
	graph.add_edge(5,11); 
	graph.add_edge(13,21); 
	graph.add_edge(21,22); 
	graph.add_edge(22,23); 
	graph.add_edge(11,13); 

	println!("graph: {graph:?}");	

	println!("n: {}, m: {} ",graph.n, graph.m);
	println!("vertices: {:?}",graph.vertices);
	println!("edges: {:?}",graph.edges);
	println!("get_neighbors(1): {:?}",graph.get_neighbors(1));

	println!("dfs(1): {:?}",graph.depth_first_search(1)); 
} 
