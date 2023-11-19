use std::{env, thread, time};
use swayipc::{Connection, Error, Fallible, Node, NodeType, Rect};

#[derive(Debug, PartialEq)]
enum Direction {
    NE,
    SE,
    SW,
    NW,
}

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    let mut direction = Direction::NE;

    let display_dimensions = ipc.get_tree()?.rect;
    ipc.run_command("floating on")?;
    loop {
        direction = get_next_direction(&mut ipc, direction, display_dimensions)?;
        move_focused_window(&mut ipc, &direction);
        thread::sleep(time::Duration::from_millis(get_speed_from_args()));
    }
}

fn get_speed_from_args() -> u64 {
    let args: Vec<String> = env::args().collect();
    let mut speed: u64 = 7;
    match args.len() {
        2.. => match args[1].parse::<u64>() {
            Ok(r) => speed = r,
            Err(_) => {}
        },
        _ => (),
    };
    speed
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

fn get_next_direction(
    ipc: &mut Connection,
    old_direction: Direction,
    display_dimensions: Rect,
) -> Fallible<Direction> {
    let focused_node: Node =
        match find_focused_node(ipc.get_tree()?.nodes, ipc.get_tree()?.floating_nodes)? {
            Some(r) => r,
            None => {
                return Err(Error::CommandFailed(
                    "Could not find focused node".to_string(),
                ))
            }
        };
    println!("{:?}", old_direction);
    println!("{:?}", focused_node.rect);
    println!("{:?}", display_dimensions);
    println!(
        "{}, {}",
        focused_node.rect.x - display_dimensions.x,
        focused_node.rect.y - display_dimensions.y
    );

    match old_direction {
        Direction::NE => {
            if focused_node.rect.x + focused_node.rect.width - display_dimensions.x
                > display_dimensions.width
            {
                return Ok(Direction::NW);
            }
            if focused_node.rect.y < display_dimensions.y {
                return Ok(Direction::SE);
            }
        }

        Direction::SE => {
            if focused_node.rect.x + focused_node.rect.width - display_dimensions.x
                > display_dimensions.width
            {
                return Ok(Direction::SW);
            }
            if focused_node.rect.y + focused_node.rect.height - display_dimensions.y
                > display_dimensions.height
            {
                return Ok(Direction::NE);
            }
        }

        Direction::SW => {
            if focused_node.rect.x < display_dimensions.x {
                return Ok(Direction::SE);
            }
            if focused_node.rect.y + focused_node.rect.height - display_dimensions.height
                > display_dimensions.y
            {
                return Ok(Direction::NW);
            }
        }

        Direction::NW => {
            if focused_node.rect.x <= display_dimensions.x {
                return Ok(Direction::NE);
            }
            if focused_node.rect.y <= display_dimensions.y {
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
