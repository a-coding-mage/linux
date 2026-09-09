// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OLPC-specific OFW device tree support code.
 *
 * Paul Mackerras\tAugust 1996.
 * Copyright (C) 1996-2005 Paul Mackerras.
 *
 *  Adapted for 64bit PowerPC by Dave Engebretsen and Peter Bergner.
 *  Adapted for sparc by David S. Miller
 *  Adapted for x86/OLPC by Andres Salomon
 */

// Kernel types, OFW helpers, logging, allocation, and constants are supplied
// by the surrounding kernel translation unit.

unsafe fn olpc_dt_getsibling(mut node: phandle) -> phandle {
    let args: [*const core::ffi::c_void; 1] = [node as *const _ as *const _];
    let mut res: [*mut core::ffi::c_void; 1] = [&mut node as *mut _ as *mut _];

    if node as s32 == -1 { return 0; }
    if olpc_ofw("peer", args.as_ptr(), res.as_mut_ptr()) != 0 || node as s32 == -1 { return 0; }
    node
}

unsafe fn olpc_dt_getchild(mut node: phandle) -> phandle {
    let args: [*const core::ffi::c_void; 1] = [node as *const _ as *const _];
    let mut res: [*mut core::ffi::c_void; 1] = [&mut node as *mut _ as *mut _];

    if node as s32 == -1 { return 0; }
    if olpc_ofw("child", args.as_ptr(), res.as_mut_ptr()) != 0 || node as s32 == -1 {
        pr_err!("PROM: olpc_dt_getchild: fetching child failed!\n");
        return 0;
    }
    node
}

unsafe fn olpc_dt_getproplen(node: phandle, prop: *const c_char) -> c_int {
    let args: [*const core::ffi::c_void; 2] = [node as *const _ as *const _, prop as *const _];
    let mut len: c_int = 0;
    let mut res: [*mut core::ffi::c_void; 1] = [&mut len as *mut _ as *mut _];
    if node as s32 == -1 { return -1; }
    if olpc_ofw("getproplen", args.as_ptr(), res.as_mut_ptr()) != 0 {
        pr_err!("PROM: olpc_dt_getproplen: getproplen failed!\n");
        return -1;
    }
    len
}

unsafe fn olpc_dt_getproperty(node: phandle, prop: *const c_char, buf: *mut c_char, bufsize: c_int) -> c_int {
    let mut plen = olpc_dt_getproplen(node, prop);
    if plen > bufsize || plen < 1 { return -1; }
    let args: [*const core::ffi::c_void; 4] = [node as *const _ as *const _, prop as *const _, buf as *const _, plen as *const _ as *const _];
    let mut res: [*mut core::ffi::c_void; 1] = [&mut plen as *mut _ as *mut _];
    if olpc_ofw("getprop", args.as_ptr(), res.as_mut_ptr()) != 0 {
        pr_err!("PROM: olpc_dt_getproperty: getprop failed!\n");
        return -1;
    }
    plen
}

unsafe fn olpc_dt_nextprop(node: phandle, prev: *mut c_char, buf: *mut c_char) -> c_int {
    *buf = 0;
    let args: [*const core::ffi::c_void; 3] = [node as *const _ as *const _, prev as *const _, buf as *const _];
    let mut success = 0;
    let mut res: [*mut core::ffi::c_void; 1] = [&mut success as *mut _ as *mut _];
    if node as s32 == -1 { return -1; }
    if olpc_ofw("nextprop", args.as_ptr(), res.as_mut_ptr()) != 0 || success != 1 { return -1; }
    0
}

unsafe fn olpc_dt_pkg2path(node: phandle, buf: *mut c_char, buflen: c_int, len: *mut c_int) -> c_int {
    let args: [*const core::ffi::c_void; 3] = [node as *const _ as *const _, buf as *const _, buflen as *const _ as *const _];
    let mut res: [*mut core::ffi::c_void; 1] = [len as *mut _ as *mut _];
    if node as s32 == -1 { return -1; }
    if olpc_ofw("package-to-path", args.as_ptr(), res.as_mut_ptr()) != 0 || *len < 1 { return -1; }
    0
}

static mut prom_early_allocated: u32 = 0;

unsafe fn prom_early_alloc(size: c_ulong) -> *mut core::ffi::c_void {
    static mut MEM: *mut u8 = core::ptr::null_mut();
    static mut FREE_MEM: usize = 0;
    let mut res: *mut core::ffi::c_void;
    if FREE_MEM < size as usize {
        let chunk_size = core::cmp::max(PAGE_SIZE as usize, size as usize);
        res = memblock_alloc_or_panic(chunk_size, SMP_CACHE_BYTES);
        prom_early_allocated += chunk_size as u32;
        core::ptr::write_bytes(res, 0, chunk_size);
        FREE_MEM = chunk_size;
        MEM = res as *mut u8;
    }
    FREE_MEM -= size as usize;
    res = MEM as *mut core::ffi::c_void;
    MEM = MEM.add(size as usize);
    res
}

