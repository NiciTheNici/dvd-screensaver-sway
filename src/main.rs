use std::{thread, time};
use swayipc::{Connection, Error, Fallible, Node, NodeType};

#[derive(Debug, PartialEq)]
enum Direction {
    NE,
    SE,
    SW,
    NW,
}

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    let mut direction = Direction::NW;

    ipc.run_command("floating on")?;
    loop {
        direction = get_next_direction(&mut ipc, direction)?;
        move_focused_window(&mut ipc, &direction);
        thread::sleep(time::Duration::from_millis(7));
    }
}

fn find_focused_node(nodes: Vec<Node>, floating_nodes: Vec<Node>) -> Fallible<Option<Node>> {
    let all_nodes = [nodes.as_slice(), floating_nodes.as_slice()].concat();

    for node in all_nodes {
        if node.focused && node.node_type != NodeType::Workspace {
            return Ok(Some(node));
        }
        match find_focused_node(node.nodes, node.floating_nodes)? {
            Some(r) => return Ok(Some(r)),
            None => (),
        };
    }

    Ok(None)
}

fn get_next_direction(ipc: &mut Connection, old_direction: Direction) -> Fallible<Direction> {
    let focused_node: Node =
        match find_focused_node(ipc.get_tree()?.nodes, ipc.get_tree()?.floating_nodes)? {
            Some(r) => r,
            None => {
                return Err(Error::CommandFailed(
                    "Could not find focused node".to_string(),
                ))
            }
        };
    let display_dimensions = ipc.get_tree()?.rect;

    match old_direction {
        Direction::NE => {
            if focused_node.rect.x + focused_node.rect.width >= display_dimensions.width {
                return Ok(Direction::NW);
            }
            if focused_node.rect.y <= 0 {
                return Ok(Direction::SE);
            }
        }

        Direction::SE => {
            if focused_node.rect.x + focused_node.rect.width >= display_dimensions.width {
                return Ok(Direction::SW);
            }
            if focused_node.rect.y + focused_node.rect.height >= display_dimensions.height {
                return Ok(Direction::NE);
            }
        }

        Direction::SW => {
            if focused_node.rect.x <= 0 {
                return Ok(Direction::SE);
            }
            if focused_node.rect.y + focused_node.rect.height >= display_dimensions.height {
                return Ok(Direction::NW);
            }
        }

        Direction::NW => {
            if focused_node.rect.x <= 0 {
                return Ok(Direction::NE);
            }
            if focused_node.rect.y <= 0 {
                return Ok(Direction::SW);
            }
        }
    }

    Ok(old_direction)
}

fn move_focused_window(ipc: &mut Connection, direction: &Direction) {
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
