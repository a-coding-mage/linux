/* SPDX-License-Identifier: GPL-2.0 */

/* Command byte definitions tests and functions */

/* mask of bits that actually matter in a command byte */
pub const gpib_command_mask: u32 = 0x7f;

/* Possible GPIB command messages */

pub const GTL: u8 = 0x1; /* go to local */
pub const SDC: u8 = 0x4; /* selected device clear */
pub const PP_CONFIG: u8 = 0x5;
pub const GET: u8 = 0x8; /* group execute trigger */
pub const TCT: u8 = 0x9; /* take control */
pub const LLO: u8 = 0x11; /* local lockout */
pub const DCL: u8 = 0x14; /* device clear */
pub const PPU: u8 = 0x15; /* parallel poll unconfigure */
pub const SPE: u8 = 0x18; /* serial poll enable */
pub const SPD: u8 = 0x19; /* serial poll disable */
pub const CFE: u8 = 0x1f; /* configure enable */
pub const LAD: u8 = 0x20; /* value to be 'ored' in to obtain listen address */
pub const UNL: u8 = 0x3F; /* unlisten */
pub const TAD: u8 = 0x40; /* value to be 'ored' in to obtain talk address */
pub const UNT: u8 = 0x5F; /* untalk */
pub const SAD: u8 = 0x60; /* my secondary address (base) */
pub const PPE: u8 = 0x60; /* parallel poll enable (base) */
pub const PPD: u8 = 0x70; /* parallel poll disable */

/* confine address to range 0 to 30. */
#[inline]
pub fn gpib_address_restrict(mut addr: u32) -> u32 {
    addr &= 0x1f;
    if addr == 0x1f {
        addr = 0;
    }
    addr
}

#[inline]
pub fn MLA(addr: u32) -> u8 {
    (gpib_address_restrict(addr) | LAD as u32) as u8
}

#[inline]
pub fn MTA(addr: u32) -> u8 {
    (gpib_address_restrict(addr) | TAD as u32) as u8
}

#[inline]
pub fn MSA(addr: u32) -> u8 {
    ((addr & 0x1f) | SAD as u32) as u8
}

#[inline]
pub fn gpib_address_equal(pad1: u32, sad1: i32, pad2: u32, sad2: i32) -> i32 {
    if pad1 == pad2 {
        if sad1 == sad2 {
            return 1;
        }
        if sad1 < 0 && sad2 < 0 {
            return 1;
        }
    }

    0
}

#[inline]
pub fn is_PPE(command: u8) -> i32 {
    ((command & 0x70) == 0x60) as i32
}

#[inline]
pub fn is_PPD(command: u8) -> i32 {
    ((command & 0x70) == 0x70) as i32
}

#[inline]
pub fn in_addressed_command_group(command: u8) -> i32 {
    ((command & 0x70) == 0x0) as i32
}

#[inline]
pub fn in_universal_command_group(command: u8) -> i32 {
    ((command & 0x70) == 0x10) as i32
}

#[inline]
pub fn in_listen_address_group(command: u8) -> i32 {
    ((command & 0x60) == 0x20) as i32
}

#[inline]
pub fn in_talk_address_group(command: u8) -> i32 {
    ((command & 0x60) == 0x40) as i32
}

#[inline]
pub fn in_primary_command_group(command: u8) -> i32 {
    (in_addressed_command_group(command) != 0
        || in_universal_command_group(command) != 0
        || in_listen_address_group(command) != 0
        || in_talk_address_group(command) != 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
