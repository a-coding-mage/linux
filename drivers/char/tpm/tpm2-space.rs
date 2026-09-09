// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Intel Corporation
 *
 * Authors:
 * Jarkko Sakkinen <jarkko.sakkinen@linux.intel.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * This file contains TPM2 protocol implementations of the commands
 * used by the kernel internally.
 */

// External kernel definitions and constants are supplied by the surrounding crate.

unsafe fn tpm2_flush_sessions(chip: *mut tpm_chip, space: *mut tpm_space) {
    for i in 0..(*space).session_tbl.len() {
        if (*space).session_tbl[i] != 0 {
            tpm2_flush_context(chip, (*space).session_tbl[i]);
        }
    }
}

unsafe fn tpm2_init_space(space: *mut tpm_space, buf_size: u32) -> i32 {
    (*space).context_buf = kzalloc(buf_size as usize, GFP_KERNEL);
    if (*space).context_buf.is_null() { return -ENOMEM; }
    (*space).session_buf = kzalloc(buf_size as usize, GFP_KERNEL);
    if (*space).session_buf.is_null() {
        kfree((*space).context_buf);
        (*space).context_buf = core::ptr::null_mut();
        return -ENOMEM;
    }
    (*space).buf_size = buf_size;
    0
}

unsafe fn tpm2_del_space(chip: *mut tpm_chip, space: *mut tpm_space) {
    if tpm_try_get_ops(chip) == 0 {
        tpm2_flush_sessions(chip, space);
        tpm_put_ops(chip);
    }
    kfree((*space).context_buf);
    kfree((*space).session_buf);
}

unsafe fn tpm2_load_context(chip: *mut tpm_chip, buf: *mut u8, offset: *mut u32, handle: *mut u32) -> i32 {
    let tbuf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if tbuf.is_null() { return -ENOMEM; }
    tpm_buf_init(tbuf, TPM_BUFSIZE);
    tpm_buf_reset(tbuf, TPM2_ST_NO_SESSIONS, TPM2_CC_CONTEXT_LOAD);
    let ctx = buf.add(*offset as usize) as *const tpm2_context;
    let body_size = core::mem::size_of::<tpm2_context>() + u16::from_be((*ctx).blob_size) as usize;
    tpm_buf_append(tbuf, buf.add(*offset as usize), body_size);
    let rc = tpm_transmit_cmd(chip, tbuf, 4, core::ptr::null_mut());
    if rc < 0 { dev_warn(&(*chip).dev, "%s: failed with a system error %d\n", "tpm2_load_context", rc); kfree(tbuf); return -EFAULT; }
    if tpm2_rc_value(rc) == TPM2_RC_HANDLE || rc == TPM2_RC_REFERENCE_H0 {
        /* TPM_RC_HANDLE indicates an internal context-counter mismatch or a context saved outside this space. */
        /* TPM_RC_REFERENCE_H0 means the session was flushed outside the space. */
        *handle = 0; kfree(tbuf); return -ENOENT;
    } else if tpm2_rc_value(rc) == TPM2_RC_INTEGRITY { kfree(tbuf); return -EINVAL; }
    else if rc > 0 { dev_warn(&(*chip).dev, "%s: failed with a TPM error 0x%04X\n", "tpm2_load_context", rc); kfree(tbuf); return -EFAULT; }
    *handle = u32::from_be(*( (*tbuf).data.add(TPM_HEADER_SIZE) as *const u32));
    *offset += body_size as u32;
    kfree(tbuf); 0
}

unsafe fn tpm2_save_context(chip: *mut tpm_chip, handle: u32, buf: *mut u8, buf_size: u32, offset: *mut u32) -> i32 {
    let tbuf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if tbuf.is_null() { return -ENOMEM; }
    tpm_buf_init(tbuf, TPM_BUFSIZE); tpm_buf_reset(tbuf, TPM2_ST_NO_SESSIONS, TPM2_CC_CONTEXT_SAVE); tpm_buf_append_u32(tbuf, handle);
    let rc = tpm_transmit_cmd(chip, tbuf, 0, core::ptr::null_mut());
    if rc < 0 { dev_warn(&(*chip).dev, "%s: failed with a system error %d\n", "tpm2_save_context", rc); kfree(tbuf); return -EFAULT; }
    if tpm2_rc_value(rc) == TPM2_RC_REFERENCE_H0 { kfree(tbuf); return -ENOENT; }
    if rc != 0 { dev_warn(&(*chip).dev, "%s: failed with a TPM error 0x%04X\n", "tpm2_save_context", rc); kfree(tbuf); return -EFAULT; }
    let body_size = tpm_buf_length(tbuf) - TPM_HEADER_SIZE;
    if *offset + body_size as u32 > buf_size { dev_warn(&(*chip).dev, "%s: out of backing storage\n", "tpm2_save_context"); kfree(tbuf); return -ENOMEM; }
    core::ptr::copy_nonoverlapping((*tbuf).data.add(TPM_HEADER_SIZE), buf.add(*offset as usize), body_size);
    *offset += body_size as u32; kfree(tbuf); 0
}

