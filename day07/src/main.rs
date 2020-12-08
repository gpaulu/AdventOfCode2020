use std::collections::HashMap;

use petgraph::prelude::*;

fn main() {
    println!("Hello, world!");
}

fn num_colors_can_contain(rules: &str, color: &str) -> usize {
    todo!()
}

fn parse_rules(rules: &str) -> DiGraph<&str, usize> {
    let mut graph = Graph::default();
    for rule in rules.lines() {
        let mut mapping = rule.split(" bags contain ");
        let first_color = mapping.next().unwrap();
        for child in mapping.next().unwrap().split(", ") {
            if child.starts_with("no other bags") {
                break;
            }
            let first_space = child.find(' ').unwrap();
            println!("Parsing: {}", child);
            let num = child[..first_space].parse::<usize>().unwrap();
            let bag = child.find(" bag").unwrap();
            let child_color = &child[first_space + 1..bag];
            graph.update(first_color, child_color, num);
        }
    }
    graph.graph
}

#[derive(Debug, Default)]
struct Graph<'a> {
    graph: DiGraph<&'a str, usize>,
    node_map: HashMap<&'a str, NodeIndex>,
}

impl<'a> Graph<'a> {
    fn update(&mut self, from: &'a str, to: &'a str, num: usize) {
        println!(
            "Graph::update(from: \"{}\", to \"{}\", num: {}",
            from, to, num
        );
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
        let rules = parse_rules(input);
        assert_eq!(rules.node_count(), g.node_count());
    }
}
