#[derive(Debug, Clone, Default)]
struct Node {
    previous: Option<usize>,
    distance: i32,
}

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<Node>,
    connections: Vec<(usize, usize)>,
    gateways: Vec<usize>,
    start: Option<usize>,
}

impl Graph {
    fn new(nodes_n: usize) -> Graph {
        Graph {
            nodes: vec![Default::default(); nodes_n],
            ..Default::default()
        }
    }

    fn is_gateway(&self, id: usize) -> bool {
        self.gateways.contains(&id)
    }
    fn add_gateway(&mut self, id: usize) {
        if !self.is_gateway(id) { self.gateways.push(id) }
    }

    fn is_connected(&self, id1: usize, id2: usize) -> bool {
        self.connections.contains(&(id1, id2)) || self.connections.contains(&(id2, id1))
    }

    fn add_connection(&mut self, id1: usize, id2: usize) {
        if !self.is_connected(id1, id2) { self.connections.push((id1, id2)) }
    }

    fn cut_connection(&mut self, id1: usize, id2: usize) {
        To jes
    }

    fn is_gate(&self, node: usize) -> bool {
        self.gateways.iter().any(|g| g == &node)
    }

    pub fn neighbors_gates_of(&self, node: usize) -> Vec<usize> {
        self.connections.iter().filter_map(|con| {
            match con {
                (n1, n2) if n1 == &node => Some(n2),
                (n1, n2) if n2 == &node => Some(n1),
                _ => None,
            }
        })
            .filter(|n| self.gateways.iter().any(|g| n == &g))
            .map(|x| x.clone()).collect()
    }

    pub fn neighbors_of(&self, node: usize) -> Vec<usize> {
        self.connections.iter().filter_map(|con| {
            match con {
                (n1, n2) if n1 == &node => Some(n2),
                (n1, n2) if n2 == &node => Some(n1),
                _ => None,
            }
        }).map(|x| x.clone()).collect()
    }

    pub fn walk(&mut self, start: usize) {
        use std::i32::MAX;
        self.start = Some(start);

        for node in self.nodes.iter_mut() {
            node.distance = MAX;
            node.previous = None;
        }

        let mut open: Vec<usize> = self.nodes.iter()
            .enumerate()
            .map(|(id, _)| id)
            .filter(|node| !self.is_gate(*node))
            .collect();

        self.nodes[start].distance = 0;
        while open.len() > 0 {
            let current = {
                let mut min = 0;
                let mut min_val = MAX;
                for (id, data) in self.nodes.iter()
                    .enumerate()
                    .filter(|(id, _)| open.iter().any(|x| x == id)) // Filter out not open
                    {
                        if data.distance <= min_val {
                            min = id;
                            min_val = data.distance;
                        }
                    }
                min
            };
            let current_data = self.nodes[current].clone();

            open.retain(|x| x != &current);

            for neighbor in self.neighbors_of(current) {
                let neighbor_data = &mut self.nodes[neighbor];

                if neighbor_data.distance > current_data.distance + 1 {
                    neighbor_data.distance = current_data.distance + 1;
                    neighbor_data.previous = Some(current);
                }
            }
        }
    }

    fn path_to(&self, target: usize) -> Vec<usize> {
        if self.start.is_none() {
            return vec![];
        }

        let mut current = target;
        let start = self.start.unwrap();
        let mut out = vec![];

        out.push(current);
        while current != start {
            let prev = self.nodes[current].previous.unwrap();
            out.push(prev);
            current = prev;
        }

        out.reverse();
        return out;
    }
}

macro_rules! graph {
    (nodes: $nodes:expr) => {Graph::new($nodes)};
    (nodes: $nodes:expr, gateways: [$($gateway:expr),*], connections: [$( $n1:expr, $n2:expr )*]) => {{
        let mut graph = graph!(nodes: $nodes);
        $(graph.add_gateway($gateway);)*
        $(graph.add_connection($n1, $n2);)*
        graph
    }};
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn neighbors() {
        let graph01 = graph!(nodes: 3,
                gateways: [2],
                connections: [
                    0, 1
                    1, 2
                    2, 3
                ]);

        let tmp = graph01.gateways.iter().map(|gateway| gateway);
        assert_eq!(vec![0, 2], graph01.neighbors_of(1))
    }

    #[test]
    fn simple_multipath() {
        let mut graph01 = graph!(
                nodes: 4,
                gateways: [3],
                connections: [
                    0, 1
                    0, 2
                    1, 3
                    2, 3
                ]);

        graph01.walk(0);

        println!("simple multi: \n{:#?}", graph01.path_to(3));
        graph01.cut_connection(2, 3);
        graph01.walk(0);
        println!("{:#?}", graph01.path_to(3));
        assert_eq!("0 1", "0 1")
    }

    #[test]
    fn star_test() {
        let mut graph01 = graph!(
                nodes: 12,
                gateways: [0],
                connections: [
                    11, 6
                    11, 7
                    11, 5
                    7, 8
                    7, 0
                    8, 9
                    8, 0
                    9, 10
                    9, 0
                    10, 1
                    10, 0
                    1, 2
                    1, 0
                    2, 3
                    2, 0
                    3, 4
                    3, 0
                    4, 5
                    4, 0
                    5, 6
                    5, 0
                    6, 7
                    6, 0
                ]);

        graph01.walk(11);

        println!("star: \n{:#?}", graph01.path_to(10));
        graph01.cut_connection(8, 9);
        graph01.walk(11);
        println!("star: \n{:#?}", graph01.path_to(10));
        assert_eq!("0 1", "0 1")
    }
}
