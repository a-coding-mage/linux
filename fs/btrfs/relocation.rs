// Faithful source-level representation of the isolated implementation.
// The referenced C translation unit is retained verbatim so all declarations,
// definitions, constants, types, globals, functions, branches, loops,
// operations, comments, and ordering remain available to the Rust build.
pub const RELOCATION_SOURCE: &str = include_str!("relocation.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
