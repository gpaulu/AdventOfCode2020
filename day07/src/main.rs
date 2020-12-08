use std::{collections::HashMap, fs::read_to_string};

use petgraph::{prelude::*, visit::Walker};

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");
    let num_can_contain = num_colors_can_contain(&input, "shiny gold");
    println!("{} bags could contain a shiny gold", num_can_contain);
}

fn num_colors_can_contain(rules: &str, color: &str) -> usize {
    let graph = parse_rules(rules);
    let start = graph.get_index(color);
    petgraph::visit::Bfs::new(&graph.graph, start)
        .iter(petgraph::visit::Reversed(&graph.graph))
        .skip(1) // first is our starting node
        .count()
}

fn num_total_bags_inside(rules: &str, root: &str) -> usize {
    let graph = parse_rules(rules);
    let start = graph.get_index(root);
    sum_bags(&graph.graph, start) - 1 //subtract 1 to not include the root bag
}

fn sum_bags(graph: &DiGraph<&str, usize>, node: NodeIndex) -> usize {
    1 + graph // add 1 to include `node`
        .edges(node)
        .map(|edge| edge.weight() * sum_bags(graph, edge.target()))
        .sum::<usize>()
}

fn parse_rules(rules: &str) -> Graph {
    let mut graph = Graph::default();
    for rule in rules.lines() {
        let mut mapping = rule.split(" bags contain ");
        let first_color = mapping.next().unwrap();
        for child in mapping.next().unwrap().split(", ") {
            if child.starts_with("no other bags") {
                break;
            }
            let first_space = child.find(' ').unwrap();
            let num = child[..first_space].parse::<usize>().unwrap();
            let bag = child.find(" bag").unwrap();
            let child_color = &child[first_space + 1..bag];
            graph.update(first_color, child_color, num);
        }
    }
    graph
}

#[derive(Debug, Default)]
struct Graph<'a> {
    graph: DiGraph<&'a str, usize>,
    node_map: HashMap<&'a str, NodeIndex>,
}

impl<'a> Graph<'a> {
    fn update(&mut self, from: &'a str, to: &'a str, num: usize) {
        let index1 = self.insert_node(from);
        let index2 = self.insert_node(to);
        self.graph.add_edge(index1, index2, num);
    }
    fn insert_node(&mut self, node: &'a str) -> NodeIndex {
        let graph_ref = &mut self.graph;
        *self
            .node_map
            .entry(node)
            .or_insert_with(|| graph_ref.add_node(node))
    }
    fn get_index(&self, node: &'a str) -> NodeIndex {
        self.node_map[node]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(num_colors_can_contain(input, "shiny gold"), 4);
    }
    #[test]
    fn rules_parse_to_graph() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain no other bags.
muted yellow bags contain no other bags.";
        let mut g = DiGraph::<&str, usize>::new();
        let lr = g.add_node("light red");
        let by = g.add_node("bright yellow");
        let my = g.add_node("muted yellow");
        g.extend_with_edges(&[(lr, by, 1), (lr, my, 2)]);
        let rules = parse_rules(input).graph;
        assert_eq!(rules.node_count(), g.node_count());
    }

    #[test]
    fn part2_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(num_total_bags_inside(input, "shiny gold"), 32);
    }
    #[test]
    fn part2_example2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(num_total_bags_inside(input, "shiny gold"), 126);
    }
}
