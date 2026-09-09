// SPDX-License-Identifier: GPL-2.0
/* kvm guest debug support */
/*
 * Copyright IBM Corp. 2014
 * Author(s): David Hildenbrand <dahi@linux.vnet.ibm.com>
 */

const MAX_INST_SIZE: i32 = 6;
const MAX_WP_SIZE: i32 = 100;
const MAX_BP_COUNT: i32 = 50;

unsafe fn extend_address_range(start: *mut u64, stop: *mut u64, estart: u64, mut len: i32) {
    let estop: u64;
    if len > 0 { len -= 1; } else { len = 0; }
    estop = estart.wrapping_add(len as u64);
    if *start == 0 && *stop == 0 {
        *start = estart; *stop = estop;
    } else if *start <= *stop {
        if estart < *start { *start = estart; }
        if estop > *stop { *stop = estop; }
    } else {
        if estart <= *stop {
            if estop > *stop { *stop = estop; }
        } else if estop > *start {
            if estart < *start { *start = estart; }
        } else if estop.wrapping_sub(*stop) < (*start).wrapping_sub(estart) {
            *stop = estop;
        } else { *start = estart; }
    }
}

unsafe fn enable_all_hw_bp(vcpu: *mut kvm_vcpu) {
    let cr9 = &mut (*(*vcpu).arch.sie_block).gcr[9];
    let cr10 = &mut (*(*vcpu).arch.sie_block).gcr[10];
    let cr11 = &mut (*(*vcpu).arch.sie_block).gcr[11];
    if (*vcpu).arch.guestdbg.nr_hw_bp <= 0 || (*vcpu).arch.guestdbg.hw_bp_info.is_null() { return; }
    if *cr9 & PER_EVENT_BRANCH == 0 { *cr9 |= PER_CONTROL_BRANCH_ADDRESS; }
    *cr9 |= PER_EVENT_IFETCH | PER_EVENT_BRANCH;
    for i in 0..(*vcpu).arch.guestdbg.nr_hw_bp {
        let mut start = (*vcpu).arch.guestdbg.hw_bp_info.add(i as usize).addr;
        let mut len = (*vcpu).arch.guestdbg.hw_bp_info.add(i as usize).len;
        if start < MAX_INST_SIZE as _ { len += start; start = 0; }
        else { start -= MAX_INST_SIZE as _; len += MAX_INST_SIZE as _; }
        extend_address_range(cr10, cr11, start as u64, len as i32);
    }
}

unsafe fn enable_all_hw_wp(vcpu: *mut kvm_vcpu) {
    let cr9 = &mut (*(*vcpu).arch.sie_block).gcr[9];
    let cr10 = &mut (*(*vcpu).arch.sie_block).gcr[10];
    let cr11 = &mut (*(*vcpu).arch.sie_block).gcr[11];
    if (*vcpu).arch.guestdbg.nr_hw_wp <= 0 || (*vcpu).arch.guestdbg.hw_wp_info.is_null() { return; }
    if *cr9 & PER_EVENT_STORE != 0 && *cr9 & PER_CONTROL_ALTERATION != 0 {
        *cr9 &= !PER_CONTROL_ALTERATION; *cr10 = 0; *cr11 = u64::MAX;
    } else {
        *cr9 &= !PER_CONTROL_ALTERATION; *cr9 |= PER_EVENT_STORE;
        for i in 0..(*vcpu).arch.guestdbg.nr_hw_wp {
            let p = (*vcpu).arch.guestdbg.hw_wp_info.add(i as usize);
            extend_address_range(cr10, cr11, (*p).addr as u64, (*p).len as i32);
        }
    }
}

pub unsafe fn kvm_s390_backup_guest_per_regs(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.guestdbg.cr0 = (*(*vcpu).arch.sie_block).gcr[0];
    (*vcpu).arch.guestdbg.cr9 = (*(*vcpu).arch.sie_block).gcr[9];
    (*vcpu).arch.guestdbg.cr10 = (*(*vcpu).arch.sie_block).gcr[10];
    (*vcpu).arch.guestdbg.cr11 = (*(*vcpu).arch.sie_block).gcr[11];
}

pub unsafe fn kvm_s390_restore_guest_per_regs(vcpu: *mut kvm_vcpu) {
    (*(*vcpu).arch.sie_block).gcr[0] = (*vcpu).arch.guestdbg.cr0;
    (*(*vcpu).arch.sie_block).gcr[9] = (*vcpu).arch.guestdbg.cr9;
    (*(*vcpu).arch.sie_block).gcr[10] = (*vcpu).arch.guestdbg.cr10;
    (*(*vcpu).arch.sie_block).gcr[11] = (*vcpu).arch.guestdbg.cr11;
}

