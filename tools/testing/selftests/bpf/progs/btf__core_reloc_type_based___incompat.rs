// Dependency intent from C: #include "core_reloc_types.h"

extern "C" {
    pub type core_reloc_type_based___incompat;
}

#[no_mangle]
pub extern "C" fn f(_x: core_reloc_type_based___incompat) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
