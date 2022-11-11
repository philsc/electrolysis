load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "324c2a86a8708d30475f324846b35965c432b63a35567ed2b5051b86791ce345",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.13.0/rules_rust-v0.13.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    version = "1.65.0",
)


load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.Bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    annotations = {
        "libgit2-sys": [crate.annotation(
            gen_build_script = False,
            # libgit2 comes from @rules_rust//crate_universe/3rdparty:third_party_deps.bzl
            deps = ["@libgit2"],
        )],
    },
    packages = {
        "clap": crate.spec(version="4.0.23", features=["derive"]),
        "git2": crate.spec(version="0.15.0", default_features=False),
        "tokio": crate.spec(version="1.21.2", features=["full"]),
        "toml": crate.spec(version="0.5.9"),
        "serde": crate.spec(version="1.0.147", features = ["derive"]),
    },
    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

http_archive(
    name = "libgit2",
    build_file = "@rules_rust//crate_universe/3rdparty:BUILD.libgit2.bazel",
    sha256 = "8de872a0f201b33d9522b817c92e14edb4efad18dae95cf156cf240b2efff93e",
    # The version here should match the version used with the Rust crate `libgit2-sys`
    # https://github.com/rust-lang/git2-rs/tree/libgit2-sys-0.14.0+1.5.0/libgit2-sys
    strip_prefix = "libgit2-1.5.0",
    urls = ["https://github.com/libgit2/libgit2/archive/refs/tags/v1.5.0.tar.gz"],
)
