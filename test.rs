mod graph;
mod kosaraju;
mod components;

#[cfg(test)]
mod test {
    use super::*;
  #[test]
  #[should_panic]
  fn test_graph_builder_empty() {
    let filename = "empty.txt";
    let graph = graph::graph_builder(filename);
    assert_eq!(graph.len(), 0);
  }  
  #[test]

fn test_graph_builder_full() {
    let filename = "data.txt";
    let graph = graph::graph_builder(filename);
    assert_eq!(graph.len(), 15);
}
  #[test]
 fn test_kosaraju_components() {
    let filename = "data.txt";
    let graph = graph::graph_builder(filename);
    let components = kosaraju::kosaraju(&graph);
    let truth = 4;
    assert_eq!(components.len(), truth);
 }
 
 #[test]
 fn test_kosaraju_contains() {
    let filename = "data.txt";
    let graph = graph::graph_builder(filename);
    let components = kosaraju::kosaraju(&graph);
    assert!(components.contains(&vec![12]));
    assert!(components.contains(&vec![13]));
    assert!(components.contains(&vec![14]));
 }

 #[test]
 fn test_find_signficiant_components() {
    let filename = "data.txt";
    let graph = graph::graph_builder(filename);
    let components = kosaraju::kosaraju(&graph);
    let total_nodes = 15;
    let significant = components::find_significant_components(components, total_nodes);
    let truth = 4;
    let result = significant.len();
    assert_eq!(result, truth);
 }

 #[test]
 fn test_find_large_components() {
    let filename = "data.txt";
    let graph = graph::graph_builder(filename);
    let components = kosaraju::kosaraju(&graph);
    let total_nodes = 15;
    let threshold = 2;
    let large = components::find_large_components(&components, total_nodes, threshold);
    let truth = 1;
    let result = large.len();
    assert_eq!(result, truth);
 }


}