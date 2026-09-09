// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  PowerPC version
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 *
 *  Dave Engebretsen <engebret@us.ibm.com>
 *      Rework for PPC64 port.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut boot_cpuid: u32;
    fn smp_processor_id() -> u32;
    fn setup_kuap(disabled: bool);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn kasprintf(flags: u32, fmt: *const u8, ...) -> *mut i8;
    fn kmem_cache_create(
        name: *const i8,
        size: usize,
        align: usize,
        flags: u32,
        ctor: Option<unsafe extern "C" fn(*mut u8)>,
    ) -> *mut kmem_cache;
    fn kfree(ptr: *mut i8);
    fn panic(fmt: *const u8, ... ) -> !;
}

#[repr(C)]
pub struct pgd_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

type PhysAddr = u64;

extern "C" {
    static KERNELBASE: usize;
}

#[no_mangle]
pub static mut memstart_addr: PhysAddr = !0u64;
#[no_mangle]
pub static mut kernstart_addr: PhysAddr = 0;
#[no_mangle]
pub static mut kernstart_virt_addr: usize = 0;

#[no_mangle]
pub static mut disable_kuep: bool = true;
#[no_mangle]
pub static mut disable_kuap: bool = true;

#[cfg(CONFIG_KFENCE)]
#[no_mangle]
pub static mut kfence_disabled: bool = false;
#[cfg(CONFIG_KFENCE)]
#[no_mangle]
pub static mut kfence_early_init: bool = false;

unsafe extern "C" fn parse_nosmep(_p: *mut i8) -> i32 {
    // !IS_ENABLED(CONFIG_PPC_BOOK3S_64)
    if false {
        return 0;
    }

    disable_kuep = true;
    pr_warn(b"Disabling Kernel Userspace Execution Prevention\0".as_ptr());
    0
}

// early_param("nosmep", parse_nosmep);

unsafe extern "C" fn parse_nosmap(_p: *mut i8) -> i32 {
    disable_kuap = true;
    pr_warn(b"Disabling Kernel Userspace Access Protection\0".as_ptr());
    0
}

// early_param("nosmap", parse_nosmap);

#[no_mangle]
pub unsafe extern "C" fn setup_kuep(disabled: bool) {
    // !IS_ENABLED(CONFIG_PPC_KUEP)
    if disabled {
        return;
    }

    if smp_processor_id() != boot_cpuid {
        return;
    }

    pr_info(b"Activating Kernel Userspace Execution Prevention\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn setup_kup() {
    setup_kuap(disable_kuap);
    setup_kuep(disable_kuep);
}

macro_rules! define_ctor {
    ($name:ident, $shift:expr) => {
        unsafe extern "C" fn $name(addr: *mut u8) {
            core::ptr::write_bytes(
                addr,
                0,
                core::mem::size_of::<pgd_t>() << $shift,
            );
        }
    };
}

define_ctor!(ctor_0, 0);
define_ctor!(ctor_1, 1);
define_ctor!(ctor_2, 2);
define_ctor!(ctor_3, 3);
define_ctor!(ctor_4, 4);
define_ctor!(ctor_5, 5);
define_ctor!(ctor_6, 6);
define_ctor!(ctor_7, 7);
define_ctor!(ctor_8, 8);
define_ctor!(ctor_9, 9);
define_ctor!(ctor_10, 10);
define_ctor!(ctor_11, 11);
define_ctor!(ctor_12, 12);
define_ctor!(ctor_13, 13);
define_ctor!(ctor_14, 14);
define_ctor!(ctor_15, 15);

unsafe fn ctor(shift: i32) -> Option<unsafe extern "C" fn(*mut u8)> {
    // BUILD_BUG_ON(MAX_PGTABLE_INDEX_SIZE != 15);
    match shift {
        0 => Some(ctor_0),
        1 => Some(ctor_1),
        2 => Some(ctor_2),
        3 => Some(ctor_3),
        4 => Some(ctor_4),
        5 => Some(ctor_5),
        6 => Some(ctor_6),
        7 => Some(ctor_7),
        8 => Some(ctor_8),
        9 => Some(ctor_9),
        10 => Some(ctor_10),
        11 => Some(ctor_11),
        12 => Some(ctor_12),
        13 => Some(ctor_13),
        14 => Some(ctor_14),
        15 => Some(ctor_15),
        _ => None,
    }
}

pub const MAX_PGTABLE_INDEX_SIZE: usize = 15;
#[no_mangle]
pub static mut pgtable_cache: [*mut kmem_cache; MAX_PGTABLE_INDEX_SIZE + 1] =
    [core::ptr::null_mut(); MAX_PGTABLE_INDEX_SIZE + 1];

pub unsafe fn pgtable_cache_add(shift: u32) {
    let table_size = core::mem::size_of::<pgd_t>() << shift;
    let mut align = table_size;
    let minalign = MAX_PGTABLE_INDEX_SIZE + 1;
    let mut new: *mut kmem_cache = core::ptr::null_mut();

    assert!(minalign.is_power_of_two());
    assert!(shift as usize <= MAX_PGTABLE_INDEX_SIZE);

    if !pgtable_cache[shift as usize].is_null() {
        return;
    }

    align = core::cmp::max(align, minalign);
    let name = kasprintf(0, b"pgtable-2^%d\0".as_ptr(), shift);
    if !name.is_null() {
        new = kmem_cache_create(
            name,
            table_size,
            align,
            0,
            ctor(shift as i32),
        );
    }
    if new.is_null() {
        panic(b"Could not allocate pgtable cache for order %d\0".as_ptr(), shift);
    }

    kfree(name);
    pgtable_cache[shift as usize] = new;
    pr_debug(b"Allocated pgtable cache for order %d\n\0".as_ptr(), shift);
}

pub unsafe fn pgtable_cache_init() {
    pgtable_cache_add(PGD_INDEX_SIZE);

    if PMD_CACHE_INDEX != 0 {
        pgtable_cache_add(PMD_CACHE_INDEX);
    }

    if PUD_CACHE_INDEX != 0 {
        pgtable_cache_add(PUD_CACHE_INDEX);
    }
}

// Build-time constants supplied by the surrounding kernel translation.
extern "C" {
    static PGD_INDEX_SIZE: u32;
    static PMD_CACHE_INDEX: u32;
    static PUD_CACHE_INDEX: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
