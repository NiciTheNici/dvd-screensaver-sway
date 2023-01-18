use swayipc::{Connection, Fallible, Node, NodeType};

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    let focused_node = find_focused_node(ipc.get_tree()?.nodes, ipc.get_tree()?.floating_nodes);
    println!("{:#?}", focused_node);
    Ok(())
}

fn find_focused_node(nodes: Vec<Node>, floating_nodes: Vec<Node>) -> Fallible<Option<Node>> {
    let all_nodes = [nodes.as_slice(), floating_nodes.as_slice()].concat();

    for node in all_nodes {
        if node.focused && node.node_type != NodeType::Workspace {
            println!("{}", node.name.as_ref().unwrap());
            return Ok(Some(node));
        }
        match find_focused_node(node.nodes, node.floating_nodes)? {
            Some(r) => return Ok(Some(r)),
            None => (),
        };
    }
    Ok(None)
}
