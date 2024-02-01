#![allow(dead_code)]
// Простой направленный взвешенный разреженные граф

use graph::{*};
mod graph {
    use std::fmt::{Debug, Display};
    use std::ops::Add;
    use std::collections::VecDeque;
   
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct IndexEdge(usize);
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct IndexVertex(usize);

    impl IndexVertex{
        pub fn new(value: usize) -> Self{
            Self(value)
        }
    }

    pub struct Data<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default + Add<Output = W> + Copy>{
        pub from: T,
        pub to: Option<(T,W)>
    }

    pub struct Graph<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default>{
        vertexes: StoreVertexes<T,W>,
        edges: StoreEdges<W>,
        queue_visit: VecDeque<(IndexVertex,W,Option<IndexVertex>)>
    }

    #[derive(Debug)]
    pub struct Vertex<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default>{
        payload: T,
        edges: Vec<IndexEdge>,
        sum_weight: Option<W>,
        previous_vertex: Option<IndexVertex>,
        visited: bool
    }
 
    #[derive(Debug)]
    pub struct Edge<W: PartialEq + Display + Debug + Default>{
        weight: W,
        from_vertex: IndexVertex,
        to_vertex: IndexVertex,
    }

    #[derive(Debug)]
    struct StoreEdges<W: PartialEq + Display + Debug + Default>{
        edges: Vec<Option<Edge<W>>> 
    }
    #[derive(Debug)]
    struct StoreVertexes<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default>{
        vertexes: Vec<Option<Vertex<T,W>>> 
    }
    
    /// Graph
    impl<T: PartialEq + Display + Debug + Clone + Ord,W: PartialEq + Display + Debug + Default + Add<Output = W> + Copy + PartialOrd + Ord> Graph <T,W>{
        pub fn new(size: usize) -> Self{
            Self{vertexes: StoreVertexes::new(size), edges: StoreEdges::new(size), queue_visit: VecDeque::with_capacity(9)}
        }
        pub fn new_with_data(data: Vec<Data<T,W>>) -> Self{
            let len = data.len();
            let mut vertexes:Vec<T> = Vec::with_capacity(len);
            let mut edges:Vec<(T,T,W)> = Vec::with_capacity(len);
            for i in data{
                vertexes.push(i.from.clone());
                if let Some((to,w)) = i.to{
                    vertexes.push(to.clone());
                    edges.push((i.from,to,w));
                }
                
            }
            vertexes.sort(); 
            vertexes.dedup(); 
            
            let mut graph = Graph{vertexes: StoreVertexes::new(len), edges: StoreEdges::new(len), queue_visit: VecDeque::with_capacity(9)};

            for d in vertexes.iter(){
                graph.add_vertex(Vertex::new(d.clone()));
            }
            for (from,to,w) in edges{
                let from_vertex = IndexVertex(vertexes.binary_search(&from).unwrap());
                let to_vertex = IndexVertex(vertexes.binary_search(&to).unwrap());
                let edge = Edge::new(w,from_vertex,to_vertex);
                let index_edge = graph.add_edge(edge);

                let from_vertex = graph.get_mut_vertex(&from_vertex).unwrap();
                from_vertex.add_edge(index_edge);
            }
            graph
        }

        pub fn add(&mut self, data: Data<T,W>){
            let index_from =  if self.vertexes.contains(&data.from){
                self.vertexes.find(&data.from).unwrap()
            }else{
                let from_vertex = Vertex::new(data.from);
                let index_from = self.add_vertex(from_vertex);
                index_from 
            }; 
            if let Some(to) = data.to{
                let index_to = if self.vertexes.contains(&to.0){
                    self.vertexes.find(&to.0).unwrap()
                }else{
                    let to_vertex = Vertex::new(to.0);
                    let index_to = self.add_vertex(to_vertex);
                    index_to 
                }; 
                if index_from == index_to{
                    panic!("Data is not correct. Identical indices");
                }
                if !self.edges.contains(&to.1,&index_from,&index_to){
                    
                    let edge = Edge::new(to.1,index_from.clone(),index_to);
                    let index_edge = self.add_edge(edge);
                    let from_vertex = self.get_mut_vertex(&index_from).unwrap();
                    from_vertex.add_edge(index_edge);
                }else{ 
                    panic!("Data is not correct. Edges not found");
                }   
            }
        }

