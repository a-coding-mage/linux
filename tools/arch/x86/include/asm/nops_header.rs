// SPDX-License-Identifier: GPL-2.0

// Translated from arch/x86/include/asm/nops.h.
// Dependency intent from C header: <asm/asm.h> provides _ASM_BYTES(...).

/*
 * Define nops for use with alternative() and for tracing.
 */

/*
 * C condition: #ifndef CONFIG_64BIT
 *
 * Generic 32bit nops from GAS:
 *
 * 1: nop
 * 2: movl %esi,%esi
 * 3: leal 0x0(%esi),%esi
 * 4: leal 0x0(%esi,%eiz,1),%esi
 * 5: leal %ds:0x0(%esi,%eiz,1),%esi
 * 6: leal 0x0(%esi),%esi
 * 7: leal 0x0(%esi,%eiz,1),%esi
 * 8: leal %ds:0x0(%esi,%eiz,1),%esi
 *
 * Except 5 and 8, which are DS prefixed 4 and 7 resp, where GAS would emit 2
 * nop instructions.
 */
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP1: &[u8] = &[0x90];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP2: &[u8] = &[0x89, 0xf6];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP3: &[u8] = &[0x8d, 0x76, 0x00];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP4: &[u8] = &[0x8d, 0x74, 0x26, 0x00];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP5: &[u8] = &[0x3e, 0x8d, 0x74, 0x26, 0x00];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP6: &[u8] = &[0x8d, 0xb6, 0x00, 0x00, 0x00, 0x00];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP7: &[u8] = &[0x8d, 0xb4, 0x26, 0x00, 0x00, 0x00, 0x00];
#[cfg(not(target_pointer_width = "64"))]
pub const BYTES_NOP8: &[u8] = &[0x3e, 0x8d, 0xb4, 0x26, 0x00, 0x00, 0x00, 0x00];

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP_MAX: usize = 8;

/*
 * C condition: #else / CONFIG_64BIT
 *
 * Generic 64bit nops from GAS:
 *
 * 1: nop
 * 2: osp nop
 * 3: nopl (%eax)
 * 4: nopl 0x00(%eax)
 * 5: nopl 0x00(%eax,%eax,1)
 * 6: osp nopl 0x00(%eax,%eax,1)
 * 7: nopl 0x00000000(%eax)
 * 8: nopl 0x00000000(%eax,%eax,1)
 * 9: cs nopl 0x00000000(%eax,%eax,1)
 * 10: osp cs nopl 0x00000000(%eax,%eax,1)
 * 11: osp osp cs nopl 0x00000000(%eax,%eax,1)
 */
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP1: &[u8] = &[0x90];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP2: &[u8] = &[0x66, 0x90];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP3: &[u8] = &[0x0f, 0x1f, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP4: &[u8] = &[0x0f, 0x1f, 0x40, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP5: &[u8] = &[0x0f, 0x1f, 0x44, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP6: &[u8] = &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP7: &[u8] = &[0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP8: &[u8] = &[0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP9: &[u8] = &[0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP10: &[u8] = &[0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
#[cfg(target_pointer_width = "64")]
pub const BYTES_NOP11: &[u8] = &[0x66, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];

#[cfg(target_pointer_width = "64")]
pub const ASM_NOP9: &str = ".byte 0x2e,0x0f,0x1f,0x84,0x00,0x00,0x00,0x00,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP10: &str = ".byte 0x66,0x2e,0x0f,0x1f,0x84,0x00,0x00,0x00,0x00,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP11: &str = ".byte 0x66,0x66,0x2e,0x0f,0x1f,0x84,0x00,0x00,0x00,0x00,0x00";

#[cfg(target_pointer_width = "64")]
pub const ASM_NOP_MAX: usize = 11;

pub const ASM_NOP1: &str = ".byte 0x90";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP2: &str = ".byte 0x89,0xf6";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP2: &str = ".byte 0x66,0x90";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP3: &str = ".byte 0x8d,0x76,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP3: &str = ".byte 0x0f,0x1f,0x00";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP4: &str = ".byte 0x8d,0x74,0x26,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP4: &str = ".byte 0x0f,0x1f,0x40,0x00";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP5: &str = ".byte 0x3e,0x8d,0x74,0x26,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP5: &str = ".byte 0x0f,0x1f,0x44,0x00,0x00";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP6: &str = ".byte 0x8d,0xb6,0x00,0x00,0x00,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP6: &str = ".byte 0x66,0x0f,0x1f,0x44,0x00,0x00";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP7: &str = ".byte 0x8d,0xb4,0x26,0x00,0x00,0x00,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP7: &str = ".byte 0x0f,0x1f,0x80,0x00,0x00,0x00,0x00";

#[cfg(not(target_pointer_width = "64"))]
pub const ASM_NOP8: &str = ".byte 0x3e,0x8d,0xb4,0x26,0x00,0x00,0x00,0x00";
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP8: &str = ".byte 0x0f,0x1f,0x84,0x00,0x00,0x00,0x00,0x00";

// C condition: #ifndef __ASSEMBLER__
unsafe extern "C" {
    pub static x86_nops: *const *const ::core::ffi::c_uchar;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
