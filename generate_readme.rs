use std::{
    fmt::{Display, Formatter, Write as FmtWrite},
    io::Write,
};

use serde::Deserialize;

const JSON: &str = include_str!("./snapshots.json");
const TEMPLATE: &str = include_str!("./TEMPLATE.md");

#[derive(Deserialize)]
struct Node {
    node: NodeType,
    networks: Vec<Network>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum NodeType {
    Pathfinder,
}

#[derive(Deserialize)]
struct Network {
    network: NetworkType,
    snapshots: Vec<Snapshot>,
}

#[derive(Deserialize)]
enum NetworkType {
    #[serde(rename = "mainnet")]
    Mainnet,
    #[serde(rename = "goerli-1")]
    Goerli1,
    #[serde(rename = "goerli-2")]
    Goerli2,
}

#[derive(Deserialize)]
struct Snapshot {
    block: u64,
    version: String,
}

impl NodeType {
    fn downloand_link_base(&self) -> &str {
        match self {
            Self::Pathfinder => "https://pathfinder-backup.zklend.com/",
        }
    }
}

impl Snapshot {
    fn file_name(&self, network: &NetworkType) -> String {
        format!("{}-v{}-{}.tar.xz", network, self.version, self.block)
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pathfinder => write!(f, "pathfinder"),
        }
    }
}

impl Display for NetworkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Goerli1 => write!(f, "goerli-1"),
            Self::Goerli2 => write!(f, "goerli-2"),
        }
    }
}

fn main() {
    let nodes: Vec<Node> = serde_json::from_str(JSON).expect("unable to deserialize JSON");

    let mut snapshot_list = String::new();

    let mut example_network: Option<String> = None;
    let mut example_file_name: Option<String> = None;

    for node in nodes.iter() {
        writeln!(snapshot_list, "## `{}`", node.node).unwrap();

        for network in node.networks.iter() {
            writeln!(snapshot_list).unwrap();
            writeln!(snapshot_list, "### `{}`", network.network).unwrap();
            writeln!(snapshot_list).unwrap();

            writeln!(
                snapshot_list,
                "| Network | Version | Block | Download Link |"
            )
            .unwrap();
            writeln!(snapshot_list, "| - | - | - | - |").unwrap();

            for snapshot in network.snapshots.iter() {
                let network_name = format!("{}", network.network);
                let file_name = snapshot.file_name(&network.network);

                if example_network.is_none() {
                    example_network = Some(network_name.clone());
                }
                if example_file_name.is_none() {
                    example_file_name = Some(file_name.clone());
                }

                writeln!(
                    snapshot_list,
                    "| `{}` | `v{}` | `{}` | {}{}/{} |",
                    network_name,
                    snapshot.version,
                    snapshot.block,
                    node.node.downloand_link_base(),
                    network_name,
                    file_name
                )
                .unwrap();
            }
        }
    }

    let example_network = example_network.expect("no example network generated");
    let example_file_name = example_file_name.expect("no example file name generated");

    let full_readme = TEMPLATE
        .replace("EXAMPLE_NETWORK", &example_network)
        .replace("EXAMPLE_FILE_NAME", &example_file_name)
        .replace("LIST_OF_SNAPSHOTS", &snapshot_list);

    let mut file = std::fs::File::create("./README.md").expect("unable to create file");

    file.write_all(full_readme.as_bytes())
        .expect("unable to write to file");
}
