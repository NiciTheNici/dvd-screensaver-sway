use swayipc::{Connection, Fallible, Node};

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    let focused_node = find_focused_node(ipc.get_tree()?.nodes);
    println!("{:#?}", focused_node);
    Ok(())
}

fn find_focused_node(nodes: Vec<Node>) -> Fallible<Option<Node>> {
    for node in nodes {
        match node.focused {
            true => {
                println!("{}", node.name.as_ref().unwrap());
                return Ok(Some(node));
            }
            false => (),
        }
        match find_focused_node(node.nodes)? {
            Some(r) => return Ok(Some(r)),
            None => (),
        };
    }
    Ok(None)
}
