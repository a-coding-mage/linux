// SPDX-License-Identifier: GPL-2.0-only
/* OMAP powerdomain control.  C headers and symbols are supplied by the
 * surrounding kernel translation unit. */

const PWRDM_TRACE_STATES_FLAG: u32 = 1 << 31;
const PWRDM_STATE_NOW: i32 = 0;
const PWRDM_STATE_PREV: i32 = 1;
const ALREADYACTIVE_SWITCH: u8 = 0;
const FORCEWAKEUP_SWITCH: u8 = 1;
const LOWPOWERSTATE_SWITCH: u8 = 2;

static mut ARCH_PWRDM: *mut pwrdm_ops = core::ptr::null_mut();

unsafe fn _pwrdm_lookup(name: *const core::ffi::c_char) -> *mut powerdomain {
    let mut p: *mut powerdomain = core::ptr::null_mut();
    list_for_each_entry!(t, pwrdm_list, node, {
        if strcmp(name, (*t).name) == 0 { p = t; break; }
    });
    p
}

static mut pwrdm_list: list_head = LIST_HEAD_INIT!(pwrdm_list);

unsafe fn _pwrdm_register(p: *mut powerdomain) -> i32 {
    if p.is_null() || (*p).name.is_null() { return -EINVAL; }
    if cpu_is_omap44xx() && (*p).prcm_partition == OMAP4430_INVALID_PRCM_PARTITION {
        pr_err!("powerdomain: {}: missing OMAP4 PRCM partition ID\n", cstr!((*p).name));
        return -EINVAL;
    }
    if !_pwrdm_lookup((*p).name).is_null() { return -EEXIST; }
    if !ARCH_PWRDM.is_null() {
        if let Some(f) = (*ARCH_PWRDM).pwrdm_has_voltdm {
            if !f() { goto!(skip_voltdm); }
        }
    }
    let v = voltdm_lookup((*p).voltdm.name);
    if v.is_null() { pr_err!("powerdomain: voltagedomain does not exist\n"); return -EINVAL; }
    (*p).voltdm.ptr = v;
    INIT_LIST_HEAD!(&mut (*p).voltdm_node);
    skip_voltdm:
    spin_lock_init!(&mut (*p)._lock);
    list_add!(&mut (*p).node, &mut pwrdm_list);
    for x in (*p).state_counter.iter_mut() { *x = 0; }
    (*p).ret_logic_off_counter = 0;
    for i in 0..(*p).banks as usize { (*p).ret_mem_off_counter[i] = 0; }
    if !ARCH_PWRDM.is_null() { if let Some(f) = (*ARCH_PWRDM).pwrdm_wait_transition { f(p); } }
    (*p).state = pwrdm_read_pwrst(p);
    (*p).state_counter[(*p).state as usize] = 1;
    pr_debug!("powerdomain: registered\n");
    0
}

unsafe fn _update_logic_membank_counters(p: *mut powerdomain) {
    let l = pwrdm_read_prev_logic_pwrst(p);
    if (*p).pwrsts_logic_ret == PWRSTS_OFF_RET && l == PWRDM_POWER_OFF { (*p).ret_logic_off_counter += 1; }
    for i in 0..(*p).banks { let m = pwrdm_read_prev_mem_pwrst(p, i); if (*p).pwrsts_mem_ret[i as usize] == PWRSTS_OFF_RET && m == PWRDM_POWER_OFF { (*p).ret_mem_off_counter[i as usize] += 1; } }
}

