use swayipc::{Connection, Error, Fallible, Node, NodeType};

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    let focused_node =
        find_focused_node(ipc.get_tree()?.nodes, ipc.get_tree()?.floating_nodes)?.unwrap();
    println!("{:#?}", focused_node.node_type);
    float_node(&mut ipc)?;
    move_window(&mut ipc, Direction::NW);
    println!("{:#?}", focused_node.node_type);
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

enum Direction {
    NE,
    ES,
    SW,
    NW,
}

fn move_window(ipc: &mut Connection, direction: Direction) {
    match direction {
        Direction::NE => {
            _ = ipc.run_command(format!("move up {}", 1));
            _ = ipc.run_command(format!("move right {}", 1));
        }
        Direction::ES => {
            _ = ipc.run_command(format!("move up {}", -1));
            _ = ipc.run_command(format!("move right {}", 1));
        }
        Direction::SW => {
            _ = ipc.run_command(format!("move up {}", -1));
            _ = ipc.run_command(format!("move right {}", -1));
        }
        Direction::NW => {
            _ = ipc.run_command(format!("move up {}", 1));
            _ = ipc.run_command(format!("move right {}", -1));
        }
    }
}

fn float_node(ipc: &mut Connection) -> Result<Vec<Result<(), Error>>, Error> {
    ipc.run_command("floating on")
}
