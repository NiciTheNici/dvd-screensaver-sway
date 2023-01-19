use std::thread::sleep_ms;

use swayipc::{Connection, Error, Fallible, Node, NodeType};

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    // move_window(&mut ipc, Direction::NW);
    let mut direction = Direction::NE;
    loop {
        direction = get_next_direction(&mut ipc, direction)?;
        println!("{:#?}", direction);
        move_window(&mut ipc, &direction);
        sleep_ms(100);
    }
}

fn find_focused_node(nodes: Vec<Node>, floating_nodes: Vec<Node>) -> Fallible<Option<Node>> {
    let all_nodes = [nodes.as_slice(), floating_nodes.as_slice()].concat();

    for node in all_nodes {
        if node.focused && node.node_type != NodeType::Workspace {
            println!("{}", node.name.as_ref().unwrap());
            println!("{:#?}", node.rect);
            return Ok(Some(node));
        }
        match find_focused_node(node.nodes, node.floating_nodes)? {
            Some(r) => return Ok(Some(r)),
            None => (),
        };
    }
    Ok(None)
}

#[derive(Debug, PartialEq)]
enum Direction {
    NE,
    SE,
    SW,
    NW,
}

fn get_next_direction(ipc: &mut Connection, old_direction: Direction) -> Fallible<Direction> {
    let focused_node =
        find_focused_node(ipc.get_tree()?.nodes, ipc.get_tree()?.floating_nodes)?.unwrap();
    let display_dimensions = ipc.get_tree()?.rect;
    println!("AAAAAAAAAA{:?}", focused_node.rect);
    if focused_node.rect.y <= 0 && old_direction == Direction::NE {
        return Ok(Direction::SE);
    }
    // println!("{:#?}", display_dimensions);
    Ok(old_direction)
}

fn move_window(ipc: &mut Connection, direction: &Direction) {
    match direction {
        Direction::NE => {
            _ = ipc.run_command(format!("move up {}", 1));
            _ = ipc.run_command(format!("move right {}", 1));
        }
        Direction::SE => {
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
