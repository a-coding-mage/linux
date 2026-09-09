// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2008 Michael Ellerman, IBM Corporation.
 */

// Translated from the Linux PowerPC code-patching self-tests.

unsafe fn instr_is_branch_to_addr(instr: *const u32, addr: usize) -> i32 {
    if instr_is_branch_iform(ppc_inst_read(instr)) != 0 ||
       instr_is_branch_bform(ppc_inst_read(instr)) != 0 {
        return (branch_target(instr) == addr) as i32;
    }
    0
}

unsafe fn test_trampoline() {
    core::arch::asm!("nop; nop");
}

macro_rules! check {
    ($x:expr) => {
        if !($x) {
            pr_err!("code-patching: test failed at line {}\n", line!());
        }
    };
}

unsafe fn test_branch_iform() {
    let mut instr: ppc_inst_t;
    let mut tmp = [0u32; 2];
    let iptr = tmp.as_mut_ptr();
    let addr = tmp.as_mut_ptr() as usize;
    let mut err: i32;

    check!(instr_is_branch_iform(ppc_inst(0x48000000)) != 0);
    check!(instr_is_branch_iform(ppc_inst(0x4bffffff)) != 0);
    check!(instr_is_branch_iform(ppc_inst(0xcbffffff)) == 0);
    check!(instr_is_branch_iform(ppc_inst(0x7bffffff)) == 0);
    check!(instr_is_branch_iform(ppc_inst(0x48000001)) != 0);
    check!(instr_is_branch_iform(ppc_inst(0x4bfffffd)) != 0);
    check!(instr_is_branch_iform(ppc_inst(0x4bff00fd)) != 0);
    check!(instr_is_branch_iform(ppc_inst(0x7bfffffd)) == 0);

    ppc_inst_write(iptr, ppc_inst(0x48000103)); check!(instr_is_branch_to_addr(iptr, 0x100) != 0);
    ppc_inst_write(iptr, ppc_inst(0x480420ff)); check!(instr_is_branch_to_addr(iptr, 0x420fc) != 0);
    ppc_inst_write(iptr, ppc_inst(0x49fffffc)); check!(instr_is_branch_to_addr(iptr, addr + 0x1fffffc) != 0);
    ppc_inst_write(iptr, ppc_inst(0x4bfffffc)); check!(instr_is_branch_to_addr(iptr, addr - 4) != 0);
    ppc_inst_write(iptr, ppc_inst(0x4a000000)); check!(instr_is_branch_to_addr(iptr, addr - 0x2000000) != 0);

    err = create_branch(&mut instr, iptr, addr, BRANCH_SET_LINK); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr) != 0);
    err = create_branch(&mut instr, iptr, addr - 0x100, BRANCH_SET_LINK); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr - 0x100) != 0);
    err = create_branch(&mut instr, iptr, addr + 0x100, 0); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr + 0x100) != 0);
    err = create_branch(&mut instr, iptr, addr - 0x2000000, BRANCH_SET_LINK); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr - 0x2000000) != 0);
    err = create_branch(&mut instr, iptr, addr - 0x2000004, BRANCH_SET_LINK); check!(err != 0);
    err = create_branch(&mut instr, iptr, addr + 0x2000000, BRANCH_SET_LINK); check!(err != 0);
    err = create_branch(&mut instr, iptr, addr + 3, BRANCH_SET_LINK); check!(err != 0);
    err = create_branch(&mut instr, iptr, addr, 0xfffffffc); ppc_inst_write(iptr, instr);
    check!(instr_is_branch_to_addr(iptr, addr) != 0); check!(ppc_inst_equal(instr, ppc_inst(0x48000000)) != 0);
}

unsafe fn test_create_function_call() {
    let iptr = ppc_function_entry(test_trampoline) as *mut u32;
    let dest = ppc_function_entry(test_create_function_call);
    let mut instr: ppc_inst_t;
    create_branch(&mut instr, iptr, dest, BRANCH_SET_LINK);
    patch_instruction(iptr, instr);
    check!(instr_is_branch_to_addr(iptr, dest) != 0);
}

