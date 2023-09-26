
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "label")]
enum ComputationGraph {
    #[serde(rename = "plaintext")]
    Plaintext {},

    #[serde(rename = "key0")]
    Key0 {},

    #[serde(rename = "key1")]
    Key1 {},

    #[serde(rename = "key2")]
    Key2 {},

    #[serde(rename = "key3")]
    Key3 {},

    #[serde(rename = "key4")]
    Key4 {},

    #[serde(rename = "key5")]
    Key5 {},

    #[serde(rename = "key6")]
    Key6 {},

    #[serde(rename = "key7")]
    Key7 {},

    #[serde(rename = "key8_11")]
    Key8_11 {},

    #[serde(rename = "key12_15")]
    Key12_15 {},

    #[serde(rename = "right")]
    Right {src: i32},

    #[serde(rename = "F")]
    F {subkey: i32, value: i32},

    #[serde(rename = "copy")]
    Copy {src: i32},

    #[serde(rename = "left")]
    Left {src: i32},

    #[serde(rename = "swap")]
    Swap {left: i32, right: i32},

    #[serde(rename = "xor")]
    Xor {a: i32, b: i32},

    #[serde(rename = "ciphertext")]
    Ciphertext {src: i32}
}