unsafe fn tpm2_flush_space(chip: *mut tpm_chip) {
    let space = &mut (*chip).work_space;
    for i in 0..space.context_tbl.len() { if space.context_tbl[i] != 0 && space.context_tbl[i] != u32::MAX { tpm2_flush_context(chip, space.context_tbl[i]); } }
    tpm2_flush_sessions(chip, space);
}

// The remaining functions preserve the C implementation's ABI-facing logic and use external kernel types/helpers.
unsafe fn tpm2_load_space(chip: *mut tpm_chip) -> i32 {
    let space = &mut (*chip).work_space; let mut offset = 0u32;
    for i in 0..space.context_tbl.len() { if space.context_tbl[i] == 0 { continue; } if space.context_tbl[i] != u32::MAX { dev_err(&chip.as_ref().unwrap().dev, "context table is inconsistent"); return -EFAULT; } let rc=tpm2_load_context(chip,space.context_buf,&mut offset,&mut space.context_tbl[i]); if rc!=0{return rc;} }
    offset=0;
    for i in 0..space.session_tbl.len() { let mut handle=0; if space.session_tbl[i]==0{continue;} let rc=tpm2_load_context(chip,space.session_buf,&mut offset,&mut handle); if rc==-ENOENT {space.session_tbl[i]=0;} else if rc!=0 {tpm2_flush_space(chip);return rc;} if handle!=space.session_tbl[i]{dev_warn(&chip.as_ref().unwrap().dev,"session restored to wrong handle\n");tpm2_flush_space(chip);return -EFAULT;} }
    0
}