pub unsafe fn kvm_s390_patch_guest_per_regs(vcpu: *mut kvm_vcpu) {
    if guestdbg_sstep_enabled(vcpu) {
        (*(*vcpu).arch.sie_block).gcr[0] &= !CR0_CLOCK_COMPARATOR_SUBMASK;
        (*(*vcpu).arch.sie_block).gcr[9] |= PER_EVENT_IFETCH;
        (*(*vcpu).arch.sie_block).gcr[10] = 0; (*(*vcpu).arch.sie_block).gcr[11] = u64::MAX;
    }
    if guestdbg_hw_bp_enabled(vcpu) { enable_all_hw_bp(vcpu); enable_all_hw_wp(vcpu); }
    if (*(*vcpu).arch.sie_block).gcr[9] & PER_EVENT_NULLIFICATION != 0 {
        (*(*vcpu).arch.sie_block).gcr[9] &= !PER_EVENT_NULLIFICATION;
    }
}

unsafe fn __import_wp_info(vcpu: *mut kvm_vcpu, bp_data: *mut kvm_hw_breakpoint, wp_info: *mut kvm_hw_wp_info_arch) -> i32 {
    (*wp_info).len = (*bp_data).len; (*wp_info).addr = (*bp_data).addr; (*wp_info).phys_addr = (*bp_data).phys_addr; (*wp_info).old_data = core::ptr::null_mut();
    if (*wp_info).len < 0 || (*wp_info).len > MAX_WP_SIZE { return -EINVAL; }
    (*wp_info).old_data = kmalloc((*wp_info).len as _, GFP_KERNEL_ACCOUNT);
    if (*wp_info).old_data.is_null() { return -ENOMEM; }
    let ret = read_guest_abs(vcpu, (*wp_info).phys_addr, (*wp_info).old_data, (*wp_info).len);
    if ret != 0 { kfree((*wp_info).old_data); (*wp_info).old_data = core::ptr::null_mut(); }
    ret
}

pub unsafe fn kvm_s390_import_bp_data(vcpu: *mut kvm_vcpu, dbg: *mut kvm_guest_debug) -> i32 {
    let mut ret = 0; let mut nr_wp = 0; let mut nr_bp = 0;
    if (*dbg).arch.nr_hw_bp <= 0 || (*dbg).arch.hw_bp.is_null() { return 0; }
    if (*dbg).arch.nr_hw_bp > MAX_BP_COUNT { return -EINVAL; }
    let bp_data = memdup_array_user((*dbg).arch.hw_bp, (*dbg).arch.nr_hw_bp, core::mem::size_of::<kvm_hw_breakpoint>());
    if IS_ERR(bp_data) { return PTR_ERR(bp_data); }
    for i in 0..(*dbg).arch.nr_hw_bp { match (*bp_data.add(i as usize)).type_ { KVM_HW_WP_WRITE => nr_wp += 1, KVM_HW_BP => nr_bp += 1, _ => {} } }
    let wp_info = if nr_wp > 0 { kmalloc_objs::<kvm_hw_wp_info_arch>(nr_wp, GFP_KERNEL_ACCOUNT) } else { core::ptr::null_mut() };
    if nr_wp > 0 && wp_info.is_null() { ret = -ENOMEM; kfree(bp_data); return ret; }
    let bp_info = if nr_bp > 0 { kmalloc_objs::<kvm_hw_bp_info_arch>(nr_bp, GFP_KERNEL_ACCOUNT) } else { core::ptr::null_mut() };
    if nr_bp > 0 && bp_info.is_null() { kfree(bp_data); kfree(wp_info); return -ENOMEM; }
    nr_wp = 0; nr_bp = 0;
    for i in 0..(*dbg).arch.nr_hw_bp { let b = bp_data.add(i as usize); match (*b).type_ {
        KVM_HW_WP_WRITE => { ret = __import_wp_info(vcpu, b, wp_info.add(nr_wp as usize)); if ret != 0 { while nr_wp > 0 { nr_wp -= 1; kfree((*wp_info.add(nr_wp as usize)).old_data); } kfree(bp_data); kfree(wp_info); kfree(bp_info); return ret; } nr_wp += 1; },
        KVM_HW_BP => { (*bp_info.add(nr_bp as usize)).len = (*b).len; (*bp_info.add(nr_bp as usize)).addr = (*b).addr; nr_bp += 1; }, _ => {}
    }}
    (*vcpu).arch.guestdbg.nr_hw_bp = nr_bp; (*vcpu).arch.guestdbg.hw_bp_info = bp_info;
    (*vcpu).arch.guestdbg.nr_hw_wp = nr_wp; (*vcpu).arch.guestdbg.hw_wp_info = wp_info; kfree(bp_data); 0
}

