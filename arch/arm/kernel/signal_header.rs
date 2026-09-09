// Dependency from <asm/ucontext.h>; the corresponding Rust type is supplied
// by the surrounding translation unit.

#[repr(C)]
pub struct sigframe {
    pub uc: ucontext,
    pub retcode: [core::ffi::c_ulong; 4],
}

#[repr(C)]
pub struct rt_sigframe {
    pub info: siginfo,
    pub sig: sigframe,
}

unsafe extern "C" {
    pub fn get_signal_page() -> *mut page;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
