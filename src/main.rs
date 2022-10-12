
enum NodeKind {
    Fixed, Argument
}

enum NodeData {
    U8(Option<u8>), F32(Option<f32>), STRING(Option<String>)
}

impl NodeData {
    fn representative_string(&self) -> String {
        match self {
            NodeData::U8(data) => {
                match data {
                    None => String::from("Any u8"),
                    Some(value) => value.to_string()
                }
            },
            NodeData::F32(data) => {
                match data {
                    None => String::from("Any f32"),
                    Some(value) => value.to_string()
                }
            },
            NodeData::STRING(data) => {
                match data {
                    None => String::from("Any String"),
                    Some(value) => value.to_string()
                }
            },
        }
    }
}

struct Node {
    kind: NodeKind,
    data: NodeData,
    nodes: Vec<Node>
}

impl Node {
    fn fixed_u8_new(value: u8) -> Self {
        Self {
            kind: NodeKind::Fixed,
            data: NodeData::U8(Some(value)),
            nodes: Vec::new()
        }
    }

    fn fixed_string_new(value: String) -> Self {
        Self {
            kind: NodeKind::Fixed,
            data: NodeData::STRING(Some(value)),
            nodes: Vec::new()
        }
    }

    fn any_u8_new() -> Self {
        Self {
            kind: NodeKind::Argument,
            data: NodeData::U8(None),
            nodes: Vec::new()
        }
    }

    fn check(&self, input: String) -> Vec<String> {
        if input.is_empty() && self.nodes.is_empty() {
            vec![self.data.representative_string()]
        }
        else {
            match &self.data {
                NodeData::U8(data) => {
                    match self.kind {
                        NodeKind::Fixed => {
                            let stringed = data.unwrap().to_string();
                            match stringed.starts_with(&input) {
                                true => vec![stringed],
                                false => vec![]
                            }
                        },
                        NodeKind::Argument => {
                            match input.parse::<u8>() {
                                Ok(_) => vec![String::from("Any u8")],
                                Err(_) => vec![]
                            }
                        }
                    }
                },
                NodeData::F32(data) => {
                    match self.kind {
                        NodeKind::Fixed => {
                            let stringed = data.unwrap().to_string();
                            match stringed.starts_with(&input) {
                                true => vec![stringed],
                                false => vec![]
                            }
                        },
                        NodeKind::Argument => {
                            match input.parse::<f32>() {
                                Ok(_) => vec![String::from("Any f32")],
                                Err(_) => vec![]
                            }
                        }
                    }
                },
                NodeData::STRING(data) => {
                    match self.kind {
                        NodeKind::Fixed => {
                            let stringed = data.as_ref().unwrap();
                            match stringed.starts_with(&input) {
                                true => vec![stringed.clone()],
                                false => vec![]
                            }
                        },
                        NodeKind::Argument => {
                            match input.parse::<String>() {
                                Ok(_) => vec![String::from("Any String")],
                                Err(_) => vec![]
                            }
                        }
                    }
                }
            }

            /*let mut responses: Vec<String> = Vec::new(); do dis
            for node in &self.nodes {
                responses.extend(node.check(input.clone()))
            }

            responses*/
        }
    }
}

struct Command {
    name: String,
    nodes: Vec<Node>
}

impl Command {
    fn new(name: String) -> Self {
        Self {
            name,
            nodes: Vec::new()
        }
    }

    fn add_node(self, node: Node) -> Self {
        let mut nodes = self.nodes;
        nodes.push(node);

        Self {
            name: self.name,
            nodes
        }
    }

    fn check(&self, input: String) -> Vec<String> {
        let mut responses: Vec<String> = Vec::new();
        for node in &self.nodes {
            responses.extend(node.check(input.clone()))
        }

        responses
    }
}

struct Commander {
    commands: Vec<Command>
}

fn get_until_first_space(input: &String) -> String {
    let space = input.find(" ");
    match space {
        None => input.clone(),
        Some(index) => input[..index].to_string()
    }
}

fn remove_until_first_space(input: &String) -> String {
    let space = input.find(" ");
    match space {
        None => String::new(),
        Some(index) => input[index + 1..].to_string()
    }
}

impl Commander {
    fn new() -> Self {
        Commander {
            commands: Vec::new()
        }
    }

    fn add(&mut self, command: Command) {
        self.commands.push(command);
    }

    fn search(&self, input: String) -> Vec<String> {
        let mut completions: Vec<String> = Vec::new();

        let comm_name = get_until_first_space(&input);

        for command in &self.commands {
            if command.name.eq_ignore_ascii_case(&comm_name) {
                return command.check(remove_until_first_space(&input));
            }
            else if command.name.starts_with(&comm_name) {
                completions.push(command.name.clone());
            }
        }

        completions
    }
}

fn main() {
    let command = Command::new(String::from("console"))
        .add_node(Node::fixed_string_new(String::from("clear")));

    let mut commander = Commander::new();
    commander.add(command);

    let mut input = String::new();
    println!("Enter command: ");
    std::io::stdin().read_line(&mut input).expect("error reading user input");
    input.pop(); input.pop();

    println!("{:?}", commander.search(input));
}
