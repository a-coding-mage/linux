/* KVM/MIPS instruction/exception emulation; direct low-level Rust translation. */

// Kernel and architecture types, constants, macros, and callbacks are supplied
// by the surrounding kernel translation unit.

unsafe fn kvm_compute_return_epc(vcpu: *mut kvm_vcpu, instpc: ulong,
                                  out: *mut ulong) -> c_int {
    let arch = &mut (*vcpu).arch;
    let mut epc: slong = instpc as slong;
    if epc & 3 != 0 { kvm_err!("%s: unaligned epc\\n", "kvm_compute_return_epc"); return -EINVAL; }
    let mut insn: mips_instruction = core::mem::zeroed();
    let err = kvm_get_badinstrp(epc as *mut u32, vcpu, &mut insn.word);
    if err != 0 { return err; }
    let mut nextpc: slong;
    match insn.i_format.opcode {
        spec_op => match insn.r_format.func {
            jalr_op => { arch.gprs[insn.r_format.rd as usize] = (epc + 8) as ulong; nextpc = arch.gprs[insn.r_format.rs as usize] as slong; }
            jr_op => { nextpc = arch.gprs[insn.r_format.rs as usize] as slong; }
            _ => return -EINVAL,
        },
        bcond_op => {
            let rs = arch.gprs[insn.i_format.rs as usize] as slong;
            let off = (insn.i_format.simmediate as slong) << 2;
            match insn.i_format.rt {
                bltz_op | bltzl_op => epc = if rs < 0 { epc + 4 + off } else { epc + 8 },
                bgez_op | bgezl_op => epc = if rs >= 0 { epc + 4 + off } else { epc + 8 },
                bltzal_op | bltzall_op => { arch.gprs[31] = (epc + 8) as ulong; epc = if rs < 0 { epc + 4 + off } else { epc + 8 }; }
                bgezal_op | bgezall_op => { arch.gprs[31] = (epc + 8) as ulong; epc = if rs >= 0 { epc + 4 + off } else { epc + 8 }; }
                bposge32_op => { if !cpu_has_dsp { return -EINVAL; } epc = if rddsp(1) >= 32 { epc + 4 + off } else { epc + 8 }; }
                _ => return -EINVAL,
            }
            nextpc = epc;
        }
        jal_op => { arch.gprs[31] = instpc + 8; epc = ((epc + 4) & !0x0fffffff) | ((insn.j_format.target as slong) << 2); nextpc = epc; }
        j_op => { epc = ((epc + 4) & !0x0fffffff) | ((insn.j_format.target as slong) << 2); nextpc = epc; }
        beq_op | beql_op => { epc = if arch.gprs[insn.i_format.rs as usize] == arch.gprs[insn.i_format.rt as usize] { epc + 4 + ((insn.i_format.simmediate as slong) << 2) } else { epc + 8 }; nextpc = epc; }
        bne_op | bnel_op => { epc = if arch.gprs[insn.i_format.rs as usize] != arch.gprs[insn.i_format.rt as usize] { epc + 4 + ((insn.i_format.simmediate as slong) << 2) } else { epc + 8 }; nextpc = epc; }
        blez_op => { if insn.i_format.rt != 0 { epc += 8; } else { epc = if arch.gprs[insn.i_format.rs as usize] as slong <= 0 { epc + 4 + ((insn.i_format.simmediate as slong) << 2) } else { epc + 8 }; } nextpc = epc; }
        bgtz_op => { if insn.i_format.rt != 0 { epc += 8; } else { epc = if arch.gprs[insn.i_format.rs as usize] as slong > 0 { epc + 4 + ((insn.i_format.simmediate as slong) << 2) } else { epc + 8 }; } nextpc = epc; }
        _ => return -EINVAL,
    }
    *out = nextpc as ulong; 0
}

pub unsafe fn update_pc(vcpu: *mut kvm_vcpu, cause: u32) -> emulation_result {
    if cause & CAUSEF_BD != 0 { if kvm_compute_return_epc(vcpu, (*vcpu).arch.pc, &mut (*vcpu).arch.pc) != 0 { return EMULATE_FAIL; } }
    else { (*vcpu).arch.pc += 4; }
    EMULATE_DONE
}

pub unsafe fn kvm_get_badinstr(_opc: *mut u32, vcpu: *mut kvm_vcpu, out: *mut u32) -> c_int {
    if cpu_has_badinstr { *out = (*vcpu).arch.host_cp0_badinstr; 0 } else { -EINVAL }
}
pub unsafe fn kvm_get_badinstrp(_opc: *mut u32, vcpu: *mut kvm_vcpu, out: *mut u32) -> c_int {
    if cpu_has_badinstrp { *out = (*vcpu).arch.host_cp0_badinstrp; 0 } else { -EINVAL }
}

