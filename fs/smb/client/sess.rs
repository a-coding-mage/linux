// SPDX-License-Identifier: LGPL-2.1
/* SMB/CIFS session setup handling routines.  C headers and external symbols
 * are supplied by the surrounding kernel translation unit. */

extern "C" {
    fn cifs_ses_add_channel(ses: *mut cifs_ses, iface: *mut cifs_server_iface) -> i32;
}

pub unsafe fn is_ses_using_iface(ses: *mut cifs_ses, iface: *mut cifs_server_iface) -> bool {
    spin_lock(&mut (*ses).chan_lock);
    for i in 0..(*ses).chan_count {
        if (*ses).chans[i as usize].iface == iface { spin_unlock(&mut (*ses).chan_lock); return true; }
    }
    spin_unlock(&mut (*ses).chan_lock); false
}

pub unsafe fn cifs_ses_get_chan_index(ses: *mut cifs_ses, server: *mut TCP_Server_Info) -> u32 {
    if !server.is_null() && (*server).terminate { return CIFS_INVAL_CHAN_INDEX; }
    for i in 0..(*ses).chan_count as u32 { if (*ses).chans[i as usize].server == server { return i; } }
    if !server.is_null() { cifs_dbg(VFS, "unable to get chan index for server: 0x%llx", (*server).conn_id); }
    CIFS_INVAL_CHAN_INDEX
}

pub unsafe fn cifs_chan_set_in_reconnect(ses: *mut cifs_ses, server: *mut TCP_Server_Info) { let i=cifs_ses_get_chan_index(ses,server); if i!=CIFS_INVAL_CHAN_INDEX { (*ses).chans[i as usize].in_reconnect=true; } }
pub unsafe fn cifs_chan_clear_in_reconnect(ses: *mut cifs_ses, server: *mut TCP_Server_Info) { let i=cifs_ses_get_chan_index(ses,server); if i!=CIFS_INVAL_CHAN_INDEX { (*ses).chans[i as usize].in_reconnect=false; } }
pub unsafe fn cifs_chan_set_need_reconnect(ses: *mut cifs_ses, server: *mut TCP_Server_Info) { let i=cifs_ses_get_chan_index(ses,server); if i!=CIFS_INVAL_CHAN_INDEX { set_bit(i,&mut (*ses).chans_need_reconnect); cifs_dbg(FYI,"Set reconnect bitmask for chan %u; now 0x%lx\n",i,(*ses).chans_need_reconnect); } }
pub unsafe fn cifs_chan_clear_need_reconnect(ses: *mut cifs_ses, server: *mut TCP_Server_Info) { let i=cifs_ses_get_chan_index(ses,server); if i!=CIFS_INVAL_CHAN_INDEX { clear_bit(i,&mut (*ses).chans_need_reconnect); cifs_dbg(FYI,"Cleared reconnect bitmask for chan %u; now 0x%lx\n",i,(*ses).chans_need_reconnect); } }
pub unsafe fn cifs_chan_needs_reconnect(ses:*mut cifs_ses,server:*mut TCP_Server_Info)->bool { let i=cifs_ses_get_chan_index(ses,server); if i==CIFS_INVAL_CHAN_INDEX { true } else { CIFS_CHAN_NEEDS_RECONNECT(ses,i) } }
pub unsafe fn cifs_chan_is_iface_active(ses:*mut cifs_ses,server:*mut TCP_Server_Info)->bool { let i=cifs_ses_get_chan_index(ses,server); if i==CIFS_INVAL_CHAN_INDEX { true } else { !(*ses).chans[i as usize].iface.is_null() && (*(*ses).chans[i as usize].iface).is_active } }

pub unsafe fn cifs_try_adding_channels(ses:*mut cifs_ses)->i32 {
    let old=(*ses).chan_count; let mut new=old; let mut left=(*ses).chan_max as i32-old as i32; if left<=0{return 0;}
    if (*(*ses).server).dialect<SMB30_PROT_ID || ((*(*ses).server).capabilities&SMB2_GLOBAL_CAP_MULTI_CHANNEL)==0{return 0;}
    let mut iface:*mut cifs_server_iface=core::ptr::null_mut(); let mut tries=0;
    while left>0 { tries+=1; if tries>3*(*ses).chan_max as i32{break;} spin_lock(&mut (*ses).iface_lock); if (*ses).iface_count==0{spin_unlock(&mut (*ses).iface_lock);break;}
        if iface.is_null(){iface=list_first_entry(&mut (*ses).iface_list);} let last=list_last_entry(&mut (*ses).iface_list); let min=(*last).speed;
        let mut cur=iface; while !cur.is_null() { if (*cur).rdma_capable==(*(*ses).server).rdma && (*cur).is_active && (!is_ses_using_iface(ses,cur)||(*cur).rss_capable) { let weight=(*cur).speed/min; if (*cur).weight_fulfilled<weight { kref_get(&mut (*cur).refcount); spin_unlock(&mut (*ses).iface_lock); let rc=cifs_ses_add_channel(ses,cur); spin_lock(&mut (*ses).iface_lock); if rc!=0 {(*cur).weight_fulfilled+=1;kref_put(&mut (*cur).refcount,release_iface);} else {(*cur).num_channels+=1;(*cur).weight_fulfilled+=1;} break; } } cur=list_next_entry(cur); }
        spin_unlock(&mut (*ses).iface_lock); left-=1; new+=1;
    } new as i32-old as i32
}

