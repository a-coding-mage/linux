/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by linux/io.h and asm-generic/int-ll64.h. */
extern "C" {
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn readl_relaxed(addr: *const u32) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u32);
    fn ioread32(addr: *const u32) -> u32;
    fn iowrite32(value: u32, addr: *mut u32);
    fn ioread32be(addr: *const u32) -> u32;
    fn iowrite32be(value: u32, addr: *mut u32);
}

pub unsafe fn hi_lo_readq(addr: *const u8) -> u64 {
    let p = addr as *const u32;
    let high = readl(p.add(1));
    let low = readl(p);
    low as u64 + ((high as u64) << 32)
}

pub unsafe fn hi_lo_writeq(val: u64, addr: *mut u8) {
    writel((val >> 32) as u32, addr.add(4) as *mut u32);
    writel(val as u32, addr as *mut u32);
}

pub unsafe fn hi_lo_readq_relaxed(addr: *const u8) -> u64 {
    let p = addr as *const u32;
    let high = readl_relaxed(p.add(1));
    let low = readl_relaxed(p);
    low as u64 + ((high as u64) << 32)
}

pub unsafe fn hi_lo_writeq_relaxed(val: u64, addr: *mut u8) {
    writel_relaxed((val >> 32) as u32, addr.add(4) as *mut u32);
    writel_relaxed(val as u32, addr as *mut u32);
}

pub unsafe fn readq(addr: *const u8) -> u64 {
    hi_lo_readq(addr)
}

pub unsafe fn writeq(val: u64, addr: *mut u8) {
    hi_lo_writeq(val, addr)
}

pub unsafe fn readq_relaxed(addr: *const u8) -> u64 {
    hi_lo_readq_relaxed(addr)
}

pub unsafe fn writeq_relaxed(val: u64, addr: *mut u8) {
    hi_lo_writeq_relaxed(val, addr)
}

pub unsafe fn ioread64_hi_lo(addr: *const u8) -> u64 {
    let high = ioread32(addr.add(core::mem::size_of::<u32>()) as *const u32);
    let low = ioread32(addr as *const u32);
    low as u64 + ((high as u64) << 32)
}

pub unsafe fn iowrite64_hi_lo(val: u64, addr: *mut u8) {
    iowrite32((val >> 32) as u32, addr.add(core::mem::size_of::<u32>()));
    iowrite32(val as u32, addr);
}

pub unsafe fn ioread64be_hi_lo(addr: *const u8) -> u64 {
    let high = ioread32be(addr as *const u32);
    let low = ioread32be(addr.add(core::mem::size_of::<u32>()) as *const u32);
    low as u64 + ((high as u64) << 32)
}

pub unsafe fn iowrite64be_hi_lo(val: u64, addr: *mut u8) {
    iowrite32be((val >> 32) as u32, addr);
    iowrite32be(val as u32, addr.add(core::mem::size_of::<u32>()));
}

/* CONFIG_GENERIC_IOMAP && CONFIG_64BIT may select the external __ioread64
 * and __iowrite64 variants; the header's local fallback is represented here. */
pub const ioread64_is_nonatomic: () = ();
pub unsafe fn ioread64(addr: *const u8) -> u64 {
    ioread64_hi_lo(addr)
}

pub const iowrite64_is_nonatomic: () = ();
pub unsafe fn iowrite64(val: u64, addr: *mut u8) {
    iowrite64_hi_lo(val, addr)
}

pub const ioread64be_is_nonatomic: () = ();
pub unsafe fn ioread64be(addr: *const u8) -> u64 {
    ioread64be_hi_lo(addr)
}

pub const iowrite64be_is_nonatomic: () = ();
pub unsafe fn iowrite64be(val: u64, addr: *mut u8) {
    iowrite64be_hi_lo(val, addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