        pub fn add_vertex(&mut self, vertex: Vertex<T,W>) -> IndexVertex{
            self.vertexes.add(vertex)
        }
        pub fn add_edge(&mut self, edge: Edge<W>) -> IndexEdge{
            self.edges.add(edge)
        }
        pub fn path_build(&self, start_vertex: IndexVertex, end_vertex: IndexVertex) -> Vec<&T>{
            let mut path = vec![];
            let mut end_vertex = Some(end_vertex);
            loop {
                //println!("previous={:?}",end_vertex);
                if let Some(index_vertex) = end_vertex{
                    if let Some(vertex) = self.get_vertex(&index_vertex) {
                        path.push(&vertex.payload);
                        if vertex.previous_vertex == Some(start_vertex){
                            break;
                        } 
                        end_vertex = vertex.previous_vertex;
                    }
                }else{
                    end_vertex = None;
                    break;
                }  
            }
            if let Some(vertex) = self.get_vertex(&start_vertex) {
                path.push(&vertex.payload);
            }
            path.reverse();
 
            path
        }
        pub fn get_vertex(&self, index: &IndexVertex) -> Option<&Vertex<T,W>>{
            self.vertexes.get_vertex(index)
        }
        fn get_mut_vertex(&mut self, index: &IndexVertex) -> Option<&mut Vertex<T,W>>{
            self.vertexes.get_mut_vertex(index)
        }

        pub fn breadth_first_search_with_deque<'a,'b:'a>(&'b self, start_vertex: T, ret: &mut Vec<&'a T>) {
            let mut index_vertex: &IndexVertex;
            let mut deque: VecDeque<&IndexVertex> = VecDeque::with_capacity(9);
            if !self.vertexes.contains(&start_vertex){
                return ();
            }
           
            let index_start_vertex = self.vertexes.find(&start_vertex).unwrap();
            deque.push_back(&index_start_vertex);
                
            while !deque.is_empty() {
                index_vertex = deque.pop_front().unwrap();
                let vertex = self.vertexes.get_vertex(index_vertex).unwrap();
                if !ret.contains(&&vertex.payload){
                    ret.push(&vertex.payload);
                }
                
                for indexes_vertex in self.adjacency_vertexes(index_vertex){
                    deque.push_back(indexes_vertex);
                }
            }
             
        }

        fn adjacency_vertexes(&self, index_vertex: &IndexVertex) -> Vec<&IndexVertex> {
            let mut ret = Vec::with_capacity(1);
            if let Some(vertex) = self.vertexes.get_vertex(index_vertex){
                for index_edge in vertex.edges.iter(){
                    if let Some(edge) = self.edges.get_edge(index_edge){
                        ret.push(&edge.to_vertex);
                    }
                }
            }
            ret
        }

        /// DOT specification.
        /// TODO: open http://www.webgraphviz.com/?tab=map
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display(&self) -> String {
            let mut buf: String = "".into();
            for vertex in self.vertexes.vertexes().iter().filter(|el|el.is_some()).map(|el|el.as_ref().unwrap()){
                for index_edge in vertex.edges.iter(){
                    if let Some(edge) = self.edges.get_edge(&index_edge){
                        if let Some(to_vertex) = self.vertexes.get_vertex(&edge.to_vertex){
                            buf.push_str(&format!("\t{n1}->{n2} [label=\"{weight}\"];\n",n1=vertex.payload, n2=to_vertex.payload, weight=edge.weight));
                        }
                    }
                } 
                if vertex.edges.is_empty(){
                    buf.push_str(&format!("\t{n1}\n",n1=vertex.payload));
                }
            }
            format!("\n\ndigraph G {{\n\trankdir=LR;\n\tsize=\"10\";\n\tnode [shape = circle];\n\tratio = fill;\n\tnode [style=filled fontcolor=\"black\"];\n{}}}",buf) 
        }

