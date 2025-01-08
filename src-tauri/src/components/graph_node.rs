use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct GraphNode {
    id: String, // SHA1 of the branch/tag object
    name: String,
    commit_sha: String,              // Commit SHA1 the node points to
    node_type: String,               // Node type: "branch" or "tag"
    parent1: Option<Box<GraphNode>>, // First parent, if any
    parent2: Option<Box<GraphNode>>, // Second parent, if any
    children: Vec<GraphNode>,        // Direct descendants  // Closest branch node IDs of child nodes
                                     // timestamp: Option<String>,
                                     // parents: Vec<usize>, // Closest branch node IDs of parent nodes
                                     // num_children: usize,
                                     // firstchild: Option<String>,
}

impl GraphNode {
    fn new(name: String, id: String, commit_sha: String, node_type: String) -> Self {
        GraphNode {
            name,
            id,
            commit_sha,
            node_type,
            parent1: None,
            parent2: None,
            children: vec![],
        }
    }
}
