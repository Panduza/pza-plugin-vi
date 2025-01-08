# pza-plugin-vi
Panduza Virtual Instruments

```bash
cargo install cargo-post
cargo post build --features plugin
```

# full usage

```json
{
    "devices": [
        {
            "name": "Virtual REPL",
            "dref": "vi.repl"
        },
        {
            "name": "Attribute Tester",
            "dref": "vi.tester"
        }
    ]
}
```

# Boolean Vector Emulation

You can provides a list of name to this driver.

It will generate as many boolean as names you provided.

```json
{
    "devices": [
        {
            "dref": "vi.boolean_vector",
            "name": "virtual",
            "settings": {
                "elements": [
                    "sensor",
                    "relais",
                    "f002"
                ]
            }
        }
    ]
}
```

