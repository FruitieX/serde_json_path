# serde_json_path

`serde_json_path` allows you to use [JSONPath](jsonpath) to query the [`serde_json::Value`][serde_json_value] type.

[![Crates.io](https://img.shields.io/crates/v/serde_json_path)](https://crates.io/crates/serde_json_path)
[![Crates.io](https://img.shields.io/crates/d/serde_json_path)](https://crates.io/crates/serde_json_path)
[![Documentation](https://docs.rs/serde_json_path/badge.svg)][docs]

## Learn More

* See the [Crate Documentation][docs] for usage and examples.
* See the [IETF JSONPath Specification][jp_spec] for more details about JSONPath and examples of its usage.
* Try it out in the [Sandbox](https://serdejsonpath.live)

## Planned Development

* [Function Expressions][func_ext_issue]
* [Improved Error Handling][error_issue]

## License

This project is licensed under the [MIT license][license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `serde_json_path` by you, shall be licensed as MIT, without any
additional terms or conditions.

[docs]: https://docs.rs/serde_json_path
[jsonpath]: https://datatracker.ietf.org/wg/jsonpath/about/
[serde_json_value]: https://docs.rs/serde_json/latest/serde_json/enum.Value.html
[license]: https://github.com/hiltontj/serde_json_path/blob/main/LICENSE-MIT
[func_ext_issue]: https://github.com/hiltontj/serde_json_path/issues/1
[error_issue]: https://github.com/hiltontj/serde_json_path/issues/4
[jp_spec]: https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-10.html