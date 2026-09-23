// SPDX-License-Identifier: GPL-2.0-only
//! Export records for translated implementations that own a kernel C ABI.
//!
//! Only the implementation-owning crate imports this module. Pure Rust API
//! consumers must not emit a second export record. The record layout matches
//! `include/linux/export.h`; modpost supplies the final kernel symbol tables.
//! Version CRCs come from the implementation's own DWARF, not C declarations.
//!
//! This CONFIG_MODVERSIONS token is a fixdep dependency: changing module
//! versioning must rebuild each exporting object and regenerate its CRCs.

/// Register an unmangled C ABI symbol, preserving its license and namespace.
///
/// `$name` is the exact external name; `$symbol` is the defining Rust item,
/// which may instead use `#[export_name]`. Licenses are either `""` or `"GPL"`.
/// Namespaces may contain printable ASCII except quotes, backslashes and braces:
/// these restrictions prevent injection into the assembler/template strings.
/// The usual identifier namespaces and `module:name,name*` form are supported.
///
/// `sym` references the real item, retaining its function/object symbol kind
/// and preventing a misspelled Rust path from becoming an unresolved symbol.
/// Local record labels are intentionally not Rust globals or exported items.
#[allow(unused_macros)] // Linkage-only bridge crates use the other interface.
macro_rules! export_symbol {
    ($name:ident, $symbol:path, $license:literal, $namespace:literal) => {
        const _: () = {
            let license = $license.as_bytes();
            assert!(
                license.is_empty()
                    || (license.len() == 3
                        && license[0] == b'G'
                        && license[1] == b'P'
                        && license[2] == b'L'),
                "export license must be empty or GPL"
            );
            let namespace = $namespace.as_bytes();
            let mut i = 0;
            while i < namespace.len() {
                let byte = namespace[i];
                assert!(
                    byte >= b' '
                        && byte <= b'~'
                        && byte != b'"'
                        && byte != b'\\'
                        && byte != b'{'
                        && byte != b'}',
                    "export namespace contains unsupported assembler characters"
                );
                i += 1;
            }
        };

        #[cfg(target_pointer_width = "64")]
        core::arch::global_asm!(
            concat!(
                ".pushsection .export_symbol,\"a\"\n",
                "__export_symbol_", stringify!($name), ":\n",
                ".asciz \"", $license, "\"\n",
                ".asciz \"", $namespace, "\"\n",
                ".balign 8\n.quad {address}\n.popsection\n",
            ),
            address = sym $symbol,
        );
        #[cfg(target_pointer_width = "32")]
        core::arch::global_asm!(
            concat!(
                ".pushsection .export_symbol,\"a\"\n",
                "__export_symbol_", stringify!($name), ":\n",
                ".asciz \"", $license, "\"\n",
                ".asciz \"", $namespace, "\"\n",
                ".balign 4\n.long {address}\n.popsection\n",
            ),
            address = sym $symbol,
        );
    };
}

/// Export a linker name from Kbuild's generated Rust-library symbol lists.
///
/// Unlike `export_symbol!`, this metadata-only bridge does not own the item.
/// Generic/mangled Rust items cannot be expressed as typed paths in a different
/// crate. Reference the exact linkage name without inventing an `extern` type;
/// the defining object supplies its symbol kind, DWARF and version checksum.
///
/// This interface is restricted to the existing GPL-only, empty-namespace Rust
/// library bridge. Ordinary native C ABI owners must use the typed macro above.
#[allow(unused_macros)] // Typed implementation owners do not use linker names.
macro_rules! export_symbol_linkage_gpl {
    ($symbol:ident) => {
        const _: () = {
            let name = stringify!($symbol).as_bytes();
            let mut i = 0;
            while i < name.len() {
                let byte = name[i];
                assert!(
                    byte == b'_'
                        || (byte >= b'a' && byte <= b'z')
                        || (byte >= b'A' && byte <= b'Z')
                        || (i != 0 && byte >= b'0' && byte <= b'9'),
                    "Rust-library export names must be ASCII C identifiers"
                );
                i += 1;
            }
        };

        #[cfg(target_pointer_width = "64")]
        core::arch::global_asm!(concat!(
            ".pushsection .export_symbol,\"a\"\n",
            "__export_symbol_",
            stringify!($symbol),
            ":\n",
            ".asciz \"GPL\"\n.asciz \"\"\n",
            ".balign 8\n.quad ",
            stringify!($symbol),
            "\n.popsection\n",
        ));
        #[cfg(target_pointer_width = "32")]
        core::arch::global_asm!(concat!(
            ".pushsection .export_symbol,\"a\"\n",
            "__export_symbol_",
            stringify!($symbol),
            ":\n",
            ".asciz \"GPL\"\n.asciz \"\"\n",
            ".balign 4\n.long ",
            stringify!($symbol),
            "\n.popsection\n",
        ));
    };
}

// Each importing crate uses either typed ownership or linkage-only metadata.
#[allow(unused_imports)]
pub(crate) use {export_symbol, export_symbol_linkage_gpl};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
