// SPDX-License-Identifier: GPL-2.0
/*
 * Trapped io support
 *
 * Copyright (C) 2008 Magnus Damm
 *
 * Intercept io operations by trapping.
 */

// C dependencies supplied by the surrounding kernel translation unit.

const TRAPPED_PAGES_MAX: usize = 16;

#[cfg(CONFIG_HAS_IOPORT_MAP)]
#[no_mangle]
pub static mut trapped_io: list_head = LIST_HEAD_INIT;

#[cfg(CONFIG_HAS_IOMEM)]
#[no_mangle]
pub static mut trapped_mem: list_head = LIST_HEAD_INIT;

static mut trapped_lock: spinlock_t = DEFINE_SPINLOCK_INIT;
static mut trapped_io_disable: i32 = 0;

unsafe extern "C" fn trapped_io_setup(_unused: *mut c_char) -> i32 {
    trapped_io_disable = 1;
    1
}

pub unsafe extern "C" fn register_trapped_io(tiop: *mut trapped_io) -> i32 {
    let mut len: c_ulong = 0;
    let mut flags: c_ulong = 0;
    let mut pages: [*mut page; TRAPPED_PAGES_MAX] = [core::ptr::null_mut(); TRAPPED_PAGES_MAX];
    let mut k: c_int;
    let n: c_int;

    if unlikely(trapped_io_disable != 0) {
        return 0;
    }

    // structure must be page aligned
    if (tiop as c_ulong) & (PAGE_SIZE - 1) != 0 {
        return bad_trapped_io();
    }

    k = 0;
    while k < (*tiop).num_resources {
        let res = (*tiop).resource.add(k as usize);
        len = len.wrapping_add(roundup(resource_size(res), PAGE_SIZE));
        flags |= (*res).flags;
        k += 1;
    }

    // support IORESOURCE_IO or MEM, not both
    if hweight_long(flags) != 1 {
        return bad_trapped_io();
    }

    n = (len >> PAGE_SHIFT) as c_int;
    if n >= TRAPPED_PAGES_MAX as c_int {
        return bad_trapped_io();
    }

    k = 0;
    while k < n {
        pages[k as usize] = virt_to_page(tiop as *const c_void);
        k += 1;
    }

    (*tiop).virt_base = vmap(pages.as_mut_ptr(), n as c_ulong, VM_MAP, PAGE_NONE);
    if (*tiop).virt_base.is_null() {
        return bad_trapped_io();
    }

    len = 0;
    k = 0;
    while k < (*tiop).num_resources {
        let res = (*tiop).resource.add(k as usize);
        pr_info!("trapped io 0x{:08x} overrides {} 0x{:08x}\n",
                 (*tiop).virt_base.add(len as usize) as c_ulong,
                 if (*res).flags & IORESOURCE_IO != 0 { "io" } else { "mmio" },
                 (*res).start);
        len = len.wrapping_add(roundup(resource_size(res), PAGE_SIZE));
        k += 1;
    }

    (*tiop).magic = IO_TRAPPED_MAGIC;
    INIT_LIST_HEAD(&mut (*tiop).list);
    spin_lock_irq(&mut trapped_lock);
    #[cfg(CONFIG_HAS_IOPORT_MAP)]
    if flags & IORESOURCE_IO != 0 {
        list_add(&mut (*tiop).list, &mut trapped_io);
    }
    #[cfg(CONFIG_HAS_IOMEM)]
    if flags & IORESOURCE_MEM != 0 {
        list_add(&mut (*tiop).list, &mut trapped_mem);
    }
    spin_unlock_irq(&mut trapped_lock);
    0
}

unsafe fn bad_trapped_io() -> i32 {
    pr_warn!("unable to install trapped io filter\n");
    -1
}

pub unsafe extern "C" fn match_trapped_io_handler(list: *mut list_head, offset: c_ulong, _size: c_ulong) -> *mut c_void {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut trapped_lock, &mut flags);
    let mut tiop: *mut trapped_io = core::ptr::null_mut();
    list_for_each_entry!(tiop, list, list, {
        let mut voffs: c_ulong = 0;
        let mut k = 0;
        while k < (*tiop).num_resources {
            let res = (*tiop).resource.add(k as usize);
            if (*res).start == offset {
                spin_unlock_irqrestore(&mut trapped_lock, flags);
                return (*tiop).virt_base.add(voffs as usize) as *mut c_void;
            }
            voffs = voffs.wrapping_add(roundup(resource_size(res), PAGE_SIZE));
            k += 1;
        }
    });
    spin_unlock_irqrestore(&mut trapped_lock, flags);
    core::ptr::null_mut()
}

