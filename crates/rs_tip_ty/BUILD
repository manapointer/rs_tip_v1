load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("@crate_index//:defs.bzl", "all_crate_deps")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "rs_tip_ty",
    srcs = glob(["src/**/*.rs"]),
    deps = all_crate_deps(
        normal = True,
    ) + ["//crates/rs_tip_syntax"],
)
