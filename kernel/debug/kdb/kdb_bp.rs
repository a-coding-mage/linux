// SPDX-License-Identifier: GPL-2.0
/* Kernel Debugger Architecture Independent Breakpoint Handler */

// C dependencies are supplied by the surrounding kernel translation unit.

pub static mut kdb_breakpoints: [kdb_bp_t; KDB_MAXBPT] = [kdb_bp_t { ..unsafe { core::mem::zeroed() } }; KDB_MAXBPT];

unsafe fn kdb_setsinglestep(_regs: *mut pt_regs) {
    KDB_STATE_SET!(DOING_SS);
}

static mut kdb_rwtypes: [&'static str; 5] = [
    "Instruction(i)",
    "Instruction(Register)",
    "Data Write",
    "I/O",
    "Data Access",
];

unsafe fn kdb_bptype(bp: *mut kdb_bp_t) -> &'static str {
    if (*bp).bp_type < 0 || (*bp).bp_type > 4 {
        return "";
    }
    kdb_rwtypes[(*bp).bp_type as usize]
}

unsafe fn kdb_parsebp(argc: i32, argv: *const *const core::ffi::c_char,
                      nextargp: *mut i32, bp: *mut kdb_bp_t) -> i32 {
    let mut nextarg = *nextargp;
    let diag: i32;

    (*bp).bph_length = 1;
    if (argc + 1) != nextarg {
        if strncasecmp((*argv.add(nextarg as usize)) as *const _, b"datar\0".as_ptr() as *const _, 6) == 0 {
            (*bp).bp_type = BP_ACCESS_WATCHPOINT;
        } else if strncasecmp((*argv.add(nextarg as usize)) as *const _, b"dataw\0".as_ptr() as *const _, 6) == 0 {
            (*bp).bp_type = BP_WRITE_WATCHPOINT;
        } else if strncasecmp((*argv.add(nextarg as usize)) as *const _, b"inst\0".as_ptr() as *const _, 5) == 0 {
            (*bp).bp_type = BP_HARDWARE_BREAKPOINT;
        } else {
            return KDB_ARGCOUNT;
        }
        (*bp).bph_length = 1;
        nextarg += 1;
        if (argc + 1) != nextarg {
            let mut len: u64 = 0;
            diag = kdbgetularg(*argv.add(nextarg as usize), &mut len);
            if diag != 0 { return diag; }
            if len > 8 { return KDB_BADLENGTH; }
            (*bp).bph_length = len;
            nextarg += 1;
        }
        if (argc + 1) != nextarg { return KDB_ARGCOUNT; }
    }
    *nextargp = nextarg;
    0
}

unsafe fn _kdb_bp_remove(bp: *mut kdb_bp_t) -> i32 {
    let mut ret = 1;
    if !(*bp).bp_installed { return ret; }
    if (*bp).bp_type == 0 {
        ret = dbg_remove_sw_break((*bp).bp_addr);
    } else {
        ret = arch_kgdb_ops.remove_hw_breakpoint((*bp).bp_addr, (*bp).bph_length, (*bp).bp_type);
    }
    if ret == 0 { (*bp).bp_installed = 0; }
    ret
}

unsafe fn kdb_handle_bp(regs: *mut pt_regs, bp: *mut kdb_bp_t) {
    if KDB_DEBUG!(BP) { kdb_printf(b"regs->ip = 0x%lx\n\0".as_ptr() as *const _, instruction_pointer(regs)); }
    kdb_setsinglestep(regs);
    (*bp).bp_delay = 0;
    (*bp).bp_delayed = 1;
}

unsafe fn _kdb_bp_install(regs: *mut pt_regs, bp: *mut kdb_bp_t) -> i32 {
    let ret;
    if KDB_DEBUG!(BP) { kdb_printf(b"%s: bp_installed %d\n\0".as_ptr() as *const _, b"_kdb_bp_install\0".as_ptr(), (*bp).bp_installed); }
    if !KDB_STATE!(SSBPT) { (*bp).bp_delay = 0; }
    if (*bp).bp_installed { return 1; }
    if (*bp).bp_delay || ((*bp).bp_delayed && KDB_STATE!(DOING_SS)) {
        if KDB_DEBUG!(BP) { kdb_printf(b"%s: delayed bp\n\0".as_ptr() as *const _, b"_kdb_bp_install\0".as_ptr()); }
        kdb_handle_bp(regs, bp); return 0;
    }
    if (*bp).bp_type == 0 { ret = dbg_set_sw_break((*bp).bp_addr); }
    else { ret = arch_kgdb_ops.set_hw_breakpoint((*bp).bp_addr, (*bp).bph_length, (*bp).bp_type); }
    if ret == 0 { (*bp).bp_installed = 1; }
    else {
        kdb_printf(b"%s: failed to set breakpoint at 0x%lx\n\0".as_ptr() as *const _, b"_kdb_bp_install\0".as_ptr(), (*bp).bp_addr);
        if (*bp).bp_type == 0 { kdb_printf(b"Software breakpoints are unavailable.\n  Boot the kernel with rodata=off\n  OR use hw breaks: help bph\n\0".as_ptr() as *const _); }
        return 1;
    }
    0
}

