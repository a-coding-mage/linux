// SPDX-License-Identifier: GPL-2.0-only
/* n2-drv.c: Niagara-2 RNG driver.
 * Copyright (C) 2008, 2011 David S. Miller <davem@davemloft.net>
 */

// Linux and n2rng declarations are supplied by the surrounding kernel
// translation unit.  Their names and ABI are intentionally preserved here.

const DRV_MODULE_NAME: &str = "n2rng";
const DRV_MODULE_VERSION: &str = "0.3";
const DRV_MODULE_RELDATE: &str = "Jan 7, 2017";
static mut VERSION: &[u8] = b"n2rng v0.3 (Jan 7, 2017)\n";

unsafe fn n2rng_hv_err_trans(hv_err: c_ulong) -> c_int {
    match hv_err { HV_EOK => 0, HV_EWOULDBLOCK => -EAGAIN, HV_ENOACCESS => -EPERM,
        HV_EIO => -EIO, HV_EBUSY => -EBUSY, HV_EBADALIGN | HV_ENORADDR => -EFAULT,
        _ => -EINVAL }
}

unsafe fn n2rng_generic_read_control_v2(ra: c_ulong, unit: c_ulong) -> c_ulong {
    let (mut hv_err, mut state, mut ticks, mut watchdog_delta, mut watchdog_status) = (0,0,0,0,0);
    let (mut block, mut busy) = (0, 0);
    loop {
        hv_err = sun4v_rng_ctl_read_v2(ra, unit, &mut state, &mut ticks, &mut watchdog_delta, &mut watchdog_status);
        if hv_err == HV_EOK { break; }
        if hv_err == HV_EBUSY { busy += 1; if busy >= N2RNG_BUSY_LIMIT { break; } udelay(1); }
        else if hv_err == HV_EWOULDBLOCK { block += 1; if block >= N2RNG_BLOCK_LIMIT { break; } __delay(ticks); }
        else { break; }
    } hv_err
}

unsafe fn n2rng_control_settle_v2(np: *mut n2rng, unit: c_int) -> c_ulong {
    n2rng_generic_read_control_v2(__pa((*np).scratch_control.as_mut_ptr()), unit as c_ulong)
}

unsafe fn n2rng_write_ctl_one(np: *mut n2rng, unit: c_int, state: c_ulong, control_ra: c_ulong, watchdog_timeout: c_ulong, ticks: *mut c_ulong) -> c_ulong {
    let mut hv_err;
    if (*np).hvapi_major == 1 { hv_err = sun4v_rng_ctl_write_v1(control_ra, state, watchdog_timeout, ticks); }
    else { hv_err = sun4v_rng_ctl_write_v2(control_ra, state, watchdog_timeout, unit as c_ulong); if hv_err == HV_EOK { hv_err = n2rng_control_settle_v2(np, unit); } *ticks = N2RNG_ACCUM_CYCLES_DEFAULT; }
    hv_err
}

unsafe fn n2rng_generic_read_data(data_ra: c_ulong) -> c_int {
    let (mut ticks, mut hv_err, mut block, mut hcheck) = (0,0,0,0);
    loop { hv_err = sun4v_rng_data_read(data_ra, &mut ticks); if hv_err == HV_EOK { return 0; }
        if hv_err == HV_EWOULDBLOCK { block += 1; if block >= N2RNG_BLOCK_LIMIT { return -EWOULDBLOCK; } __delay(ticks); }
        else if hv_err == HV_ENOACCESS { return -EPERM; }
        else if hv_err == HV_EIO { hcheck += 1; if hcheck >= N2RNG_HCHECK_LIMIT { return -EIO; } udelay(10000); }
        else { return -ENODEV; }
    }
}

