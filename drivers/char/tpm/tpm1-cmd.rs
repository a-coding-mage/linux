// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of tpm1-cmd.c. Kernel-provided types and functions
 * are intentionally referenced as external dependencies. */

const TPM_UNDEFINED: u8 = 0;

static TPM1_ORDINAL_DURATION: [u8; TPM_MAX_ORDINAL] = [
    TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,
    TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_LONG,TPM_LONG,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_LONG,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_MEDIUM,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_LONG,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_MEDIUM,TPM_LONG,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_LONG,TPM_MEDIUM,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_MEDIUM,TPM_MEDIUM,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_LONG,TPM_UNDEFINED,TPM_MEDIUM,TPM_LONG,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_LONG,TPM_LONG,TPM_MEDIUM,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_LONG,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_UNDEFINED,TPM_SHORT,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_MEDIUM,TPM_MEDIUM,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_LONG,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_MEDIUM,TPM_SHORT,TPM_MEDIUM,TPM_MEDIUM,TPM_MEDIUM,TPM_MEDIUM,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_MEDIUM,TPM_UNDEFINED,TPM_MEDIUM,TPM_MEDIUM,TPM_MEDIUM,TPM_UNDEFINED,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_SHORT,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_LONG,TPM_MEDIUM,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_UNDEFINED,TPM_SHORT,TPM_UNDEFINED,TPM_MEDIUM,
];

extern "C" {
    fn tpm1_getcap(chip: *mut tpm_chip, subcap_id: u32, cap: *mut cap_t, desc: *const core::ffi::c_char, min_cap_length: usize) -> isize;
    fn tpm_transmit_cmd(chip: *mut tpm_chip, buf: *mut tpm_buf, min: usize, desc: *const core::ffi::c_char) -> i32;
}

pub unsafe fn tpm1_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> usize {
    let mut duration_idx = TPM_UNDEFINED as usize;
    if ordinal < TPM_MAX_ORDINAL as u32 { duration_idx = TPM1_ORDINAL_DURATION[ordinal as usize] as usize; }
    let duration = if duration_idx != TPM_UNDEFINED as usize { (*chip).duration[duration_idx] } else { 0 };
    if duration <= 0 { 2 * 60 * HZ as usize } else { duration }
}

unsafe fn tpm1_startup(chip: *mut tpm_chip) -> i32 {
    let buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    tpm_buf_init(buf, TPM_BUFSIZE); tpm_buf_reset(buf, TPM_TAG_RQU_COMMAND, TPM_ORD_STARTUP);
    tpm_buf_append_u16(buf, TPM_ST_CLEAR); tpm_transmit_cmd(chip, buf, 0, c"attempting to start the TPM".as_ptr())
}

pub unsafe fn tpm1_get_timeouts(chip: *mut tpm_chip) -> i32 {
    let mut cap = core::mem::zeroed::<cap_t>(); let mut old = [0usize;4]; let mut eff = [0usize;4];
    let mut rc = tpm1_getcap(chip, TPM_CAP_PROP_TIS_TIMEOUT, &mut cap, core::ptr::null(), core::mem::size_of::<_>()) as i32;
    if rc == TPM_ERR_INVALID_POSTINIT { if tpm1_startup(chip) != 0 { return rc; } rc=tpm1_getcap(chip,TPM_CAP_PROP_TIS_TIMEOUT,&mut cap,c"attempting to determine the timeouts".as_ptr(),core::mem::size_of::<_>()) as i32; }
    if rc != 0 { return rc; }
    old[0]=jiffies_to_usecs((*chip).timeout_a); old[1]=jiffies_to_usecs((*chip).timeout_b); old[2]=jiffies_to_usecs((*chip).timeout_c); old[3]=jiffies_to_usecs((*chip).timeout_d);
    eff[0]=be32_to_cpu(cap.timeout.a) as usize; eff[1]=be32_to_cpu(cap.timeout.b) as usize; eff[2]=be32_to_cpu(cap.timeout.c) as usize; eff[3]=be32_to_cpu(cap.timeout.d) as usize;
    for i in 0..4 { if eff[i]==0 { eff[i]=old[i]; (*chip).timeout_adjusted=true; } }
    if !(*chip).timeout_adjusted && eff[0] != 0 && eff[0] < 1000 { for x in &mut eff { *x *= 1000; } (*chip).timeout_adjusted=true; }
    (*chip).timeout_a=usecs_to_jiffies(eff[0]); (*chip).timeout_b=usecs_to_jiffies(eff[1]); (*chip).timeout_c=usecs_to_jiffies(eff[2]); (*chip).timeout_d=usecs_to_jiffies(eff[3]);
    rc=tpm1_getcap(chip,TPM_CAP_PROP_TIS_DURATION,&mut cap,c"attempting to determine the durations".as_ptr(),core::mem::size_of::<_>()) as i32; if rc != 0 { return rc; }
    (*chip).duration[TPM_SHORT as usize]=usecs_to_jiffies(be32_to_cpu(cap.duration.tpm_short) as usize); (*chip).duration[TPM_MEDIUM as usize]=usecs_to_jiffies(be32_to_cpu(cap.duration.tpm_medium) as usize); (*chip).duration[TPM_LONG as usize]=usecs_to_jiffies(be32_to_cpu(cap.duration.tpm_long) as usize); (*chip).duration[TPM_LONG_LONG as usize]=0; (*chip).flags |= TPM_CHIP_FLAG_HAVE_TIMEOUTS; 0
}

pub unsafe fn tpm1_pcr_extend(chip:*mut tpm_chip,pcr_idx:u32,hash:*const u8,log_msg:*const core::ffi::c_char)->i32 { let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL); if b.is_null(){return -ENOMEM;} tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_PCR_EXTEND);tpm_buf_append_u32(b,pcr_idx);tpm_buf_append(b,hash,TPM_DIGEST_SIZE);tpm_transmit_cmd(chip,b,TPM_DIGEST_SIZE,log_msg) }