pub unsafe fn kdb_bp_install(regs: *mut pt_regs) {
    for i in 0..KDB_MAXBPT {
        let bp = &mut kdb_breakpoints[i] as *mut kdb_bp_t;
        if KDB_DEBUG!(BP) { kdb_printf(b"%s: bp %d bp_enabled %d\n\0".as_ptr() as *const _, b"kdb_bp_install\0".as_ptr(), i, (*bp).bp_enabled); }
        if (*bp).bp_enabled { _kdb_bp_install(regs, bp); }
    }
}

pub unsafe fn kdb_bp_remove() {
    let mut i = KDB_MAXBPT as isize - 1;
    while i >= 0 {
        let bp = &mut kdb_breakpoints[i as usize] as *mut kdb_bp_t;
        if KDB_DEBUG!(BP) { kdb_printf(b"%s: bp %d bp_enabled %d\n\0".as_ptr() as *const _, b"kdb_bp_remove\0".as_ptr(), i, (*bp).bp_enabled); }
        if (*bp).bp_enabled { _kdb_bp_remove(bp); }
        i -= 1;
    }
}

unsafe fn kdb_printbp(bp: *mut kdb_bp_t, i: i32) {
    kdb_printf(b"%s \0".as_ptr() as *const _, kdb_bptype(bp).as_ptr());
    kdb_printf(b"BP #%d at \0".as_ptr() as *const _, i);
    kdb_symbol_print((*bp).bp_addr, core::ptr::null_mut(), KDB_SP_DEFAULT);
    if (*bp).bp_enabled { kdb_printf(b"\n    is enabled \0".as_ptr() as *const _); }
    else { kdb_printf(b"\n    is disabled\0".as_ptr() as *const _); }
    kdb_printf(b"  addr at %016lx, hardtype=%d installed=%d\n\n\0".as_ptr() as *const _, (*bp).bp_addr, (*bp).bp_type, (*bp).bp_installed);
}

// The remaining command handlers and registration table retain the C command interface.
// External kernel-specific declarations and macro definitions are intentionally referenced here.

pub unsafe fn kdb_ss(argc: i32, _argv: *const *const core::ffi::c_char) -> i32 {
    if argc != 0 { return KDB_ARGCOUNT; }
    KDB_STATE_SET!(DOING_SS);
    KDB_CMD_SS
}

pub unsafe fn kdb_initbptab() {
    for i in 0..KDB_MAXBPT { core::ptr::write_bytes(&mut kdb_breakpoints[i], 0, 1); kdb_breakpoints[i].bp_free = 1; }
    kdb_register_table(bptab.as_mut_ptr(), bptab.len());
    if arch_kgdb_ops.flags & KGDB_HW_BREAKPOINT != 0 { kdb_register_table(&mut bphcmd, 1); }
}

