// SPDX-License-Identifier: GPL-2.0-only
/* mac80211 debugfs for wireless PHYs */

// Dependencies supplied by the surrounding kernel translation.
const DEBUGFS_FORMAT_BUFFER_SIZE: usize = 100;

pub unsafe fn mac80211_format_buffer(
    userbuf: *mut core::ffi::c_char, count: usize, ppos: *mut i64,
    fmt: *const core::ffi::c_char, mut args: ...,
) -> isize {
    let mut buf = [0 as core::ffi::c_char; DEBUGFS_FORMAT_BUFFER_SIZE];
    let res = vscnprintf(buf.as_mut_ptr(), buf.len(), fmt, args);
    simple_read_from_buffer(userbuf, count, ppos, buf.as_ptr(), res)
}

macro_rules! readonly_file {
    ($name:ident, $fmt:expr, $value:expr) => {
        unsafe extern "C" fn $name##_read(file: *mut file, userbuf: *mut core::ffi::c_char,
                                           count: usize, ppos: *mut i64) -> isize {
            let local = (*file).private_data as *mut ieee80211_local;
            mac80211_format_buffer(userbuf, count, ppos, $fmt, $value)
        }
        static mut $name##_OPS: debugfs_short_fops = debugfs_short_fops {
            read: Some($name##_read), write: None, llseek: Some(generic_file_llseek),
        };
    };
}

readonly_file!(hw_conf, c"%x\n", (*local).hw.conf.flags);
readonly_file!(user_power, c"%d\n", (*local).user_power_level);
readonly_file!(power, c"%d\n", (*local).hw.conf.power_level);
readonly_file!(total_ps_buffered, c"%d\n", (*local).total_ps_buffered);
readonly_file!(wep_iv, c"%#08x\n", (*local).wep_iv & 0xffffff);
readonly_file!(rate_ctrl_alg, c"%s\n", if !(*local).rate_ctrl.is_null() { (*(*local).rate_ctrl).ops.name } else { c"hw/driver".as_ptr() });

unsafe extern "C" fn aqm_read(file: *mut file, user_buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let local = (*file).private_data as *mut ieee80211_local;
    let fq = &mut (*local).fq;
    let mut buf = [0i8; 200];
    spin_lock_bh(&mut fq.lock);
    let len = scnprintf(buf.as_mut_ptr(), buf.len(), c"access name value\nR fq_flows_cnt %u\nR fq_backlog %u\nR fq_overlimit %u\nR fq_overmemory %u\nR fq_collisions %u\nR fq_memory_usage %u\nRW fq_memory_limit %u\nRW fq_limit %u\nRW fq_quantum %u\n", fq.flows_cnt, fq.backlog, fq.overmemory, fq.overlimit, fq.collisions, fq.memory_usage, fq.memory_limit, fq.limit, fq.quantum);
    spin_unlock_bh(&mut fq.lock);
    simple_read_from_buffer(user_buf, count, ppos, buf.as_ptr(), len)
}
unsafe extern "C" fn aqm_write(file: *mut file, user_buf: *const i8, count: usize, _ppos: *mut i64) -> isize {
    let local = (*file).private_data as *mut ieee80211_local; let mut buf=[0i8;100];
    if count >= buf.len() { return -EINVAL; } if copy_from_user(buf.as_mut_ptr(),user_buf,count)!=0{return -EFAULT;}
    if count != 0 && buf[count-1] == b'\n' as i8 {buf[count-1]=0} else {buf[count]=0}
    let mut v=0u32;
    if sscanf(buf.as_ptr(),c"fq_limit %u",&mut v)==1 {(*local).fq.limit=v;return count as isize}
    if sscanf(buf.as_ptr(),c"fq_memory_limit %u",&mut v)==1 {(*local).fq.memory_limit=v;return count as isize}
    if sscanf(buf.as_ptr(),c"fq_quantum %u",&mut v)==1 {(*local).fq.quantum=v;return count as isize} -EINVAL
}
static mut aqm_ops: debugfs_short_fops=debugfs_short_fops{read:Some(aqm_read),write:Some(aqm_write),llseek:Some(default_llseek)};

