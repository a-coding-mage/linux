// Dependency from C source: #include "core_reloc_types.h"

#[allow(non_camel_case_types)]
type core_reloc_size = crate::core_reloc_size;

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn f(x: core_reloc_size) {}