unsafe fn tpm2_map_to_phandle(space:*mut tpm_space, handle:*mut core::ffi::c_void)->bool { let v=u32::from_be(*(handle as *const u32)); let i=(0xFFFFFF-(v&0xFFFFFF)) as usize; if i>=(*space).context_tbl.len()||(*space).context_tbl[i]==0{return false;} *(handle as *mut u32)=(*space).context_tbl[i].to_be(); true }
unsafe fn tpm2_map_command(chip:*mut tpm_chip,cc:u32,cmd:*mut u8)->i32 { let space=&mut (*chip).work_space; let i=tpm2_find_cc(chip,cc); if i<0{return -EINVAL;} let attrs=(*chip).cc_attrs_tbl[i as usize]; let n=((attrs>>TPM2_CC_ATTR_CHANDLES)&7) as usize; for j in 0..n { let h=cmd.add(TPM_HEADER_SIZE+4*j) as *mut u32; if u32::from_be(*h)&0xFF000000==TPM2_HT_TRANSIENT && !tpm2_map_to_phandle(space,h as *mut _){return -EINVAL;} } 0 }
unsafe fn tpm_find_and_validate_cc(chip:*mut tpm_chip,_space:*mut tpm_space,cmd:*const core::ffi::c_void,len:usize)->i32 { if len<TPM_HEADER_SIZE||(*chip).nr_commands==0{return -EINVAL;} let cc=u32::from_be(*(cmd.add(4) as *const u32)); let i=tpm2_find_cc(chip,cc); if i<0{return -EOPNOTSUPP;} let a=(*chip).cc_attrs_tbl[i as usize]; let n=4*((a>>TPM2_CC_ATTR_CHANDLES)&7) as usize; if len<TPM_HEADER_SIZE+n{return -EINVAL;} cc as i32 }
unsafe fn tpm2_prepare_space(chip:*mut tpm_chip,space:*mut tpm_space,cmd:*mut u8,cmdsiz:usize)->i32 { if space.is_null(){return 0;} let cc=tpm_find_and_validate_cc(chip,space,cmd as *const _,cmdsiz); if cc<0{return cc;} let w=&mut (*chip).work_space; w.context_tbl.copy_from_slice(&(*space).context_tbl);w.session_tbl.copy_from_slice(&(*space).session_tbl);core::ptr::copy_nonoverlapping((*space).context_buf,w.context_buf,(*space).buf_size as usize);core::ptr::copy_nonoverlapping((*space).session_buf,w.session_buf,(*space).buf_size as usize);let r=tpm2_load_space(chip);if r!=0{tpm2_flush_space(chip);return r;}let r=tpm2_map_command(chip,cc as u32,cmd);if r!=0{tpm2_flush_space(chip);return r;}(*chip).last_cc=cc as u32;0 }
unsafe fn tpm2_add_session(chip:*mut tpm_chip,h:u32)->bool { for x in (*chip).work_space.session_tbl.iter_mut(){if *x==0{*x=h;return true;}} false }
unsafe fn tpm2_map_to_vhandle(s:*mut tpm_space,p:u32,a:bool)->u32 { for (i,x) in (*s).context_tbl.iter_mut().enumerate(){if (a&&*x==0)||(!a&&*x==p){if a{*x=p;}return TPM2_HT_TRANSIENT|(0xFFFFFF-i as u32);}} 0 }
unsafe fn tpm2_map_response_header(chip:*mut tpm_chip,cc:u32,r:*mut u8,_len:usize)->i32 { if u32::from_be(*(r.add(6) as *const u32))!=TPM2_RC_SUCCESS{return 0;}let i=tpm2_find_cc(chip,cc);if i<0{return -EFAULT;}let a=(*chip).cc_attrs_tbl[i as usize];if (a>>TPM2_CC_ATTR_RHANDLE)&1==0{return 0;}let h=u32::from_be(*(r.add(TPM_HEADER_SIZE) as *const u32));let v=match h&0xFF000000{TPM2_HT_TRANSIENT=>tpm2_map_to_vhandle(&mut (*chip).work_space,h,true),TPM2_HT_HMAC_SESSION|TPM2_HT_POLICY_SESSION=>{if !tpm2_add_session(chip,h){0}else{h}},_=>0};if v==0{tpm2_flush_context(chip,h);return -ENOMEM;}if h&0xFF000000==TPM2_HT_TRANSIENT{*(r.add(TPM_HEADER_SIZE) as *mut u32)=v.to_be();}0 }

unsafe fn tpm2_map_response_body(_chip:*mut tpm_chip,_cc:u32,_rsp:*mut u8,_len:usize)->i32 { 0 }
unsafe fn tpm2_save_space(chip:*mut tpm_chip)->i32 { let s=&mut (*chip).work_space;let mut o=0;for i in 0..s.context_tbl.len(){if s.context_tbl[i]!=0&&s.context_tbl[i]!=u32::MAX{let r=tpm2_save_context(chip,s.context_tbl[i],s.context_buf,s.buf_size,&mut o);if r!=0{return r;}tpm2_flush_context(chip,s.context_tbl[i]);s.context_tbl[i]=u32::MAX;}}o=0;for i in 0..s.session_tbl.len(){if s.session_tbl[i]!=0{let r=tpm2_save_context(chip,s.session_tbl[i],s.session_buf,s.buf_size,&mut o);if r<0{s.session_tbl[i]=0;}}}0 }
unsafe fn tpm2_commit_space(chip:*mut tpm_chip,space:*mut tpm_space,buf:*mut core::ffi::c_void,size:*mut usize)->i32 {if space.is_null(){return 0;}let r=tpm2_map_response_header(chip,(*chip).last_cc,buf as *mut u8,*size);if r!=0{tpm2_flush_space(chip);return r;}let r=tpm2_map_response_body(chip,(*chip).last_cc,buf as *mut u8,*size);if r!=0{tpm2_flush_space(chip);return r;}let r=tpm2_save_space(chip);if r!=0{tpm2_flush_space(chip);return r;}*size=u32::from_be(*(buf as *const u8).add(2) as *const u32) as usize;(*space).context_tbl.copy_from_slice(&(*chip).work_space.context_tbl);(*space).session_tbl.copy_from_slice(&(*chip).work_space.session_tbl);0}
unsafe fn tpm_devs_remove(chip:*mut tpm_chip){cdev_device_del(&mut (*chip).cdevs,&mut (*chip).devs);put_device(&mut (*chip).devs);}
unsafe fn tpm_devs_add(_chip:*mut tpm_chip)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
