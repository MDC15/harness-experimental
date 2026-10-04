use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn inward_layers_do_not_import_outward_layers_or_frameworks() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    assert_forbidden(
        &root.join("domain"),
        &[
            "crate::application",
            "crate::infrastructure",
            "crate::interface",
            "serde",
            "clap",
            "fs2",
            "std::fs",
            "std::process",
        ],
    );
    assert_forbidden(
        &root.join("application"),
        &[
            "crate::infrastructure",
            "crate::interface",
            "serde",
            "clap",
            "fs2",
            "std::fs",
            "std::process",
        ],
    );
    assert_forbidden(&root.join("infrastructure"), &["crate::interface"]);
    assert_forbidden(&root.join("interface"), &["crate::infrastructure"]);
}

#[test]
fn composition_root_is_the_only_layer_wiring_infrastructure_to_interface() {
    let main =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs")).unwrap();
    assert!(main.contains("harness::infrastructure"));
    assert!(main.contains("harness::interface"));
    assert!(main.contains("CoreApplication::new"));
    assert!(main.contains("SelfUpdateApplication::new"));
}

#[test]
fn allowed_nested_dependencies_and_non_rust_files_pass() {
    assert_forbidden(&fixture("allowed"), &["crate::interface"]);
}

#[test]
fn forbidden_dependency_in_an_immediate_module_is_rejected() {
    assert_fixture_rejected("immediate", "module.rs");
}

#[test]
fn forbidden_dependency_in_a_deeply_nested_module_is_rejected() {
    assert_fixture_rejected("nested", "first/second/module.rs");
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/architecture")
        .join(name)
}

fn assert_fixture_rejected(name: &str, module: &str) {
    let failure = std::panic::catch_unwind(|| {
        assert_forbidden(&fixture(name), &["crate::interface"]);
    })
    .expect_err("forbidden fixture must fail the architecture guard");
    let diagnostic = failure.downcast_ref::<String>().unwrap();
    let module_path = fixture(name).join(module.split('/').collect::<PathBuf>());
    assert!(
        diagnostic.contains(module_path.to_str().unwrap()),
        "{diagnostic}"
    );
    assert!(
        diagnostic.contains("forbidden dependency crate::interface"),
        "{diagnostic}"
    );
    assert!(diagnostic.contains("docs/ARCHITECTURE.md"), "{diagnostic}");
    assert!(
        diagnostic.contains("depend on an inward layer"),
        "{diagnostic}"
    );
}

fn assert_forbidden(root: &Path, forbidden: &[&str]) {
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            assert_forbidden(&path, forbidden);
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for pattern in forbidden {
            assert!(
                !source.contains(pattern),
                "{} imports forbidden dependency {pattern}: violates Rust dependency direction \
                 (docs/ARCHITECTURE.md). Remove the outward dependency and depend on an inward layer; \
                 wire concrete implementations in main.rs.",
                path.display()
            );
        }
    }
}
