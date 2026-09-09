/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding build: asm/page.h (PAGE_SIZE).

/*
 * .boot.data section is shared between the decompressor code and the
 * decompressed kernel. The decompressor will store values in it, and copy
 * over to the decompressed image before starting it.
 *
 * .boot.data variables are kept in separate .boot.data.<var name> sections,
 * which are sorted by alignment first, then by name before being merged
 * into single .boot.data section. This way big holes cased by page aligned
 * structs are avoided and linker produces consistent result.
 */
pub const BOOT_DATA: &str = r#"
. = ALIGN(PAGE_SIZE);
.boot.data : {
    __boot_data_start = .;
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.boot.data*)))
    __boot_data_end = .;
}
"#;

/*
 * .boot.preserved.data is similar to .boot.data, but it is not part of the
 * .init section and thus will be preserved for later use in the decompressed
 * kernel.
 */
pub const BOOT_DATA_PRESERVED: &str = r#"
. = ALIGN(PAGE_SIZE);
.boot.preserved.data : {
    __boot_data_preserved_start = .;
    *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.boot.preserved.data*)))
    __boot_data_preserved_end = .;
}
"#;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
