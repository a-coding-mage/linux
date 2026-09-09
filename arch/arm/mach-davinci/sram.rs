// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * mach-davinci/sram.c - DaVinci simple SRAM allocator
 *
 * Copyright (C) 2009 David Brownell
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

type SizeT = usize;
type DmaAddrT = u64;
type PhysAddrT = u64;

#[repr(C)]
pub struct GenPool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DavinciSocInfo {
    pub sram_dma: DmaAddrT,
    pub sram_len: u32,
}

extern "C" {
    static mut davinci_soc_info: DavinciSocInfo;

    fn gen_pool_dma_alloc(pool: *mut GenPool, size: SizeT, dma: *mut DmaAddrT) -> *mut c_void;
    fn gen_pool_free(pool: *mut GenPool, addr: usize, size: SizeT);
    fn gen_pool_create(order: u32, nid: i32) -> *mut GenPool;
    fn gen_pool_add_virt(
        pool: *mut GenPool,
        virt: usize,
        phys: PhysAddrT,
        size: SizeT,
        nid: i32,
    ) -> i32;
    fn ioremap(phys: PhysAddrT, size: SizeT) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn ilog2(value: u32) -> u32;
    fn min_t_unsigned(a: u32, b: u32) -> u32;
    fn warn_on(condition: bool);
}

const SRAM_SIZE: u32 = 0; // Defined by sram.h.
const SRAM_GRANULARITY: u32 = 0; // Defined by sram.h.

static mut sram_pool: *mut GenPool = core::ptr::null_mut();

pub unsafe extern "C" fn sram_get_gen_pool() -> *mut GenPool
{
	return sram_pool;
}

pub unsafe extern "C" fn sram_alloc(len: SizeT, dma: *mut DmaAddrT) -> *mut c_void
{
	let dma_base: DmaAddrT = davinci_soc_info.sram_dma;

	if !dma.is_null() {
		*dma = 0;
	}
	if sram_pool.is_null() || (!dma.is_null() && dma_base == 0) {
		return core::ptr::null_mut();
	}

	return gen_pool_dma_alloc(sram_pool, len, dma);
}

pub unsafe extern "C" fn sram_free(addr: *mut c_void, len: SizeT)
{
	gen_pool_free(sram_pool, addr as usize, len);
}

/*
 * REVISIT This supports CPU and DMA access to/from SRAM, but it
 * doesn't (yet?) support some other notable uses of SRAM:  as TCM
 * for data and/or instructions; and holding code needed to enter
 * and exit suspend states (while DRAM can't be used).
 */
pub unsafe extern "C" fn sram_init() -> i32
{
	let phys: PhysAddrT = davinci_soc_info.sram_dma;
	let mut len: u32 = davinci_soc_info.sram_len;
	let mut status: i32 = 0;
	let mut addr: *mut c_void;

	if len != 0 {
		len = min_t_unsigned(len, SRAM_SIZE);
		sram_pool = gen_pool_create(ilog2(SRAM_GRANULARITY), -1);
		if sram_pool.is_null() {
			status = -12; // -ENOMEM
		}
	}

	if !sram_pool.is_null() {
		addr = ioremap(phys, len as SizeT);
		if addr.is_null() {
			return -12; // -ENOMEM
		}
		status = gen_pool_add_virt(sram_pool, addr as usize, phys, len as SizeT, -1);
		if status < 0 {
			iounmap(addr);
		}
	}

	warn_on(status < 0);
	return status;
}

// EXPORT_SYMBOL(sram_alloc)
// EXPORT_SYMBOL(sram_free)
// core_initcall(sram_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
