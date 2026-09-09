// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2020-2023 Loongson Technology Corporation Limited */

// Kernel and architecture dependencies are supplied by the surrounding crate.

unsafe fn kvm_emu_cpucfg(vcpu: *mut kvm_vcpu, inst: larch_inst) -> i32 {
    if inst.reg2_format.opcode != cpucfg_op { return EMULATE_FAIL; }
    let rd = inst.reg2_format.rd;
    let rj = inst.reg2_format.rj;
    (*vcpu).stat.cpucfg_exits += 1;
    let index = (*vcpu).arch.gprs[rj] as u32;
    preempt_disable();
    match index {
        0..=(KVM_MAX_CPUCFG_REGS - 1) => (*vcpu).arch.gprs[rd] = (*vcpu).arch.cpucfg[index as usize] as _ ,
        CPUCFG_KVM_SIG => (*vcpu).arch.gprs[rd] = *(KVM_SIGNATURE as *const u32) as _,
        CPUCFG_KVM_FEATURE => (*vcpu).arch.gprs[rd] = ((*vcpu).kvm.as_ref().unwrap().arch.pv_features & LOONGARCH_PV_FEAT_MASK) as _,
        _ => (*vcpu).arch.gprs[rd] = 0,
    }
    preempt_enable();
    EMULATE_DONE
}

unsafe fn kvm_emu_read_csr(vcpu: *mut kvm_vcpu, csrid: i32) -> u64 {
    let csr = (*vcpu).arch.csr;
    if get_gcsr_flag(csrid) & SW_GCSR != 0 { kvm_read_sw_gcsr(csr, csrid) }
    else { pr_warn_once!("Unsupported csrrd 0x%x with pc %lx\n", csrid, (*vcpu).arch.pc); 0 }
}

unsafe fn kvm_emu_write_csr(vcpu: *mut kvm_vcpu, csrid: i32, val: u64) -> u64 {
    let csr = (*vcpu).arch.csr;
    if get_gcsr_flag(csrid) & SW_GCSR != 0 {
        let old = kvm_read_sw_gcsr(csr, csrid); kvm_write_sw_gcsr(csr, csrid, val); old
    } else { pr_warn_once!("Unsupported csrwr 0x%x with pc %lx\n", csrid, (*vcpu).arch.pc); 0 }
}

unsafe fn kvm_emu_xchg_csr(vcpu: *mut kvm_vcpu, csrid: i32, mask: u64, val: u64) -> u64 {
    let csr = (*vcpu).arch.csr;
    if get_gcsr_flag(csrid) & SW_GCSR != 0 {
        let old = kvm_read_sw_gcsr(csr, csrid);
        kvm_write_sw_gcsr(csr, csrid, (old & !mask) | (val & mask)); old
    } else { pr_warn_once!("Unsupported csrxchg 0x%x with pc %lx\n", csrid, (*vcpu).arch.pc); 0 }
}

unsafe fn kvm_handle_csr(vcpu: *mut kvm_vcpu, inst: larch_inst) -> i32 {
    let rd = inst.reg2csr_format.rd; let rj = inst.reg2csr_format.rj; let csrid = inst.reg2csr_format.csr;
    if csrid >= LOONGARCH_CSR_PERFCTRL0 && csrid <= (*vcpu).arch.max_pmu_csrid && kvm_guest_has_pmu(&(*vcpu).arch) {
        (*vcpu).arch.pc -= 4; kvm_make_request(KVM_REQ_PMU, vcpu); return EMULATE_DONE;
    }
    match rj { 0 => (*vcpu).arch.gprs[rd] = kvm_emu_read_csr(vcpu, csrid),
        1 => { let v = kvm_emu_write_csr(vcpu, csrid, (*vcpu).arch.gprs[rd]); (*vcpu).arch.gprs[rd] = v; },
        _ => { let v = kvm_emu_xchg_csr(vcpu, csrid, (*vcpu).arch.gprs[rj], (*vcpu).arch.gprs[rd]); (*vcpu).arch.gprs[rd] = v; } }
    EMULATE_DONE
}

