
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum IncomingEdges {
    #[serde(rename = "copy")]
    Copy {src: i32},

    #[serde(rename = "F")]
    F {subkey: i32, value: i32},

    #[serde(rename = "ciphertext")]
    Ciphertext {src: i32},

    #[serde(rename = "left")]
    Left {src: i32},

    #[serde(rename = "right")]
    Right {src: i32},

    #[serde(rename = "swap")]
    Swap {left: i32, right: i32},

    #[serde(rename = "xor")]
    Xor {a: i32, b: i32}
}