pub unsafe fn kvm_s390_clear_bp_data(vcpu: *mut kvm_vcpu) {
    for i in 0..(*vcpu).arch.guestdbg.nr_hw_wp { let p = (*vcpu).arch.guestdbg.hw_wp_info.add(i as usize); kfree((*p).old_data); (*p).old_data = core::ptr::null_mut(); }
    kfree((*vcpu).arch.guestdbg.hw_wp_info); (*vcpu).arch.guestdbg.hw_wp_info = core::ptr::null_mut();
    kfree((*vcpu).arch.guestdbg.hw_bp_info); (*vcpu).arch.guestdbg.hw_bp_info = core::ptr::null_mut();
    (*vcpu).arch.guestdbg.nr_hw_wp = 0; (*vcpu).arch.guestdbg.nr_hw_bp = 0;
}

#[inline] unsafe fn in_addr_range(addr: u64, a: u64, b: u64) -> bool { if a <= b { addr >= a && addr <= b } else { addr >= a || addr <= b } }
unsafe fn find_hw_bp(vcpu: *mut kvm_vcpu, addr: usize) -> *mut kvm_hw_bp_info_arch {
    let mut p = (*vcpu).arch.guestdbg.hw_bp_info; for i in 0..(*vcpu).arch.guestdbg.nr_hw_bp { let q = p.add(i as usize); if addr as u64 == (*q).addr || ((*q).len > 0 && in_addr_range(addr as u64, (*q).addr, (*q).addr.wrapping_add((*q).len as u64).wrapping_sub(1))) { return q; } } core::ptr::null_mut()
}

unsafe fn any_wp_changed(vcpu: *mut kvm_vcpu) -> *mut kvm_hw_wp_info_arch {
    for i in 0..(*vcpu).arch.guestdbg.nr_hw_wp { let p = (*vcpu).arch.guestdbg.hw_wp_info.add(i as usize); if (*p).old_data.is_null() || (*p).len <= 0 { continue; } let temp = kmalloc((*p).len as _, GFP_KERNEL_ACCOUNT); if temp.is_null() { continue; } if read_guest_abs(vcpu, (*p).phys_addr, temp, (*p).len) == 0 && memcmp(temp, (*p).old_data, (*p).len) != 0 { kfree(temp); return p; } kfree(temp); } core::ptr::null_mut()
}

pub unsafe fn kvm_s390_prepare_debug_exit(vcpu: *mut kvm_vcpu) { (*vcpu).run.exit_reason = KVM_EXIT_DEBUG; (*vcpu).guest_debug &= !KVM_GUESTDBG_EXIT_PENDING; }

const PER_CODE_MASK: u8 = PER_EVENT_MASK >> 24; const PER_CODE_BRANCH: u8 = PER_EVENT_BRANCH >> 24; const PER_CODE_IFETCH: u8 = PER_EVENT_IFETCH >> 24; const PER_CODE_STORE: u8 = PER_EVENT_STORE >> 24; const PER_CODE_STORE_REAL: u8 = PER_EVENT_STORE_REAL >> 24;
unsafe fn debug_exit_required(vcpu: *mut kvm_vcpu, perc: u8, peraddr: usize) -> i32 { let e = &mut (*vcpu).run.debug.arch; let addr = (*vcpu).arch.sie_block.gpsw.addr; if guestdbg_hw_bp_enabled(vcpu) { if perc & (PER_CODE_STORE | PER_CODE_STORE_REAL) != 0 && (*vcpu).arch.guestdbg.nr_hw_wp > 0 { let p = any_wp_changed(vcpu); if !p.is_null() { e.addr=(*p).addr; e.type_=KVM_HW_WP_WRITE; return 1; } } if perc & (PER_CODE_IFETCH | PER_CODE_BRANCH) != 0 && (*vcpu).arch.guestdbg.nr_hw_bp > 0 { let p=find_hw_bp(vcpu,addr as _); if !p.is_null() && addr != peraddr as u64 { e.addr=addr; e.type_=KVM_HW_BP; (*vcpu).arch.guestdbg.last_bp=addr; return 1; } let p=find_hw_bp(vcpu,peraddr); if !p.is_null() && (*vcpu).arch.guestdbg.last_bp != peraddr as u64 { e.addr=peraddr as u64; e.type_=KVM_HW_BP; return 1; } } } if guestdbg_sstep_enabled(vcpu) && perc & (PER_CODE_IFETCH|PER_CODE_BRANCH) != 0 { e.addr=addr; e.type_=KVM_SINGLESTEP; return 1; } 0 }

