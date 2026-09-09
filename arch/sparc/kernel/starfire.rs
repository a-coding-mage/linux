// SPDX-License-Identifier: GPL-2.0
/*
 * starfire.c: Starfire/E10000 support.
 *
 * Copyright (C) 1998 David S. Miller (davem@redhat.com)
 * Copyright (C) 2000 Anton Blanchard (anton@samba.org)
 */

// Linux and SPARC kernel dependencies supplied by other translation units.

type Phandle = u32;
type S32 = i32;

unsafe extern "C" {
    fn prom_finddevice(path: *const core::ffi::c_char) -> Phandle;
    fn prom_printf(format: *const core::ffi::c_char, ...);
    fn prom_halt() -> !;
    fn upa_readl(address: usize) -> u32;
    fn upa_writel(value: u32, address: usize);
    fn printk(format: *const core::ffi::c_char, ...);
    fn panic(format: *const core::ffi::c_char) -> !;
}

/*
 * A few places around the kernel check this to see if
 * they need to call us to do things in a Starfire specific
 * way.
 */
pub static mut this_is_starfire: i32 = 0;

pub unsafe fn check_if_starfire() {
    let ssnode: Phandle = prom_finddevice(c"/ssp-serial".as_ptr());
    if ssnode != 0 && (ssnode as S32) != -1 {
        this_is_starfire = 1;
    }
}

/*
 * Each Starfire board has 32 registers which perform translation
 * and delivery of traditional interrupt packets into the extended
 * Starfire hardware format.  Essentially UPAID's now have 2 more
 * bits than in all previous Sun5 systems.
 */
#[repr(C)]
pub struct starfire_irqinfo {
    pub imap_slots: [usize; 32],
    pub tregs: [usize; 32],
    pub next: *mut starfire_irqinfo,
    pub upaid: i32,
    pub hwmid: i32,
}

static mut sflist: *mut starfire_irqinfo = core::ptr::null_mut();

/* Beam me up Scott(McNeil)y... */
pub unsafe fn starfire_hookup(upaid: i32) {
    let p = Box::into_raw(Box::new(starfire_irqinfo {
        imap_slots: [0; 32],
        tregs: [0; 32],
        next: core::ptr::null_mut(),
        upaid: 0,
        hwmid: 0,
    }));

    if p.is_null() {
        prom_printf(c"starfire_hookup: No memory, this is insane.\n".as_ptr());
        prom_halt();
    }

    let mut treg_base: usize = 0x100fc000000;
    let hwmid: usize = (((upaid as usize) & 0x3c) << 1)
        | (((upaid as usize) & 0x40) >> 4)
        | ((upaid as usize) & 0x3);
    (*p).hwmid = hwmid as i32;
    treg_base = treg_base.wrapping_add(hwmid << 33);
    treg_base = treg_base.wrapping_add(0x200);
    for i in 0..32usize {
        (*p).imap_slots[i] = 0;
        (*p).tregs[i] = treg_base.wrapping_add(i.wrapping_mul(0x10));
        /* Lets play it safe and not overwrite existing mappings */
        if upa_readl((*p).tregs[i]) != 0 {
            (*p).imap_slots[i] = 0xdeadbeaf;
        }
    }
    (*p).upaid = upaid;
    (*p).next = sflist;
    sflist = p;
}

pub unsafe fn starfire_translate(imap: usize, mut upaid: u32) -> u32 {
    let mut p: *mut starfire_irqinfo;
    let bus_hwmid: u32 = ((imap >> 33) & 0x7f) as u32;

    p = sflist;
    while !p.is_null() {
        if (*p).hwmid == bus_hwmid as i32 {
            break;
        }
        p = (*p).next;
    }
    if p.is_null() {
        prom_printf(c"XFIRE: Cannot find irqinfo for imap %016lx\n".as_ptr(), imap);
        prom_halt();
    }
    let mut i = 0usize;
    while i < 32 {
        if (*p).imap_slots[i] == imap || (*p).imap_slots[i] == 0 {
            break;
        }
        i += 1;
    }
    if i == 32 {
        printk(c"starfire_translate: Are you kidding me?\n".as_ptr());
        panic(c"Lucy in the sky....\0".as_ptr());
    }
    (*p).imap_slots[i] = imap;

    /* map to real upaid */
    upaid = ((upaid & 0x3c) << 1) | ((upaid & 0x40) >> 4) | (upaid & 0x3);

    upa_writel(upaid, (*p).tregs[i]);

    i as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
