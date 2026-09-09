/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Linux signal context definitions. */

pub const FP_XSTATE_MAGIC1: u32 = 0x4658_5053;
pub const FP_XSTATE_MAGIC2: u32 = 0x4658_5045;
pub const FP_XSTATE_MAGIC2_SIZE: usize = core::mem::size_of::<u32>();

#[repr(C)]
pub struct _fpx_sw_bytes {
    pub magic1: u32,
    pub extended_size: u32,
    pub xfeatures: u64,
    pub xstate_size: u32,
    pub padding: [u32; 7],
}

#[repr(C)]
pub struct _fpreg {
    pub significand: [u16; 4],
    pub exponent: u16,
}

#[repr(C)]
pub struct _fpxreg {
    pub significand: [u16; 4],
    pub exponent: u16,
    pub padding: [u16; 3],
}

#[repr(C)]
pub struct _xmmreg {
    pub element: [u32; 4],
}

pub const X86_FXSR_MAGIC: u32 = 0x0000;

#[repr(C)]
pub union _fpstate_32__bindgen_ty_1 {
    pub padding1: [u32; 44],
    pub padding: [u32; 44],
}

#[repr(C)]
pub union _fpstate_32__bindgen_ty_2 {
    pub padding2: [u32; 12],
    pub sw_reserved: _fpx_sw_bytes,
}

#[repr(C)]
pub struct _fpstate_32 {
    pub cw: u32,
    pub sw: u32,
    pub tag: u32,
    pub ipoff: u32,
    pub cssel: u32,
    pub dataoff: u32,
    pub datasel: u32,
    pub _st: [_fpreg; 8],
    pub status: u16,
    pub magic: u16,
    pub _fxsr_env: [u32; 6],
    pub mxcsr: u32,
    pub reserved: u32,
    pub _fxsr_st: [_fpxreg; 8],
    pub _xmm: [_xmmreg; 8],
    pub __bindgen_anon_1: _fpstate_32__bindgen_ty_1,
    pub __bindgen_anon_2: _fpstate_32__bindgen_ty_2,
}

#[repr(C)]
pub union _fpstate_64__bindgen_ty_1 {
    pub reserved3: [u32; 12],
    pub sw_reserved: _fpx_sw_bytes,
}

#[repr(C)]
pub struct _fpstate_64 {
    pub cwd: u16,
    pub swd: u16,
    pub twd: u16,
    pub fop: u16,
    pub rip: u64,
    pub rdp: u64,
    pub mxcsr: u32,
    pub mxcsr_mask: u32,
    pub st_space: [u32; 32],
    pub xmm_space: [u32; 64],
    pub reserved2: [u32; 12],
    pub __bindgen_anon_1: _fpstate_64__bindgen_ty_1,
}

#[cfg(target_arch = "x86")]
pub type _fpstate = _fpstate_32;
#[cfg(not(target_arch = "x86"))]
pub type _fpstate = _fpstate_64;

#[repr(C)]
pub struct _header {
    pub xfeatures: u64,
    pub reserved1: [u64; 2],
    pub reserved2: [u64; 5],
}

#[repr(C)]
pub struct _ymmh_state {
    pub ymmh_space: [u32; 64],
}

#[repr(C)]
pub struct _xstate {
    pub fpstate: _fpstate,
    pub xstate_hdr: _header,
    pub ymmh: _ymmh_state,
}

#[repr(C)]
pub struct sigcontext_32 {
    pub gs: u16, pub __gsh: u16,
    pub fs: u16, pub __fsh: u16,
    pub es: u16, pub __esh: u16,
    pub ds: u16, pub __dsh: u16,
    pub di: u32, pub si: u32, pub bp: u32, pub sp: u32,
    pub bx: u32, pub dx: u32, pub cx: u32, pub ax: u32,
    pub trapno: u32, pub err: u32, pub ip: u32,
    pub cs: u16, pub __csh: u16,
    pub flags: u32,
    pub sp_at_signal: u32,
    pub ss: u16, pub __ssh: u16,
    pub fpstate: u32,
    pub oldmask: u32,
    pub cr2: u32,
}

#[repr(C)]
pub struct sigcontext_64 {
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub di: u64, pub si: u64, pub bp: u64, pub bx: u64,
    pub dx: u64, pub ax: u64, pub cx: u64, pub sp: u64,
    pub ip: u64, pub flags: u64,
    pub cs: u16, pub gs: u16, pub fs: u16, pub ss: u16,
    pub err: u64, pub trapno: u64, pub oldmask: u64, pub cr2: u64,
    pub fpstate: u64,
    pub reserved1: [u64; 8],
}

#[cfg(all(feature = "kernel", target_arch = "x86"))]
pub type sigcontext = sigcontext_32;
#[cfg(all(feature = "kernel", not(target_arch = "x86")))]
pub type sigcontext = sigcontext_64;

/* Legacy user-space definitions, retained for ABI compatibility. */
pub type _fpstate_ia32 = _fpstate_32;
pub type sigcontext_ia32 = sigcontext_32;

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct sigcontext {
    pub gs: u16, pub __gsh: u16,
    pub fs: u16, pub __fsh: u16,
    pub es: u16, pub __esh: u16,
    pub ds: u16, pub __dsh: u16,
    pub edi: u32, pub esi: u32, pub ebp: u32, pub esp: u32,
    pub ebx: u32, pub edx: u32, pub ecx: u32, pub eax: u32,
    pub trapno: u32, pub err: u32, pub eip: u32,
    pub cs: u16, pub __csh: u16,
    pub eflags: u32,
    pub esp_at_signal: u32,
    pub ss: u16, pub __ssh: u16,
    pub fpstate: *mut _fpstate,
    pub oldmask: u32,
    pub cr2: u32,
}

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub union sigcontext_64__bindgen_ty_1 {
    pub ss: u16,
    pub __pad0: u16,
}

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub struct sigcontext {
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rdi: u64, pub rsi: u64, pub rbp: u64, pub rbx: u64,
    pub rdx: u64, pub rax: u64, pub rcx: u64, pub rsp: u64,
    pub rip: u64, pub eflags: u64,
    pub cs: u16,
    pub gs: u16,
    pub fs: u16,
    pub __bindgen_anon_1: sigcontext_64__bindgen_ty_1,
    pub err: u64, pub trapno: u64, pub oldmask: u64, pub cr2: u64,
    pub fpstate: *mut _fpstate,
    #[cfg(target_pointer_width = "32")]
    pub __fpstate_pad: u32,
    pub reserved1: [u64; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