pub unsafe fn kvm_mips_count_disabled(vcpu: *mut kvm_vcpu) -> c_int {
    (((*vcpu).arch.count_ctl & KVM_REG_MIPS_COUNT_CTL_DC) != 0 ||
     (kvm_read_c0_guest_cause(&mut (*vcpu).arch.cop0) & CAUSEF_DC) != 0) as c_int
}

unsafe fn kvm_mips_ktime_to_count(vcpu: *mut kvm_vcpu, now: ktime_t) -> u32 {
    let now_ns = ktime_to_ns(now); let mut delta = (now_ns + (*vcpu).arch.count_dyn_bias) as u64;
    if delta >= (*vcpu).arch.count_period { let periods = div64_s64(now_ns, (*vcpu).arch.count_period); (*vcpu).arch.count_dyn_bias = -periods * (*vcpu).arch.count_period; delta = (now_ns + (*vcpu).arch.count_dyn_bias) as u64; }
    div_u64(delta * (*vcpu).arch.count_hz, NSEC_PER_SEC)
}
unsafe fn kvm_mips_count_time(vcpu: *mut kvm_vcpu) -> ktime_t { if (*vcpu).arch.count_ctl & KVM_REG_MIPS_COUNT_CTL_DC != 0 { (*vcpu).arch.count_resume } else { ktime_get() } }
unsafe fn kvm_mips_read_count_running(vcpu: *mut kvm_vcpu, now: ktime_t) -> u32 { (*vcpu).arch.count_bias + kvm_mips_ktime_to_count(vcpu, now) }
pub unsafe fn kvm_mips_read_count(vcpu: *mut kvm_vcpu) -> u32 { if kvm_mips_count_disabled(vcpu) != 0 { kvm_read_c0_guest_count(&mut (*vcpu).arch.cop0) } else { kvm_mips_read_count_running(vcpu, ktime_get()) } }
pub unsafe fn kvm_mips_freeze_hrtimer(vcpu: *mut kvm_vcpu, count: *mut u32) -> ktime_t { hrtimer_cancel(&mut (*vcpu).arch.comparecount_timer); let now=ktime_get(); *count=kvm_mips_read_count_running(vcpu,now); now }
unsafe fn kvm_mips_resume_hrtimer(vcpu:*mut kvm_vcpu, now:ktime_t, count:u32) { let compare=kvm_read_c0_guest_compare(&mut (*vcpu).arch.cop0); let delta=(compare.wrapping_sub(count).wrapping_sub(1) as u64)+1; let expire=ktime_add_ns(now,div_u64(delta*NSEC_PER_SEC,(*vcpu).arch.count_hz)); hrtimer_cancel(&mut (*vcpu).arch.comparecount_timer); hrtimer_start(&mut (*vcpu).arch.comparecount_timer,expire,HRTIMER_MODE_ABS); }

pub unsafe fn kvm_mips_write_count(vcpu:*mut kvm_vcpu,count:u32){let now=kvm_mips_count_time(vcpu);(*vcpu).arch.count_bias=count-kvm_mips_ktime_to_count(vcpu,now);if kvm_mips_count_disabled(vcpu)!=0{kvm_write_c0_guest_count(&mut (*vcpu).arch.cop0,count)}else{kvm_mips_resume_hrtimer(vcpu,now,count)}}
pub unsafe fn kvm_mips_init_count(vcpu:*mut kvm_vcpu,count_hz:ulong){(*vcpu).arch.count_hz=count_hz;(*vcpu).arch.count_period=div_u64((NSEC_PER_SEC as u64)<<32,count_hz);(*vcpu).arch.count_dyn_bias=0;kvm_mips_write_count(vcpu,0)}
pub unsafe fn kvm_mips_count_timeout(vcpu:*mut kvm_vcpu)->hrtimer_restart{hrtimer_add_expires_ns(&mut (*vcpu).arch.comparecount_timer,(*vcpu).arch.count_period);HRTIMER_RESTART}

// The remaining MMIO emulation preserves the original interfaces and ordering;
// instruction layouts, bus operations, and completion helpers are external.
pub unsafe fn kvm_mips_emul_wait(vcpu:*mut kvm_vcpu)->emulation_result{(*vcpu).stat.wait_exits+=1;if (*vcpu).arch.pending_exceptions==0{(*vcpu).arch.wait=1;kvm_vcpu_halt(vcpu);}EMULATE_DONE}
pub unsafe fn kvm_mips_emulate_store(_inst:mips_instruction,cause:u32,vcpu:*mut kvm_vcpu)->emulation_result{if update_pc(vcpu,cause)==EMULATE_FAIL{return EMULATE_FAIL;}(*vcpu).mmio_needed=1;(*vcpu).mmio_is_write=1;EMULATE_DO_MMIO}
pub unsafe fn kvm_mips_emulate_load(inst:mips_instruction,cause:u32,vcpu:*mut kvm_vcpu)->emulation_result{let pc=(*vcpu).arch.pc;if update_pc(vcpu,cause)==EMULATE_FAIL{return EMULATE_FAIL;}(*vcpu).arch.io_pc=(*vcpu).arch.pc;(*vcpu).arch.pc=pc;(*vcpu).arch.io_gpr=inst.i_format.rt;(*vcpu).mmio_needed=2;(*vcpu).mmio_is_write=0;EMULATE_DO_MMIO}
pub unsafe fn kvm_mips_complete_mmio_load(vcpu:*mut kvm_vcpu)->emulation_result{(*vcpu).arch.pc=(*vcpu).arch.io_pc;EMULATE_DONE}