unsafe fn _pwrdm_state_switch(p: *mut powerdomain, flag: i32) -> i32 {
    if p.is_null() { return -EINVAL; }
    let state = pwrdm_read_pwrst(p);
    let prev = match flag { PWRDM_STATE_NOW => (*p).state, PWRDM_STATE_PREV => { let x=pwrdm_read_prev_pwrst(p); if x>=0 && (*p).state!=x { (*p).state_counter[x as usize]+=1; } if x==PWRDM_POWER_RET { _update_logic_membank_counters(p); } let n=pwrdm_read_next_pwrst(p); if n!=x { let ts=PWRDM_TRACE_STATES_FLAG | (((n as u32)&OMAP_POWERSTATE_MASK)<<8) | ((x as u32)&OMAP_POWERSTATE_MASK); trace_power_domain_target((*p).name,ts,raw_smp_processor_id()); } x }, _ => return -EINVAL };
    if state != prev { (*p).state_counter[state as usize] += 1; }
    pm_dbg_update_time(p, prev); (*p).state=state; 0
}
unsafe fn _pwrdm_pre_transition_cb(p:*mut powerdomain,_:*mut core::ffi::c_void)->i32 { pwrdm_clear_all_prev_pwrst(p); _pwrdm_state_switch(p,PWRDM_STATE_NOW); 0 }
unsafe fn _pwrdm_post_transition_cb(p:*mut powerdomain,_:*mut core::ffi::c_void)->i32 { _pwrdm_state_switch(p,PWRDM_STATE_PREV); 0 }

unsafe fn _pwrdm_save_clkdm_state_and_activate(p:*mut powerdomain,c:u8,s:u8)->u8 { if c<PWRDM_POWER_ON { if c>s && (*p).flags&PWRDM_HAS_LOWPOWERSTATECHANGE!=0 && !ARCH_PWRDM.is_null() && (*ARCH_PWRDM).pwrdm_set_lowpwrstchange.is_some() { LOWPOWERSTATE_SWITCH } else { clkdm_deny_idle_nolock((*p).pwrdm_clkdms[0]); FORCEWAKEUP_SWITCH } } else { ALREADYACTIVE_SWITCH } }
unsafe fn _pwrdm_restore_clkdm_state(p:*mut powerdomain,s:u8) { match s { FORCEWAKEUP_SWITCH=>clkdm_allow_idle_nolock((*p).pwrdm_clkdms[0]), LOWPOWERSTATE_SWITCH=>{ if (*p).flags&PWRDM_HAS_LOWPOWERSTATECHANGE!=0 { if let Some(f)=(*ARCH_PWRDM).pwrdm_set_lowpwrstchange { f(p); } } pwrdm_state_switch_nolock(p); }, _=>{} } }

pub unsafe fn pwrdm_register_platform_funcs(po:*mut pwrdm_ops)->i32 { if po.is_null(){return -EINVAL;} if !ARCH_PWRDM.is_null(){return -EEXIST;} ARCH_PWRDM=po; 0 }
pub unsafe fn pwrdm_register_pwrdms(ps:*mut *mut powerdomain)->i32 { if ARCH_PWRDM.is_null(){return -EEXIST;} if ps.is_null(){return -EINVAL;} let mut p=ps; while !(*p).is_null(){_pwrdm_register(*p);p=p.add(1);} 0 }

pub unsafe fn pwrdm_complete_init()->i32 { if list_empty!(&pwrdm_list){return -EACCES;} list_for_each_entry!(p,pwrdm_list,node,{pwrdm_set_next_pwrst(p,PWRDM_POWER_ON);}); 0 }
pub unsafe fn pwrdm_lock(p:*mut powerdomain){spin_lock_irqsave!(&mut (*p)._lock,&mut (*p)._lock_flags);}
pub unsafe fn pwrdm_unlock(p:*mut powerdomain){spin_unlock_irqrestore!(&mut (*p)._lock,&(*p)._lock_flags);}
pub unsafe fn pwrdm_lookup(n:*const core::ffi::c_char)->*mut powerdomain{if n.is_null(){core::ptr::null_mut()}else{_pwrdm_lookup(n)}}
pub unsafe fn pwrdm_for_each(f:Option<unsafe fn(*mut powerdomain,*mut core::ffi::c_void)->i32>,u:*mut core::ffi::c_void)->i32{if f.is_none(){return -EINVAL}let mut r=0;list_for_each_entry!(p,pwrdm_list,node,{r=f.unwrap()(p,u);if r!=0{break;}});r}
pub unsafe fn pwrdm_add_clkdm(p:*mut powerdomain,c:*mut clockdomain)->i32 {if p.is_null()||c.is_null(){return -EINVAL;} for i in 0..PWRDM_MAX_CLKDMS {if (*p).pwrdm_clkdms[i].is_null(){(*p).pwrdm_clkdms[i]=c;return 0;}} WARN_ON!(1);-ENOMEM}
pub unsafe fn pwrdm_get_mem_bank_count(p:*mut powerdomain)->i32{if p.is_null(){-EINVAL}else{(*p).banks}}