unsafe fn n2rng_read_diag_data_one(np: *mut n2rng, unit: c_ulong, data_ra: c_ulong, data_len: c_ulong, ticks: *mut c_ulong) -> c_ulong {
    let hv_err = if (*np).hvapi_major == 1 { sun4v_rng_data_read_diag_v1(data_ra, data_len, ticks) } else { let e = sun4v_rng_data_read_diag_v2(data_ra, data_len, unit, ticks); if *ticks == 0 { *ticks = N2RNG_ACCUM_CYCLES_DEFAULT; } e }; hv_err
}
unsafe fn n2rng_generic_read_diag_data(np: *mut n2rng, unit: c_ulong, data_ra: c_ulong, data_len: c_ulong) -> c_int {
    let (mut ticks, mut hv_err, mut block) = (0,0,0); loop { hv_err = n2rng_read_diag_data_one(np,unit,data_ra,data_len,&mut ticks); if hv_err==HV_EOK{return 0;} if hv_err==HV_EWOULDBLOCK {block+=1;if block>=N2RNG_BLOCK_LIMIT{return -EWOULDBLOCK;}__delay(ticks)} else if hv_err==HV_ENOACCESS{return -EPERM} else if hv_err==HV_EIO{return -EIO} else{return -ENODEV} }
}
unsafe fn n2rng_generic_write_control(np:*mut n2rng, control_ra:c_ulong, unit:c_ulong, state:c_ulong)->c_int { let (mut ticks,mut hv_err,mut block,mut busy)=(0,0,0,0); loop {hv_err=n2rng_write_ctl_one(np,unit,state,control_ra,(*np).wd_timeo,&mut ticks);if hv_err==HV_EOK{return 0} if hv_err==HV_EWOULDBLOCK{block+=1;if block>=N2RNG_BLOCK_LIMIT{return -EWOULDBLOCK}__delay(ticks)}else if hv_err==HV_EBUSY{busy+=1;if busy>=N2RNG_BUSY_LIMIT{return -EBUSY}udelay(1)}else{return -ENODEV}}}

unsafe fn n2rng_try_read_ctl(np:*mut n2rng)->c_int { let mut x=0; let mut e=if (*np).hvapi_major==1{sun4v_rng_get_diag_ctl()}else{let z=sun4v_rng_ctl_read_v2(0,!0,&mut x,&mut x,&mut x,&mut x);if z==HV_EWOULDBLOCK||z==HV_ENOACCESS{z}else{HV_EOK}};n2rng_hv_err_trans(e) }
unsafe fn n2rng_control_default(np:*mut n2rng, ctl:c_int)->u64 { let mut v=if (*np).data.chip_version==1{(2<<RNG_v1_CTL_ASEL_SHIFT)|((N2RNG_ACCUM_CYCLES_DEFAULT as u64)<<RNG_v1_CTL_WAIT_SHIFT)|RNG_CTL_LFSR}else{(2<<RNG_v2_CTL_ASEL_SHIFT)|((N2RNG_ACCUM_CYCLES_DEFAULT as u64)<<RNG_v2_CTL_WAIT_SHIFT)|RNG_CTL_LFSR}; if ctl<3 {v|=((ctl+1) as u64)<<if (*np).data.chip_version==1{RNG_v1_CTL_VCO_SHIFT}else{RNG_v2_CTL_VCO_SHIFT};v|=RNG_CTL_ES1<<ctl}else if ctl==3{v|=RNG_CTL_ES1|RNG_CTL_ES2|RNG_CTL_ES3} v }
unsafe fn n2rng_control_swstate_init(np:*mut n2rng){(*np).flags|=N2RNG_FLAG_CONTROL;(*np).health_check_sec=N2RNG_HEALTH_CHECK_SEC_DEFAULT;(*np).accum_cycles=N2RNG_ACCUM_CYCLES_DEFAULT;(*np).wd_timeo=N2RNG_WD_TIMEO_DEFAULT;for i in 0..(*np).num_units{for j in 0..4{(*np).units.add(i as usize).as_mut().unwrap().control[j]=n2rng_control_default(np,j as c_int)}}(*np).hv_state=HV_RNG_STATE_UNCONFIGURED}
unsafe fn n2rng_grab_diag_control(np:*mut n2rng)->c_int{let(mut err,mut busy)=(-ENODEV,0);for _ in 0..100{err=n2rng_try_read_ctl(np);if err!=-EAGAIN{break}busy+=1;if busy>100{return -ENODEV}udelay(1)}err}
unsafe fn n2rng_init_control(np:*mut n2rng)->c_int{let e=n2rng_grab_diag_control(np);if e==-EPERM{return 0}if e!=0{return e}n2rng_control_swstate_init(np);0}

