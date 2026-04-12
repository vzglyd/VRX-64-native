use std::io::{Cursor, Read};

use vzglyd_native::slide_loader::{LoadError, load_slide_from_wasm_bytes};
use vzglyd_slide::WorldVertex;

const LOADING_VZGLYD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/loading.vzglyd"));

fn extract_entry(bytes: &[u8], name: &str) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("valid zip");
    let mut entry = archive.by_name(name).expect("entry exists");
    let mut buf = Vec::new();
    entry.read_to_end(&mut buf).expect("read entry");
    buf
}

fn read_manifest_abi(bytes: &[u8]) -> u32 {
    let raw = extract_entry(bytes, "manifest.json");
    let value: serde_json::Value = serde_json::from_slice(&raw).expect("manifest json");
    value["abi_version"]
        .as_u64()
        .expect("manifest abi_version")
        .try_into()
        .expect("abi_version fits in u32")
}

fn make_wrong_abi_wasm(version: u32) -> Vec<u8> {
    wat::parse_str(format!(
        r#"
        (module
          (func (export "vzglyd_abi_version") (result i32)
            i32.const {version})
        )
        "#
    ))
    .expect("WAT parse")
}

#[test]
fn smoke_abi_version_consistent() {
    let manifest_abi = read_manifest_abi(LOADING_VZGLYD);
    assert_eq!(
        manifest_abi,
        vzglyd_slide::ABI_VERSION,
        "manifest abi_version {manifest_abi} != crate ABI_VERSION {}",
        vzglyd_slide::ABI_VERSION
    );
}

#[test]
fn smoke_archive_structure() {
    let mut archive = zip::ZipArchive::new(Cursor::new(LOADING_VZGLYD)).expect("valid zip");
    let names: Vec<_> = (0..archive.len())
        .map(|idx| {
            archive
                .by_index(idx)
                .expect("archive entry")
                .name()
                .to_owned()
        })
        .collect();

    assert!(names.iter().any(|name| name == "manifest.json"));
    assert!(names.iter().any(|name| name == "slide.wasm"));
}

#[test]
fn smoke_loading_slide_loads() {
    let wasm = extract_entry(LOADING_VZGLYD, "slide.wasm");
    let loaded =
        load_slide_from_wasm_bytes::<WorldVertex>(&wasm).expect("loading slide should load");

    assert_eq!(loaded.spec.name, "loading_scene");
}

#[test]
fn smoke_abi_mismatch_is_detected() {
    let bad_wasm = make_wrong_abi_wasm(vzglyd_slide::ABI_VERSION + 1);

    match load_slide_from_wasm_bytes::<WorldVertex>(&bad_wasm) {
        Err(LoadError::AbiVersion { found, expected }) => {
            assert_eq!(expected, vzglyd_slide::ABI_VERSION);
            assert_eq!(found, vzglyd_slide::ABI_VERSION + 1);
        }
        Ok(_) => panic!("expected AbiVersion error, got Ok"),
        Err(error) => panic!("expected AbiVersion error, got {error:?}"),
    }
}