unsafe fn kdb_bp(argc: i32, argv: *const *const core::ffi::c_char) -> i32 {
    if argc == 0 {
        for i in 0..KDB_MAXBPT { let bp = &mut kdb_breakpoints[i] as *mut kdb_bp_t; if !(*bp).bp_free { kdb_printbp(bp, i as i32); } }
        return 0;
    }
    let mut nextarg = 1;
    let mut addr = 0u64;
    let mut offset = 0i64;
    let mut symname: *mut core::ffi::c_char = core::ptr::null_mut();
    let mut template: kdb_bp_t = core::mem::zeroed();
    let mut diag = kdbgetaddrarg(argc, argv, &mut nextarg, &mut addr, &mut offset, &mut symname);
    if diag != 0 { return diag; }
    template.bp_addr = addr;
    if template.bp_addr == 0 { return KDB_BADINT; }
    diag = kgdb_validate_break_address(template.bp_addr); if diag != 0 { return diag; }
    let mut bpno = KDB_MAXBPT;
    for i in 0..KDB_MAXBPT { if kdb_breakpoints[i].bp_free { bpno = i; break; } }
    if bpno == KDB_MAXBPT { return KDB_TOOMANYBPT; }
    if strcmp(*argv, b"bph\0".as_ptr() as *const _) == 0 {
        template.bp_type = BP_HARDWARE_BREAKPOINT;
        diag = kdb_parsebp(argc, argv, &mut nextarg, &mut template); if diag != 0 { return diag; }
    } else { template.bp_type = BP_BREAKPOINT; }
    for i in 0..KDB_MAXBPT { let old = &kdb_breakpoints[i] as *const kdb_bp_t; if !(*old).bp_free && (*old).bp_addr == template.bp_addr { return KDB_DUPBPT; } }
    template.bp_enabled = 1;
    kdb_breakpoints[bpno] = template;
    kdb_breakpoints[bpno].bp_free = 0;
    kdb_printbp(&mut kdb_breakpoints[bpno], bpno as i32);
    0
}

unsafe fn kdb_bc(argc: i32, argv: *const *const core::ffi::c_char) -> i32 {
    if argc != 1 { return KDB_ARGCOUNT; }
    let cmd = if strcmp(*argv, b"be\0".as_ptr() as *const _) == 0 { 1 } else if strcmp(*argv, b"bd\0".as_ptr() as *const _) == 0 { 2 } else { 0 };
    let mut low = KDB_MAXBPT; let mut high = 0; let mut addr = 0u64;
    if strcmp(*argv.add(1), b"*\0".as_ptr() as *const _) == 0 { low = 0; high = KDB_MAXBPT; }
    else { let d = kdbgetularg(*argv.add(1), &mut addr); if d != 0 { return d; } if addr < KDB_MAXBPT as u64 { low = addr as usize; high = low + 1; } else { for i in 0..KDB_MAXBPT { if kdb_breakpoints[i].bp_addr == addr { low=i; high=i+1; break; } } } }
    let mut done = 0;
    for i in low..high { let bp = &mut kdb_breakpoints[i]; if bp.bp_free { continue; } done += 1; match cmd { 0 => { bp.bp_enabled=0; bp.bp_addr=0; bp.bp_free=1; }, 1 => bp.bp_enabled=1, _ => bp.bp_enabled=0 } if bp.bp_delay && (cmd==0 || cmd==2) { bp.bp_delay=0; KDB_STATE_CLEAR!(SSBPT); } }
    if done == 0 { KDB_BPTNOTFOUND } else { 0 }
}

static mut bptab: [kdbtab_t; 6] = [
    kdbtab_t { name: b"bp\0".as_ptr(), func: Some(kdb_bp), usage: b"[<vaddr>]\0".as_ptr(), help: b"Set/Display breakpoints\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS },
    kdbtab_t { name: b"bl\0".as_ptr(), func: Some(kdb_bp), usage: b"[<vaddr>]\0".as_ptr(), help: b"Display breakpoints\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS },
    kdbtab_t { name: b"bc\0".as_ptr(), func: Some(kdb_bc), usage: b"<bpnum>\0".as_ptr(), help: b"Clear Breakpoint\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL },
    kdbtab_t { name: b"be\0".as_ptr(), func: Some(kdb_bc), usage: b"<bpnum>\0".as_ptr(), help: b"Enable Breakpoint\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL },
    kdbtab_t { name: b"bd\0".as_ptr(), func: Some(kdb_bc), usage: b"<bpnum>\0".as_ptr(), help: b"Disable Breakpoint\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL },
    kdbtab_t { name: b"ss\0".as_ptr(), func: Some(kdb_ss), usage: b"\0".as_ptr(), help: b"Single Step\0".as_ptr(), minlen: 1, flags: KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS },
];

static mut bphcmd: kdbtab_t = kdbtab_t { name: b"bph\0".as_ptr(), func: Some(kdb_bp), usage: b"[<vaddr>]\0".as_ptr(), help: b"[datar [length]|dataw [length]]   Set hw brk\0".as_ptr(), minlen: 0, flags: KDB_ENABLE_FLOW_CTRL | KDB_REPEAT_NO_ARGS };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
