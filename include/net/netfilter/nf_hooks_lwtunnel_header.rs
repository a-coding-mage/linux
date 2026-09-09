// Declarations corresponding to <linux/sysctl.h> and <linux/types.h>.

// CONFIG_SYSCTL
#[cfg(CONFIG_SYSCTL)]
extern "C" {
    pub fn nf_hooks_lwtunnel_sysctl_handler(
        table: *const ctl_table,
        write: core::ffi::c_int,
        buffer: *mut core::ffi::c_void,
        lenp: *mut usize,
        ppos: *mut loff_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
