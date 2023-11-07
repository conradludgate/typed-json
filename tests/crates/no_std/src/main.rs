fn main() {
    let input = 1;
    let str = serde_json_core::to_string::<_, 256>(&typed_json::json!({
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
    .unwrap();

    assert_eq!(str, "{\"foo\":1,\"bar\":[1],\"baz\":{\"code\":1,\"extra\":null,\"this\":{\"is\":{\"a\":[1,{\"really\":{\"deep\":[\"object\",1,null,true,false]}}]}}}}")
}
