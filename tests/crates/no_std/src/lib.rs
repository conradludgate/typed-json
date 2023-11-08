#![no_std]
pub fn foo() -> Result<serde_json_core::heapless::String<256>, serde_json_core::ser::Error> {
    let input = 1;
    serde_json_core::to_string(&typed_json::json!({
        "foo": input,
        "bar": [input],
        "baz": {
            "code": input,
            "extra": null,
            "this": {
                "is": {
                    "a": [
                        input,
                        {
                            "really": {
                                "deep": ["object", input, null, true, false]
                            }
                        }
                    ]
                }
            }
        },
    }))
}