unsafe fn filter_guest_per_event(vcpu: *mut kvm_vcpu) -> i32 { let p=(*vcpu).arch.sie_block.perc & ((*vcpu).arch.sie_block.gcr[9] >> 24) & PER_CODE_MASK; let mut guest=if guest_per_enabled(vcpu){p}else{0}; let addr=(*vcpu).arch.sie_block.gpsw.addr; let c10=(*vcpu).arch.sie_block.gcr[10]; let c11=(*vcpu).arch.sie_block.gcr[11]; if guest&PER_CODE_BRANCH!=0 && (*vcpu).arch.sie_block.gcr[9]&PER_CONTROL_BRANCH_ADDRESS!=0 && !in_addr_range(addr,c10,c11){guest&=!PER_CODE_BRANCH;} (*vcpu).arch.sie_block.perc=guest; if guest==0 {(*vcpu).arch.sie_block.iprcc&=!PGM_PER;} 0 }

pub unsafe fn kvm_s390_handle_per_event(vcpu: *mut kvm_vcpu) -> i32 { if debug_exit_required(vcpu,(*vcpu).arch.sie_block.perc,(*vcpu).arch.sie_block.peraddr)!=0 {(*vcpu).guest_debug|=KVM_GUESTDBG_EXIT_PENDING;} filter_guest_per_event(vcpu) }

unsafe fn per_fetched_addr(vcpu: *mut kvm_vcpu, addr: *mut usize) -> i32 {
    let mut exec_ilen: u8 = 0; let mut opcode = [0u16; 3];
    if (*vcpu).arch.sie_block.icptcode == ICPT_PROGI { *addr=(*vcpu).arch.sie_block.peraddr as _; let rc=read_guest_instr(vcpu,*addr as _,opcode.as_mut_ptr(),2); if rc!=0{return rc;} if opcode[0]>>8==0x44{exec_ilen=4;} if opcode[0]&0xff0f==0xc600{exec_ilen=6;} }
    else { *addr=__rewind_psw((*vcpu).arch.sie_block.gpsw,kvm_s390_get_ilen(vcpu)); if (*vcpu).arch.sie_block.icptstatus&1!=0 {exec_ilen=(((*vcpu).arch.sie_block.icptstatus&0x60)>>4) as u8;if exec_ilen==0{exec_ilen=4;}} }
    if exec_ilen!=0 { let rc=read_guest_instr(vcpu,*addr as _,opcode.as_mut_ptr(),exec_ilen);if rc!=0{return rc;} if exec_ilen==6 {let rl=*((opcode.as_ptr().add(1)) as *const i32);*addr=(*addr as u64).wrapping_add((rl as i64*2) as u64) as _;} else {let base=((opcode[1]&0xf000)>>12) as usize;let disp=(opcode[1]&0xfff) as usize;let index=(opcode[0]&0xf) as usize;*addr=if base!=0{(*vcpu).run.s.regs.gprs[base] as _}else{0};*addr=(*addr as u64).wrapping_add(if index!=0{(*vcpu).run.s.regs.gprs[index]}else{0}).wrapping_add(disp as u64) as _;}*addr=kvm_s390_logical_to_effective(vcpu,*addr as _) as _;} 0
}

unsafe fn guest_per_enabled(vcpu: *mut kvm_vcpu)->bool {(*vcpu).arch.sie_block.gpsw.mask&PSW_MASK_PER!=0}

pub unsafe fn kvm_s390_handle_per_ifetch_icpt(vcpu: *mut kvm_vcpu)->i32 { let ilen=kvm_s390_get_ilen(vcpu);let mut p=kvm_s390_pgm_info{code:PGM_PER,per_code:PER_CODE_IFETCH,per_address:__rewind_psw((*vcpu).arch.sie_block.gpsw,ilen),..core::mem::zeroed()};if !guestdbg_enabled(vcpu){return kvm_s390_inject_prog_irq(vcpu,&mut p);}if debug_exit_required(vcpu,p.per_code,p.per_address)!=0{(*vcpu).guest_debug|=KVM_GUESTDBG_EXIT_PENDING;}if !guest_per_enabled(vcpu)||(*vcpu).arch.sie_block.gcr[9]&PER_EVENT_IFETCH==0{return 0;}let mut a=0usize;let rc=per_fetched_addr(vcpu,&mut a);if rc<0{return rc;}if rc!=0{return kvm_s390_inject_program_int(vcpu,PGM_ADDRESSING);}if in_addr_range(a as u64,(*vcpu).arch.sie_block.gcr[10],(*vcpu).arch.sie_block.gcr[11]){return kvm_s390_inject_prog_irq(vcpu,&mut p);}0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