unsafe fn lookup_tiop(address: c_ulong) -> *mut trapped_io {
    let pgd_k = swapper_pg_dir.add(pgd_index(address) as usize);
    if !pgd_present(*pgd_k) { return core::ptr::null_mut(); }
    let p4d_k = p4d_offset(pgd_k, address);
    if !p4d_present(*p4d_k) { return core::ptr::null_mut(); }
    let pud_k = pud_offset(p4d_k, address);
    if !pud_present(*pud_k) { return core::ptr::null_mut(); }
    let pmd_k = pmd_offset(pud_k, address);
    if !pmd_present(*pmd_k) { return core::ptr::null_mut(); }
    let pte_k = pte_offset_kernel(pmd_k, address);
    pfn_to_kaddr(pte_pfn(*pte_k)) as *mut trapped_io
}

unsafe fn lookup_address(tiop: *mut trapped_io, address: c_ulong) -> c_ulong {
    let mut vaddr = (*tiop).virt_base as c_ulong;
    let mut k = 0;
    while k < (*tiop).num_resources {
        let res = (*tiop).resource.add(k as usize);
        let len = roundup(resource_size(res), PAGE_SIZE);
        if address < vaddr.wrapping_add(len) { return (*res).start.wrapping_add(address.wrapping_sub(vaddr)); }
        vaddr = vaddr.wrapping_add(len);
        k += 1;
    }
    0
}

unsafe fn copy_word(src_addr: c_ulong, src_len: c_int, dst_addr: c_ulong, dst_len: c_int) -> u64 {
    let tmp = match src_len { 1 => __raw_readb(src_addr) as u64, 2 => __raw_readw(src_addr) as u64, 4 => __raw_readl(src_addr) as u64, 8 => __raw_readq(src_addr), _ => 0 };
    match dst_len { 1 => __raw_writeb(tmp, dst_addr), 2 => __raw_writew(tmp, dst_addr), 4 => __raw_writel(tmp, dst_addr), 8 => __raw_writeq(tmp, dst_addr), _ => () }
    tmp
}

unsafe fn from_device(dst: *mut c_void, src: *const c_void, cnt: c_ulong) -> c_ulong {
    let mut tiop = lookup_tiop(src as c_ulong);
    warn_on!(tiop.is_null() || (*tiop).magic != IO_TRAPPED_MAGIC);
    let src_addr = lookup_address(tiop, src as c_ulong);
    if src_addr == 0 { return cnt; }
    copy_word(src_addr, core::cmp::max(cnt, (*tiop).minimum_bus_width as c_ulong / 8) as c_int, dst as c_ulong, cnt as c_int);
    0
}

unsafe fn to_device(dst: *mut c_void, src: *const c_void, cnt: c_ulong) -> c_ulong {
    let tiop = lookup_tiop(dst as c_ulong);
    warn_on!(tiop.is_null() || (*tiop).magic != IO_TRAPPED_MAGIC);
    let dst_addr = lookup_address(tiop, dst as c_ulong);
    if dst_addr == 0 { return cnt; }
    copy_word(src as c_ulong, cnt as c_int, dst_addr, core::cmp::max(cnt, (*tiop).minimum_bus_width as c_ulong / 8) as c_int);
    0
}

static mut trapped_io_access: mem_access = mem_access { from_device, to_device };

pub unsafe extern "C" fn handle_trapped_io(regs: *mut pt_regs, address: c_ulong) -> c_int {
    if trapped_io_disable != 0 || lookup_tiop(address).is_null() { return 0; }
    warn_on!(user_mode(regs));
    let mut instruction: insn_size_t = 0;
    if copy_from_kernel_nofault(&mut instruction as *mut _ as *mut c_void, (*regs).pc as *const c_void, core::mem::size_of::<insn_size_t>()) != 0 { return 0; }
    (handle_unaligned_access(instruction, regs, &mut trapped_io_access, 1, address) == 0) as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
