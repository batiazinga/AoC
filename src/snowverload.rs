use std::collections::HashMap;

mod graph {
    use rand::Rng;
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub struct Contractable {
        vertices: Vec<u32>,
        edges: Vec<HashMap<usize, u32>>,
    }

    impl Contractable {
        pub fn new() -> Contractable {
            Contractable {
                vertices: Vec::new(),
                edges: Vec::new(),
            }
        }

        pub fn create_vertex(&mut self) -> usize {
            self.vertices.push(1);
            self.edges.push(HashMap::new());

            self.vertices.len() - 1
        }

        pub fn connect(&mut self, x: usize, y: usize) {
            self.edges[x].insert(y, 1);
            self.edges[y].insert(x, 1);
        }

        pub fn num_vertices(&self) -> usize {
            let mut size = 0;
            for multiplicity in self.vertices.iter() {
                if multiplicity > &0 {
                    size += 1;
                }
            }
            size
        }

        fn vertices(&self) -> impl Iterator<Item = usize> + '_ {
            self.vertices.iter().enumerate().filter_map(|v| {
                if v.1 != &0 {
                    Some(v.0.clone())
                } else {
                    None
                }
            })
        }

        fn pick_vertex(&self) -> usize {
            let index = rand::thread_rng().gen_range(0..self.num_vertices());
            let mut count = 0usize;
            for i in 0..self.vertices.len() {
                if self.vertices[i] == 0 {
                    continue;
                }
                if count == index {
                    return i;
                }
                count += 1;
            }
            panic!("failed to pick a vertex");
        }

        fn vertex_multiplicity(&self, v: usize) -> u32 {
            self.vertices[v]
        }

        fn next_vertices(&self, v: usize) -> impl Iterator<Item = &usize> {
            self.edges[v].keys()
        }

        fn edge_multiplicity(&self, x: usize, y: usize) -> u32 {
            if let Some(multiplicity) = self.edges[x].get(&y) {
                return *multiplicity;
            }
            0
        }

