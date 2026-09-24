// SPDX-License-Identifier: GPL-2.0-only

//! Owned module and symbol records shared by the modpost passes.

use crate::file2alias::Alias;

#[derive(Default)]
pub(crate) struct Options {
    pub(crate) module_enabled: bool,
    pub(crate) rust_vmlinux_export: bool,
    pub(crate) modversions: bool,
    pub(crate) all_versions: bool,
    pub(crate) basic_versions: bool,
    pub(crate) extended_versions: bool,
    pub(crate) external: bool,
    pub(crate) warn_unresolved: bool,
    pub(crate) mismatch_error: bool,
    pub(crate) trim: bool,
    pub(crate) ignore_missing: bool,
    pub(crate) allow_missing_namespace: bool,
    pub(crate) dumps: Vec<String>,
    pub(crate) objects: Vec<String>,
    pub(crate) files_source: Option<String>,
    pub(crate) dump_output: Option<String>,
    pub(crate) namespace_output: Option<String>,
    pub(crate) whitelist: Option<String>,
}

pub(crate) struct Module {
    pub(crate) name: String,
    pub(crate) dump: Option<String>,
    pub(crate) vmlinux: bool,
    pub(crate) gpl_compatible: bool,
    pub(crate) has_init: bool,
    pub(crate) has_cleanup: bool,
    pub(crate) word_size: usize,
    pub(crate) source_version: Option<String>,
    pub(crate) exports: Vec<usize>,
    pub(crate) unresolved: Vec<Unresolved>,
    pub(crate) imported_namespaces: Vec<String>,
    pub(crate) missing_namespaces: Vec<String>,
    pub(crate) aliases: Vec<Alias>,
    pub(crate) no_trim: Vec<String>,
}

impl Module {
    pub(crate) fn new(name: String) -> Self {
        Self {
            vmlinux: name == "vmlinux",
            name,
            dump: None,
            gpl_compatible: true,
            has_init: false,
            has_cleanup: false,
            word_size: 8,
            source_version: None,
            exports: Vec::new(),
            unresolved: Vec::new(),
            imported_namespaces: Vec::new(),
            missing_namespaces: Vec::new(),
            aliases: Vec::new(),
            no_trim: Vec::new(),
        }
    }
}

pub(crate) struct Export {
    pub(crate) name: String,
    pub(crate) module: usize,
    pub(crate) namespace: String,
    pub(crate) crc: Option<u32>,
    pub(crate) is_function: bool,
    pub(crate) gpl_only: bool,
    pub(crate) used: bool,
}

pub(crate) struct Unresolved {
    pub(crate) name: String,
    pub(crate) module: Option<usize>,
    pub(crate) crc: Option<u32>,
    pub(crate) weak: bool,
}

impl Unresolved {
    pub(crate) fn new(name: String, weak: bool) -> Self {
        Self {
            name,
            module: None,
            crc: None,
            weak,
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
