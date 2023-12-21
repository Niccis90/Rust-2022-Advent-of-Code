use petgraph::{
    graph::{self, Graph},
    visit::Bfs,
    EdgeDirection,
};

mod data;
#[derive(Debug)]
struct Entity {
    name: String,
    size: usize,
    is_folder: bool,
}

fn get_file_graph(string: String) -> Graph<Entity, usize> {
    let commands: Vec<&str> = string.trim_end().split("\n$ ").collect();
    let mut graph = Graph::<Entity, usize>::new();
    let first_node_index = graph.add_node(Entity {
        name: ("/".to_string()),
        size: 0,
        is_folder: true,
    });
    let mut folder_stack = vec![first_node_index];
    let graph = commands[2..commands.len()]
        .iter()
        .fold(graph, |mut graph, line| {
            let current_node_index = folder_stack
                .last()
                .expect("no current node index")
                .to_owned();
            if line.starts_with("cd") {
                let folder = line[3..line.len()].to_string();
                if folder == ".." {
                    folder_stack.pop();
                } else {
                    let new_node_index = graph
                        .neighbors_directed(current_node_index, EdgeDirection::Outgoing)
                        .find(|node_index| {
                            graph
                                .node_weight(*node_index)
                                .expect("Found node with no weight")
                                .name
                                == folder
                        })
                        .expect("couldnt find node index for requested folder {folder}");
                    folder_stack.push(new_node_index);
                }
            }
            if line.starts_with("ls") {
                let split_files = line[3..line.len()].split("\n");
                let mut folder_size: usize = 0;
                for file in split_files {
                    let parts: Vec<&str> = file.split(" ").collect();
                    let entity_name = parts[1].to_string();
                    let is_folder = parts[0] == "dir";
                    let entity_size = if is_folder {
                        0 as usize
                    } else {
                        parts[0]
                            .parse::<usize>()
                            .expect("File size should be number")
                    };
                    let node_index_for_new_entity = graph.add_node(Entity {
                        name: entity_name,
                        size: entity_size,
                        is_folder: is_folder,
                    });
                    graph.add_edge(current_node_index, node_index_for_new_entity, 1);
                    if !is_folder {
                        folder_size += entity_size;
                    }
                }
                folder_stack.iter().for_each(|node_index| {
                    graph
                        .node_weight_mut(*node_index)
                        .expect("node index without weight")
                        .size += folder_size;
                })
            }
            graph
        });
    graph
}

fn part1(graph: &Graph<Entity, usize>) -> usize {
    const MAX_FOLDER_SIZE: usize = 100000;
    let mut bfs = Bfs::new(
        &graph,
        graph.node_indices().nth(0).expect("no 0th index in graph"),
    );

    let mut result: usize = 0;
    while let Some(visited) = bfs.next(&graph) {
        let entity = graph
            .node_weight(visited)
            .expect("found node with no weight");
        if entity.is_folder && entity.size <= MAX_FOLDER_SIZE {
            result += entity.size;
        }
    }
    result
}

fn part2(graph: &Graph<Entity, usize>) -> usize {
    const TOTAL_SIZE: usize = 70000000;
    const NEEDED_SIZE: usize = 30000000;
    let available_space = TOTAL_SIZE
        - graph
            .node_weight(graph.node_indices().nth(0).expect("i dont know"))
            .expect("something")
            .size;
    let minimum_size_to_delete = NEEDED_SIZE - available_space;
    let mut bfs = Bfs::new(&graph, graph.node_indices().nth(0).expect("not found"));
    let mut result: usize = TOTAL_SIZE;
    while let Some(visited) = bfs.next(&graph) {
        let entity = graph.node_weight(visited).expect("no found node");
        if entity.is_folder && entity.size >= minimum_size_to_delete && entity.size < result {
            result = entity.size;
        }
    }
    result
}

fn main() {
    let data1 = data::DATA;
    let data = format!("{}{}", "\n", data1);
    let graph1 = get_file_graph(data);
    let result1 = part1(&graph1);
    let result2 = part2(&graph1);
    println!("{:?}", result2)
}
