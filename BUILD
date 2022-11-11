load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "electrolysis",
    srcs = ["main.rs"],
    deps = [
        "@crate_index//:tokio",
        "@crate_index//:clap",
        "@crate_index//:git2",
        "@crate_index//:toml",
        "@crate_index//:serde",
    ],
)

