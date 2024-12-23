extern crate filelib;
use std::collections::{HashMap, HashSet};

pub use filelib::load_no_blanks;
use itertools::Itertools;
use log::info;

type Node<'a> = &'a str;
type AdjacencyGraph<'a> = HashMap<Node<'a>, HashSet<Node<'a>>>;

fn parse_connections(string_list: &Vec<String>) -> AdjacencyGraph {
    let mut graph = AdjacencyGraph::new();

    for line in string_list {
        let (node_a, node_b) = line.split_once("-").unwrap();
        graph.entry(node_a).or_insert(HashSet::new()).insert(node_b);
        graph.entry(node_b).or_insert(HashSet::new()).insert(node_a);
    }
    return graph;
}

fn find_triangles<'a>(graph: &AdjacencyGraph<'a>) -> HashSet<Vec<Node<'a>>> {
    let mut triangles: HashSet<Vec<Node<'a>>> = HashSet::new();
    for node in graph.keys() {
        let neighbors = &graph[*node];

        for neighbor_pair in neighbors.iter().combinations(2) {
            let neighbor_a = *neighbor_pair.first().unwrap();
            let neighbor_b = *neighbor_pair.last().unwrap();
            if graph[neighbor_a].contains(neighbor_b) {
                let mut triangle = vec![*node, *neighbor_a, *neighbor_b];
                triangle.sort();
                info!("triangle found: {:?}", triangle);
                triangles.insert(triangle);
            }
        }
    }
    return triangles;
}

/// Find triplets of connected computers that start with t
/// ```
/// let vec1: Vec<String> = vec![
///     "kh-tc",
///     "qp-kh",
///     "de-cg",
///     "ka-co",
///     "yn-aq",
///     "qp-ub",
///     "cg-tb",
///     "vc-aq",
///     "tb-ka",
///     "wh-tc",
///     "yn-cg",
///     "kh-ub",
///     "ta-co",
///     "de-co",
///     "tc-td",
///     "tb-wq",
///     "wh-td",
///     "ta-ka",
///     "td-qp",
///     "aq-cg",
///     "wq-ub",
///     "ub-vc",
///     "de-ta",
///     "wq-aq",
///     "wq-vc",
///     "wh-yn",
///     "ka-de",
///     "kh-ta",
///     "co-tc",
///     "wh-qp",
///     "tb-vc",
///     "td-yn"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_a(&vec1), 7);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let connections = parse_connections(string_list);
    let triangles = find_triangles(&connections);
    let filtered: Vec<_> = triangles
        .into_iter()
        .filter(|nodes| nodes.iter().find(|element| element.starts_with("t")) != None)
        .collect();
    return filtered.len();
}

fn bron_kerbosch<'a>(
    result: &mut HashSet<Node<'a>>,
    potential: &mut HashSet<Node<'a>>,
    excluded: &mut HashSet<Node<'a>>,
    graph: &AdjacencyGraph<'a>,
    cliques: &mut Vec<HashSet<Node<'a>>>,
) {
    if potential.is_empty() && excluded.is_empty() {
        cliques.push(result.clone());
    }

    while let Some(u) = potential.iter().copied().next() {
        let u_neighbors: HashSet<Node> = graph[u].clone();
        let mut p_intersect_u_neighbors: HashSet<Node> =
            potential.intersection(&u_neighbors).copied().collect();
        let mut x_intersect_u_neighbors: HashSet<Node> =
            excluded.intersection(&u_neighbors).copied().collect();
        result.insert(u);

        bron_kerbosch(
            result,
            &mut p_intersect_u_neighbors,
            &mut x_intersect_u_neighbors,
            graph,
            cliques,
        );

        result.remove(u);
        potential.remove(u);
        excluded.insert(u);
    }
}

fn find_all_cliques<'a>(graph: &AdjacencyGraph<'a>) -> Vec<HashSet<Node<'a>>> {
    let mut cliques: Vec<HashSet<Node>> = Vec::new();
    let mut all_nodes: HashSet<Node> = graph.keys().map(|x| *x).collect();
    let mut result = HashSet::new();
    let mut excluded = HashSet::new();
    bron_kerbosch(
        &mut result,
        &mut all_nodes,
        &mut excluded,
        graph,
        &mut cliques,
    );
    return cliques;
}

/// Find largest combination that are all connected to each other.
/// ```
/// let vec1: Vec<String> = vec![
///     "kh-tc",
///     "qp-kh",
///     "de-cg",
///     "ka-co",
///     "yn-aq",
///     "qp-ub",
///     "cg-tb",
///     "vc-aq",
///     "tb-ka",
///     "wh-tc",
///     "yn-cg",
///     "kh-ub",
///     "ta-co",
///     "de-co",
///     "tc-td",
///     "tb-wq",
///     "wh-td",
///     "ta-ka",
///     "td-qp",
///     "aq-cg",
///     "wq-ub",
///     "ub-vc",
///     "de-ta",
///     "wq-aq",
///     "wq-vc",
///     "wh-yn",
///     "ka-de",
///     "kh-ta",
///     "co-tc",
///     "wh-qp",
///     "tb-vc",
///     "td-yn"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_b(&vec1), "co,de,ka,ta");
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> String {
    let connections = parse_connections(string_list);
    let all_cliques = find_all_cliques(&connections);
    let mut max_clique: Vec<&str> = all_cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .into_iter()
        .cloned()
        .collect();
    max_clique.sort();
    return max_clique.join(",");
}