pub unsafe fn pwrdm_set_next_pwrst(p:*mut powerdomain,s:u8)->i32 {if p.is_null()||(*p).pwrsts&(1<<s)==0{return -EINVAL;} if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_set_next_pwrst{trace_power_domain_target((*p).name,s,raw_smp_processor_id());return f(p,s);}}-EINVAL}
pub unsafe fn pwrdm_read_next_pwrst(p:*mut powerdomain)->i32{if p.is_null(){return -EINVAL} if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_read_next_pwrst{return f(p)}}-EINVAL}
pub unsafe fn pwrdm_read_pwrst(p:*mut powerdomain)->i32{if p.is_null(){return -EINVAL} if (*p).pwrsts==PWRSTS_ON{return PWRDM_POWER_ON} if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_read_pwrst{return f(p)}}-EINVAL}
pub unsafe fn pwrdm_read_prev_pwrst(p:*mut powerdomain)->i32{if p.is_null(){return -EINVAL} if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_read_prev_pwrst{return f(p)}}-EINVAL}

pub unsafe fn pwrdm_set_logic_retst(p:*mut powerdomain,s:u8)->i32{if p.is_null()||(*p).pwrsts_logic_ret&(1<<s)==0{return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_set_logic_retst{return f(p,s)}}-EINVAL}
pub unsafe fn pwrdm_set_mem_onst(p:*mut powerdomain,b:u8,s:u8)->i32{if p.is_null(){return -EINVAL}if (*p).banks<=b{return -EEXIST}if (*p).pwrsts_mem_on[b as usize]&(1<<s)==0{return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_set_mem_onst{return f(p,b,s)}}-EINVAL}
pub unsafe fn pwrdm_set_mem_retst(p:*mut powerdomain,b:u8,s:u8)->i32{if p.is_null(){return -EINVAL}if (*p).banks<=b{return -EEXIST}if (*p).pwrsts_mem_ret[b as usize]&(1<<s)==0{return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_set_mem_retst{return f(p,b,s)}}-EINVAL}
macro_rules! read1 {($n:ident,$f:ident)=>{pub unsafe fn $n(p:*mut powerdomain)->i32{if p.is_null(){return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).$f{return f(p)}}-EINVAL}}}
read1!(pwrdm_read_logic_pwrst,pwrdm_read_logic_pwrst); read1!(pwrdm_read_prev_logic_pwrst,pwrdm_read_prev_logic_pwrst); read1!(pwrdm_read_logic_retst,pwrdm_read_logic_retst);
macro_rules! read2 {($n:ident,$f:ident)=>{pub unsafe fn $n(p:*mut powerdomain,mut b:u8)->i32{if p.is_null()||(*p).banks<=b{return -EINVAL}if (*p).flags&PWRDM_HAS_MPU_QUIRK!=0{b=1}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).$f{return f(p,b)}}-EINVAL}}}
read2!(pwrdm_read_mem_pwrst,pwrdm_read_mem_pwrst);read2!(pwrdm_read_prev_mem_pwrst,pwrdm_read_prev_mem_pwrst);read2!(pwrdm_read_mem_retst,pwrdm_read_mem_retst);
pub unsafe fn pwrdm_clear_all_prev_pwrst(p:*mut powerdomain)->i32{if p.is_null(){return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_clear_all_prev_pwrst{return f(p)}}-EINVAL}
pub unsafe fn pwrdm_enable_hdwr_sar(p:*mut powerdomain)->i32{if p.is_null()||(*p).flags&PWRDM_HAS_HDWR_SAR==0{return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_enable_hdwr_sar{return f(p)}}-EINVAL}
pub unsafe fn pwrdm_disable_hdwr_sar(p:*mut powerdomain)->i32{if p.is_null()||(*p).flags&PWRDM_HAS_HDWR_SAR==0{return -EINVAL}if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_disable_hdwr_sar{return f(p)}}-EINVAL}
pub unsafe fn pwrdm_has_hdwr_sar(p:*mut powerdomain)->bool{!p.is_null()&&(*p).flags&PWRDM_HAS_HDWR_SAR!=0}
pub unsafe fn pwrdm_state_switch_nolock(p:*mut powerdomain)->i32{if p.is_null()||ARCH_PWRDM.is_null(){return -EINVAL}let r=(*ARCH_PWRDM).pwrdm_wait_transition.unwrap()(p);if r==0{_pwrdm_state_switch(p,PWRDM_STATE_NOW)}else{r}}
pub unsafe fn pwrdm_state_switch(p:*mut powerdomain)->i32{pwrdm_lock(p);let r=pwrdm_state_switch_nolock(p);pwrdm_unlock(p);r}
pub unsafe fn pwrdm_pre_transition(p:*mut powerdomain)->i32{if !p.is_null(){_pwrdm_pre_transition_cb(p,core::ptr::null_mut())}0}
pub unsafe fn pwrdm_post_transition(p:*mut powerdomain)->i32{if !p.is_null(){_pwrdm_post_transition_cb(p,core::ptr::null_mut())}0}

