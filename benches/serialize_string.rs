fn main() {
    divan::main()
}

#[divan::bench]
fn serde_json() -> String {
    let input = divan::black_box(1);
    serde_json::to_string(&serde_json::json!({
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
    .unwrap()
}

#[divan::bench]
fn typed_json() -> String {
    let input = divan::black_box(1);
    serde_json::to_string(&typed_json::json!({
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
    .unwrap()
}

#[divan::bench]
fn typed_json_core() -> serde_json_core::heapless::String<256> {
    let input = divan::black_box(1);
    serde_json_core::to_string::<_, 256>(&typed_json::json!({
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
    .unwrap()
}
