/* SPDX-License-Identifier: GPL-2.0 */

// UPA level registers and defines.

// UPA Config Register
pub const UPA_CONFIG_RESV: u64 = 0xffffffffc0000000; // Reserved.
pub const UPA_CONFIG_PCON: u64 = 0x000000003fc00000; // Depth of various sys queues.
pub const UPA_CONFIG_MID: u64 = 0x00000000003e0000; // Module ID.
pub const UPA_CONFIG_PCAP: u64 = 0x000000000001ffff; // Port Capabilities.

// UPA Port ID Register
pub const UPA_PORTID_FNP: u64 = 0xff00000000000000; // Hardcoded to 0xfc on ultra.
pub const UPA_PORTID_RESV: u64 = 0x00fffff800000000; // Reserved.
pub const UPA_PORTID_ECCVALID: u64 = 0x0000000400000000; // Zero if mod can generate ECC
pub const UPA_PORTID_ONEREAD: u64 = 0x0000000200000000; // Set if mod generates P_RASB
pub const UPA_PORTID_PINTRDQ: u64 = 0x0000000180000000; // # outstanding P_INT_REQ's
pub const UPA_PORTID_PREQDQ: u64 = 0x000000007e000000; // slave-wr's to mod supported
pub const UPA_PORTID_PREQRD: u64 = 0x0000000001e00000; // # incoming P_REQ's supported
pub const UPA_PORTID_UPACAP: u64 = 0x00000000001f0000; // UPA capabilities of mod
pub const UPA_PORTID_ID: u64 = 0x000000000000ffff; // Module Identification bits

// UPA I/O space accessors. The original definitions are enabled only for
// __KERNEL__ and non-assembler builds.

#[inline]
pub unsafe fn _upa_readb(addr: usize) -> u8 {
    let ret: u8;
    core::arch::asm!("lduba\t[{addr}] {asi}, {ret}\t/* upa_readb */",
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
        ret = lateout(reg) ret,
    );
    ret
}

#[inline]
pub unsafe fn _upa_readw(addr: usize) -> u16 {
    let ret: u16;
    core::arch::asm!("lduha\t[{addr}] {asi}, {ret}\t/* upa_readw */",
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
        ret = lateout(reg) ret,
    );
    ret
}

#[inline]
pub unsafe fn _upa_readl(addr: usize) -> u32 {
    let ret: u32;
    core::arch::asm!("lduwa\t[{addr}] {asi}, {ret}\t/* upa_readl */",
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
        ret = lateout(reg) ret,
    );
    ret
}

#[inline]
pub unsafe fn _upa_readq(addr: usize) -> usize {
    let ret: usize;
    core::arch::asm!("ldxa\t[{addr}] {asi}, {ret}\t/* upa_readq */",
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
        ret = lateout(reg) ret,
    );
    ret
}

#[inline]
pub unsafe fn _upa_writeb(b: u8, addr: usize) {
    core::arch::asm!("stba\t{b}, [{addr}] {asi}\t/* upa_writeb */",
        b = in(reg) b,
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
    );
}

#[inline]
pub unsafe fn _upa_writew(w: u16, addr: usize) {
    core::arch::asm!("stha\t{w}, [{addr}] {asi}\t/* upa_writew */",
        w = in(reg) w,
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
    );
}

#[inline]
pub unsafe fn _upa_writel(l: u32, addr: usize) {
    core::arch::asm!("stwa\t{l}, [{addr}] {asi}\t/* upa_writel */",
        l = in(reg) l,
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
    );
}

#[inline]
pub unsafe fn _upa_writeq(q: usize, addr: usize) {
    core::arch::asm!("stxa\t{q}, [{addr}] {asi}\t/* upa_writeq */",
        q = in(reg) q,
        addr = in(reg) addr,
        asi = const ASI_PHYS_BYPASS_EC_E,
    );
}

#[inline]
pub unsafe fn upa_readb(addr: impl Into<usize>) -> u8 {
    _upa_readb(addr.into())
}

#[inline]
pub unsafe fn upa_readw(addr: impl Into<usize>) -> u16 {
    _upa_readw(addr.into())
}

#[inline]
pub unsafe fn upa_readl(addr: impl Into<usize>) -> u32 {
    _upa_readl(addr.into())
}

#[inline]
pub unsafe fn upa_readq(addr: impl Into<usize>) -> usize {
    _upa_readq(addr.into())
}

#[inline]
pub unsafe fn upa_writeb(b: u8, addr: impl Into<usize>) {
    _upa_writeb(b, addr.into())
}

#[inline]
pub unsafe fn upa_writew(w: u16, addr: impl Into<usize>) {
    _upa_writew(w, addr.into())
}

#[inline]
pub unsafe fn upa_writel(l: u32, addr: impl Into<usize>) {
    _upa_writel(l, addr.into())
}

#[inline]
pub unsafe fn upa_writeq(q: usize, addr: impl Into<usize>) {
    _upa_writeq(q, addr.into())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