unsafe extern "C" fn airtime_flags_read(file:*mut file,user_buf:*mut i8,count:usize,ppos:*mut i64)->isize {let l=(*file).private_data as *mut ieee80211_local;let mut b=[0i8;128];let mut p=b.as_mut_ptr();let e=unsafe{p.add(127)};if (*l).airtime_flags&AIRTIME_USE_TX!=0{p=p.add(scnprintf(p,e.offset_from(p) as usize,c"AIRTIME_TX\t(%lx)\n",AIRTIME_USE_TX) as usize)}if (*l).airtime_flags&AIRTIME_USE_RX!=0{p=p.add(scnprintf(p,e.offset_from(p) as usize,c"AIRTIME_RX\t(%lx)\n",AIRTIME_USE_RX) as usize)}simple_read_from_buffer(user_buf,count,ppos,b.as_ptr(),strlen(b.as_ptr()))}
unsafe extern "C" fn airtime_flags_write(file:*mut file,user_buf:*const i8,count:usize,_:*mut i64)->isize{let l=(*file).private_data as *mut ieee80211_local;let r=kstrtou16_from_user(user_buf,count,0,&mut (*l).airtime_flags);if r!=0{r as isize}else{count as isize}}
static mut airtime_flags_ops:debugfs_short_fops=debugfs_short_fops{read:Some(airtime_flags_read),write:Some(airtime_flags_write),llseek:Some(default_llseek)};

unsafe extern "C" fn aql_pending_read(file:*mut file,u:*mut i8,c:usize,p:*mut i64)->isize{let l=(*file).private_data as *mut ieee80211_local;let mut b=[0i8;400];let n=scnprintf(b.as_mut_ptr(),b.len(),c"AC     AQL pending\nVO     %u us\nVI     %u us\nBE     %u us\nBK     %u us\nMC     %u us\ntotal  %u us\n",atomic_read(&(*l).aql_ac_pending_airtime[IEEE80211_AC_VO]),atomic_read(&(*l).aql_ac_pending_airtime[IEEE80211_AC_VI]),atomic_read(&(*l).aql_ac_pending_airtime[IEEE80211_AC_BE]),atomic_read(&(*l).aql_ac_pending_airtime[IEEE80211_AC_BK]),atomic_read(&(*l).aql_mc_pending_airtime),atomic_read(&(*l).aql_total_pending_airtime));simple_read_from_buffer(u,c,p,b.as_ptr(),n)}
static mut aql_pending_ops:debugfs_short_fops=debugfs_short_fops{read:Some(aql_pending_read),write:None,llseek:Some(default_llseek)};

// The remaining handlers retain the C control flow and kernel calls.
unsafe extern "C" fn force_tx_status_read(file:*mut file,u:*mut i8,c:usize,p:*mut i64)->isize{let l=(*file).private_data as *mut ieee80211_local;let mut b=[0i8;3];let n=scnprintf(b.as_mut_ptr(),b.len(),c"%d\n",(*l).force_tx_status as i32);simple_read_from_buffer(u,c,p,b.as_ptr(),n)}
unsafe extern "C" fn force_tx_status_write(file:*mut file,u:*const i8,c:usize,_:*mut i64)->isize{let l=(*file).private_data as *mut ieee80211_local;let mut v=false;let r=kstrtobool_from_user(u,c,&mut v);if r!=0{return r as isize}(*l).force_tx_status=v;c as isize}
static mut force_tx_status_ops:debugfs_short_fops=debugfs_short_fops{read:Some(force_tx_status_read),write:Some(force_tx_status_write),llseek:Some(default_llseek)};

static hw_flag_names:[*const i8;NUM_IEEE80211_HW_FLAGS]=[/* FLAG names supplied by kernel constants */];

pub unsafe fn debugfs_hw_add(local:*mut ieee80211_local){let phyd=(*(*local).hw.wiphy).debugfsdir;if phyd.is_null(){return}(*local).debugfs.keys=debugfs_create_dir(c"keys".as_ptr(),phyd);debugfs_create_u32(c"aql_threshold".as_ptr(),0o600,phyd,&mut (*local).aql_threshold);let statsd=debugfs_create_dir(c"statistics".as_ptr(),phyd);debugfs_create_file(c"total_ps_buffered".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"wep_iv".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"rate_ctrl_alg".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"queues".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"misc".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"user_power".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"power".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"hw_conf".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"aql_pending".as_ptr(),0o400,phyd,local,core::ptr::null());debugfs_create_file(c"airtime_flags".as_ptr(),0o600,phyd,local,core::ptr::null());debugfs_create_file(c"aql_txq_limit".as_ptr(),0o400,phyd,local,core::ptr::null());let _=statsd;}

// External kernel types and functions referenced above are intentionally unresolved here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
