use serde;
use serde::de;
use serde::{Deserialize, Serialize, Deserializer};
use serde_json::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Node {
    pub last_update: u32, 
    pub pub_key: String,
    pub alias: String,
    pub addresses: Vec<Address>,
    pub color: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Address {
    pub network: String,
    pub addr: SocketAddr,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Edge {
    pub channel_id: String,
    pub chan_point: String,
    pub last_update: u32,
    pub node1_pub: String,
    pub node2_pub: String,
    #[serde(deserialize_with = "from_str")]
    pub capacity: u32,
    pub node1_policy: Option<NodePolicy>,
    pub node2_policy: Option<NodePolicy>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NodePolicy {
    pub time_lock_delta: u16,
    #[serde(deserialize_with = "from_str")]
    pub min_htlc: u64,
    #[serde(deserialize_with = "from_str")]
    pub fee_base_msat: u64,
    #[serde(deserialize_with = "from_str")]
    pub fee_rate_milli_msat: u64,
    pub disabled: bool,
    #[serde(deserialize_with = "from_str")]
    pub max_htlc_msat: u64,
    pub last_update: u32,
}

pub fn from_json_str(json_str: &str) -> Result<Graph,Error> {
    serde_json::from_str(json_str)
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where T: FromStr,
      T::Err: Display,
      D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing_works() {
        let data = r##"{
            "nodes": [
                {
                    "last_update": 1567764428,
                    "pub_key": "0200424bd89b5282c310e10a52fd783070556f947b54d93f73fd89534ce0cba708",
                    "alias": "WHENBTC",
                    "addresses": [
                        {
                            "network": "tcp",
                            "addr": "67.166.1.116:9735"
                        }
                    ],
                    "color": "#3399ff"
                }
            ],
            "edges": [
                {
                    "channel_id": "659379322247708673",
                    "chan_point": "ae07c9fe78e6a1057902441f599246d735bac33be7b159667006757609fb5a86:1",
                    "last_update": 1571278793,
                    "node1_pub": "02899d09a65c5ca768c42b12e57d0497bfdf8ac1c46b0dcc0d4faefcdbc01304c1",
                    "node2_pub": "0298f6074a454a1f5345cb2a7c6f9fce206cd0bf675d177cdbf0ca7508dd28852f",
                    "capacity": "1000000",
                    "node1_policy": null,
                    "node2_policy": {
                        "time_lock_delta": 14,
                        "min_htlc": "1000",
                        "fee_base_msat": "1000",
                        "fee_rate_milli_msat": "1",
                        "disabled": false,
                        "max_htlc_msat": "990000000",
                        "last_update": 1571278793
                    }
                }
            ]
            }"##;
        let parsed_graph = from_json_str(data).unwrap();
        let parsed_node = parsed_graph.nodes.first().unwrap();
        let parsed_edge= parsed_graph.edges.first().unwrap();
        
        let address = Address { 
            network:"tcp".to_string(), 
            addr: FromStr::from_str("67.166.1.116:9735").unwrap(),
        };

        let node = Node { 
            last_update: 1567764428, 
            pub_key: "0200424bd89b5282c310e10a52fd783070556f947b54d93f73fd89534ce0cba708".to_string(),
            alias: "WHENBTC".to_string(), 
            addresses: vec![address], 
            color: "#3399ff".to_string(), 
        };

        let n1_policy = None;
        let n2_policy = Some(NodePolicy {
            time_lock_delta: 14,
            min_htlc: 1000,
            fee_base_msat: 1000,
            fee_rate_milli_msat: 1,
            disabled: false,
            max_htlc_msat: 990000000,
            last_update: 1571278793
        });

        let edge = Edge {
            channel_id: "659379322247708673".to_string(),
            chan_point: "ae07c9fe78e6a1057902441f599246d735bac33be7b159667006757609fb5a86:1".to_string(),
            last_update: 1571278793,
            node1_pub: "02899d09a65c5ca768c42b12e57d0497bfdf8ac1c46b0dcc0d4faefcdbc01304c1".to_string(),
            node2_pub: "0298f6074a454a1f5345cb2a7c6f9fce206cd0bf675d177cdbf0ca7508dd28852f".to_string(),
            capacity: 1000000,
            node1_policy: n1_policy,
            node2_policy: n2_policy,
        };
        assert_eq!(*parsed_node, node);
        assert_eq!(*parsed_edge, edge);
    }
}
