mod graph;
mod kosaraju;
mod components;

fn main() {
    let stanford= "stanford.txt";
    let stanford_graph = graph::graph_builder(stanford);
    let stanford_total_nodes = stanford_graph.len();
    let stanford_components = kosaraju::kosaraju(&stanford_graph);
    let stanford_length = stanford_components.len();
    println!("Number of total Stanford nodes: {}", stanford_total_nodes);
    println!("Number of total Stanford strongly connected components: {}", stanford_length);

    let stanford_threshold = 100;
    
   components::find_large_components(&stanford_components, stanford_total_nodes, stanford_threshold);
   components::find_significant_components(stanford_components, stanford_total_nodes);

    let google = "google.txt";
    let google_graph = graph::graph_builder(google);
    let google_total_nodes = google_graph.len();
    let google_components = kosaraju::kosaraju(&google_graph);
    let google_length = google_components.len();
    println!("Number of total Google nodes: {}", google_total_nodes);
    println!("Number of total Google strongly connected components: {}", google_length);

    let google_threshold = 100;
    
   components::find_large_components(&google_components, google_total_nodes, google_threshold);
   components::find_significant_components(google_components, google_total_nodes);

}
