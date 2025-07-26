use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

// Structs
#[derive(Debug)]
struct Node {
    label: String,
    links: Vec<Rc<RefCell<Link>>>,
}

#[derive(Debug)]
struct Link {
    label: String,
    node: Rc<RefCell<Node>>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

// Impl
impl Node {
    fn new(label: &str) -> Self {
        Self {
            label: label.into(),
            links: vec![],
        }
    }

    fn add_link(&mut self, predicate: &str, node: Rc<RefCell<Node>>) {
        let link = Link::new(predicate, node);
        self.links.push(Rc::new(RefCell::new(link)));
    }

    fn basic_search(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];

        for link in &self.links {
            strs.push(format!("{}", link.borrow().label.clone()))
        }

        strs
    }

    fn advanced_search(&self) -> Vec<String> {
        let mut strs: Vec<String> = vec![];

        for link in &self.links {
            strs.push(format!(
                "{} {}",
                link.borrow().label.clone(),
                link.borrow().node.borrow().label.clone()
            ))
        }

        strs
    }
}

impl Link {
    fn new(label: &str, node: Rc<RefCell<Node>>) -> Self {
        Self {
            label: label.into(),
            node,
        }
    }
}

impl Graph {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn add_node(&mut self, label: &str) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node::new(label)));
        self.nodes.push(Rc::clone(&node));
        node
    }

    fn get_node(&self, label: &str) -> Option<Rc<RefCell<Node>>> {
        self.nodes
            .iter()
            .find(|n| n.borrow().label == label)
            .cloned()
    }

    fn link(&mut self, obj: &str, relation: &str, subj: &str) {
        let node_1 = self.get_or_create_node(obj);
        let node_2 = self.get_or_create_node(subj);
        node_1.borrow_mut().add_link(relation, node_2);
    }

    fn get_or_create_node(&mut self, label: &str) -> Rc<RefCell<Node>> {
        self.get_node(label).unwrap_or_else(|| self.add_node(label))
    }
}

impl Into<String> for Graph {
    fn into(self) -> String {
        let mut strs: Vec<String> = Vec::new();

        for node in &self.nodes {
            for link in &node.borrow().links {
                let link = link.borrow();

                strs.push(format!(
                    "a {} {} {}",
                    node.borrow().label,
                    link.label.replace('-', " "),
                    link.node.borrow().label
                ));
            }
        }

        strs.join(". ")
    }
}

impl Into<Graph> for String {
    fn into(self) -> Graph {
        let mut graph = Graph::new();
        let sentences = self.trim().split(". ").collect::<Vec<_>>();

        for sentence in sentences {
            let words: Vec<&str> = sentence.split_whitespace().collect();

            if words.len() != 4 || words[0] != "a" {
                continue;
            }

            let subj = words[1];
            let rel = words[2].replace(' ', "-");
            let target = words[3];

            graph.link(subj, &rel, target);
        }

        graph
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test Case 1: Knoten "Katze" wird erfolgreich erstellt
    #[test]
    fn test_1_create_node() {
        let mut graph = Graph::new();
        let node = graph.add_node("katze");

        assert_eq!(node.borrow().label, "katze");
        assert!(graph.get_node("katze").is_some());
    }

    // Test Case 2: Kante "jagt" zwischen Knoten "Katze" und "Maus" erstellt
    #[test]
    fn test_2_create_edge() {
        let mut graph = Graph::new();
        graph.link("katze", "jagt", "maus");

        let katze = graph.get_node("katze").unwrap();
        let links = &katze.borrow().links;

        assert_eq!(links.len(), 1);

        let link = links[0].borrow();

        assert_eq!(link.label, "jagt");
        assert_eq!(link.node.borrow().label, "maus");
    }

    // Test Case 3: Knoten "Maus" wird gefunden
    #[test]
    fn test_3_find_node() {
        let mut graph = Graph::new();
        graph.add_node("maus");

        let result = graph.get_node("maus");

        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().label, "maus");
    }

    // Test Case 4: Verbindungen wie "Katze jagt" werden angezeigt
    #[test]
    fn test_4_basic_search() {
        let mut graph = Graph::new();
        graph.link("katze", "jagt", "maus");
        graph.link("katze", "sieht", "hund");

        let katze = graph.get_node("katze").unwrap();
        let results = katze.borrow().basic_search();

        assert_eq!(results.len(), 2);
        assert!(results.contains(&"jagt".into()));
        assert!(results.contains(&"sieht".into()));
    }

    // Test Case 5: Verbindungen wie "Katze jagt Maus" werden angezeigt
    #[test]
    fn test_5_advanced_search() {
        let mut graph = Graph::new();
        graph.link("katze", "jagt", "maus");
        graph.link("katze", "sieht", "hund");

        let katze = graph.get_node("katze").unwrap();
        let results = katze.borrow().advanced_search();

        assert_eq!(results.len(), 2);
        assert!(results.contains(&"jagt maus".into()));
        assert!(results.contains(&"sieht hund".into()));
    }

    // Test Case 6: Textliche Darstellung des Graphen
    #[test]
    fn test_6_serialize_graph() {
        let mut graph = Graph::new();
        graph.link("katze", "jagt", "maus");
        graph.link("maus", "frisst", "kaese");

        let text: String = graph.into();

        assert!(text.contains("a katze jagt maus"));
        assert!(text.contains("a maus frisst kaese"));
    }

    // Test Case 7: Ursprüngliche Struktur wird korrekt wiederhergestellt
    #[test]
    fn test_7_deserialize_graph() {
        let mut graph = Graph::new();
        graph.link("katze", "jagt", "maus");

        let text: String = graph.into();
        let restored: Graph = text.clone().into();

        let katze = restored.get_node("katze").unwrap();
        let links = &katze.borrow().links;

        assert_eq!(links.len(), 1);

        let link = links[0].borrow();

        assert_eq!(link.label, "jagt");
        assert_eq!(link.node.borrow().label, "maus");
    }

    // Test Case 8: Datei enthält gültige Darstellung
    #[test]
    #[ignore]
    fn test_8_save_graph_to_file() {
        // nothing to test as not implemented
        assert!(false)
    }

    // Test Case 9: Graph wird vollständig und korrekt geladen
    #[test]
    #[ignore]
    fn test_9_read_graph_from_file() {
        // nothing to test as not implemented
        assert!(false)
    }
}
