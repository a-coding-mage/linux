// Dependency from C source: core_reloc_types.h
use crate::core_reloc_types::core_reloc_nesting;

#[no_mangle]
pub extern "C" fn f(x: core_reloc_nesting) {
    let _ = x;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
