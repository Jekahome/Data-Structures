#![allow(dead_code)]
#![allow(unused_imports)]

// Простой направленный взвешенный разреженные граф

use sdws_graph::{Graph, IndexVertex, PrepareInput, Vertex};
mod sdws_graph {
    use serde::{Deserialize, Serialize};
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashSet, VecDeque};
    use std::fmt::{Debug, Display};
    use std::ops::Add;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct IndexVertex(usize);

    impl IndexVertex {
        pub fn new(value: usize) -> Self {
            Self(value)
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PrepareInput<T, W> {
        pub from: T,
        pub to: Option<(T, W)>,
    }

    pub struct Graph<T, W> {
        vertexes: Vec<Option<Vertex<T, W>>>,
        edges: Vec<Option<Vec<(W, IndexVertex)>>>,
    }

    #[derive(Debug, PartialEq)]
    pub struct Vertex<T, W> {
        payload: T,
        sum_weight: Option<W>,
        previous_vertex: Option<IndexVertex>,
        visited: bool,
    }

    /// Graph
    impl<
            T: PartialEq + Display + Debug + Clone + Ord,
            W: PartialEq + Display + Debug + Default + Add<Output = W> + Copy + PartialOrd + Ord,
        > Graph<T, W>
    {
        pub fn new() -> Self {
            Self {
                vertexes: vec![],
                edges: vec![],
            }
        }

        pub fn new_with_prepare_input(data: Vec<PrepareInput<T, W>>) -> Self {
            let len = data.len();
            let mut vertexes: Vec<T> = Vec::with_capacity(len);

            for i in data.iter() {
                vertexes.push(i.from.clone());
                if let Some((ref to, _w)) = i.to {
                    vertexes.push(to.clone());
                }
            }
            vertexes.sort();
            vertexes.dedup();
            vertexes.truncate(vertexes.len());
            let len = vertexes.len();
            let mut graph = Graph {
                vertexes: Vec::with_capacity(len),
                edges: vec![None; len],
            };
            for d in vertexes.iter() {
                graph.add_vertex(Vertex::new(d.clone()));
            }
            for i in data {
                let from_vertex = IndexVertex(vertexes.binary_search(&i.from).unwrap());
                if let Some((to, w)) = i.to {
                    let to_vertex = IndexVertex(vertexes.binary_search(&to).unwrap());
                    graph.add_edge(from_vertex, to_vertex, w);
                }
            }
            graph
        }

        pub fn add(&mut self, data: PrepareInput<T, W>) -> bool {
            let index_from = if self.vertex_contains(&data.from) {
                self.find_vertex(&data.from).unwrap()
            } else {
                let from_vertex = Vertex::new(data.from);
                let index_from = self.add_vertex(from_vertex);
                index_from
            };
            if let Some(to) = data.to {
                let index_to = if self.vertex_contains(&to.0) {
                    self.find_vertex(&to.0).unwrap()
                } else {
                    let to_vertex = Vertex::new(to.0);
                    let index_to = self.add_vertex(to_vertex);
                    index_to
                };
                if index_from == index_to {
                    panic!("Data is not correct. Identical indices");
                }
                if let Some(edges) = self.get_edges(&index_from) {
                    if edges
                        .iter()
                        .filter(|&&edge| edge.0 == to.1 && edge.1 == index_to)
                        .count()
                        == 0
                    {
                        self.add_edge(index_from, index_to, to.1);
                        return true;
                    }
                }
            }
            false
        }

        pub fn add_vertex(&mut self, vertex: Vertex<T, W>) -> IndexVertex {
            self.vertexes.push(Some(vertex));
            IndexVertex(self.vertexes.len() - 1)
        }

        fn add_edge(
            &mut self,
            index_from_vertex: IndexVertex,
            index_to_vertex: IndexVertex,
            weight: W,
        ) {
            if let Some(ref mut edges) = self.edges[index_from_vertex.0] {
                edges.push((weight, index_to_vertex)); // возможны дубликаты!
            } else {
                self.edges[index_from_vertex.0] = Some(vec![(weight, index_to_vertex)]);
            }
        }

        fn reset_dijkstras(&mut self) {
            for vertex in self.vertexes.iter_mut() {
                if let Some(vertex) = vertex {
                    vertex.sum_weight = None;
                    vertex.previous_vertex = None;
                    vertex.visited = false;
                }
            }
        }

        fn vertex_contains(&self, payload: &T) -> bool {
            self.vertexes
                .iter()
                .filter(|el| {
                    if let Some(el) = el {
                        el.eq(payload)
                    } else {
                        false
                    }
                })
                .count()
                > 0
        }

        fn find_vertex(&self, payload: &T) -> Option<IndexVertex> {
            self.vertexes
                .iter()
                .position(|vertex| {
                    if let Some(n) = vertex {
                        n.eq(payload)
                    } else {
                        false
                    }
                })
                .and_then(|el| Some(el))
                .map(|el| IndexVertex(el))
        }

        fn get_mut_vertex(&mut self, index: &IndexVertex) -> Option<&mut Vertex<T, W>> {
            unsafe { self.vertexes.get_unchecked_mut(index.0).as_mut() }
        }

        fn get_vertex(&self, index: &IndexVertex) -> Option<&Vertex<T, W>> {
            unsafe { self.vertexes.get_unchecked(index.0).as_ref() }
        }

        fn get_edges(&self, index_vertex: &IndexVertex) -> Option<&Vec<(W, IndexVertex)>> {
            if let Some(edge) = self.edges.get(index_vertex.0) {
                return edge.as_ref();
            }
            None
        }

        pub fn path_build(&self, start_vertex: IndexVertex, end_vertex: IndexVertex) -> Vec<&T> {
            let mut path = vec![];
            let mut end_vertex = Some(end_vertex);
            loop {
                if let Some(index_vertex) = end_vertex {
                    if let Some(vertex) = self.get_vertex(&index_vertex) {
                        path.push(&vertex.payload);
                        if vertex.previous_vertex == Some(start_vertex) {
                            break;
                        }
                        end_vertex = vertex.previous_vertex;
                    }
                } else {
                    break;
                }
            }
            if let Some(vertex) = self.get_vertex(&start_vertex) {
                path.push(&vertex.payload);
            }
            path.reverse();
            path
        }

        pub fn breadth_first_search_with_deque<'a, 'b: 'a>(
            &'b self,
            start_vertex: T,
            ret: &mut Vec<&'a T>,
        ) {
            let mut index_vertex: &IndexVertex;
            let mut deque: VecDeque<&IndexVertex> = VecDeque::with_capacity(256);
            if !self.vertex_contains(&start_vertex) {
                return ();
            }
            let index_start_vertex = self.find_vertex(&start_vertex).unwrap();
            deque.push_back(&index_start_vertex);
            while !deque.is_empty() {
                index_vertex = deque.pop_front().unwrap();
                let vertex = self.get_vertex(index_vertex).unwrap();
                if !ret.contains(&&vertex.payload) {
                    ret.push(&vertex.payload);
                }
                for indexes_vertex in self.adjacency_vertexes(index_vertex) {
                    deque.push_back(indexes_vertex);
                }
            }
        }

        fn adjacency_vertexes(&self, index_vertex: &IndexVertex) -> Vec<&IndexVertex> {
            let mut ret = Vec::with_capacity(1);
            if let Some(from_to) = self.get_edges(&index_vertex) {
                for (weight, to_vertex) in from_to {
                    ret.push(to_vertex);
                }
            }
            ret
        }

        /// DOT specification.
        /// TODO: open http://www.webgraphviz.com/?tab=map
        /// or https://dreampuf.github.io/GraphvizOnline/
        pub fn display_dot(&self) -> String {
            let mut display: String = "".into();
            for vertex in self
                .vertexes
                .iter()
                .filter(|el| el.is_some())
                .map(|el| el.as_ref().unwrap())
            {
                let index_vertex = self.find_vertex(&vertex.payload).unwrap();
                let edges = self.get_edges(&index_vertex);
                if let Some(from) = edges {
                    if from.len() > 0 {
                        for (weight, to_vertex) in from.iter() {
                            let v = self.get_vertex(to_vertex).unwrap();
                            display.push_str(&format!(
                                "\t{v1}->{v2} [label=\"{weight}\"];\n",
                                v1 = vertex.payload,
                                v2 = v.payload,
                                weight = weight
                            ));
                        }
                    } else {
                        display.push_str(&format!("\t{v1}\n", v1 = vertex.payload));
                    }
                } else {
                    display.push_str(&format!("\t{v1}\n", v1 = vertex.payload));
                }
            }
            format!("\n\ndigraph G {{\n\trankdir=LR;\n\tsize=\"10\";\n\tnode [shape = circle];\n\tratio = fill;\n\tnode [style=filled fontcolor=\"black\"];\n{}}}",display)
        }

        pub fn display_dot_with_path(&self, path: &Vec<T>) -> String {
            let mut display: String = "".into();
            let prev = &path[0];
            let mut pair_path = HashSet::with_capacity(path.len());
            let _ = path.iter().skip(1).fold(prev, |prev, next| {
                pair_path.insert(format!("{}_{}", prev, next));
                &next
            });
            for vertex in self
                .vertexes
                .iter()
                .filter(|el| el.is_some())
                .map(|el| el.as_ref().unwrap())
            {
                let index_vertex = self.find_vertex(&vertex.payload).unwrap();
                let edges = self.get_edges(&index_vertex);
                if let Some(from) = edges {
                    if from.len() > 0 {
                        for (weight, to_vertex) in from.iter() {
                            let vertex_to = self.get_vertex(to_vertex).unwrap();
                            if pair_path
                                .contains(&format!("{}_{}", vertex.payload, vertex_to.payload))
                            {
                                display.push_str(&format!(
                                    "\t{v1} [color=\"red\"];\n",
                                    v1 = vertex.payload
                                ));
                                display.push_str(&format!(
                                    "\t{v2} [color=\"red\"];\n",
                                    v2 = vertex_to.payload
                                ));
                                display.push_str(&format!(
                                    "\t{v1}->{v2} [color=\"red\", label=\"{weight}\"];\n",
                                    v1 = vertex.payload,
                                    v2 = vertex_to.payload,
                                    weight = weight
                                ));
                            } else {
                                display.push_str(&format!(
                                    "\t{v1}->{v2} [label=\"{weight}\"];\n",
                                    v1 = vertex.payload,
                                    v2 = vertex_to.payload,
                                    weight = weight
                                ));
                            }
                        }
                    } else {
                        display.push_str(&format!("\t{v1}\n", v1 = vertex.payload));
                    }
                } else {
                    display.push_str(&format!("\t{v1}\n", v1 = vertex.payload));
                }
            }
            format!("\n\ndigraph G {{\n\trankdir=LR;\n\tsize=\"10\";\n\tnode [shape = circle];\n\tratio = fill;\n\tnode [style=filled fontcolor=\"black\"];\n{}}}",display)
        }

        pub fn dijkstras_algorithm(&mut self, from: &T, to: &T) -> Option<(W, Vec<&T>)> {
            self.reset_dijkstras();
            let start_weight = W::default();
            if !self.vertex_contains(from) || !self.vertex_contains(to) {
                return None;
            }
            let index_from_vertex = self.find_vertex(from).unwrap();
            let from_vertex = self.get_mut_vertex(&index_from_vertex);
            if from_vertex.is_none() {
                return None;
            }
            let from_vertex = from_vertex.unwrap();
            from_vertex.visited = true;

            let mut heap_queue_visit: BinaryHeap<MinWeight<W>> = BinaryHeap::with_capacity(64);

            if let Some(from_to) = self.get_edges(&index_from_vertex) {
                for (weight, to_vertex) in from_to {
                    heap_queue_visit.push(MinWeight::new(
                        start_weight + *weight,
                        *to_vertex,
                        index_from_vertex,
                    ));
                }
            }
            let index_to_vertex = self.find_vertex(to).unwrap();
            while let Some(MinWeight(mut sum_weight, next_vertex, previous_vertex)) =
                heap_queue_visit.pop()
            {
                if let Some(from_vertex) = self.get_mut_vertex(&next_vertex) {
                    if from_vertex.visited {
                        continue;
                    }
                    if let Some(ref mut sw) = &mut from_vertex.sum_weight {
                        if sw > &mut sum_weight {
                            *sw = sum_weight;
                            (*from_vertex).previous_vertex = Some(previous_vertex);
                            (*from_vertex).visited = false;
                        }
                    } else {
                        (*from_vertex).sum_weight = Some(sum_weight);
                        (*from_vertex).previous_vertex = Some(previous_vertex);
                    }
                    if index_to_vertex == next_vertex {
                        break;
                    }
                    if !from_vertex.visited {
                        (*from_vertex).visited = true;
                        if let Some(from_to) = self.get_edges(&next_vertex) {
                            for (weight, to_vertex) in from_to {
                                /*if let Some(v) = self.get_vertex(&to_vertex){
                                    if v.visited{
                                        continue;
                                    }
                                }*/
                                heap_queue_visit.push(MinWeight::new(
                                    sum_weight + *weight,
                                    *to_vertex,
                                    next_vertex,
                                ));
                            }
                        }
                    }
                }
            }
            let vertex = self.get_vertex(&index_to_vertex).unwrap();
            if vertex.sum_weight.is_none() {
                return None;
            }
            Some((
                vertex.sum_weight.unwrap(),
                self.path_build(index_from_vertex, index_to_vertex),
            ))
        }
    }

    #[derive(Debug)]
    struct MinWeight<W>(W, IndexVertex, IndexVertex);

    impl<W> MinWeight<W> {
        fn new(sum_weight: W, next_vertex: IndexVertex, previous_vertex: IndexVertex) -> Self {
            Self(sum_weight, next_vertex, previous_vertex)
        }
    }

    impl<W: PartialOrd> PartialEq for MinWeight<W> {
        #[inline]
        fn eq(&self, other: &MinWeight<W>) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl<W: PartialOrd> Eq for MinWeight<W> {}

    impl<W: PartialOrd> PartialOrd for MinWeight<W> {
        #[inline]
        fn partial_cmp(&self, other: &MinWeight<W>) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<W: PartialOrd> Ord for MinWeight<W> {
        #[inline]
        fn cmp(&self, other: &MinWeight<W>) -> Ordering {
            let a = &self.0;
            let b = &other.0;
            if a == b {
                Ordering::Equal
            } else if a < b {
                Ordering::Greater
            } else if a > b {
                Ordering::Less
            } else if a.ne(a) && b.ne(b) {
                // these are the NaN cases
                Ordering::Equal
            } else if a.ne(a) {
                // Order NaN less, so that it is last in the MinScore order
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    /// Vertex
    impl<T: PartialEq, W> Vertex<T, W> {
        fn new(payload: T) -> Self {
            Self {
                payload,
                sum_weight: None,
                previous_vertex: None,
                visited: false,
            }
        }

        fn eq(&self, payload: &T) -> bool {
            &self.payload == payload
        }
    }

    /// PrepareInput
    impl<T, W> PrepareInput<T, W> {
        pub fn new(from: T, to: Option<(T, W)>) -> Self {
            Self { from, to }
        }
    }
}

/// $ cargo miri test
/// $ cargo test simple_directed_weighted_sparse_graph -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test graph::simple_directed_weighted_sparse_graph::tests::test_dijkstras_success -- --nocapture
    #[test]
    fn test_dijkstras_success() {
        let input = gen_input();
        let from = input[0].from;
        let to = input[input.len() - 1].from;
        let mut graph: Graph<i32, i32> = Graph::new_with_prepare_input(input);

        if let Some((sum_weight, mut path)) = graph.dijkstras_algorithm(&from, &to) {
            println!(
                "Weight = {} \nNumber of vertices = {}",
                sum_weight,
                path.len()
            );
            let path: Vec<i32> = path.iter_mut().map(|p| p.clone()).collect();
            println!("Display Graph: {}", graph.display_dot_with_path(&path));
        } else {
            println!("the vertices are not connected");
        }
    }

    // $ cargo test graph::simple_directed_weighted_sparse_graph::tests::test_bfs_success -- --nocapture
    #[test]
    fn test_bfs_success() {
        let input: Vec<PrepareInput<String, u8>> = vec![
            PrepareInput::new("A0".to_string(), Some(("B1".to_string(), 4))), // A 0
            PrepareInput::new("B1".to_string(), Some(("D2".to_string(), 10))), // B 1, D 2
            PrepareInput::new("D2".to_string(), Some(("F3".to_string(), 11))), // F 3
            PrepareInput::new("A0".to_string(), Some(("C4".to_string(), 2))), // C 4
            PrepareInput::new("B1".to_string(), Some(("C4".to_string(), 5))),
            PrepareInput::new("C4".to_string(), Some(("E5".to_string(), 3))), // E 5
            PrepareInput::new("E5".to_string(), Some(("D2".to_string(), 4))),
        ];
        let mut graph: Graph<String, u8> = Graph::new();
        for el in input {
            graph.add(el);
        }
        println!("Display Graph:{}", graph.display_dot());
        let mut vertexes = vec![];
        graph.breadth_first_search_with_deque("A0".to_string(), &mut vertexes);
        println!("\nBreadth fist search:");
        for vertex in vertexes {
            print!("{}-", vertex);
        }
    }

    fn gen_input() -> Vec<PrepareInput<i32, i32>> {
        use rand::{thread_rng, Rng};
        let indexes = 50; //16_777_216; // 2^26 67_108_864
        let mut rng = thread_rng();
        let mut vertexes = vec![];
        for i in 0..indexes {
            vertexes.push(i);
        }

        let mut data: Vec<PrepareInput<i32, i32>> = vec![];
        let mut index = 0;
        {
            loop {
                if index == indexes {
                    break;
                }
                let from = vertexes[index] as i32;
                #[allow(unused_assignments)]
                let mut to: i32 = -1i32;
                loop {
                    let i = rng.gen_range(0..indexes);
                    to = vertexes[i] as i32;
                    if from != to {
                        data.push(PrepareInput::new(from, Some((to, rng.gen_range(1..10)))));
                        break;
                    }
                }
                let to2 = vertexes[rng.gen_range(0..indexes)] as i32;
                if from != to && to2 != to && from != to2 {
                    data.push(PrepareInput::new(from, Some((to2, rng.gen_range(1..10)))));
                }
                index += 1;
            }
        }
        data
    }
}
