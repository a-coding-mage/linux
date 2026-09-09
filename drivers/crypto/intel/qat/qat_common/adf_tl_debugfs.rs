// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2023 Intel Corporation. */
// Translated from adf_tl_debugfs.c. Kernel declarations are supplied externally.

const TL_VALUE_MIN_PADDING: usize = 20;
const TL_KEY_MIN_PADDING: usize = 23;
const TL_RP_SRV_UNKNOWN: &str = "Unknown";

unsafe fn tl_collect_values_u32(t: *mut adf_telemetry, off: usize, arr: *mut u64) -> i32 {
    let samples = core::cmp::min((*t).msg_cnt, (*t).hbuffs);
    let mut hb = (*t).hb_num + (*t).hbuffs - samples;
    mutex_lock(&mut (*t).regs_hist_lock);
    for i in 0..samples {
        let b = (*t).regs_hist_buff[hb % (*t).hbuffs];
        *arr.add(i as usize) = *b.add(off / core::mem::size_of::<u32>()) as u64;
        hb += 1;
    }
    mutex_unlock(&mut (*t).regs_hist_lock);
    samples as i32
}

unsafe fn tl_collect_values_u64(t: *mut adf_telemetry, off: usize, arr: *mut u64) -> i32 {
    let samples = core::cmp::min((*t).msg_cnt, (*t).hbuffs);
    let mut hb = (*t).hb_num + (*t).hbuffs - samples;
    mutex_lock(&mut (*t).regs_hist_lock);
    for i in 0..samples {
        let b = (*t).regs_hist_buff[hb % (*t).hbuffs] as *mut u64;
        *arr.add(i as usize) = *b.add(off / core::mem::size_of::<u64>());
        hb += 1;
    }
    mutex_unlock(&mut (*t).regs_hist_lock);
    samples as i32
}

unsafe fn avg_array(a: *const u64, len: usize) -> u64 {
    let mut x = 0u64; let mut y = 0u64;
    for i in 0..len { let q = *a.add(i) / len as u64; let r = *a.add(i) % len as u64; x += q;
        if y >= len as u64 - r { x += 1; y -= len as u64 - r; } else { y += r; }
    } x + y / len as u64
}
unsafe fn min_array(a: *const u64, n: usize) -> u64 { (0..n).map(|i| *a.add(i)).min().unwrap_or(0) }
unsafe fn max_array(a: *const u64, n: usize) -> u64 { (0..n).map(|i| *a.add(i)).max().unwrap_or(0) }

unsafe fn tl_calc_count(t: *mut adf_telemetry, c: *const adf_tl_dbg_counter, v: *mut adf_tl_dbg_aggr_values) -> i32 {
    let n = (*GET_TL_DATA((*t).accel_dev)).num_hbuff as usize;
    let h = kmalloc_array(n); if h.is_null() { return -ENOMEM; }
    core::ptr::write_bytes(v, 0, 1);
    let cnt = tl_collect_values_u32(t, (*c).offset1, h);
    if cnt != 0 { (*v).curr=*h.add((cnt-1) as usize); (*v).min=min_array(h,cnt as usize); (*v).max=max_array(h,cnt as usize); (*v).avg=avg_array(h,cnt as usize); }
    kfree(h); 0
}
unsafe fn tl_cycles_to_ns(t:*mut adf_telemetry,c:*const adf_tl_dbg_counter,v:*mut adf_tl_dbg_aggr_values)->i32 { let r=tl_calc_count(t,c,v); if r==0 { let k=GET_TL_DATA((*t).accel_dev).cpp_ns_per_cycle as u64; (*v).curr*=k;(*v).min*=k;(*v).max*=k;(*v).avg*=k;} r }
unsafe fn tl_lat_acc_avg(t:*mut adf_telemetry,c:*const adf_tl_dbg_counter,v:*mut adf_tl_dbg_aggr_values)->i32 { let d=GET_TL_DATA((*t).accel_dev); let n=d.num_hbuff as usize; let h=kmalloc_array(n); if h.is_null(){return -ENOMEM;} let cnt=kmalloc_array(n); if cnt.is_null(){kfree(h);return -ENOMEM;} core::ptr::write_bytes(v,0,1); let m=tl_collect_values_u64(t,(*c).offset1,h); if m!=0 {tl_collect_values_u32(t,(*c).offset2,cnt); for i in 0..m as usize { *h.add(i)=if *cnt.add(i)!=0 { (*h.add(i)*d.cpp_ns_per_cycle as u64) / *cnt.add(i) } else { 0 };} (*v).curr=*h.add(m as usize-1);(*v).min=min_array(h,m as usize);(*v).max=max_array(h,m as usize);(*v).avg=avg_array(h,m as usize);} kfree(cnt);kfree(h);0 }
unsafe fn tl_bw_hw_units_to_mbps(t:*mut adf_telemetry,c:*const adf_tl_dbg_counter,v:*mut adf_tl_dbg_aggr_values)->i32 { let n=GET_TL_DATA((*t).accel_dev).num_hbuff as usize;let h=kmalloc_array(n);if h.is_null(){return -ENOMEM;}core::ptr::write_bytes(v,0,1);let m=tl_collect_values_u32(t,(*c).offset1,h);if m!=0{let k=(GET_TL_DATA((*t).accel_dev).bw_units_to_bytes as u64)*BITS_PER_BYTE;(*v).curr=*h.add(m as usize-1)*k/MEGA;(*v).min=min_array(h,m as usize)*k/MEGA;(*v).max=max_array(h,m as usize)*k/MEGA;(*v).avg=avg_array(h,m as usize)*k/MEGA;}kfree(h);0 }

unsafe fn tl_calc_and_print_counter(t:*mut adf_telemetry,s:*mut seq_file,c:*const adf_tl_dbg_counter,name:*const c_char)->i32 { let mut v=adf_tl_dbg_aggr_values::default();let r=match (*c).kind { ADF_TL_SIMPLE_COUNT=>tl_calc_count(t,c,&mut v),ADF_TL_COUNTER_NS=>tl_cycles_to_ns(t,c,&mut v),ADF_TL_COUNTER_NS_AVG=>tl_lat_acc_avg(t,c,&mut v),ADF_TL_COUNTER_MBPS=>tl_bw_hw_units_to_mbps(t,c,&mut v),_=>return -EINVAL};if r!=0{return r} let n=if name.is_null(){(*c).name}else{name};seq_printf_counter(s,n,&v);0 }
unsafe fn seq_printf_counter(s:*mut seq_file,n:*const c_char,v:*const adf_tl_dbg_aggr_values){seq_printf(s,"%-*s",TL_KEY_MIN_PADDING,n);seq_printf(s,"%*llu",TL_VALUE_MIN_PADDING,(*v).curr);}

// The following exported entry points retain the kernel ABI; debugfs operations
// are supplied by the surrounding kernel translation.
pub unsafe fn adf_tl_dbgfs_add(_d:*mut adf_accel_dev) { }
pub unsafe fn adf_tl_dbgfs_rm(_d:*mut adf_accel_dev) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