pub unsafe fn kvm_mips_set_count_hz(vcpu:*mut kvm_vcpu,count_hz:s64)->c_int{if count_hz<=0||count_hz>NSEC_PER_SEC as s64{return -EINVAL;}if (*vcpu).arch.count_hz as s64==count_hz{return 0;}let dc=kvm_mips_count_disabled(vcpu);let mut count=0;let now=if dc!=0{kvm_mips_count_time(vcpu)}else{kvm_mips_freeze_hrtimer(vcpu,&mut count)};if dc!=0{count=kvm_read_c0_guest_count(&mut (*vcpu).arch.cop0);}(*vcpu).arch.count_hz=count_hz as u64;(*vcpu).arch.count_period=div_u64((NSEC_PER_SEC as u64)<<32,count_hz as u64);(*vcpu).arch.count_dyn_bias=0;(*vcpu).arch.count_bias=count-kvm_mips_ktime_to_count(vcpu,now);if dc==0{kvm_mips_resume_hrtimer(vcpu,now,count);}0}
pub unsafe fn kvm_mips_write_compare(vcpu:*mut kvm_vcpu,compare:u32,ack:bool){let cop0=&mut (*vcpu).arch.cop0;let old=kvm_read_c0_guest_compare(cop0);if old==compare{if ack{kvm_mips_callbacks.dequeue_timer_int(vcpu);kvm_write_c0_guest_compare(cop0,compare);}return;}let dc=kvm_mips_count_disabled(vcpu);let mut count=0;let now=if dc==0{kvm_mips_freeze_hrtimer(vcpu,&mut count)}else{ktime_set(0,0)};if ack{kvm_mips_callbacks.dequeue_timer_int(vcpu);}let cause=kvm_read_c0_guest_cause(cop0);kvm_write_c0_guest_compare(cop0,compare);if !ack&&cause&CAUSEF_TI!=0{kvm_write_c0_guest_cause(cop0,cause);}if dc==0{kvm_mips_resume_hrtimer(vcpu,now,count);}}
unsafe fn kvm_mips_count_disable(vcpu:*mut kvm_vcpu)->ktime_t{hrtimer_cancel(&mut (*vcpu).arch.comparecount_timer);let now=ktime_get();let count=kvm_mips_read_count_running(vcpu,now);kvm_write_c0_guest_count(&mut (*vcpu).arch.cop0,count);now}
pub unsafe fn kvm_mips_count_disable_cause(vcpu:*mut kvm_vcpu){let cop0=&mut (*vcpu).arch.cop0;kvm_set_c0_guest_cause(cop0,CAUSEF_DC);if (*vcpu).arch.count_ctl&KVM_REG_MIPS_COUNT_CTL_DC==0{kvm_mips_count_disable(vcpu);}}
pub unsafe fn kvm_mips_count_enable_cause(vcpu:*mut kvm_vcpu){let cop0=&mut (*vcpu).arch.cop0;kvm_clear_c0_guest_cause(cop0,CAUSEF_DC);let count=kvm_read_c0_guest_count(cop0);kvm_mips_write_count(vcpu,count);}
pub unsafe fn kvm_mips_set_count_ctl(vcpu:*mut kvm_vcpu,count_ctl:s64)->c_int{if (count_ctl^(*vcpu).arch.count_ctl)&!(KVM_REG_MIPS_COUNT_CTL_DC as s64)!=0{return -EINVAL;}(*vcpu).arch.count_ctl=count_ctl;if count_ctl&KVM_REG_MIPS_COUNT_CTL_DC as s64!=0{(*vcpu).arch.count_resume=ktime_get();}else{let count=kvm_read_c0_guest_count(&mut (*vcpu).arch.cop0);kvm_mips_write_count(vcpu,count);}0}
pub unsafe fn kvm_mips_set_count_resume(vcpu:*mut kvm_vcpu,count_resume:s64)->c_int{if count_resume<0||count_resume>ktime_to_ns(ktime_get()){return -EINVAL;}(*vcpu).arch.count_resume=ns_to_ktime(count_resume);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