unsafe fn test_branch_bform() {
    let mut instr: ppc_inst_t;
    let mut tmp = [0u32; 2];
    let iptr = tmp.as_mut_ptr();
    let addr = iptr as usize;
    let mut err: i32;
    let flags: u32;

    check!(instr_is_branch_bform(ppc_inst(0x40000000)) != 0); check!(instr_is_branch_bform(ppc_inst(0x43ffffff)) != 0);
    check!(instr_is_branch_bform(ppc_inst(0xc3ffffff)) == 0); check!(instr_is_branch_bform(ppc_inst(0x7bffffff)) == 0);
    ppc_inst_write(iptr, ppc_inst(0x43ff0103)); check!(instr_is_branch_to_addr(iptr, 0x100) != 0);
    ppc_inst_write(iptr, ppc_inst(0x43ff20ff)); check!(instr_is_branch_to_addr(iptr, 0x20fc) != 0);
    ppc_inst_write(iptr, ppc_inst(0x43ff7ffc)); check!(instr_is_branch_to_addr(iptr, addr + 0x7ffc) != 0);
    ppc_inst_write(iptr, ppc_inst(0x43fffffc)); check!(instr_is_branch_to_addr(iptr, addr - 4) != 0);
    ppc_inst_write(iptr, ppc_inst(0x43ff8000)); check!(instr_is_branch_to_addr(iptr, addr - 0x8000) != 0);
    flags = 0x3ff000 | BRANCH_SET_LINK;
    err = create_cond_branch(&mut instr, iptr, addr, flags); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr) != 0);
    err = create_cond_branch(&mut instr, iptr, addr - 0x100, flags); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr - 0x100) != 0);
    err = create_cond_branch(&mut instr, iptr, addr + 0x100, flags); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr + 0x100) != 0);
    err = create_cond_branch(&mut instr, iptr, addr - 0x8000, flags); ppc_inst_write(iptr, instr); check!(instr_is_branch_to_addr(iptr, addr - 0x8000) != 0);
    err = create_cond_branch(&mut instr, iptr, addr - 0x8004, flags); check!(err != 0);
    err = create_cond_branch(&mut instr, iptr, addr + 0x8000, flags); check!(err != 0);
    err = create_cond_branch(&mut instr, iptr, addr + 3, flags); check!(err != 0);
    err = create_cond_branch(&mut instr, iptr, addr, 0xfffffffc); ppc_inst_write(iptr, instr);
    check!(instr_is_branch_to_addr(iptr, addr) != 0); check!(ppc_inst_equal(instr, ppc_inst(0x43ff0000)) != 0);
}

unsafe fn test_translate_branch() {
    let buf = vmalloc(PAGE_ALIGN(0x2000000 + 1)); check!(!buf.is_null()); if buf.is_null() { return; }
    let mut instr: ppc_inst_t;
    let mut p: *mut u8; let mut q: *mut u8; let mut addr: usize;
    p = buf as *mut u8; addr = p as usize; create_branch(&mut instr, p, addr, 0); ppc_inst_write(p, instr); check!(instr_is_branch_to_addr(p, addr) != 0);
    q = p.add(4); translate_branch(&mut instr, q, p); ppc_inst_write(q, instr); check!(instr_is_branch_to_addr(q, addr) != 0);
    p = buf as *mut u8; addr = p as usize; create_branch(&mut instr, p, addr, 0); ppc_inst_write(p, instr); q = (buf as *mut u8).add(0x2000000); translate_branch(&mut instr, q, p); ppc_inst_write(q, instr); check!(instr_is_branch_to_addr(p, addr) != 0); check!(instr_is_branch_to_addr(q, addr) != 0); check!(ppc_inst_equal(ppc_inst_read(q), ppc_inst(0x4a000000)) != 0);
    p = (buf as *mut u8).add(0x2000000); addr = p as usize; create_branch(&mut instr, p, addr, 0); ppc_inst_write(p, instr); q = (buf as *mut u8).add(4); translate_branch(&mut instr, q, p); ppc_inst_write(q, instr); check!(instr_is_branch_to_addr(p, addr) != 0); check!(instr_is_branch_to_addr(q, addr) != 0); check!(ppc_inst_equal(ppc_inst_read(q), ppc_inst(0x49fffffc)) != 0);
    p = buf as *mut u8; addr = 0x1000000 + buf as usize; create_branch(&mut instr, p, addr, BRANCH_SET_LINK); ppc_inst_write(p, instr); q = (buf as *mut u8).add(0x1400000); translate_branch(&mut instr, q, p); ppc_inst_write(q, instr); check!(instr_is_branch_to_addr(q, addr) != 0);
    p = (buf as *mut u8).add(0x1000000); addr = 0x2000000 + buf as usize; create_branch(&mut instr, p, addr, 0); ppc_inst_write(p, instr); q = (buf as *mut u8).add(4); translate_branch(&mut instr, q, p); ppc_inst_write(q, instr); check!(instr_is_branch_to_addr(q, addr) != 0);
    vfree(buf);
}

