use cbindgen::{Config, ExportConfig, StructConfig, Style};
use std::collections::HashMap;
use cbindgen::ItemType::Structs;

fn main() {
    let cargo_dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/../";

    let stdlib_dir: String = format!("{}/stdlib", cargo_dir);
    let stdint_dir: String = format!("{}/stdint", cargo_dir);
    let bits_stdint_intn_dir: String = format!("{}/bits/stdint_intn", cargo_dir);
    let bits_stdint_uintn_dir: String = format!("{}/bits/stdint_uintn", cargo_dir);
    let string_dir: String = format!("{}/string", cargo_dir);
    let stddef_dir: String = format!("{}/stddef", cargo_dir);

    cbindgen::Builder::new()
        .with_config(
            Config {
                structure: StructConfig {
                    derive_constructor: true,
                    derive_eq: true,
                    derive_neq: true,
                    derive_lt: true,
                    derive_lte: true,
                    derive_gt: true,
                    derive_gte: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_crate(stdlib_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include_guard("_STDLIB_H")
        .with_include("stdint.h")
        .with_include("stddef.h")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/stdlib.h", cargo_dir));

    cbindgen::Builder::new()
        .with_config(
            Config {
                structure: StructConfig {
                    derive_constructor: true,
                    derive_eq: true,
                    derive_neq: true,
                    derive_lt: true,
                    derive_lte: true,
                    derive_gt: true,
                    derive_gte: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_crate(stdint_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include("bits/stdint-intn.h")
        .with_include("bits/stdint-uintn.h")
        .with_include_guard("_STDINT_H")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/stdint.h", cargo_dir));

    cbindgen::Builder::new()
        .with_config(Config {
            export: ExportConfig {
                rename: {
                    let mut map = HashMap::new();
                    map.insert("rint8_t" .to_owned(), "int8_t" .to_owned());
                    map.insert("rint16_t".to_owned(), "int16_t".to_owned());
                    map.insert("rint32_t".to_owned(), "int32_t".to_owned());
                    map.insert("rint64_t".to_owned(), "int64_t".to_owned());
                    map
                },
                ..Default::default()
            },
            structure: StructConfig {
                derive_constructor: true,
                derive_eq: true,
                derive_neq: true,
                derive_lt: true,
                derive_lte: true,
                derive_gt: true,
                derive_gte: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_crate(bits_stdint_intn_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include_guard("_BITS_STDINT_INTN_H")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/bits/stdint-intn.h", cargo_dir));

    cbindgen::Builder::new()
        .with_config(Config {
            export: ExportConfig {
                rename: {
                    let mut map = HashMap::new();
                    map.insert("ruint8_t" .to_owned(), "uint8_t" .to_owned());
                    map.insert("ruint16_t".to_owned(), "uint16_t".to_owned());
                    map.insert("ruint32_t".to_owned(), "uint32_t".to_owned());
                    map.insert("ruint64_t".to_owned(), "uint64_t".to_owned());
                    map
                },
                ..Default::default()
            },
            structure: StructConfig {
                derive_constructor: true,
                derive_eq: true,
                derive_neq: true,
                derive_lt: true,
                derive_lte: true,
                derive_gt: true,
                derive_gte: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_crate(bits_stdint_uintn_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include_guard("_BITS_STDINT_UINTN_H")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/bits/stdint-uintn.h", cargo_dir));

    cbindgen::Builder::new()
        .with_config(
            Config {
                structure: StructConfig {
                    derive_constructor: true,
                    derive_eq: true,
                    derive_neq: true,
                    derive_lt: true,
                    derive_lte: true,
                    derive_gt: true,
                    derive_gte: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_crate(string_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include_guard("_STRING_H")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/string.h", cargo_dir));

    cbindgen::Builder::new()
        .with_config(Config {
            export: ExportConfig {
                rename: {
                    let mut map = HashMap::new();
                    map.insert("rsize_t" .to_owned(), "size_t" .to_owned());
                    map
                },
                ..Default::default()
            },
            structure: StructConfig {
                derive_constructor: true,
                derive_eq: true,
                derive_neq: true,
                derive_lt: true,
                derive_lte: true,
                derive_gt: true,
                derive_gte: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_crate(stddef_dir)
        .with_language(cbindgen::Language::C)
        .with_autogen_warning("// Warning: This file was autogenerated with cbindgen.\n// To add bindings, edit the source and rerun cbindgen.")
        .with_no_includes()
        .with_include_guard("_STDDEF_H")
        .with_line_length(80)
        .with_define("target_pointer_width", "64", "__WORDSIZE) && (__WORDSIZE == 64")
        .with_style(Style::Tag)
        .generate()
        .unwrap()
        .write_to_file(format!("{}/stddef.h", cargo_dir));
}
