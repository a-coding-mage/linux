/* SPDX-License-Identifier: GPL-2.0 */

/* C includes and CONFIG_PPC_PSERIES are represented by external dependencies and cfg. */

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn poll_pending() -> c_long { plpar_hcall_norets(H_POLL_PENDING) }

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn cede_processor() -> c_long {
    /* We cannot call tracepoints inside RCU idle regions, so must not trace H_CEDE. */
    plpar_hcall_norets_notrace(H_CEDE)
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn vpa_call(mut flags: c_ulong, cpu: c_ulong, vpa: c_ulong) -> c_long {
    flags <<= H_VPA_FUNC_SHIFT;
    plpar_hcall_norets(H_REGISTER_VPA, flags, cpu, vpa)
}

#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn unregister_vpa(cpu: c_ulong) -> c_long { vpa_call(H_VPA_DEREG_VPA, cpu, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn register_vpa(cpu: c_ulong, vpa: c_ulong) -> c_long { vpa_call(H_VPA_REG_VPA, cpu, vpa) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn unregister_slb_shadow(cpu: c_ulong) -> c_long { vpa_call(H_VPA_DEREG_SLB, cpu, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn register_slb_shadow(cpu: c_ulong, vpa: c_ulong) -> c_long { vpa_call(H_VPA_REG_SLB, cpu, vpa) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn unregister_dtl(cpu: c_ulong) -> c_long { vpa_call(H_VPA_DEREG_DTL, cpu, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn register_dtl(cpu: c_ulong, vpa: c_ulong) -> c_long { vpa_call(H_VPA_REG_DTL, cpu, vpa) }

/* Invokes H_HTM with hardware target, operation, and type encoded in the arguments. */
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn htm_call(flags: c_ulong, target: c_ulong, operation: c_ulong,
                       param1: c_ulong, param2: c_ulong, param3: c_ulong) -> c_long {
    plpar_hcall_norets(H_HTM, flags, target, operation, param1, param2, param3)
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn htm_hcall_wrapper(flags: c_ulong, nodeindex: c_ulong, nodalchipindex: c_ulong,
                                coreindexonchip: c_ulong, type_: c_ulong, htm_op: c_ulong,
                                param1: c_ulong, param2: c_ulong, param3: c_ulong) -> c_long {
    htm_call(H_HTM_FLAGS_HARDWARE_TARGET | flags,
             H_HTM_TARGET_NODE_INDEX(nodeindex) |
             H_HTM_TARGET_NODAL_CHIP_INDEX(nodalchipindex) |
             H_HTM_TARGET_CORE_INDEX_ON_CHIP(coreindexonchip),
             H_HTM_OP(htm_op) | H_HTM_TYPE(type_), param1, param2, param3)
}

#[cfg(CONFIG_PPC_PSERIES)]
extern "C" { pub fn vpa_init(cpu: c_int); }

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_enter(flags: c_ulong, hpte_group: c_ulong, hpte_v: c_ulong,
                              hpte_r: c_ulong, slot: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall(H_ENTER, retbuf.as_mut_ptr(), flags, hpte_group, hpte_v, hpte_r);
    *slot = retbuf[0]; rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_remove(flags: c_ulong, ptex: c_ulong, avpn: c_ulong,
                               old_pteh_ret: *mut c_ulong, old_ptel_ret: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall(H_REMOVE, retbuf.as_mut_ptr(), flags, ptex, avpn);
    *old_pteh_ret = retbuf[0]; *old_ptel_ret = retbuf[1]; rc
}

/* plpar_pte_remove_raw can be called in real mode. It calls plpar_hcall_raw. */
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_remove_raw(flags: c_ulong, ptex: c_ulong, avpn: c_ulong,
                                   old_pteh_ret: *mut c_ulong, old_ptel_ret: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall_raw(H_REMOVE, retbuf.as_mut_ptr(), flags, ptex, avpn);
    *old_pteh_ret = retbuf[0]; *old_ptel_ret = retbuf[1]; rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_read(flags: c_ulong, ptex: c_ulong, old_pteh_ret: *mut c_ulong,
                             old_ptel_ret: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall(H_READ, retbuf.as_mut_ptr(), flags, ptex);
    *old_pteh_ret = retbuf[0]; *old_ptel_ret = retbuf[1]; rc
}

/* plpar_pte_read_raw can be called in real mode. It calls plpar_hcall_raw. */
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_read_raw(flags: c_ulong, ptex: c_ulong, old_pteh_ret: *mut c_ulong,
                                 old_ptel_ret: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall_raw(H_READ, retbuf.as_mut_ptr(), flags, ptex);
    *old_pteh_ret = retbuf[0]; *old_ptel_ret = retbuf[1]; rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_read_4(flags: c_ulong, ptex: c_ulong, ptes: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL9_BUFSIZE];
    let rc = plpar_hcall9(H_READ, retbuf.as_mut_ptr(), flags | H_READ_4, ptex);
    core::ptr::copy_nonoverlapping(retbuf.as_ptr(), ptes, 8); rc
}

/* plpar_pte_read_4_raw can be called in real mode. */
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_pte_read_4_raw(flags: c_ulong, ptex: c_ulong, ptes: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL9_BUFSIZE];
    let rc = plpar_hcall9_raw(H_READ, retbuf.as_mut_ptr(), flags | H_READ_4, ptex);
    core::ptr::copy_nonoverlapping(retbuf.as_ptr(), ptes, 8); rc
}

#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_pte_protect(flags: c_ulong, ptex: c_ulong, avpn: c_ulong) -> c_long { plpar_hcall_norets(H_PROTECT, flags, ptex, avpn) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_resize_hpt_prepare(flags: c_ulong, shift: c_ulong) -> c_long { plpar_hcall_norets(H_RESIZE_HPT_PREPARE, flags, shift) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_resize_hpt_commit(flags: c_ulong, shift: c_ulong) -> c_long { plpar_hcall_norets(H_RESIZE_HPT_COMMIT, flags, shift) }

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_tce_get(liobn: c_ulong, ioba: c_ulong, tce_ret: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall(H_GET_TCE, retbuf.as_mut_ptr(), liobn, ioba); *tce_ret = retbuf[0]; rc
}
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_tce_put(liobn: c_ulong, ioba: c_ulong, tceval: c_ulong) -> c_long { plpar_hcall_norets(H_PUT_TCE, liobn, ioba, tceval) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_tce_put_indirect(liobn: c_ulong, ioba: c_ulong, page: c_ulong, count: c_ulong) -> c_long { plpar_hcall_norets(H_PUT_TCE_INDIRECT, liobn, ioba, page, count) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_tce_stuff(liobn: c_ulong, ioba: c_ulong, tceval: c_ulong, count: c_ulong) -> c_long { plpar_hcall_norets(H_STUFF_TCE, liobn, ioba, tceval, count) }

/* Set various resource mode parameters. */
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_set_mode(mflags: c_ulong, resource: c_ulong, value1: c_ulong, value2: c_ulong) -> c_long { plpar_hcall_norets(H_SET_MODE, mflags, resource, value1, value2) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn enable_reloc_on_exceptions() -> c_long { plpar_set_mode(3, H_SET_MODE_RESOURCE_ADDR_TRANS_MODE, 0, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn disable_reloc_on_exceptions() -> c_long { plpar_set_mode(0, H_SET_MODE_RESOURCE_ADDR_TRANS_MODE, 0, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn enable_big_endian_exceptions() -> c_long { plpar_set_mode(0, H_SET_MODE_RESOURCE_LE, 0, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn enable_little_endian_exceptions() -> c_long { plpar_set_mode(1, H_SET_MODE_RESOURCE_LE, 0, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_set_ciabr(ciabr: c_ulong) -> c_long { plpar_set_mode(0, H_SET_MODE_RESOURCE_SET_CIABR, ciabr, 0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_set_watchpoint0(dawr0: c_ulong, dawrx0: c_ulong) -> c_long { plpar_set_mode(0, H_SET_MODE_RESOURCE_SET_DAWR0, dawr0, dawrx0) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_set_watchpoint1(dawr1: c_ulong, dawrx1: c_ulong) -> c_long { plpar_set_mode(0, H_SET_MODE_RESOURCE_SET_DAWR1, dawr1, dawrx1) }
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe fn plpar_signal_sys_reset(cpu: c_long) -> c_long { plpar_hcall_norets(H_SIGNAL_SYS_RESET, cpu) }

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_get_cpu_characteristics(p: *mut h_cpu_char_result) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE];
    let rc = plpar_hcall(H_GET_CPU_CHARACTERISTICS, retbuf.as_mut_ptr());
    if rc == H_SUCCESS { (*p).character = retbuf[0]; (*p).behaviour = retbuf[1]; } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_create(flags: c_ulong, guest_id: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let mut token = !0 as c_ulong; let mut rc;
    loop { rc = plpar_hcall(H_GUEST_CREATE, retbuf.as_mut_ptr(), flags, token);
        if rc == H_SUCCESS { *guest_id = retbuf[0]; }
        if rc == H_BUSY { token = retbuf[0]; cond_resched(); }
        if H_IS_LONG_BUSY(rc) { token = retbuf[0]; msleep(get_longbusy_msecs(rc)); rc = H_BUSY; }
        if rc != H_BUSY { break; }
    } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_create_vcpu(_flags: c_ulong, guest_id: c_ulong, vcpu_id: c_ulong) -> c_long {
    let mut rc; loop { rc = plpar_hcall_norets(H_GUEST_CREATE_VCPU, 0, guest_id, vcpu_id); if rc == H_BUSY { cond_resched(); } if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; } if rc != H_BUSY { break; } } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_set_state(flags: c_ulong, guest_id: c_ulong, vcpu_id: c_ulong, data_buffer: c_ulong, data_size: c_ulong, failed_index: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let rc;
    loop { rc = plpar_hcall(H_GUEST_SET_STATE, retbuf.as_mut_ptr(), flags, guest_id, vcpu_id, data_buffer, data_size); if rc == H_BUSY { cpu_relax(); continue; } if H_IS_LONG_BUSY(rc) { mdelay(get_longbusy_msecs(rc)); continue; } if rc == H_INVALID_ELEMENT_ID || rc == H_INVALID_ELEMENT_SIZE || rc == H_INVALID_ELEMENT_VALUE { *failed_index = retbuf[0]; } break; } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_get_state(flags: c_ulong, guest_id: c_ulong, vcpu_id: c_ulong, data_buffer: c_ulong, data_size: c_ulong, failed_index: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let rc;
    loop { rc = plpar_hcall(H_GUEST_GET_STATE, retbuf.as_mut_ptr(), flags, guest_id, vcpu_id, data_buffer, data_size); if rc == H_BUSY { cpu_relax(); continue; } if H_IS_LONG_BUSY(rc) { mdelay(get_longbusy_msecs(rc)); continue; } if rc == H_INVALID_ELEMENT_ID || rc == H_INVALID_ELEMENT_SIZE || rc == H_INVALID_ELEMENT_VALUE { *failed_index = retbuf[0]; } break; } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_run_vcpu(flags: c_ulong, guest_id: c_ulong, vcpu_id: c_ulong, trap: *mut c_int, failed_index: *mut c_ulong) -> c_long {
    let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let rc = plpar_hcall(H_GUEST_RUN_VCPU, retbuf.as_mut_ptr(), flags, guest_id, vcpu_id);
    if rc == H_SUCCESS { *trap = retbuf[0] as c_int; } else if rc == H_INVALID_ELEMENT_ID || rc == H_INVALID_ELEMENT_SIZE || rc == H_INVALID_ELEMENT_VALUE { *failed_index = retbuf[0]; } rc
}

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_delete(flags: c_ulong, guest_id: u64) -> c_long { let mut rc; loop { rc = plpar_hcall_norets(H_GUEST_DELETE, flags, guest_id); if rc == H_BUSY { cond_resched(); } if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; } if rc != H_BUSY { break; } } rc }
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_set_capabilities(flags: c_ulong, capabilities: c_ulong) -> c_long { let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let mut rc; loop { rc = plpar_hcall(H_GUEST_SET_CAPABILITIES, retbuf.as_mut_ptr(), flags, capabilities); if rc == H_BUSY { cond_resched(); } if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; } if rc != H_BUSY { break; } } rc }
#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn plpar_guest_get_capabilities(flags: c_ulong, capabilities: *mut c_ulong) -> c_long { let mut retbuf = [0 as c_ulong; PLPAR_HCALL_BUFSIZE]; let mut rc; loop { rc = plpar_hcall(H_GUEST_GET_CAPABILITIES, retbuf.as_mut_ptr(), flags); if rc == H_BUSY { cond_resched(); } if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; } if rc != H_BUSY { break; } } if rc == H_SUCCESS { *capabilities = retbuf[0]; } rc }

#[cfg(CONFIG_PPC_PSERIES)]
pub unsafe fn pseries_rpt_invalidate(pid: u64, target: u64, type_: u64, page_sizes: u64, start: u64, end: u64) -> c_long {
    let mut rc; loop { rc = plpar_hcall_norets(H_RPT_INVALIDATE, pid, target, type_, page_sizes, start, end); if rc == H_BUSY { cpu_relax(); continue; } if rc == H_SUCCESS { return rc; }
        let all = if type_ & H_RPTI_TYPE_NESTED != 0 { H_RPTI_TYPE_NESTED | H_RPTI_TYPE_NESTED_ALL } else { H_RPTI_TYPE_ALL };
        loop { rc = plpar_hcall_norets(H_RPT_INVALIDATE, pid, target, all, page_sizes, 0, !0u64); if rc == H_BUSY { cpu_relax(); continue; } if rc == H_SUCCESS { return rc; } BUG(); }
    }
}

#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_set_ciabr(_: c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_pte_read_4(_: c_ulong, _: c_ulong, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn pseries_rpt_invalidate(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_create_vcpu(_: c_ulong, _: c_ulong, _: c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_get_state(_: c_ulong, _: c_ulong, _: c_ulong, _: c_ulong, _: c_ulong, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_set_state(_: c_ulong, _: c_ulong, _: c_ulong, _: c_ulong, _: c_ulong, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_run_vcpu(_: c_ulong, _: c_ulong, _: c_ulong, _: *mut c_int, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_create(_: c_ulong, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_delete(_: c_ulong, _: u64) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_get_capabilities(_: c_ulong, _: *mut c_ulong) -> c_long { 0 }
#[cfg(not(CONFIG_PPC_PSERIES))]
pub unsafe fn plpar_guest_set_capabilities(_: c_ulong, _: c_ulong) -> c_long { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