static mut prom_olpc_ops: of_pdt_ops = of_pdt_ops {
    nextprop: Some(olpc_dt_nextprop), getproplen: Some(olpc_dt_getproplen),
    getproperty: Some(olpc_dt_getproperty), getchild: Some(olpc_dt_getchild),
    getsibling: Some(olpc_dt_getsibling), pkg2path: Some(olpc_dt_pkg2path),
};

unsafe fn olpc_dt_finddevice(path: *const c_char) -> phandle {
    let args: [*const core::ffi::c_void; 1] = [path as *const _];
    let mut node = 0;
    let mut res: [*mut core::ffi::c_void; 1] = [&mut node as *mut _ as *mut _];
    if olpc_ofw("finddevice", args.as_ptr(), res.as_mut_ptr()) != 0 { pr_err!("olpc_dt: finddevice failed!\n"); return 0; }
    if node as s32 == -1 { return 0; }
    node
}

unsafe fn olpc_dt_interpret(words: *const c_char) -> c_int {
    let args: [*const core::ffi::c_void; 1] = [words as *const _];
    let mut result = 0;
    let mut res: [*mut core::ffi::c_void; 1] = [&mut result as *mut _ as *mut _];
    if olpc_ofw("interpret", args.as_ptr(), res.as_mut_ptr()) != 0 { pr_err!("olpc_dt: interpret failed!\n"); return -1; }
    result
}

unsafe fn olpc_dt_get_board_revision() -> u32 {
    let node = olpc_dt_finddevice(c"/".as_ptr());
    if node == 0 { return 0; }
    let mut rev: __be32 = 0;
    if olpc_dt_getproperty(node, c"board-revision-int".as_ptr(), &mut rev as *mut _ as *mut c_char, core::mem::size_of::<__be32>() as c_int) < 0 { return 0; }
    be32_to_cpu(rev)
}

unsafe fn olpc_dt_compatible_match(node: phandle, compat: *const c_char) -> c_int {
    let mut buf = [0i8; 64];
    let plen = olpc_dt_getproperty(node, c"compatible".as_ptr(), buf.as_mut_ptr(), 64);
    if plen <= 0 { return 0; }
    let mut p = 0;
    while p < plen as usize {
        if strcmp(buf.as_ptr().add(p), compat) == 0 { return 1; }
        p += strlen(buf.as_ptr().add(p)) + 1;
    }
    0
}

unsafe fn olpc_dt_fixup() {
    let node = olpc_dt_finddevice(c"/battery@0".as_ptr());
    if node == 0 { return; }
    let board_rev = olpc_dt_get_board_revision();
    if board_rev == 0 { return; }
    if board_rev >= olpc_board_pre(0xd0) {
        if olpc_dt_compatible_match(node, c"olpc,xo1.5-battery".as_ptr()) != 0 { return; }
        olpc_dt_interpret(c"\" /battery@0\" find-device".as_ptr());
        olpc_dt_interpret(c"  \" olpc,xo1.5-battery\" +compatible".as_ptr());
        olpc_dt_interpret(c"device-end".as_ptr());
        if olpc_dt_compatible_match(node, c"olpc,xo1-battery".as_ptr()) != 0 { return; }
        olpc_dt_interpret(c"\" /pci/display@1\" find-device".as_ptr());
        olpc_dt_interpret(c"  new-device".as_ptr()); olpc_dt_interpret(c"    \" dcon\" device-name".as_ptr());
        olpc_dt_interpret(c"    \" olpc,xo1-dcon\" +compatible".as_ptr()); olpc_dt_interpret(c"  finish-device".as_ptr()); olpc_dt_interpret(c"device-end".as_ptr());
    } else {
        if olpc_dt_compatible_match(node, c"olpc,xo1-battery".as_ptr()) != 0 { return; }
        olpc_dt_interpret(c"\" /pci/display@1,1\" find-device".as_ptr()); olpc_dt_interpret(c"  new-device".as_ptr()); olpc_dt_interpret(c"    \" dcon\" device-name".as_ptr());
        olpc_dt_interpret(c"    \" olpc,xo1-dcon\" +compatible".as_ptr()); olpc_dt_interpret(c"  finish-device".as_ptr()); olpc_dt_interpret(c"device-end".as_ptr());
        olpc_dt_interpret(c"\" /rtc\" find-device".as_ptr()); olpc_dt_interpret(c" \" olpc,xo1-rtc\" +compatible".as_ptr()); olpc_dt_interpret(c"device-end".as_ptr());
    }
    olpc_dt_interpret(c"\" /battery@0\" find-device".as_ptr());
    olpc_dt_interpret(c"  \" olpc,xo1-battery\" +compatible".as_ptr()); olpc_dt_interpret(c"device-end".as_ptr());
}

unsafe fn olpc_dt_build_devicetree() {
    if !olpc_ofw_is_installed() { return; }
    olpc_dt_fixup();
    let root = olpc_dt_getsibling(0);
    if root == 0 { pr_err!("PROM: unable to get root node from OFW!\n"); return; }
    of_pdt_build_devicetree(root, &mut prom_olpc_ops);
    pr_info!("PROM DT: Built device tree with {} bytes of memory.\n", prom_early_allocated);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