unsafe fn test_prefixed_patching() {
    let iptr = ppc_function_entry(test_trampoline) as *mut u32; let expected = [OP_PREFIX << 26, 0]; let inst = ppc_inst_prefix(OP_PREFIX << 26, 0);
    if !IS_ENABLED(CONFIG_PPC64) { return; } patch_instruction(iptr, inst); check!(memcmp(iptr as *const _, expected.as_ptr() as *const _, core::mem::size_of_val(&expected)) == 0);
}

unsafe fn test_multi_instruction_patching() {
    let mut code = [0u32; 32]; let buf = vzalloc(PAGE_SIZE * 8); check!(!buf.is_null()); if buf.is_null() { return; }
    let inst32 = PPC_RAW_NOP(); let inst64 = ppc_inst_prefix(OP_PREFIX << 26 | 3 << 24, PPC_RAW_TRAP()); let mut addr32: *mut u32;
    addr32 = (buf as *mut u8).add(PAGE_SIZE) as *mut u32; check!(patch_instructions(addr32.add(1), &inst32, 12, true) == 0); check!(*addr32 == 0); check!(*addr32.add(1) == inst32); check!(*addr32.add(2) == inst32); check!(*addr32.add(3) == inst32); check!(*addr32.add(4) == 0);
    addr32 = (buf as *mut u8).add(PAGE_SIZE * 3) as *mut u32; for i in 0..code.len() { code[i] = i as u32 + 1; } check!(patch_instructions(addr32.add(1), code.as_ptr(), core::mem::size_of_val(&code), false) == 0); check!(*addr32 == 0); check!(memcmp(addr32.add(1) as *const _, code.as_ptr() as *const _, core::mem::size_of_val(&code)) == 0); check!(*addr32.add(33) == 0);
    addr32 = (buf as *mut u8).add(PAGE_SIZE * 4 - 8) as *mut u32; check!(patch_instructions(addr32.add(1), &inst32, 12, true) == 0); check!(*addr32.add(1) == inst32); check!(*addr32.add(2) == inst32); check!(*addr32.add(3) == inst32);
    addr32 = (buf as *mut u8).add(PAGE_SIZE * 6 - 12) as *mut u32; check!(patch_instructions(addr32.add(1), code.as_ptr(), core::mem::size_of_val(&code), false) == 0); vfree(buf);
}

unsafe fn test_data_patching() { let buf = vzalloc(PAGE_SIZE); check!(!buf.is_null()); if buf.is_null() { return; } let p = (buf as *mut u8).add(128) as *mut u32; *p.add(1)=0xa0a1a2a3; *p.add(2)=0xb0b1b2b3; check!(patch_uint(p.add(1),0xc0c1c2c3)==0); check!(*p.add(1)==0xc0c1c2c3); check!(*p.add(2)==0xb0b1b2b3); check!(*p==0); check!(*p.add(3)==0); if IS_ENABLED(CONFIG_PPC64) { check!(patch_ulong(p.add(1),0xd0d1d2d3)==-EINVAL); } check!(patch_ulong(p.add(2),0xd0d1d2d3)==0); vfree(buf); }

unsafe fn test_code_patching() -> i32 {
    pr_info!("Running code patching self-tests ...\n");
    test_branch_iform(); test_branch_bform(); test_create_function_call();
    test_translate_branch(); test_prefixed_patching(); test_multi_instruction_patching(); test_data_patching();
    0
}

late_initcall!(test_code_patching);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