pub unsafe fn kvm_emu_iocsr(inst: larch_inst, run: *mut kvm_run, vcpu: *mut kvm_vcpu) -> i32 {
    let rd = inst.reg2_format.rd; let rj = inst.reg2_format.rj; let opcode = inst.reg2_format.opcode;
    (*run).iocsr_io.phys_addr = (*vcpu).arch.gprs[rj] as _; (*run).iocsr_io.is_write = 0;
    let val = &mut (*vcpu).arch.gprs[rd] as *mut _ as *mut u8;
    (*run).iocsr_io.len = match opcode { iocsrrdb_op|iocsrwrb_op=>1, iocsrrdh_op|iocsrwrh_op=>2, iocsrrdw_op|iocsrwrw_op=>4, iocsrrdd_op|iocsrwrd_op=>8, _=>return EMULATE_FAIL };
    if matches!(opcode, iocsrwrb_op|iocsrwrh_op|iocsrwrw_op|iocsrwrd_op) { (*run).iocsr_io.is_write=1; }
    let idx=srcu_read_lock(&(*vcpu).kvm.srcu);
    let ret=if (*run).iocsr_io.is_write!=0 { kvm_io_bus_write(vcpu,KVM_IOCSR_BUS,(*run).iocsr_io.phys_addr,(*run).iocsr_io.len,val as *mut _) } else { (*vcpu).arch.io_gpr=rd; kvm_io_bus_read(vcpu,KVM_IOCSR_BUS,(*run).iocsr_io.phys_addr,(*run).iocsr_io.len,(*run).iocsr_io.data.as_mut_ptr()) };
    srcu_read_unlock(&(*vcpu).kvm.srcu,idx);
    if ret==0 { if (*run).iocsr_io.is_write==0 { kvm_complete_iocsr_read(vcpu,run); } return EMULATE_DONE; }
    if (*run).iocsr_io.is_write!=0 { core::ptr::copy_nonoverlapping(val,(*run).iocsr_io.data.as_mut_ptr(),(*run).iocsr_io.len as usize); }
    EMULATE_DO_IOCSR
}

pub unsafe fn kvm_complete_iocsr_read(vcpu:*mut kvm_vcpu,run:*mut kvm_run)->i32 { let p=&mut (*vcpu).arch.gprs[(*vcpu).arch.io_gpr] as *mut _ as *mut u8; match (*run).iocsr_io.len {1=>*p=(*run).iocsr_io.data[0],2=>*(p as *mut i16)=*( (*run).iocsr_io.data.as_ptr() as *const i16),4=>*(p as *mut i32)=*( (*run).iocsr_io.data.as_ptr() as *const i32),8=>*(p as *mut i64)=*( (*run).iocsr_io.data.as_ptr() as *const i64),_=>return EMULATE_FAIL}; EMULATE_DONE }

pub unsafe fn kvm_emu_idle(vcpu:*mut kvm_vcpu)->i32 { (*vcpu).stat.idle_exits+=1; trace_kvm_exit_idle(vcpu,KVM_TRACE_EXIT_IDLE); if !kvm_arch_vcpu_runnable(vcpu){kvm_vcpu_halt(vcpu);} EMULATE_DONE }

unsafe fn kvm_trap_handle_gspr(vcpu:*mut kvm_vcpu)->i32 { let mut inst=larch_inst{word:(*vcpu).arch.badi}; let pc=(*vcpu).arch.pc; update_pc(&mut (*vcpu).arch); let mut er=EMULATE_FAIL; match (inst.word>>24)&0xff {0=>er=kvm_emu_cpucfg(vcpu,inst),4=>er=kvm_handle_csr(vcpu,inst),6=>match (inst.word>>22)&0x3ff {0x18=>er=EMULATE_DONE,0x19=>match (inst.word>>15)&0x1ffff {0xc90=>er=kvm_emu_iocsr(inst,(*vcpu).run,vcpu),0xc91=>er=kvm_emu_idle(vcpu),_=>{}},_=>{}},_=>{}} if er==EMULATE_FAIL {(*vcpu).arch.pc=pc;} er }

unsafe fn kvm_handle_gspr(vcpu:*mut kvm_vcpu,_ecode:i32)->i32 { match kvm_trap_handle_gspr(vcpu) {EMULATE_DONE=>RESUME_GUEST,EMULATE_DO_MMIO=>{(*(*vcpu).run).exit_reason=KVM_EXIT_MMIO;RESUME_HOST},EMULATE_DO_IOCSR=>{(*(*vcpu).run).exit_reason=KVM_EXIT_LOONGARCH_IOCSR;RESUME_HOST},_=>{kvm_queue_exception(vcpu,EXCCODE_INE,0);RESUME_GUEST}} }

// The remaining MMIO, fault, hypercall, and dispatch handlers retain the C control flow.
pub unsafe fn kvm_complete_mmio_read(vcpu:*mut kvm_vcpu,run:*mut kvm_run)->i32 { update_pc(&mut (*vcpu).arch); let p=&mut (*vcpu).arch.gprs[(*vcpu).arch.io_gpr] as *mut _ as *mut u8; match (*run).mmio.len {1=>if (*vcpu).mmio_needed==2{*(p as *mut i8)=(*run).mmio.data[0] as i8}else{*p=(*run).mmio.data[0]},2=>*(p as *mut i16)=*( (*run).mmio.data.as_ptr() as *const i16),4=>*(p as *mut i32)=*( (*run).mmio.data.as_ptr() as *const i32),8=>*(p as *mut i64)=*( (*run).mmio.data.as_ptr() as *const i64),_=>return EMULATE_FAIL}; EMULATE_DONE }