        pub fn display_with_path(&self, path: Vec<T>) -> String {
            let mut buf: String = "".into();
            for vertex in self.vertexes.vertexes().iter().filter(|el|el.is_some()).map(|el|el.as_ref().unwrap()){
                for index_edge in vertex.edges.iter(){
                    if let Some(edge) = self.edges.get_edge(&index_edge){
                        if let Some(to_vertex) = self.vertexes.get_vertex(&edge.to_vertex){
                            if path.contains(&vertex.payload) && path.contains(&to_vertex.payload){
                                buf.push_str(&format!("\t{n1}->{n2} [color=\"red\", label=\"{weight}\"];\n",n1=vertex.payload, n2=to_vertex.payload, weight=edge.weight));
                            }else{
                                buf.push_str(&format!("\t{n1}->{n2} [label=\"{weight}\"];\n",n1=vertex.payload, n2=to_vertex.payload, weight=edge.weight));
                            }
                        }
                    }
                } 
                if vertex.edges.is_empty(){
                    buf.push_str(&format!("\t{n1}\n",n1=vertex.payload));
                }
            }
            format!("\n\ndigraph G {{\n\trankdir=LR;\n\tsize=\"10\";\n\tnode [shape = circle];\n\tratio = fill;\n\tnode [style=filled fontcolor=\"black\"];\n{}}}",buf) 
        }
      
        pub fn dijkstras_algorithm(&mut self, from: &T, to: &T) -> Option<(W,Vec<&T>)>{
            self.queue_visit = VecDeque::with_capacity(9);
            self.vertexes.reset_dijkstras();

            let start_weight = W::default();
             
            if !self.vertexes.contains(from) || !self.vertexes.contains(to) { 
                return None;
            }
            let index_from_vertex = self.vertexes.find(from).unwrap();
            
            let from_vertex = self.vertexes.get_mut_vertex(&index_from_vertex);
            if from_vertex.is_none(){ 
                return None;
            }
            let from_vertex = from_vertex.unwrap();
            from_vertex.visited = true;
            self.queue_visit = VecDeque::new();
            
            for index_edge in from_vertex.edges.iter(){
                if let Some(edge) = self.edges.get_edge(index_edge){
                    self.queue_visit.push_back((edge.to_vertex,start_weight + edge.weight,Some(index_from_vertex)));
                }
            }
             
            while !self.queue_visit.is_empty() {
                let (next_vertex, sum_weight, previous_vertex) = self.queue_visit.pop_front().unwrap();

                let from_vertex = self.vertexes.get_mut_vertex(&next_vertex);
                if from_vertex.is_some(){
                    let from_vertex = from_vertex.unwrap();
                      
                    if let Some(ref mut sw) = &mut from_vertex.sum_weight{
                        if *sw > sum_weight{
                            *sw = sum_weight;
                            from_vertex.previous_vertex = previous_vertex;
                            from_vertex.visited = false;
                        } 
                    }else{
                        from_vertex.sum_weight = Some(sum_weight);
                        from_vertex.previous_vertex = previous_vertex;
                    }
                    if !from_vertex.visited{
                        from_vertex.visited = true;
                       
                        for index_edge in from_vertex.edges.iter(){
                            if let Some(edge) = self.edges.get_edge(index_edge){ 
                                self.queue_visit.push_back((edge.to_vertex,sum_weight + edge.weight,Some(next_vertex)));
                            }
                        }                 
                    }
                }
            }

            let index_to_vertex = self.vertexes.find(to).unwrap();
            let vertex = self.vertexes.get_vertex(&index_to_vertex).unwrap();
            if vertex.sum_weight.is_none(){
                return None;
            }
            Some((vertex.sum_weight.unwrap(),self.path_build(index_from_vertex, index_to_vertex)))
        }

