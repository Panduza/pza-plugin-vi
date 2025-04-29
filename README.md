# pza-plugin-vi
Panduza Virtual Instruments

```bash
cargo install cargo-post
cargo post build --features plugin
```

# Tester

The attribute tester is used as a validation tool.

It mounts all the possible attribute types and modes.

This driver is very important and any change on this driver must be validated by the core team.

```json
{
    "devices": [
        {
            "name": "Attribute Tester",
            "dref": "vi.tester"
        }
    ]
}
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

