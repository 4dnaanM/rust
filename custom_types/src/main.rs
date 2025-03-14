use std::collections::HashMap;
use std::collections::HashSet; 

#[derive(Debug)]
struct Graph<T> {
	n: usize, 
	m: usize,
	vertices: HashSet<T>,   
	edges: HashMap<T, HashSet<T>> 
}

impl<T: std::hash::Hash + Eq + Copy> Graph<T> {
	fn new() -> Self {
		 Graph::<T> {
			n:0, 
			m:0,
			vertices: HashSet::new(),
			edges: HashMap::new() 
		}
	}
	fn n(&self) -> usize {
		self.n
	}
	fn m(&self) -> usize {
		self.m	
	}
	fn vertices(&self) -> &HashSet<T> {
		&self.vertices
	}
	fn edges(&self) -> &HashMap<T,HashSet<T>> {
		&self.edges
	}
	fn contains_vertex(&self, v:T) -> bool {
		self.vertices.contains(&v)
	}
	fn contains_edge(&self, v1:T, v2:T) -> bool {
		self.vertices.contains(&v1) && 
		self.vertices.contains(&v2) && 
		self.edges.get(&v1).unwrap().contains(&v2) 
	}
	fn get_neighbors(&self, v:T) -> Option<&HashSet<T>> {
		self.edges.get(&v)
	}
	fn add_vertex(&mut self, v:T) {
		if !self.contains_vertex(v) {
			self.vertices.insert(v);
			self.edges.entry(v).or_insert(HashSet::new()); 
			self.n = self.n + 1;
		}
	}
	fn add_edge (&mut self, v1:T, v2:T) {
		if !self.contains_edge(v1,v2) {
			self.edges.entry(v1).or_insert(HashSet::new()).insert(v2);	
			self.edges.entry(v2).or_insert(HashSet::new()).insert(v1);
			self.m+=1;
		}	
	} 
}

fn main() {
		let mut graph = Graph::<i32>::new();	
	println!("graph: {graph:?}");
	
	graph.add_vertex(1);
	graph.add_vertex(2);
	println!("graph: {graph:?}");	
	
	graph.add_edge(1,2);
	println!("graph: {graph:?}");	

	println!("n: {}, m: {} ",graph.n(), graph.m());
	println!("vertices: {:?}",graph.vertices());
	println!("edges: {:?}",graph.edges());
	println!("get_neighbors(1): {:?}",graph.get_neighbors(1));
} 