        pub fn algorithm_a_star(){

        }
    }

    /// Vertex
    impl<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default> Vertex <T,W> {
        pub fn new(payload: T) -> Self{
            Self{payload, edges: Vec::with_capacity(1), sum_weight: None, previous_vertex: None, visited: false}
        }
        pub fn add_edge(&mut self, index: IndexEdge) {
            self.edges.push(index);
        }
        fn eq(&self, payload: &T) -> bool {
            &self.payload == payload
        }
    }

    /// Edge
    impl <W: PartialEq + Display + Debug + Default> Edge <W> {
        pub fn new(weight: W, from_vertex: IndexVertex, to_vertex: IndexVertex) -> Self{
            Self{weight, from_vertex, to_vertex}
        }
        fn eq(&self, weight: &W, from_vertex: &IndexVertex, to_vertex: &IndexVertex) -> bool{
            &self.weight == weight && &self.from_vertex == from_vertex && &self.to_vertex == to_vertex
        }
    }
    
    /// StoreVertexes
    impl <T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default> StoreVertexes <T,W>{
        fn new(size: usize) -> Self{
            Self{vertexes: Vec::with_capacity(size)}
        }
        fn contains(&self, payload: &T) -> bool {
            self.vertexes.iter().filter(|el| 
                if let Some(el) = el {
                    el.eq(payload)
                }else{
                    false
                }).count() > 0
        }
        fn find(&self, payload: &T) -> Option<IndexVertex> {
            self.vertexes.iter()
            .position(|vertex| if let Some(n) = vertex {n.eq(payload)}else{false})
            .and_then(|el|Some(el)).map(|el|IndexVertex(el))
        }
        fn add(&mut self, vertex: Vertex<T,W>) -> IndexVertex{
            self.vertexes.push(Some(vertex));
            IndexVertex(self.vertexes.len()-1)
        }
        fn get_vertex(&self, index: &IndexVertex) -> Option<&Vertex<T,W>>{
            if let Some(vertex) = self.vertexes.get(index.0){
                return vertex.as_ref();
            }
            None
        }

        fn get_mut_vertex(&mut self, index: &IndexVertex) -> Option<&mut Vertex<T,W>>{ 
            if let Some(vertex) = self.vertexes.get_mut(index.0){
                return vertex.as_mut();
            }
            None
        }
        fn vertexes(&self) -> &Vec<Option<Vertex<T,W>>>{ 
            &self.vertexes 
        }
        fn reset_dijkstras(&mut self){
            for vertex  in self.vertexes.iter_mut() {
                if let Some(vertex) = vertex{
                    vertex.sum_weight = None;
                    vertex.previous_vertex = None;
                    vertex.visited = false;
                }
             }  
        }
    }

    /// StoreEdges
    impl<W: PartialEq + Display + Debug + Default> StoreEdges<W>{
        fn new(size: usize) -> Self{
            Self{edges: Vec::with_capacity(size)}
        }
        fn contains(&self, weight: &W, from_vertex: &IndexVertex, to_vertex: &IndexVertex) -> bool {
            self.edges.iter().filter(|el| 
                if let Some(el) = el {
                    el.eq(weight,from_vertex,to_vertex)
                }else{
                    false
                }).count() > 0
        }
        fn find(&self, weight: &W, from_vertex: &IndexVertex, to_vertex: &IndexVertex) -> Option<IndexEdge> {
            self.edges.iter()
            .position(|edge| if let Some(e) = edge {e.eq(weight,from_vertex,to_vertex)}else{false})
            .and_then(|el|Some(el)).map(|el|IndexEdge(el))
        }
        fn add(&mut self, edge: Edge<W>) -> IndexEdge{
            self.edges.push(Some(edge));
            IndexEdge(self.edges.len()-1)
        }
        fn get_edge(&self, index: &IndexEdge) -> Option<&Edge<W>>{
           if let Some(edge) = self.edges.get(index.0){
              return edge.as_ref();
           }
           None
        }

        fn edges(&self) -> &Vec<Option<Edge<W>>>{
            &self.edges
        }
    }
    
    impl<T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default> Drop for Graph<T,W> {
        fn drop(&mut self) {
            // println!("Drop: {:?}",self.vertexes)
        }
    }

    /// Data
    impl <T: PartialEq + Display + Debug, W: PartialEq + Display + Debug + Default + Add<Output = W> + Copy> Data <T, W>{
        pub fn new(from: T, to: Option<(T,W)>) -> Self{
            Self{from, to}
        }
    }

}

