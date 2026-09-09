/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the SPARC UAPI ptrace header. */

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub const PT_REGS_MAGIC: u32 = 0x57ac6c00;

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct pt_regs {
    pub u_regs: [usize; 16],
    pub tstate: usize,
    pub tpc: usize,
    pub tnpc: usize,
    pub y: u32,
    pub magic: u32,
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct pt_regs32 { pub psr: u32, pub pc: u32, pub npc: u32, pub y: u32, pub u_regs: [u32; 16] }

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct reg_window { pub locals: [usize; 8], pub ins: [usize; 8] }

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct reg_window32 { pub locals: [u32; 8], pub ins: [u32; 8] }

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct sparc_stackf {
    pub locals: [usize; 8], pub ins: [usize; 6], pub fp: *mut sparc_stackf,
    pub callers_pc: usize, pub structptr: *mut i8, pub xargs: [usize; 6], pub xxargs: [usize; 1],
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct sparc_stackf32 {
    pub locals: [u32; 8], pub ins: [u32; 6], pub fp: u32, pub callers_pc: u32,
    pub structptr: u32, pub xargs: [u32; 6], pub xxargs: [u32; 1],
}

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[repr(C)]
pub struct sparc_trapf { pub locals: [usize; 8], pub ins: [usize; 8], pub _unused: usize, pub regs: *mut pt_regs }

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct pt_regs { pub psr: usize, pub pc: usize, pub npc: usize, pub y: usize, pub u_regs: [usize; 16] }

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct reg_window32 { pub locals: [usize; 8], pub ins: [usize; 8] }

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[repr(C)]
pub struct sparc_stackf {
    pub locals: [usize; 8], pub ins: [usize; 6], pub fp: *mut sparc_stackf,
    pub callers_pc: usize, pub structptr: *mut i8, pub xargs: [usize; 6], pub xxargs: [usize; 1],
}

/* Header-size macros are represented as compile-time constants. */
pub const TRACEREG_SZ: usize = core::mem::size_of::<pt_regs>();
pub const STACKFRAME_SZ: usize = core::mem::size_of::<sparc_stackf>();
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub const TRACEREG32_SZ: usize = core::mem::size_of::<pt_regs32>();
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
pub const STACKFRAME32_SZ: usize = core::mem::size_of::<sparc_stackf32>();

pub const UREG_G0: usize = 0; pub const UREG_G1: usize = 1; pub const UREG_G2: usize = 2; pub const UREG_G3: usize = 3;
pub const UREG_G4: usize = 4; pub const UREG_G5: usize = 5; pub const UREG_G6: usize = 6; pub const UREG_G7: usize = 7;
pub const UREG_I0: usize = 8; pub const UREG_I1: usize = 9; pub const UREG_I2: usize = 10; pub const UREG_I3: usize = 11;
pub const UREG_I4: usize = 12; pub const UREG_I5: usize = 13; pub const UREG_I6: usize = 14; pub const UREG_I7: usize = 15;
pub const UREG_FP: usize = UREG_I6; pub const UREG_RETPC: usize = UREG_I7;

pub const PT_V9_G0: usize = 0x00; pub const PT_V9_G1: usize = 0x08; pub const PT_V9_G2: usize = 0x10; pub const PT_V9_G3: usize = 0x18;
pub const PT_V9_G4: usize = 0x20; pub const PT_V9_G5: usize = 0x28; pub const PT_V9_G6: usize = 0x30; pub const PT_V9_G7: usize = 0x38;
pub const PT_V9_I0: usize = 0x40; pub const PT_V9_I1: usize = 0x48; pub const PT_V9_I2: usize = 0x50; pub const PT_V9_I3: usize = 0x58;
pub const PT_V9_I4: usize = 0x60; pub const PT_V9_I5: usize = 0x68; pub const PT_V9_I6: usize = 0x70; pub const PT_V9_FP: usize = PT_V9_I6;
pub const PT_V9_I7: usize = 0x78; pub const PT_V9_TSTATE: usize = 0x80; pub const PT_V9_TPC: usize = 0x88; pub const PT_V9_TNPC: usize = 0x90;
pub const PT_V9_Y: usize = 0x98; pub const PT_V9_MAGIC: usize = 0x9c; pub const PT_TSTATE: usize = PT_V9_TSTATE; pub const PT_TPC: usize = PT_V9_TPC; pub const PT_TNPC: usize = PT_V9_TNPC;

pub const PT_PSR: usize = 0x0; pub const PT_PC: usize = 0x4; pub const PT_NPC: usize = 0x8; pub const PT_Y: usize = 0xc;
pub const PT_G0: usize = 0x10; pub const PT_WIM: usize = PT_G0; pub const PT_G1: usize = 0x14; pub const PT_G2: usize = 0x18; pub const PT_G3: usize = 0x1c;
pub const PT_G4: usize = 0x20; pub const PT_G5: usize = 0x24; pub const PT_G6: usize = 0x28; pub const PT_G7: usize = 0x2c;
pub const PT_I0: usize = 0x30; pub const PT_I1: usize = 0x34; pub const PT_I2: usize = 0x38; pub const PT_I3: usize = 0x3c;
pub const PT_I4: usize = 0x40; pub const PT_I5: usize = 0x44; pub const PT_I6: usize = 0x48; pub const PT_FP: usize = PT_I6; pub const PT_I7: usize = 0x4c;

/* Register-window and stack-frame offsets. */
pub const RW_V9_L0: usize=0x00; pub const RW_V9_L1: usize=0x08; pub const RW_V9_L2: usize=0x10; pub const RW_V9_L3: usize=0x18; pub const RW_V9_L4: usize=0x20; pub const RW_V9_L5: usize=0x28; pub const RW_V9_L6: usize=0x30; pub const RW_V9_L7: usize=0x38;
pub const RW_V9_I0: usize=0x40; pub const RW_V9_I1: usize=0x48; pub const RW_V9_I2: usize=0x50; pub const RW_V9_I3: usize=0x58; pub const RW_V9_I4: usize=0x60; pub const RW_V9_I5: usize=0x68; pub const RW_V9_I6: usize=0x70; pub const RW_V9_I7: usize=0x78;
pub const RW_L0: usize=0x00; pub const RW_L1: usize=0x04; pub const RW_L2: usize=0x08; pub const RW_L3: usize=0x0c; pub const RW_L4: usize=0x10; pub const RW_L5: usize=0x14; pub const RW_L6: usize=0x18; pub const RW_L7: usize=0x1c;
pub const RW_I0: usize=0x20; pub const RW_I1: usize=0x24; pub const RW_I2: usize=0x28; pub const RW_I3: usize=0x2c; pub const RW_I4: usize=0x30; pub const RW_I5: usize=0x34; pub const RW_I6: usize=0x38; pub const RW_I7: usize=0x3c;

pub const SF_V9_L0: usize=0x00; pub const SF_V9_L1: usize=0x08; pub const SF_V9_L2: usize=0x10; pub const SF_V9_L3: usize=0x18; pub const SF_V9_L4: usize=0x20; pub const SF_V9_L5: usize=0x28; pub const SF_V9_L6: usize=0x30; pub const SF_V9_L7: usize=0x38;
pub const SF_V9_I0: usize=0x40; pub const SF_V9_I1: usize=0x48; pub const SF_V9_I2: usize=0x50; pub const SF_V9_I3: usize=0x58; pub const SF_V9_I4: usize=0x60; pub const SF_V9_I5: usize=0x68; pub const SF_V9_FP: usize=0x70; pub const SF_V9_PC: usize=0x78; pub const SF_V9_RETP: usize=0x80;
pub const SF_V9_XARG0: usize=0x88; pub const SF_V9_XARG1: usize=0x90; pub const SF_V9_XARG2: usize=0x98; pub const SF_V9_XARG3: usize=0xa0; pub const SF_V9_XARG4: usize=0xa8; pub const SF_V9_XARG5: usize=0xb0; pub const SF_V9_XXARG: usize=0xb8;
pub const SF_L0: usize=0x00; pub const SF_L1: usize=0x04; pub const SF_L2: usize=0x08; pub const SF_L3: usize=0x0c; pub const SF_L4: usize=0x10; pub const SF_L5: usize=0x14; pub const SF_L6: usize=0x18; pub const SF_L7: usize=0x1c;
pub const SF_I0: usize=0x20; pub const SF_I1: usize=0x24; pub const SF_I2: usize=0x28; pub const SF_I3: usize=0x2c; pub const SF_I4: usize=0x30; pub const SF_I5: usize=0x34; pub const SF_FP: usize=0x38; pub const SF_PC: usize=0x3c; pub const SF_RETP: usize=0x40;
pub const SF_XARG0: usize=0x44; pub const SF_XARG1: usize=0x48; pub const SF_XARG2: usize=0x4c; pub const SF_XARG3: usize=0x50; pub const SF_XARG4: usize=0x54; pub const SF_XARG5: usize=0x58; pub const SF_XXARG: usize=0x5c;

pub const PTRACE_SPARC_DETACH: usize=11; pub const PTRACE_GETREGS: usize=12; pub const PTRACE_SETREGS: usize=13; pub const PTRACE_GETFPREGS: usize=14; pub const PTRACE_SETFPREGS: usize=15;
pub const PTRACE_READDATA: usize=16; pub const PTRACE_WRITEDATA: usize=17; pub const PTRACE_READTEXT: usize=18; pub const PTRACE_WRITETEXT: usize=19; pub const PTRACE_GETFPAREGS: usize=20; pub const PTRACE_SETFPAREGS: usize=21;
pub const PTRACE_GETREGS64: usize=22; pub const PTRACE_SETREGS64: usize=23; pub const PTRACE_GETFPREGS64: usize=25; pub const PTRACE_SETFPREGS64: usize=26;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
