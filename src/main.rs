use swayipc::{Connection, Fallible, Node};

fn main() -> Fallible<()> {
    let mut ipc = Connection::new()?;
    find_focused(&ipc.get_tree()?.nodes)?;
    Ok(())
}

fn find_focused(nodes: &Vec<Node>) -> Fallible<()> {
    for node in nodes {
        match node.focused {
            true => println!("{}", node.name.as_ref().unwrap()),
            false => (),
        }
        find_focused(&node.nodes)?;
    }
    Ok(())
}
