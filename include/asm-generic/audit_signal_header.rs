// Translated from a C enum-fragment header.  The numeric values are supplied
// by the surrounding syscall-number definition.
#[allow(non_camel_case_types)]
#[repr(isize)]
pub enum audit_signal {
    __NR_kill,
    __NR_tgkill,
    __NR_tkill,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
