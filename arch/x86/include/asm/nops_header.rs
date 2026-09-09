/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Define nops for use with alternative() and for tracing.
 *
 * The original header selects the 32-bit or 64-bit definitions according to
 * CONFIG_64BIT.  Rust target configuration provides the corresponding
 * conditional selection here.
 */

#[cfg(not(target_pointer_width = "64"))]
{
    pub const BYTES_NOP1: &[u8] = &[0x90];
    pub const BYTES_NOP2: &[u8] = &[0x89, 0xf6];
    pub const BYTES_NOP3: &[u8] = &[0x8d, 0x76, 0x00];
    pub const BYTES_NOP4: &[u8] = &[0x8d, 0x74, 0x26, 0x00];
    pub const BYTES_NOP5: &[u8] = &[0x3e, 0x8d, 0x74, 0x26, 0x00];
    pub const BYTES_NOP6: &[u8] = &[0x8d, 0xb6, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP7: &[u8] = &[0x8d, 0xb4, 0x26, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP8: &[u8] = &[0x3e, 0x8d, 0xb4, 0x26, 0x00, 0x00, 0x00, 0x00];

    pub const ASM_NOP_MAX: usize = 8;
}

#[cfg(target_pointer_width = "64")]
{
    pub const BYTES_NOP1: &[u8] = &[0x90];
    pub const BYTES_NOP2: &[u8] = &[0x66, 0x90];
    pub const BYTES_NOP3: &[u8] = &[0x0f, 0x1f, 0x00];
    pub const BYTES_NOP4: &[u8] = &[0x0f, 0x1f, 0x40, 0x00];
    pub const BYTES_NOP5: &[u8] = &[0x0f, 0x1f, 0x44, 0x00, 0x00];
    pub const BYTES_NOP6: &[u8] = &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00];
    pub const BYTES_NOP7: &[u8] = &[0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP8: &[u8] = &[0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP9: &[u8] = &[0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP10: &[u8] = &[0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
    pub const BYTES_NOP11: &[u8] = &[0x66, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];

    pub const ASM_NOP_MAX: usize = 11;
}

#[cfg(target_pointer_width = "64")]
pub const ASM_NOP9: &[u8] = BYTES_NOP9;
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP10: &[u8] = BYTES_NOP10;
#[cfg(target_pointer_width = "64")]
pub const ASM_NOP11: &[u8] = BYTES_NOP11;

pub const ASM_NOP1: &[u8] = BYTES_NOP1;
pub const ASM_NOP2: &[u8] = BYTES_NOP2;
pub const ASM_NOP3: &[u8] = BYTES_NOP3;
pub const ASM_NOP4: &[u8] = BYTES_NOP4;
pub const ASM_NOP5: &[u8] = BYTES_NOP5;
pub const ASM_NOP6: &[u8] = BYTES_NOP6;
pub const ASM_NOP7: &[u8] = BYTES_NOP7;
pub const ASM_NOP8: &[u8] = BYTES_NOP8;

/* Declaration of the externally defined x86 NOP table. */
#[cfg(not(feature = "assembler"))]
unsafe extern "C" {
    pub static x86_nops: [*const u8; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
