/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the maximum virtual address which can be DMA'd from.
 * When CONFIG_ZONE_DMA is disabled, this is 0xffffffffUL.  The
 * CONFIG_ZONE_DMA alternative is retained below as its source-level intent.
 */
#[cfg(not(feature = "CONFIG_ZONE_DMA"))]
pub const MAX_DMA_ADDRESS: usize = 0xffff_ffff;

#[cfg(feature = "CONFIG_ZONE_DMA")]
extern "C" {
    pub static mut arm_dma_zone_size: phys_addr_t;
}

#[cfg(feature = "CONFIG_ZONE_DMA")]
extern "C" {
    pub static mut arm_dma_limit: phys_addr_t;
}

#[cfg(feature = "CONFIG_ZONE_DMA")]
pub const ARCH_LOW_ADDRESS_LIMIT: *mut phys_addr_t = unsafe {
    &raw mut arm_dma_limit
};

/*
 * Under CONFIG_ZONE_DMA, MAX_DMA_ADDRESS is the value of:
 * arm_dma_zone_size && arm_dma_zone_size < (0x100000000ULL - PAGE_OFFSET)
 *     ? (PAGE_OFFSET + arm_dma_zone_size) : 0xffffffffUL.
 * It is kept as a build-time expression because PAGE_OFFSET and
 * phys_addr_t are supplied by the surrounding architecture.
 */

#[cfg(feature = "CONFIG_ISA_DMA_API")]
pub const DMA_MODE_MASK: u32 = 0xcc;
#[cfg(feature = "CONFIG_ISA_DMA_API")]
pub const DMA_MODE_READ: u32 = 0x44;
#[cfg(feature = "CONFIG_ISA_DMA_API")]
pub const DMA_MODE_WRITE: u32 = 0x48;
#[cfg(feature = "CONFIG_ISA_DMA_API")]
pub const DMA_MODE_CASCADE: u32 = 0xc0;
#[cfg(feature = "CONFIG_ISA_DMA_API")]
pub const DMA_AUTOINIT: u32 = 0x10;

#[cfg(feature = "CONFIG_ISA_DMA_API")]
extern "C" {
    pub static mut dma_spin_lock: raw_spinlock_t;

    pub fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    pub fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);

    pub fn set_dma_page(chan: c_uint, pagenr: c_char);
    pub fn request_dma(chan: c_uint, device_id: *const c_char) -> c_int;
    pub fn free_dma(chan: c_uint);
    pub fn enable_dma(chan: c_uint);
    pub fn disable_dma(chan: c_uint);
    pub fn dma_channel_active(chan: c_uint) -> c_int;
    pub fn set_dma_sg(chan: c_uint, sg: *mut scatterlist, nr_sg: c_int);
    pub fn __set_dma_addr(chan: c_uint, addr: *mut c_void);
    pub fn set_dma_count(chan: c_uint, count: c_ulong);
    pub fn set_dma_mode(chan: c_uint, mode: c_uint);
    pub fn set_dma_speed(chan: c_uint, cycle_ns: c_int);
    pub fn get_dma_residue(chan: c_uint) -> c_int;
    pub fn isa_bus_to_virt(addr: c_ulong) -> *mut c_void;
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn claim_dma_lock() -> c_ulong {
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&raw mut dma_spin_lock, &mut flags);
    flags
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn release_dma_lock(flags: c_ulong) {
    raw_spin_unlock_irqrestore(&raw mut dma_spin_lock, flags);
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn clear_dma_ff(_chan: c_uint) {
    /* The C macro intentionally expands to nothing. */
}

#[cfg(feature = "CONFIG_ISA_DMA_API")]
#[inline]
pub unsafe fn set_dma_addr(chan: c_uint, addr: c_ulong) {
    __set_dma_addr(chan, isa_bus_to_virt(addr));
}

#[cfg(all(feature = "CONFIG_ISA_DMA_API", not(feature = "NO_DMA")))]
pub const NO_DMA: c_uint = 255;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
