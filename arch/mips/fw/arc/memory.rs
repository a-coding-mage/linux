// SPDX-License-Identifier: GPL-2.0
/*
 * memory.c: PROM library functions for acquiring/using memory descriptors
 *          given to us from the ARCS firmware.
 *
 * PROM library functions for acquiring/using memory descriptors given to us
 * from the ARCS firmware.  This is only used when CONFIG_ARC_MEMORY is set.
 */

// C header dependencies are supplied by the surrounding kernel translation.

const MAX_PROM_MEM: usize = 5;
static mut prom_mem_base: [phys_addr_t; MAX_PROM_MEM] = [0; MAX_PROM_MEM];
static mut prom_mem_size: [phys_addr_t; MAX_PROM_MEM] = [0; MAX_PROM_MEM];
static mut nr_prom_mem: c_uint = 0;

/* For ARC firmware memory functions the unit is always a 4k page. */
const ARC_PAGE_SHIFT: u32 = 12;

unsafe fn ArcGetMemoryDescriptor(Current: *mut linux_mdesc) -> *mut linux_mdesc {
    ARC_CALL1!(get_mdesc, Current) as *mut linux_mdesc
}

const mem_free: c_int = 0;
const mem_prom_used: c_int = 1;
const mem_reserved: c_int = 2;

unsafe fn memtype_classify_arcs(type_: linux_memtypes) -> c_int {
    match type_.arcs {
        arcs_fcontig | arcs_free => mem_free,
        arcs_atmp => mem_prom_used,
        arcs_eblock | arcs_rvpage | arcs_bmem | arcs_prog | arcs_aperm => mem_reserved,
        _ => {
            BUG!();
            loop {}
        }
    }
}

unsafe fn memtype_classify_arc(type_: linux_memtypes) -> c_int {
    match type_.arc {
        arc_free | arc_fcontig => mem_free,
        arc_atmp => mem_prom_used,
        arc_eblock | arc_rvpage | arc_bmem | arc_prog | arc_aperm => mem_reserved,
        _ => {
            BUG!();
            loop {}
        }
    }
}

unsafe fn prom_memtype_classify(type_: linux_memtypes) -> c_int {
    if prom_flags & PROM_FLAG_ARCS != 0 {
        return memtype_classify_arcs(type_);
    }
    memtype_classify_arc(type_)
}

#[no_mangle]
pub unsafe fn prom_meminit() {
    let mut p: *mut linux_mdesc;

    nr_prom_mem = 0;
    p = PROM_NULL_MDESC;
    loop {
        p = ArcGetMemoryDescriptor(p);
        if p.is_null() {
            break;
        }

        let base: c_ulong = (*p).base << ARC_PAGE_SHIFT;
        let size: c_ulong = (*p).pages << ARC_PAGE_SHIFT;
        let type_: c_long = prom_memtype_classify((*p).type_);

        /* ignore mirrored RAM on IP28/IP30 */
        if base < PHYS_OFFSET {
            continue;
        }

        memblock_add(base, size);

        if type_ == mem_reserved {
            memblock_reserve(base, size);
        }

        if type_ == mem_prom_used {
            memblock_reserve(base, size);
            if nr_prom_mem >= 5 {
                pr_err!("Too many ROM DATA regions");
                continue;
            }
            prom_mem_base[nr_prom_mem as usize] = base;
            prom_mem_size[nr_prom_mem as usize] = size;
            nr_prom_mem += 1;
        }
    }
}

#[no_mangle]
pub unsafe fn prom_cleanup() {}

#[no_mangle]
pub unsafe fn prom_free_prom_memory() {
    if prom_flags & PROM_FLAG_DONT_FREE_TEMP != 0 {
        return;
    }

    let mut i: c_int = 0;
    while i < nr_prom_mem as c_int {
        free_init_pages!(
            "prom memory",
            prom_mem_base[i as usize],
            prom_mem_base[i as usize] + prom_mem_size[i as usize]
        );
        i += 1;
    }

    /* at this point it isn't safe to call PROM functions */
    /* give platforms a way to do PROM cleanups */
    prom_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