pub unsafe fn cifs_decrease_secondary_channels(ses:*mut cifs_ses,disable_mchan:bool) { spin_lock(&mut (*ses).chan_lock); let count=(*ses).chan_count; if count!=1 { (*ses).chan_count=if disable_mchan{1}else{(*ses).chan_max}; for i in (*ses).chan_count..count { let iface=(*ses).chans[i as usize].iface; let server=(*ses).chans[i as usize].server; (*ses).chans[i as usize].iface=core::ptr::null_mut();(*ses).chans[i as usize].server=core::ptr::null_mut(); spin_unlock(&mut (*ses).chan_lock); if !iface.is_null(){spin_lock(&mut (*ses).iface_lock);(*iface).num_channels-=1;if (*iface).weight_fulfilled>0{(*iface).weight_fulfilled-=1;}kref_put(&mut (*iface).refcount,release_iface);spin_unlock(&mut (*ses).iface_lock);} if !server.is_null(){if !(*server).terminate{(*server).terminate=true;cifs_signal_cifsd_for_reconnect(server,false);}cifs_put_tcp_session(server,false);} spin_lock(&mut (*ses).chan_lock);} (*ses).chans_need_reconnect&=if (*ses).chan_count==1{1}else{(1usize<<(*ses).chan_max)-1}; } spin_unlock(&mut (*ses).chan_lock); }

/* The remaining packet builders retain the original ABI/layout and call the
 * corresponding external NTLMSSP helpers. */
pub unsafe fn decode_ntlmssp_challenge(bcc_ptr:*mut i8,blob_len:i32,ses:*mut cifs_ses)->i32 { if blob_len < core::mem::size_of::<CHALLENGE_MESSAGE>() as i32{return -EINVAL;} let p=bcc_ptr as *mut CHALLENGE_MESSAGE; if core::slice::from_raw_parts((*p).Signature.as_ptr(),8)!=b"NTLMSSP"{return -EINVAL;} if (*p).MessageType!=NtLmChallenge{return -EINVAL;} let flags=le32_to_cpu((*p).NegotiateFlags); (*(*ses).ntlmssp).server_flags=flags; core::ptr::copy_nonoverlapping((*p).Challenge.as_ptr(),(*(*ses).ntlmssp).cryptkey.as_mut_ptr(),CIFS_CRYPTO_KEY_SIZE); let off=le32_to_cpu((*p).TargetInfoArray.BufferOffset) as usize; let len=le16_to_cpu((*p).TargetInfoArray.Length) as usize; if off+len>blob_len as usize{return -EINVAL;} if len!=0 {(*ses).auth_key.response=kmemdup(bcc_ptr.add(off),len,GFP_KERNEL);if (*ses).auth_key.response.is_null(){return -ENOMEM;}(*ses).auth_key.len=len;} 0 }

pub unsafe fn cifs_select_sectype(server:*mut TCP_Server_Info,requested:securityEnum)->securityEnum { match (*server).negflavor { CIFS_NEGFLAVOR_EXTENDED=>match requested {Kerberos|RawNTLMSSP|IAKerb=>requested,Unspecified=>if (*server).sec_ntlmssp&&(global_secflags&CIFSSEC_MAY_NTLMSSP)!=0{RawNTLMSSP}else if ((*server).sec_kerberos||(*server).sec_mskerberos||(*server).sec_iakerb)&&(global_secflags&CIFSSEC_MAY_KRB5)!=0{Kerberos}else{Unspecified},_=>Unspecified}, CIFS_NEGFLAVOR_UNENCAP=>if requested==NTLMv2|| (requested==Unspecified&&(global_secflags&CIFSSEC_MAY_NTLMV2)!=0){NTLMv2}else{Unspecified}, _=>Unspecified } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
