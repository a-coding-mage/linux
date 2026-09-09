/* Force alignment of .toc section. */
// The original linker-script section is:
// .toc 0 : ALIGN(256) { *(.got .toc) }
// Linker-script section placement and wildcard input selection have no direct
// Rust equivalent; preserve the local constants describing that layout.
pub const TOC_SECTION_NAME: &str = ".toc";
pub const TOC_INPUT_SECTIONS: [&str; 2] = [".got", ".toc"];
pub const TOC_ALIGNMENT: usize = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