/// $ cargo test simple_directed_weighted_sparse_graph -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test graph::simple_directed_weighted_sparse_graph::tests::test_rand_success -- --nocapture
    #[test]
    fn test_rand_success() {
        use rand::{thread_rng, Rng};
        let indexes = 100_000;//16_777_216;
        let mut rng = thread_rng();
        let mut vertexes = vec![];
        for i in 0..indexes{
            vertexes.push(i);
        }
     
        let mut data: Vec<Data<i32,i32>> = vec![];
        let mut index = 0;
        {
            loop { 
                if index == indexes{
                    break;
                }
                let from = vertexes[index] as i32;
                let mut to: i32 = -1i32;
                loop{
                    let i = rng.gen_range(0..indexes);
                    to = vertexes[i] as i32;
                    if from != to{
                        data.push(Data::new(from,Some((to,rng.gen_range(1..10)))));
                        break; 
                    }
                }
                let to2 = vertexes[rng.gen_range(0..indexes)] as i32;
                if from != to && to2 != to && from != to2{
                    data.push(Data::new(from,Some((to2,rng.gen_range(1..10)))));
                    
                } 
                index+=1;
            }
        }

        let from = data[0].from;
        let to = data[data.len()-1].from;
        let mut graph: Graph<i32,i32> = Graph::new_with_data(data);

        if let Some((sum_weight, mut path)) = graph.dijkstras_algorithm(&from,&to){
            println!("WEIGHT={}",sum_weight);
            /*println!("WEIGHT={} path={:?}\n",sum_weight, &path);
            let path: Vec<i32> = path.iter_mut().map(|p|p.clone()).collect();
            println!("Display Graph:{}",graph.display_with_path(path));*/
        }else{
            println!("the vertices are not connected");
        }

    }


    // $ cargo test graph::simple_directed_weighted_sparse_graph::tests::test_success -- --nocapture
    #[test]
    fn test_success() {
        let data: Vec<Data<String,u8>> = vec![ 
            Data::new("A0".to_string(),Some(("B1".to_string(),4))),// A 0
            Data::new("B1".to_string(),Some(("D2".to_string(),10))),// B 1, D 2
          
            Data::new("D2".to_string(),Some(("F3".to_string(),11))),//  F 3
            Data::new("A0".to_string(),Some(("C4".to_string(),2))),// C 4
            Data::new("B1".to_string(),Some(("C4".to_string(),5))),
            Data::new("C4".to_string(),Some(("E5".to_string(),3))),// E 5
            Data::new("E5".to_string(),Some(("D2".to_string(),4))),
        ];
        let mut graph: Graph<String,u8> = Graph::new(10);
         
        for el in data{
            graph.add(el);
        }
        
        /*println!("Display Graph:{}",graph.display());

        let mut vertexes = vec![];
        graph.breadth_first_search_with_deque("A0".to_string(),&mut vertexes);
        println!("\nBreadth fist search:");
        for vertex in vertexes{
            print!("{}-",vertex); 
        }
        println!("\n");*/

        if let Some((sum_weight,mut path)) = graph.dijkstras_algorithm(&"A0".to_string(),&"F3".to_string()){
            println!("dijkstras_algorithm sum_weight={:?} path={:?}\n",sum_weight, &path);
            let path: Vec<String> = path.iter_mut().map(|p|p.to_owned()).collect();
            println!("Display Graph:{}",graph.display_with_path(path));
        }else{
            println!("the vertices are not connected");
        }

        
        println!("\n--------------------------------------\n");

        /*let vertex = graph.get_vertex(&IndexVertex::new(2)).unwrap();
        println!("vertex D2 sum_weight={:?} path={:?}\n",vertex.sum_weight,vertex.path);
        let path = vertex.path.clone().unwrap();
        graph.path_display(path);
        println!("\n--------------------------------------\n");*/

    }
    
    // $ cargo test graph::simple_directed_weighted_sparse_graph::tests::test_crate_simple_graph_algorithms -- --nocapture
    #[test]
    fn test_crate_simple_graph_algorithms() {
        // https://github.com/LMH01/simple_graph_algorithms/blob/master/src/lib.rs
        use simple_graph_algorithms::{Graph, algorithms::dijkstra};
        let mut graph = Graph::new();
    
        // Add new nodes to the graph
        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_node("D");
        graph.add_node("E");
        graph.add_node("F");
       
        // Add edges to the graph
        graph.add_edge(4, &"A", &"B"); // Adds an edge that leads from a to b with weight 1
        graph.add_edge(10, &"B", &"D");
        graph.add_edge(11, &"D", &"F");
        graph.add_edge(2, &"A", &"C");
        graph.add_edge(5, &"B", &"C");
        graph.add_edge(3, &"C", &"E");
        graph.add_edge(4, &"E", &"D");
        
        // Calculate the shortest path tree starting at node "a" using Dijkstra's algorithm
        let spt = dijkstra(&mut graph, &"A").unwrap();
        
        // Get the shortest distance from "a" to other nodes
        assert_eq!(spt.shortest_distance(&"F"), Some(20));

    }
 
}


/*  
              
И с Я думаю реалізацію графа краще порівнювати з petgraph 🤔 https://lib.rs/crates/petgraph


скорость и память

*/ 