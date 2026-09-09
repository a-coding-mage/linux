// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies supplied by the surrounding kernel translation are omitted.

use core::ffi::c_void;

/* address of OFW callback interface; will be NULL if OFW isn't found */
static mut olpc_ofw_cif: Option<unsafe extern "C" fn(*mut i32) -> i32> = None;

/* page dir entry containing OFW's pgdir table; filled in by head_32.S */
static mut olpc_ofw_pgd: u32 = 0;

extern "C" {
    static mut ofw_lock: c_void;
    static mut boot_params: crate::boot_params;
    static mut swapper_pg_dir: crate::pgd_t;

    fn early_ioremap(addr: usize, size: usize) -> *mut crate::pgd_t;
    fn early_iounmap(addr: *mut crate::pgd_t, size: usize);
    fn set_pgd(dst: *mut crate::pgd_t, value: crate::pgd_t);
    fn reserve_top_address(addr: usize);
}

const MAXARGS: usize = 10;
const OFW_MIN: usize = 0xff000000;
const OFW_BOUND: usize = 1 << 20;

pub unsafe fn setup_olpc_ofw_pgd() {
    let mut base: *mut crate::pgd_t;
    let ofw_pde: *mut crate::pgd_t;

    if olpc_ofw_cif.is_none() {
        return;
    }

    /* fetch OFW's PDE */
    base = early_ioremap(
        olpc_ofw_pgd as usize,
        core::mem::size_of::<u32>() * crate::PTRS_PER_PGD,
    );
    if base.is_null() {
        crate::printk(crate::KERN_ERR, "failed to remap OFW's pgd - disabling OFW!\n");
        olpc_ofw_cif = None;
        return;
    }
    ofw_pde = base.add(crate::OLPC_OFW_PDE_NR);

    /* install OFW's PDE permanently into the kernel's pgtable */
    set_pgd(
        swapper_pg_dir.add(crate::OLPC_OFW_PDE_NR),
        *ofw_pde,
    );
    /* implicit optimization barrier here due to uninline function return */

    early_iounmap(
        base,
        core::mem::size_of::<u32>() * crate::PTRS_PER_PGD,
    );
}

pub unsafe fn __olpc_ofw(
    name: *const core::ffi::c_char,
    nr_args: i32,
    args: *const *const c_void,
    nr_res: i32,
    res: *mut *mut c_void,
) -> i32 {
    let mut ofw_args = [0i32; MAXARGS + 3];
    let mut flags: usize = 0;
    let mut ret: i32;
    let mut i: i32;
    let mut p: *mut i32;

    crate::BUG_ON((nr_args + nr_res) as usize > MAXARGS);

    let cif = match olpc_ofw_cif {
        Some(cif) => cif,
        None => return -crate::EIO,
    };

    ofw_args[0] = name as usize as i32;
    ofw_args[1] = nr_args;
    ofw_args[2] = nr_res;

    p = ofw_args.as_mut_ptr().add(3);
    i = 0;
    while i < nr_args {
        *p = (*args.add(i as usize)) as usize as i32;
        i += 1;
        p = p.add(1);
    }

    /* call into ofw */
    crate::spin_lock_irqsave(&mut ofw_lock, &mut flags);
    ret = cif(ofw_args.as_mut_ptr());
    crate::spin_unlock_irqrestore(&mut ofw_lock, flags);

    if ret == 0 {
        i = 0;
        while i < nr_res {
            *(res.add(i as usize) as *mut *mut i32).read() = *p;
            i += 1;
            p = p.add(1);
        }
    }

    ret
}

pub unsafe fn olpc_ofw_present() -> bool {
    olpc_ofw_cif.is_some()
}

pub unsafe fn olpc_ofw_detect() {
    let hdr = &boot_params.olpc_ofw_header;
    let mut start: usize;

    /* ensure OFW booted us by checking for "OFW " string */
    if hdr.ofw_magic != crate::OLPC_OFW_SIG {
        return;
    }

    olpc_ofw_cif = Some(core::mem::transmute(hdr.cif_handler));

    if (hdr.cif_handler as usize) < OFW_MIN {
        crate::printk(
            crate::KERN_ERR,
            "OFW detected, but cif has invalid address 0x%lx - disabling.\n",
        );
        olpc_ofw_cif = None;
        return;
    }

    /* determine where OFW starts in memory */
    start = (hdr.cif_handler as usize) & !(OFW_BOUND - 1);
    crate::printk(
        crate::KERN_INFO,
        "OFW detected in memory, cif @ 0x%lx (reserving top %ldMB)\n",
    );
    reserve_top_address((0usize.wrapping_sub(start)));
}

pub unsafe fn olpc_ofw_is_installed() -> bool {
    olpc_ofw_cif.is_some()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