pub unsafe fn tpm1_getcap(chip:*mut tpm_chip,subcap_id:u32,cap:*mut cap_t,desc:*const core::ffi::c_char,min_cap_length:usize)->isize { let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL);if b.is_null(){return -ENOMEM as isize;}tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_GET_CAP);if subcap_id==TPM_CAP_VERSION_1_1||subcap_id==TPM_CAP_VERSION_1_2{tpm_buf_append_u32(b,subcap_id);tpm_buf_append_u32(b,0);}else{tpm_buf_append_u32(b,if subcap_id==TPM_CAP_FLAG_PERM||subcap_id==TPM_CAP_FLAG_VOL{TPM_CAP_FLAG}else{TPM_CAP_PROP});tpm_buf_append_u32(b,4);tpm_buf_append_u32(b,subcap_id);}let rc=tpm_transmit_cmd(chip,b,min_cap_length,desc) as isize;if rc==0{*cap=*(b.as_ref().unwrap().data.as_ptr().add(TPM_HEADER_SIZE+4) as *const cap_t);}rc }

pub unsafe fn tpm1_get_random(chip:*mut tpm_chip,dest:*mut u8,max:usize)->i32 { let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL);if b.is_null(){return -ENOMEM;}let mut n=core::cmp::min(max,TPM_MAX_RNG_DATA as usize) as u32;let mut total=0usize;let mut retries=5; tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_GET_RANDOM);while retries>0&&total<max{tpm_buf_append_u32(b,n);let mut rc=tpm_transmit_cmd(chip,b,4,c"attempting get random".as_ptr());if rc!=0{return if rc>0{-EIO}else{rc}}let p=b.data.as_ptr().add(TPM_HEADER_SIZE);let recd=be32_to_cpu(*(p as *const u32));if recd>n||tpm_buf_length(b)<TPM_HEADER_SIZE+4+recd as usize{return -EFAULT;}core::ptr::copy_nonoverlapping(p.add(4),dest.add(total),recd as usize);total+=recd as usize;n-=recd;tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_GET_RANDOM);retries-=1;}if total>0{total as i32}else{-EIO} }

pub unsafe fn tpm1_pcr_read(chip:*mut tpm_chip,pcr_idx:u32,res_buf:*mut u8)->i32 {let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL);if b.is_null(){return -ENOMEM;}tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_PCR_READ);tpm_buf_append_u32(b,pcr_idx);let rc=tpm_transmit_cmd(chip,b,TPM_DIGEST_SIZE,c"attempting to read a pcr value".as_ptr());if rc!=0{return rc;}if tpm_buf_length(b)<TPM_DIGEST_SIZE{return -EFAULT;}core::ptr::copy_nonoverlapping(b.data.as_ptr().add(TPM_HEADER_SIZE),res_buf,TPM_DIGEST_SIZE);0}

unsafe fn tpm1_continue_selftest(chip:*mut tpm_chip)->i32 {let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL);if b.is_null(){return -ENOMEM;}tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_CONTINUE_SELFTEST);tpm_transmit_cmd(chip,b,0,c"continue selftest".as_ptr())}
pub unsafe fn tpm1_do_selftest(chip:*mut tpm_chip)->i32 {let mut rc=tpm1_continue_selftest(chip);if rc!=0{return rc;}let mut loops=(jiffies_to_msecs(tpm1_calc_ordinal_duration(chip,TPM_ORD_CONTINUE_SELFTEST)) / 100) as u32;let mut d=[0u8;TPM_DIGEST_SIZE];while loops>0{rc=tpm1_pcr_read(chip,0,d.as_mut_ptr());if rc==-ETIME{tpm_msleep(100);loops-=1;continue;}if rc==TPM_ERR_DISABLED||rc==TPM_ERR_DEACTIVATED{return 0;}if rc!=TPM_WARN_DOING_SELFTEST{return rc;}tpm_msleep(100);loops-=1;}rc}
pub unsafe fn tpm1_auto_startup(chip:*mut tpm_chip)->i32 {let mut rc=tpm1_get_timeouts(chip);if rc==0{rc=tpm1_do_selftest(chip);}if rc>0{-ENODEV}else{rc}}
pub unsafe fn tpm1_pm_suspend(chip:*mut tpm_chip,tpm_suspend_pcr:u32)->i32 {let dummy=[0u8;TPM_DIGEST_SIZE];if tpm_suspend_pcr!=0{let _=tpm1_pcr_extend(chip,tpm_suspend_pcr,dummy.as_ptr(),c"extending dummy pcr before suspend".as_ptr());}let b=kzalloc(TPM_BUFSIZE,GFP_KERNEL);if b.is_null(){return -ENOMEM;}tpm_buf_init(b,TPM_BUFSIZE);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_SAVESTATE);let mut rc=0;for _ in 0..TPM_RETRY{rc=tpm_transmit_cmd(chip,b,0,core::ptr::null());if rc!=TPM_WARN_RETRY{break;}tpm_msleep(TPM_TIMEOUT_RETRY);tpm_buf_reset(b,TPM_TAG_RQU_COMMAND,TPM_ORD_SAVESTATE);}rc}
pub unsafe fn tpm1_get_pcr_allocation(chip:*mut tpm_chip)->i32 {(*chip).allocated_banks[0].alg_id=TPM_ALG_SHA1;(*chip).allocated_banks[0].digest_size=hash_digest_size[HASH_ALGO_SHA1];(*chip).allocated_banks[0].crypto_id=HASH_ALGO_SHA1;(*chip).nr_allocated_banks=1;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