pub unsafe fn kvm_complete_user_service(vcpu:*mut kvm_vcpu,run:*mut kvm_run)->i32 { update_pc(&mut (*vcpu).arch); kvm_write_reg(vcpu,LOONGARCH_GPR_A0,(*run).hypercall.ret); 0 }

pub unsafe fn kvm_emu_mmio_read(vcpu:*mut kvm_vcpu,inst:larch_inst)->i32 { let op=(inst.word>>24)&0xff; let rd=match op{0x24..=0x27=>inst.reg2i14_format.rd,0x28..=0x2e=>inst.reg2i12_format.rd,0x38=>inst.reg3_format.rd,_=>return EMULATE_FAIL}; let len=match op{0x24..=0x27=>if inst.reg2i14_format.opcode==ldptrw_op{4}else if inst.reg2i14_format.opcode==ldptrd_op{8}else{return EMULATE_FAIL},0x28..=0x2e=>match inst.reg2i12_format.opcode{ldb_op|ldbu_op=>1,ldh_op|ldhu_op=>2,ldw_op|ldwu_op=>4,ldd_op=>8,_=>return EMULATE_FAIL},0x38=>match inst.reg3_format.opcode{ldxb_op|ldxbu_op=>1,ldxh_op|ldxhu_op=>2,ldxw_op|ldxwu_op=>4,ldxd_op=>8,_=>return EMULATE_FAIL},_=>0}; (*vcpu).run.mmio.phys_addr=(*vcpu).arch.badv; (*vcpu).run.mmio.len=len; (*vcpu).arch.io_gpr=rd; (*vcpu).mmio_needed=2; EMULATE_DO_MMIO }
pub unsafe fn kvm_emu_mmio_write(vcpu:*mut kvm_vcpu,inst:larch_inst)->i32 { let op=(inst.word>>24)&0xff; let (rd,len)=match op{0x24..=0x27=>(inst.reg2i14_format.rd,if inst.reg2i14_format.opcode==stptrw_op{4}else if inst.reg2i14_format.opcode==stptrd_op{8}else{return EMULATE_FAIL}),0x28..=0x2e=>(inst.reg2i12_format.rd,match inst.reg2i12_format.opcode{stb_op=>1,sth_op=>2,stw_op=>4,std_op=>8,_=>return EMULATE_FAIL}),0x38=>(inst.reg3_format.rd,match inst.reg3_format.opcode{stxb_op=>1,stxh_op=>2,stxw_op=>4,stxd_op=>8,_=>return EMULATE_FAIL}),_=>return EMULATE_FAIL}; (*vcpu).run.mmio.phys_addr=(*vcpu).arch.badv; (*vcpu).run.mmio.len=len; core::ptr::copy_nonoverlapping((&(*vcpu).arch.gprs[rd]) as *const _ as *const u8,(*vcpu).run.mmio.data.as_mut_ptr(),len as usize); EMULATE_DO_MMIO }
unsafe fn kvm_handle_rdwr_fault(vcpu:*mut kvm_vcpu,write:bool,ecode:i32)->i32 { if (*vcpu).arch.badv>=(*vcpu).kvm.arch.gpa_size{kvm_queue_exception(vcpu,EXCCODE_ADE,EXSUBCODE_ADEM);return RESUME_GUEST} if kvm_handle_mm_fault(vcpu,(*vcpu).arch.badv,write,ecode)!=0 { let er=if write{kvm_emu_mmio_write(vcpu,larch_inst{word:(*vcpu).arch.badi})}else{kvm_emu_mmio_read(vcpu,larch_inst{word:(*vcpu).arch.badi})}; return if er==EMULATE_DONE{RESUME_GUEST}else if er==EMULATE_DO_MMIO{(*vcpu).run.exit_reason=KVM_EXIT_MMIO;RESUME_HOST}else{ kvm_queue_exception(vcpu,EXCCODE_ADE,EXSUBCODE_ADEM);RESUME_GUEST}} RESUME_GUEST }
unsafe fn kvm_handle_read_fault(vcpu:*mut kvm_vcpu,e:i32)->i32{kvm_handle_rdwr_fault(vcpu,false,e)}
unsafe fn kvm_handle_write_fault(vcpu:*mut kvm_vcpu,e:i32)->i32{kvm_handle_rdwr_fault(vcpu,true,e)}
unsafe fn kvm_fault_ni(vcpu:*mut kvm_vcpu,_e:i32)->i32{kvm_queue_exception(vcpu,EXCCODE_INE,0);RESUME_GUEST}
pub unsafe fn kvm_handle_fault(vcpu:*mut kvm_vcpu,fault:i32)->i32 { match fault {EXCCODE_TLBI|EXCCODE_TLBL=>kvm_handle_read_fault(vcpu,fault),EXCCODE_TLBS|EXCCODE_TLBM=>kvm_handle_write_fault(vcpu,fault),EXCCODE_GSPR=>kvm_handle_gspr(vcpu,fault),_=>kvm_fault_ni(vcpu,fault)} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