pub unsafe fn pwrdm_get_valid_lp_state(p:*mut powerdomain,logic:bool,req:u8)->u8{let states=if logic{(*p).pwrsts_logic_ret}else{(*p).pwrsts};let def=if logic{PWRDM_POWER_RET}else{PWRDM_POWER_ON};if states&BIT!(req)!=0{return req}let mut n=req;while n>0{n-=1;if states&BIT!(n)!=0{return n}}n=req+1;while states&BIT!(n)==0{if n>PWRDM_POWER_ON{return PWRDM_POWER_ON}if n==def{break}n+=1}n}
pub unsafe fn omap_set_pwrdm_state(p:*mut powerdomain,mut s:u8)->i32{if p.is_null(){return -EINVAL}while (*p).pwrsts&(1<<s)==0{if s==PWRDM_POWER_OFF{return 0}s-=1}pwrdm_lock(p);let c=pwrdm_read_pwrst(p);if c<0{pwrdm_unlock(p);return -EINVAL}let n=pwrdm_read_next_pwrst(p);if c!=s||n!=s{let sw=_pwrdm_save_clkdm_state_and_activate(p,c as u8,s);let r=pwrdm_set_next_pwrst(p,s);_pwrdm_restore_clkdm_state(p,sw);pwrdm_unlock(p);r}else{pwrdm_unlock(p);0}}
unsafe fn pwrdm_save_context(p:*mut powerdomain,_:*mut core::ffi::c_void)->i32{if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_save_context{f(p);}}0}
unsafe fn pwrdm_restore_context(p:*mut powerdomain,_:*mut core::ffi::c_void)->i32{if !ARCH_PWRDM.is_null(){if let Some(f)=(*ARCH_PWRDM).pwrdm_restore_context{f(p);}}0}
unsafe fn pwrdms_save_context(){/* iterate pwrdm_list and call pwrdm_save_context */}
unsafe fn pwrdms_restore_context(){/* iterate pwrdm_list and call pwrdm_restore_context */}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
