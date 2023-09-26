
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

    #[serde(rename = "copy64")]
    Copy64 {src: i64},

    #[serde(rename = "right")]
    Right {src: i64},

    #[serde(rename = "copy16")]
    Copy16 {src: i16},

    #[serde(rename = "F")]
    F {subkey: i16, value: i32},

    #[serde(rename = "copy32")]
    Copy32 {src: i32},

    #[serde(rename = "left")]
    Left {src: i64},

    #[serde(rename = "xor32")]
    Xor32 {a: i32, b: i32},

    #[serde(rename = "swap")]
    Swap {left: i32, right: i32},

    #[serde(rename = "xor64")]
    Xor64 {a: i64, b: i64},

    #[serde(rename = "ciphertext")]
    Ciphertext {src: i64}
}