unsafe fn n2rng_data_read(rng:*mut hwrng,data:*mut u32)->c_int{let np=(*rng).priv as *mut n2rng;let ra=__pa(&(*np).test_data);if (*np).flags&N2RNG_FLAG_READY==0{0}else if (*np).flags&N2RNG_FLAG_BUFFER_VALID!=0{(*np).flags&=!N2RNG_FLAG_BUFFER_VALID;*data=(*np).buffer;4}else if n2rng_generic_read_data(ra)==0{(*np).flags|=N2RNG_FLAG_BUFFER_VALID;(*np).buffer=(*np).test_data>>32;*data=(*np).test_data as u32;4}else{(*np).flags&=!N2RNG_FLAG_READY;if (*np).flags&N2RNG_FLAG_SHUTDOWN==0{schedule_delayed_work(&mut (*np).work,0)}0}}
unsafe fn n2rng_guest_check(np:*mut n2rng)->c_int{n2rng_generic_read_data(__pa(&(*np).test_data))}
unsafe fn advance_polynomial(poly:u64,mut val:u64,count:c_int)->u64{for _ in 0..count{let high=(val as i64)<0;val<<=1;if high{val^=poly}}val}
unsafe fn n2rng_test_buffer_find(np:*mut n2rng,val:u64)->c_int{let mut count=0;for i in 1..SELFTEST_BUFFER_WORDS{if (*np).test_buffer[i]==val{count+=1}}count}
unsafe fn n2rng_dump_test_buffer(np:*mut n2rng){for i in 0..SELFTEST_BUFFER_WORDS{dev_err(&(*np).op.dev,"Test buffer slot %d [0x%016llx]",i,(*np).test_buffer[i])}}

// The remaining control/self-test and platform-driver glue retain the C ABI;
// dependent kernel types, constants, and callbacks are supplied externally.
unsafe fn n2rng_entropy_diag_read(np:*mut n2rng,unit:c_ulong,pre:*mut u64,pre_state:u64,buffer:*mut u64,len:c_ulong,post:*mut u64,post_state:u64)->c_int{let e=n2rng_generic_write_control(np,__pa(pre),unit,pre_state);if e!=0{return e}let e=n2rng_generic_read_diag_data(np,unit,__pa(buffer),len);let _=n2rng_generic_write_control(np,__pa(post),unit,post_state);e}
unsafe fn n2rng_check_selftest_buffer(np:*mut n2rng,unit:c_ulong)->c_int{let mut val=match (*np).data.id{N2_n2_rng|N2_vf_rng|N2_kt_rng|N2_m4_rng=>RNG_v1_SELFTEST_VAL,_=>RNG_v2_SELFTEST_VAL};let mut matches=0;let mut limit=0;while limit<SELFTEST_LOOPS_MAX{matches+=n2rng_test_buffer_find(np,val);if matches>=SELFTEST_MATCH_GOAL{break}val=advance_polynomial(SELFTEST_POLY,val,1);limit+=1}if limit>=SELFTEST_LOOPS_MAX{n2rng_dump_test_buffer(np);-ENODEV}else{0}}