        fn contract(&mut self, x: usize, y: usize) {
            if !self.edges[x].contains_key(&y) {
                panic!("can only contract neighbour vertices");
            }

            let (v, old) = if x < y { (x, y) } else { (y, x) };

            let old_multiplicity = self.vertices[old];
            self.vertices[old] = 0;
            self.vertices[v] += old_multiplicity;

            let old_neighbours = self.edges[old].clone();
            self.edges[old].clear();

            for neighbour in old_neighbours.iter() {
                let old_multiplicity = self.edges[*neighbour.0].remove(&old).unwrap();
                if neighbour.0 == &v {
                    continue;
                }
                let mut multiplicity = old_multiplicity;
                if let Some(v_multiplicity) = self.edges[*neighbour.0].get(&v) {
                    multiplicity += v_multiplicity;
                }
                self.edges[*neighbour.0].insert(v, multiplicity);
                self.edges[v].insert(*neighbour.0, multiplicity);
            }
        }
    }

    pub struct Cut {
        size: u32,
        pub size_left: u32,
        pub size_right: u32,
    }

    pub fn cut(g: &mut Contractable, size: u32) -> Option<Cut> {
        while g.num_vertices() > 1 {
            let v = g.pick_vertex();
            let (cut, x, y) = min_cut_from_seed(g, v);
            println!("from {}, cut at {} <-> {}, with cost {}", v, x, y, cut.size);
            g.contract(x, y);

            if cut.size == size {
                return Some(cut);
            }
        }
        None
    }

    fn min_cut_from_seed(g: &Contractable, seed: usize) -> (Cut, usize, usize) {
        let mut a: Vec<usize> = Vec::with_capacity(g.num_vertices() - 1);
        a.push(seed);

        let mut outside_a: HashSet<usize> = HashSet::from_iter(g.vertices());
        outside_a.remove(&seed);

        while outside_a.len() > 1 {
            let most_tightly_coupled = most_tightly_coupled(&a, &outside_a, g);
            a.push(most_tightly_coupled);
            outside_a.remove(&most_tightly_coupled);
        }

        let remaining = outside_a.iter().next().unwrap();
        let coupling = a.iter().map(|v| g.edge_multiplicity(*v, *remaining)).sum();

        let mut last_added_index = a.len() - 1;
        let mut last_added = a[last_added_index];
        while g.edge_multiplicity(last_added, *remaining) == 0 {
            last_added_index -= 1;
            last_added = a[last_added_index];
        }

        (
            Cut {
                size: coupling,
                size_left: a.iter().map(|v| g.vertex_multiplicity(*v)).sum(),
                size_right: g.vertex_multiplicity(*remaining),
            },
            last_added,
            *remaining,
        )
    }

    fn most_tightly_coupled(a: &Vec<usize>, outside_a: &HashSet<usize>, g: &Contractable) -> usize {
        let mut outside_a_items = outside_a.iter();
        let mut most_tightly_coupled = outside_a_items.next().unwrap();
        let mut coupling: u32 = a
            .iter()
            .map(|v| g.edge_multiplicity(*most_tightly_coupled, *v))
            .sum();
        for item in outside_a_items {
            let new_coupling = a.iter().map(|v| g.edge_multiplicity(*item, *v)).sum();
            if new_coupling > coupling {
                coupling = new_coupling;
                most_tightly_coupled = item;
            }
        }

        *most_tightly_coupled
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_empty_graph() {
            let g = Contractable::new();
            assert_eq!(g.num_vertices(), 0);
        }

        #[test]
        fn test_graph_with_isolated_vertices() {
            let mut g = Contractable::new();
            g.create_vertex();
            g.create_vertex();
            g.create_vertex();
            assert_eq!(g.num_vertices(), 3);
        }

        #[test]
        fn test_graph_with_edges() {
            let mut g = Contractable::new();
            let v0 = g.create_vertex();
            let v1 = g.create_vertex();
            let v2 = g.create_vertex();
            let v3 = g.create_vertex();
            g.connect(v0, v1);
            g.connect(v1, v2);
            g.connect(v0, v3);
            g.connect(v2, v3);
            g.connect(v1, v3);

            let v0_neighbours = Vec::from_iter(g.next_vertices(v0));
            assert_eq!(v0_neighbours.len(), 2);

            let v1_neighbours = Vec::from_iter(g.next_vertices(v1));
            assert_eq!(v1_neighbours.len(), 3);

            let v2_neighbours = Vec::from_iter(g.next_vertices(v2));
            assert_eq!(v2_neighbours.len(), 2);

            let v3_neighbours = Vec::from_iter(g.next_vertices(v3));
            assert_eq!(v3_neighbours.len(), 3);
        }

        #[test]
        fn test_multiplicity_of_original_vertices() {
            let mut g = Contractable::new();
            let v = g.create_vertex();
            assert_eq!(g.vertex_multiplicity(v), 1);
        }

        #[test]
        fn test_multiplicity_of_original_edges() {
            let mut g = Contractable::new();
            let v0 = g.create_vertex();
            let v1 = g.create_vertex();
            g.connect(v0, v1);
            assert_eq!(g.edge_multiplicity(v0, v1), 1);
        }

        #[test]
        fn test_contract() {
            let mut g = Contractable::new();
            let v0 = g.create_vertex();
            let v1 = g.create_vertex();
            let v2 = g.create_vertex();
            let v3 = g.create_vertex();
            let v4 = g.create_vertex();
            g.connect(v0, v1);
            g.connect(v1, v2);
            g.connect(v0, v3);
            g.connect(v2, v3);
            g.connect(v1, v3);
            g.connect(v3, v4);

            g.contract(v1, v3);

            assert_eq!(g.num_vertices(), 4);

            let v0_neighbours = Vec::from_iter(g.next_vertices(v0));
            assert_eq!(v0_neighbours.len(), 1);

            assert_eq!(g.vertex_multiplicity(v1), 2);
            let v1_neighbours = Vec::from_iter(g.next_vertices(v1));
            assert_eq!(v1_neighbours.len(), 3);
            assert_eq!(g.edge_multiplicity(v0, v1), 2);
            assert_eq!(g.edge_multiplicity(v1, v2), 2);
            assert_eq!(g.edge_multiplicity(v1, v0), 2);
            assert_eq!(g.edge_multiplicity(v2, v1), 2);
            assert_eq!(g.edge_multiplicity(v1, v4), 1);
            assert_eq!(g.edge_multiplicity(v4, v1), 1);

            let v2_neighbours = Vec::from_iter(g.next_vertices(v2));
            assert_eq!(v2_neighbours.len(), 1);
        }

        #[test]
        fn test_pick_vertex() {
            let mut g = Contractable::new();
            let v0 = g.create_vertex();
            let v1 = g.create_vertex();
            let v2 = g.create_vertex();
            let v3 = g.create_vertex();
            g.connect(v0, v1);
            g.connect(v1, v2);
            g.connect(v0, v3);
            g.connect(v2, v3);
            g.connect(v1, v3);

            g.pick_vertex();
        }

        #[test]
        fn test_cut_1() {
            let mut g = Contractable::new();

            let v00 = g.create_vertex();
            let v01 = g.create_vertex();
            let v02 = g.create_vertex();
            let v03 = g.create_vertex();
            g.connect(v00, v01);
            g.connect(v00, v02);
            g.connect(v00, v03);
            g.connect(v01, v02);
            g.connect(v01, v03);
            g.connect(v02, v03);

            let v10 = g.create_vertex();
            let v11 = g.create_vertex();
            let v12 = g.create_vertex();
            let v13 = g.create_vertex();
            g.connect(v10, v11);
            g.connect(v10, v12);
            g.connect(v10, v13);
            g.connect(v11, v12);
            g.connect(v11, v13);
            g.connect(v12, v13);

            g.connect(v00, v10);

            let c = cut(&mut g, 1).unwrap();
            assert_eq!(c.size, 1);
            assert_eq!(c.size_left, 4);
            assert_eq!(c.size_right, 4);
        }
    }
}

pub fn read_graph(input: &str) -> graph::Contractable {
    let mut g = graph::Contractable::new();

    let mut vertices: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let column_index = line.find(':').unwrap();

        let str_v = String::from(&line[..column_index]);
        let v = if let Some(i) = vertices.get(&str_v) {
            *i
        } else {
            let i = g.create_vertex();
            vertices.insert(str_v, i);
            i
        };

        for str_neighbour in line[column_index + 1..]
            .split_whitespace()
            .map(|s| String::from(s))
        {
            let neighbour = if let Some(i) = vertices.get(&str_neighbour) {
                *i
            } else {
                let i = g.create_vertex();
                vertices.insert(str_neighbour, i);
                i
            };
            g.connect(v, neighbour);
        }
    }

    g
}

pub fn cut_product_size(g: &mut graph::Contractable) -> u32 {
    let cut = graph::cut(g, 3).unwrap();
    return cut.size_left * cut.size_right;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_read_graph() {
        let g = read_graph(&INPUT);
        assert_eq!(g.num_vertices(), 15);
    }

    #[test]
    fn test_cut() {
        let mut g = read_graph(&INPUT);
        assert_eq!(cut_product_size(&mut g), 54);
    }
}
