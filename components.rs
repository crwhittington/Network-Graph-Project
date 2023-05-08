pub fn find_significant_components(components: Vec<Vec<usize>>, total_nodes: usize) -> Vec<Vec<usize>> {
    let mut significant_components: Vec<Vec<usize>> = components
        .into_iter()
        .filter(|component| component.len() > (0.01 * total_nodes as f64).round() as usize)
        .collect();
    significant_components.sort_by_key(|component| -(component.len() as i32));

    println!("Number of significant components: {}", significant_components.len());
    for component in &significant_components {
        let component_size = component.len();
        let percent = ((component_size as f64 / total_nodes as f64) * 10000.0).round() / 100.0;
        println!("Component size: {} ({}%)", component_size, percent);
    }

    significant_components
}

pub fn find_large_components(components: &Vec<Vec<usize>>, total_nodes: usize, threshold: usize) -> Vec<Vec<usize>> {
    let mut large_components: Vec<Vec<usize>> = components.iter()
        .filter(|&component| component.len() > threshold)
        .cloned()
        .collect();
    large_components.sort_by_key(|component| -(component.len() as i32));

    println!("Number of large components: {}", large_components.len());
    for component in &large_components {
        let component_size = component.len();
        let percent = ((component_size as f64 / total_nodes as f64) * 10000.0).round() / 100.0;
        println!("Component size: {} ({}%)", component_size, percent);
    }
    
    large_components
}