unsafe fn n2rng_control_selftest(np:*mut n2rng,unit:c_ulong)->c_int{let(mut base,mut base3)=(0,0);match (*np).data.id{N2_n2_rng|N2_vf_rng|N2_kt_rng=>{base=RNG_v1_CTL_ASEL_NOOUT<<RNG_v1_CTL_ASEL_SHIFT;base3=base|RNG_CTL_LFSR|((RNG_v1_SELFTEST_TICKS-2)<<RNG_v1_CTL_WAIT_SHIFT)},N2_m4_rng=>{base=RNG_v2_CTL_ASEL_NOOUT<<RNG_v2_CTL_ASEL_SHIFT;base3=base|RNG_CTL_LFSR|((RNG_v1_SELFTEST_TICKS-2)<<RNG_v2_CTL_WAIT_SHIFT)},_=>{base=RNG_v2_CTL_ASEL_NOOUT<<RNG_v2_CTL_ASEL_SHIFT;base3=base|RNG_CTL_LFSR|(RNG_v2_SELFTEST_TICKS<<RNG_v2_CTL_WAIT_SHIFT)}}for i in 0..3{(*np).test_control[i]=base}(*np).test_control[3]=base3;let e=n2rng_entropy_diag_read(np,unit,(*np).test_control.as_mut_ptr(),HV_RNG_STATE_HEALTHCHECK,(*np).test_buffer.as_mut_ptr(),core::mem::size_of_val(&(*np).test_buffer) as c_ulong,&mut (*np).units.add(unit as usize).as_mut().unwrap().control[0],(*np).hv_state);if e!=0{e}else{n2rng_check_selftest_buffer(np,unit)}}
unsafe fn n2rng_control_check(np:*mut n2rng)->c_int{for i in 0..(*np).num_units{let e=n2rng_control_selftest(np,i as c_ulong);if e!=0{return e}}0}
unsafe fn n2rng_control_configure_units(np:*mut n2rng)->c_int{let(mut unit,mut err)=(0,0);while unit<(*np).num_units{let up=(*np).units.add(unit as usize);let(mut base,shift)=(0,0);if (*np).data.chip_version==1{base=((*np).accum_cycles<<RNG_v1_CTL_WAIT_SHIFT)|(RNG_v1_CTL_ASEL_NOOUT<<RNG_v1_CTL_ASEL_SHIFT)|RNG_CTL_LFSR;shift=RNG_v1_CTL_VCO_SHIFT}else{base=((*np).accum_cycles<<RNG_v2_CTL_WAIT_SHIFT)|(RNG_v2_CTL_ASEL_NOOUT<<RNG_v2_CTL_ASEL_SHIFT)|RNG_CTL_LFSR;shift=RNG_v2_CTL_VCO_SHIFT}for esrc in 0..3{(*up).control[esrc]=base|((esrc as u64)<<shift)|(RNG_CTL_ES1<<esrc)}(*up).control[3]=base|RNG_CTL_ES1|RNG_CTL_ES2|RNG_CTL_ES3;err=n2rng_generic_write_control(np,__pa((*up).control.as_mut_ptr()),unit as c_ulong,HV_RNG_STATE_CONFIGURED);if err!=0{break}unit+=1}err}
unsafe fn n2rng_work(work:*mut work_struct){let np=container_of!(work,n2rng,work.work);let mut err=if (*np).flags&N2RNG_FLAG_CONTROL==0{n2rng_guest_check(np)}else{preempt_disable();let e=n2rng_control_check(np);preempt_enable();if e==0{n2rng_control_configure_units(np)}else{e}};if err==0{(*np).flags|=N2RNG_FLAG_READY}else if (*np).flags&N2RNG_FLAG_SHUTDOWN==0{schedule_delayed_work(&mut (*np).work,HZ*2)} }
unsafe fn n2rng_driver_version(){static mut PRINTED:c_int=0;if PRINTED==0{PRINTED+=1;pr_info!("%s",VERSION.as_ptr())}}

static mut n2_template:n2rng_template=n2rng_template{id:N2_n2_rng,multi_capable:0,chip_version:1};
static mut vf_template:n2rng_template=n2rng_template{id:N2_vf_rng,multi_capable:1,chip_version:1};
static mut kt_template:n2rng_template=n2rng_template{id:N2_kt_rng,multi_capable:1,chip_version:1};
static mut m4_template:n2rng_template=n2rng_template{id:N2_m4_rng,multi_capable:1,chip_version:2};
static mut m7_template:n2rng_template=n2rng_template{id:N2_m7_rng,multi_capable:1,chip_version:2};

// OF match table, module metadata, probe/remove registration, and kernel
// allocation helpers are represented by their external Rust bindings.
extern "C" { fn n2rng_probe(op:*mut platform_device)->c_int; fn n2rng_remove(op:*mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
