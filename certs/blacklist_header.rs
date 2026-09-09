use core::ffi::c_char;

// Declared with the Linux __initconst attribute in C.
extern "C" {
    pub static blacklist_hashes: [*const c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
