// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux and ARC headers provide the types, constants, macros, and functions
// referenced below.

const ARC_PATH_MAX: usize = 256;

unsafe fn print_regs_scratch(regs: *mut pt_regs) {
    pr_cont!("BTA: 0x%08lx\n SP: 0x%08lx  FP: 0x%08lx BLK: %pS\n",
        (*regs).bta, (*regs).sp, (*regs).fp, (*regs).blink as *mut core::ffi::c_void);
    pr_cont!("LPS: 0x%08lx\tLPE: 0x%08lx\tLPC: 0x%08lx\n",
        (*regs).lp_start, (*regs).lp_end, (*regs).lp_count);

    pr_info!("r00: 0x%08lx\tr01: 0x%08lx\tr02: 0x%08lx\n\
        r03: 0x%08lx\tr04: 0x%08lx\tr05: 0x%08lx\n\
        r06: 0x%08lx\tr07: 0x%08lx\tr08: 0x%08lx\n\
        r09: 0x%08lx\tr10: 0x%08lx\tr11: 0x%08lx\n\
        r12: 0x%08lx\t",
        (*regs).r0, (*regs).r1, (*regs).r2, (*regs).r3, (*regs).r4, (*regs).r5,
        (*regs).r6, (*regs).r7, (*regs).r8, (*regs).r9, (*regs).r10,
        (*regs).r11, (*regs).r12);
}

unsafe fn print_regs_callee(regs: *mut callee_regs) {
    pr_cont!("r13: 0x%08lx\tr14: 0x%08lx\n\
        r15: 0x%08lx\tr16: 0x%08lx\tr17: 0x%08lx\n\
        r18: 0x%08lx\tr19: 0x%08lx\tr20: 0x%08lx\n\
        r21: 0x%08lx\tr22: 0x%08lx\tr23: 0x%08lx\n\
        r24: 0x%08lx\tr25: 0x%08lx\n",
        (*regs).r13, (*regs).r14, (*regs).r15, (*regs).r16, (*regs).r17,
        (*regs).r18, (*regs).r19, (*regs).r20, (*regs).r21, (*regs).r22,
        (*regs).r23, (*regs).r24, (*regs).r25);
}

unsafe fn print_task_path_n_nm(tsk: *mut task_struct) {
    let mut path_nm: *mut core::ffi::c_char = core::ptr::null_mut();
    let mut buf = [0i8; ARC_PATH_MAX];
    let mm = get_task_mm(tsk);
    if !mm.is_null() {
        let exe_file = get_mm_exe_file(mm);
        mmput(mm);
        if !exe_file.is_null() {
            path_nm = file_path(exe_file, buf.as_mut_ptr(), (ARC_PATH_MAX - 1) as i32);
            fput(exe_file);
        }
    }
    pr_info!("Path: %s\n", if !IS_ERR(path_nm) { path_nm } else { b"?\0".as_ptr() as *mut _ });
}

unsafe fn show_faulting_vma(address: c_ulong) {
    let active_mm = (*current).active_mm;
    mmap_read_lock(active_mm);
    let vma = vma_lookup(active_mm, address);
    if !vma.is_null() {
        let mut buf = [0i8; ARC_PATH_MAX];
        let mut nm = b"anon\0".as_ptr() as *mut c_char;
        if !(*vma).vm_file.is_null() {
            nm = d_path(file_user_path((*vma).vm_file), buf.as_mut_ptr(), (ARC_PATH_MAX - 1) as i32);
            if IS_ERR(nm) { nm = b"?\0".as_ptr() as *mut c_char; }
        }
        pr_info!("  @off 0x%lx in [%s]  VMA: 0x%08lx to 0x%08lx\n",
            if (*vma).vm_start < TASK_UNMAPPED_BASE { address } else { address - (*vma).vm_start },
            nm, (*vma).vm_start, (*vma).vm_end);
    } else {
        pr_info!("    @No matching VMA found\n");
    }
    mmap_read_unlock(active_mm);
}

unsafe fn show_ecr_verbose(regs: *mut pt_regs) {
    let address = (*current).thread.fault_address;
    let vec = (*regs).ecr.vec;
    let cause_code = (*regs).ecr.cause;
    if vec == ECR_V_DTLB_MISS {
        pr_cont!("Invalid %s @ 0x%08lx by insn @ %pS\n", if cause_code == 1 { "Read" } else if cause_code == 2 { "Write" } else { "EX" }, address, (*regs).ret as *mut _);
    } else if vec == ECR_V_ITLB_MISS { pr_cont!("Insn could not be fetched\n");
    } else if vec == ECR_V_MACH_CHK { pr_cont!("Machine Check (%s)\n", if cause_code == 0 { "Double Fault" } else { "Other Fatal Err" });
    } else if vec == ECR_V_PROTV {
        if cause_code == ECR_C_PROTV_INST_FETCH { pr_cont!("Execute from Non-exec Page\n"); }
        else if cause_code == ECR_C_PROTV_MISALIG_DATA && IS_ENABLED!(CONFIG_ISA_ARCOMPACT) { pr_cont!("Misaligned r/w from 0x%08lx\n", address); }
        else { pr_cont!("%s access not allowed on page\n", if cause_code == 1 { "Read" } else if cause_code == 2 { "Write" } else { "EX" }); }
    } else if vec == ECR_V_INSN_ERR { pr_cont!("Illegal Insn\n");
    // CONFIG_ISA_ARCV2 conditional branch preserved from the C source.
    } else if vec == ECR_V_MEM_ERR { if cause_code == 0 { pr_cont!("Bus Error from Insn Mem\n"); } else if cause_code == 0x10 { pr_cont!("Bus Error from Data Mem\n"); } else { pr_cont!("Bus Error, check PRM\n"); }
    } else if vec == ECR_V_MISALIGN { pr_cont!("Misaligned r/w from 0x%08lx\n", address);
    } else if vec == ECR_V_TRAP { if (*regs).ecr.param == 5 { pr_cont!("gcc generated __builtin_trap\n"); }
    } else { pr_cont!("Check Programmer's Manual\n"); }
}

pub unsafe fn show_regs(regs: *mut pt_regs) {
    let tsk = current;
    let cregs = (*tsk).thread.callee_reg as *mut callee_regs;
    preempt_enable();
    print_task_path_n_nm(tsk);
    show_regs_print_info(KERN_INFO);
    show_ecr_verbose(regs);
    if user_mode(regs) { show_faulting_vma((*regs).ret); }
    pr_info!("ECR: 0x%08lx EFA: 0x%08lx ERET: 0x%08lx\n", (*regs).ecr.full, (*current).thread.fault_address, (*regs).ret);
    pr_info!("STAT32: 0x%08lx", (*regs).status32);
    pr_cont!(" [%2s%2s%2s%2s]   ", if (*regs).status32 & STATUS_IE_MASK != 0 { "IE " } else { "" }, if (*regs).status32 & STATUS_U_MASK != 0 { "U " } else { "K " }, if (*regs).status32 & STATUS_DE_MASK != 0 { "DE " } else { "" }, if (*regs).status32 & STATUS_AE_MASK != 0 { "AE " } else { "" });
    print_regs_scratch(regs);
    if !cregs.is_null() { print_regs_callee(cregs); }
    preempt_disable();
}

pub unsafe fn show_kernel_fault_diag(str_: *const c_char, regs: *mut pt_regs, address: c_ulong) {
    (*current).thread.fault_address = address;
    pr_info!("\n%s\n", str_);
    show_regs(regs);
    if !user_mode(regs) { show_stacktrace(current, regs, KERN_DEFAULT); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
