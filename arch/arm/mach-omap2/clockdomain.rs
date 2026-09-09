// SPDX-License-Identifier: GPL-2.0-only
/* OMAP2/3/4 clockdomain framework functions */

// C includes and build-time kernel dependencies are supplied by other Rust units.

static mut CLKDM_LIST: ListHead = ListHead::new();
static mut AUTODEPS: *mut ClkdmAutodep = core::ptr::null_mut();
static mut ARCH_CLKDM: *mut ClkdmOps = core::ptr::null_mut();

extern "C" {
    fn pwrdm_lookup(name: *const i8) -> *mut Powerdomain;
    fn pwrdm_add_clkdm(pwrdm: *mut Powerdomain, clkdm: *mut Clockdomain);
    fn pwrdm_lock(pwrdm: *mut Powerdomain);
    fn pwrdm_unlock(pwrdm: *mut Powerdomain);
    fn pwrdm_state_switch_nolock(pwrdm: *mut Powerdomain) -> i32;
    fn cpu_pm_register_notifier(nb: *mut NotifierBlock) -> i32;
    fn soc_is_am43xx() -> bool;
    fn cpu_is_omap24xx() -> bool;
    fn cpu_is_omap34xx() -> bool;
    fn __clk_get_enable_count(clk: *mut Clk) -> i32;
}

#[allow(non_snake_case)]
unsafe fn _clkdm_lookup(name: *const i8) -> *mut Clockdomain {
    if name.is_null() { return core::ptr::null_mut(); }
    let mut p = unsafe { list_first_entry::<Clockdomain>(&raw mut CLKDM_LIST) };
    while !p.is_null() {
        if unsafe { strcmp(name, (*p).name) } == 0 { return p; }
        p = unsafe { list_next_entry(p) };
    }
    core::ptr::null_mut()
}

unsafe fn _clkdm_register(clkdm: *mut Clockdomain) -> i32 {
    if clkdm.is_null() || unsafe { (*clkdm).name.is_null() } { return -EINVAL; }
    let pwrdm = unsafe { pwrdm_lookup((*clkdm).pwrdm.name) };
    if pwrdm.is_null() { unsafe { pr_err!("clockdomain: powerdomain does not exist\n"); } return -EINVAL; }
    unsafe { (*clkdm).pwrdm.ptr = pwrdm; }
    if unsafe { !_clkdm_lookup((*clkdm).name).is_null() } { return -EEXIST; }
    unsafe { list_add(&mut (*clkdm).node, &raw mut CLKDM_LIST); pwrdm_add_clkdm(pwrdm, clkdm); }
    0
}

unsafe fn _clkdm_deps_lookup(clkdm: *mut Clockdomain, deps: *mut ClkdmDep) -> *mut ClkdmDep {
    if clkdm.is_null() || deps.is_null() { return ERR_PTR(-EINVAL); }
    let mut cd = deps;
    while !(*cd).clkdm_name.is_null() {
        if (*cd).clkdm.is_null() { (*cd).clkdm = _clkdm_lookup((*cd).clkdm_name); }
        if (*cd).clkdm == clkdm { break; }
        cd = cd.add(1);
    }
    if (*cd).clkdm_name.is_null() { ERR_PTR(-ENOENT) } else { cd }
}

unsafe fn _autodep_lookup(a: *mut ClkdmAutodep) {
    if a.is_null() { return; }
    let p = clkdm_lookup((*a).clkdm.name);
    (*a).clkdm.ptr = if p.is_null() { ERR_PTR(-ENOENT) } else { p };
}

unsafe fn _resolve_clkdm_deps(clkdm: *mut Clockdomain, deps: *mut ClkdmDep) {
    let mut cd = deps;
    while !cd.is_null() && !(*cd).clkdm_name.is_null() {
        if (*cd).clkdm.is_null() { (*cd).clkdm = _clkdm_lookup((*cd).clkdm_name); }
        WARN!(!(*cd).clkdm.is_null(), "clockdomain dependency lookup failed");
        cd = cd.add(1);
    }
}

unsafe fn _clkdm_add_wkdep(a: *mut Clockdomain, b: *mut Clockdomain) -> i32 {
    if a.is_null() || b.is_null() { return -EINVAL; }
    let cd = _clkdm_deps_lookup(b, (*a).wkdep_srcs);
    if IS_ERR(cd) || ARCH_CLKDM.is_null() || (*ARCH_CLKDM).clkdm_add_wkdep.is_none() { return -EINVAL; }
    (*cd).wkdep_usecount += 1;
    if (*cd).wkdep_usecount == 1 { ((*ARCH_CLKDM).clkdm_add_wkdep.unwrap())(a, b) } else { 0 }
}
unsafe fn _clkdm_del_wkdep(a: *mut Clockdomain, b: *mut Clockdomain) -> i32 {
    if a.is_null() || b.is_null() { return -EINVAL; }
    let cd = _clkdm_deps_lookup(b, (*a).wkdep_srcs);
    if IS_ERR(cd) || ARCH_CLKDM.is_null() || (*ARCH_CLKDM).clkdm_del_wkdep.is_none() { return -EINVAL; }
    (*cd).wkdep_usecount -= 1;
    if (*cd).wkdep_usecount == 0 { ((*ARCH_CLKDM).clkdm_del_wkdep.unwrap())(a, b) } else { 0 }
}
unsafe fn _clkdm_add_sleepdep(a: *mut Clockdomain, b: *mut Clockdomain) -> i32 {
    if a.is_null() || b.is_null() { return -EINVAL; }
    let cd = _clkdm_deps_lookup(b, (*a).sleepdep_srcs);
    if IS_ERR(cd) || ARCH_CLKDM.is_null() || (*ARCH_CLKDM).clkdm_add_sleepdep.is_none() { return -EINVAL; }
    (*cd).sleepdep_usecount += 1;
    if (*cd).sleepdep_usecount == 1 { ((*ARCH_CLKDM).clkdm_add_sleepdep.unwrap())(a, b) } else { 0 }
}
unsafe fn _clkdm_del_sleepdep(a: *mut Clockdomain, b: *mut Clockdomain) -> i32 {
    if a.is_null() || b.is_null() { return -EINVAL; }
    let cd = _clkdm_deps_lookup(b, (*a).sleepdep_srcs);
    if IS_ERR(cd) || ARCH_CLKDM.is_null() || (*ARCH_CLKDM).clkdm_del_sleepdep.is_none() { return -EINVAL; }
    (*cd).sleepdep_usecount -= 1;
    if (*cd).sleepdep_usecount == 0 { ((*ARCH_CLKDM).clkdm_del_sleepdep.unwrap())(a, b) } else { 0 }
}

pub unsafe fn clkdm_register_platform_funcs(co: *mut ClkdmOps) -> i32 { if co.is_null() { -EINVAL } else if !ARCH_CLKDM.is_null() { -EEXIST } else { ARCH_CLKDM=co; 0 } }
pub unsafe fn clkdm_register_clkdms(cs: *mut *mut Clockdomain) -> i32 { if ARCH_CLKDM.is_null() { return -EACCES; } if cs.is_null() { return -EINVAL; } let mut p=cs; while !(*p).is_null() { _clkdm_register(*p); p=p.add(1); } 0 }
pub unsafe fn clkdm_register_autodeps(ia: *mut ClkdmAutodep) -> i32 { if list_empty(&raw mut CLKDM_LIST) { return -EACCES; } if ia.is_null() { return -EINVAL; } if !AUTODEPS.is_null() { return -EEXIST; } AUTODEPS=ia; let mut p=ia; while !(*p).clkdm.ptr.is_null() { _autodep_lookup(p); p=p.add(1); } 0 }
unsafe extern "C" fn cpu_notifier(_nb: *mut NotifierBlock, cmd: usize, _v: *mut core::ffi::c_void) -> i32 { match cmd { CPU_CLUSTER_PM_ENTER => { if enable_off_mode { clkdm_save_context(); } }, CPU_CLUSTER_PM_EXIT => { if enable_off_mode { clkdm_restore_context(); } }, _ => {} } NOTIFY_OK }
pub unsafe fn clkdm_complete_init() -> i32 { if list_empty(&raw mut CLKDM_LIST) { return -EACCES; } let mut p=list_first_entry::<Clockdomain>(&raw mut CLKDM_LIST); while !p.is_null() { clkdm_deny_idle(p); _resolve_clkdm_deps(p,(*p).wkdep_srcs); clkdm_clear_all_wkdeps(p); _resolve_clkdm_deps(p,(*p).sleepdep_srcs); clkdm_clear_all_sleepdeps(p); p=list_next_entry(p); } if soc_is_am43xx() { static mut NB: NotifierBlock=NotifierBlock::new(); NB.notifier_call=Some(cpu_notifier); cpu_pm_register_notifier(&raw mut NB); } 0 }

pub unsafe fn clkdm_lookup(name: *const i8) -> *mut Clockdomain { _clkdm_lookup(name) }
pub unsafe fn clkdm_for_each(fn_: Option<unsafe extern "C" fn(*mut Clockdomain,*mut core::ffi::c_void)->i32>, user:*mut core::ffi::c_void)->i32 { if fn_.is_none(){return -EINVAL;} let mut p=list_first_entry::<Clockdomain>(&raw mut CLKDM_LIST); let mut r=0; while !p.is_null(){r=fn_.unwrap()(p,user);if r!=0{break} p=list_next_entry(p);} r }
pub unsafe fn clkdm_get_pwrdm(c:*mut Clockdomain)->*mut Powerdomain { if c.is_null(){core::ptr::null_mut()}else{(*c).pwrdm.ptr} }

pub unsafe fn clkdm_add_wkdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} let cd=_clkdm_deps_lookup(b,(*a).wkdep_srcs);if IS_ERR(cd){return PTR_ERR(cd)} pwrdm_lock((*cd).clkdm.pwrdm.ptr);let r=_clkdm_add_wkdep(a,b);pwrdm_unlock((*cd).clkdm.pwrdm.ptr);r }
pub unsafe fn clkdm_del_wkdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} let cd=_clkdm_deps_lookup(b,(*a).wkdep_srcs);if IS_ERR(cd){return PTR_ERR(cd)} pwrdm_lock((*cd).clkdm.pwrdm.ptr);let r=_clkdm_del_wkdep(a,b);pwrdm_unlock((*cd).clkdm.pwrdm.ptr);r }
pub unsafe fn clkdm_read_wkdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} if ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_read_wkdep.is_none(){return -EINVAL;} ((*ARCH_CLKDM).clkdm_read_wkdep.unwrap())(a,b) }
pub unsafe fn clkdm_clear_all_wkdeps(c:*mut Clockdomain)->i32 { if c.is_null()||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_clear_all_wkdeps.is_none(){-EINVAL}else{((*ARCH_CLKDM).clkdm_clear_all_wkdeps.unwrap())(c)} }
pub unsafe fn clkdm_add_sleepdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} let cd=_clkdm_deps_lookup(b,(*a).wkdep_srcs);if IS_ERR(cd){return PTR_ERR(cd)} pwrdm_lock((*cd).clkdm.pwrdm.ptr);let r=_clkdm_add_sleepdep(a,b);pwrdm_unlock((*cd).clkdm.pwrdm.ptr);r }
pub unsafe fn clkdm_del_sleepdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} let cd=_clkdm_deps_lookup(b,(*a).sleepdep_srcs);if IS_ERR(cd){return PTR_ERR(cd)} pwrdm_lock((*cd).clkdm.pwrdm.ptr);let r=_clkdm_del_sleepdep(a,b);pwrdm_unlock((*cd).clkdm.pwrdm.ptr);r }
pub unsafe fn clkdm_read_sleepdep(a:*mut Clockdomain,b:*mut Clockdomain)->i32 { if a.is_null()||b.is_null(){return -EINVAL;} if ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_read_sleepdep.is_none(){return -EINVAL;} ((*ARCH_CLKDM).clkdm_read_sleepdep.unwrap())(a,b) }
pub unsafe fn clkdm_clear_all_sleepdeps(c:*mut Clockdomain)->i32 { if c.is_null()||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_clear_all_sleepdeps.is_none(){-EINVAL}else{((*ARCH_CLKDM).clkdm_clear_all_sleepdeps.unwrap())(c)} }

unsafe fn clkdm_sleep_nolock(c:*mut Clockdomain)->i32 { if c.is_null()||(*c).flags&CLKDM_CAN_FORCE_SLEEP==0||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_sleep.is_none(){return -EINVAL;} (*c)._flags&=!_CLKDM_FLAG_HWSUP_ENABLED; let mut r=((*ARCH_CLKDM).clkdm_sleep.unwrap())(c);r|=pwrdm_state_switch_nolock((*c).pwrdm.ptr);r }
pub unsafe fn clkdm_sleep(c:*mut Clockdomain)->i32 { pwrdm_lock((*c).pwrdm.ptr);let r=clkdm_sleep_nolock(c);pwrdm_unlock((*c).pwrdm.ptr);r }
unsafe fn clkdm_wakeup_nolock(c:*mut Clockdomain)->i32 { if c.is_null()||(*c).flags&CLKDM_CAN_FORCE_WAKEUP==0||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_wakeup.is_none(){return -EINVAL;} (*c)._flags&=!_CLKDM_FLAG_HWSUP_ENABLED; let mut r=((*ARCH_CLKDM).clkdm_wakeup.unwrap())(c);r|=pwrdm_state_switch_nolock((*c).pwrdm.ptr);r }
pub unsafe fn clkdm_wakeup(c:*mut Clockdomain)->i32 { pwrdm_lock((*c).pwrdm.ptr);let r=clkdm_wakeup_nolock(c);pwrdm_unlock((*c).pwrdm.ptr);r }

pub unsafe fn clkdm_allow_idle_nolock(c:*mut Clockdomain){if c.is_null(){return;}if (*c).forcewake_count>0{(*c).forcewake_count-=1;}if (*c).forcewake_count!=0{return;}if (*c).usecount==0&&(*c).flags&CLKDM_CAN_FORCE_SLEEP!=0{clkdm_sleep_nolock(c);}if (*c).flags&CLKDM_CAN_ENABLE_AUTO==0||(*c).flags&CLKDM_MISSING_IDLE_REPORTING!=0||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_allow_idle.is_none(){return;}(*c)._flags|=_CLKDM_FLAG_HWSUP_ENABLED;((*ARCH_CLKDM).clkdm_allow_idle.unwrap())(c);pwrdm_state_switch_nolock((*c).pwrdm.ptr);}
pub unsafe fn clkdm_allow_idle(c:*mut Clockdomain){pwrdm_lock((*c).pwrdm.ptr);clkdm_allow_idle_nolock(c);pwrdm_unlock((*c).pwrdm.ptr);}
pub unsafe fn clkdm_deny_idle_nolock(c:*mut Clockdomain){if c.is_null(){return;}let was=(*c).forcewake_count;(*c).forcewake_count+=1;if was!=0{return;}if (*c).flags&CLKDM_CAN_FORCE_WAKEUP!=0{clkdm_wakeup_nolock(c);}if (*c).flags&CLKDM_CAN_DISABLE_AUTO==0||(*c).flags&CLKDM_MISSING_IDLE_REPORTING!=0||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_deny_idle.is_none(){return;}(*c)._flags&=!_CLKDM_FLAG_HWSUP_ENABLED;((*ARCH_CLKDM).clkdm_deny_idle.unwrap())(c);pwrdm_state_switch_nolock((*c).pwrdm.ptr);}
pub unsafe fn clkdm_deny_idle(c:*mut Clockdomain){pwrdm_lock((*c).pwrdm.ptr);clkdm_deny_idle_nolock(c);pwrdm_unlock((*c).pwrdm.ptr);}

pub unsafe fn clkdm_add_autodeps(c:*mut Clockdomain){if AUTODEPS.is_null()||(*c).flags&CLKDM_NO_AUTODEPS!=0{return;}let mut a=AUTODEPS;while !(*a).clkdm.ptr.is_null(){if !IS_ERR((*a).clkdm.ptr){_clkdm_add_sleepdep(c,(*a).clkdm.ptr);_clkdm_add_wkdep(c,(*a).clkdm.ptr);}a=a.add(1);}}
pub unsafe fn clkdm_del_autodeps(c:*mut Clockdomain){if AUTODEPS.is_null()||(*c).flags&CLKDM_NO_AUTODEPS!=0{return;}let mut a=AUTODEPS;while !(*a).clkdm.ptr.is_null(){if !IS_ERR((*a).clkdm.ptr){_clkdm_del_sleepdep(c,(*a).clkdm.ptr);_clkdm_del_wkdep(c,(*a).clkdm.ptr);}a=a.add(1);}}

pub unsafe fn clkdm_clk_enable(c:*mut Clockdomain,_unused:*mut Clk)->i32 {if c.is_null()||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_clk_enable.is_none(){return -EINVAL;}pwrdm_lock((*c).pwrdm.ptr);(*c).usecount+=1;if (*c).usecount>1&&!AUTODEPS.is_null(){pwrdm_unlock((*c).pwrdm.ptr);return 0;}((*ARCH_CLKDM).clkdm_clk_enable.unwrap())(c);pwrdm_state_switch_nolock((*c).pwrdm.ptr);pwrdm_unlock((*c).pwrdm.ptr);0}
pub unsafe fn clkdm_clk_disable(c:*mut Clockdomain,clk:*mut Clk)->i32 {if c.is_null()||ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_clk_disable.is_none(){return -EINVAL;}pwrdm_lock((*c).pwrdm.ptr);if !clk.is_null()&&__clk_get_enable_count(clk)==0&&(*c).usecount==0{pwrdm_unlock((*c).pwrdm.ptr);return 0;}if (*c).usecount==0{pwrdm_unlock((*c).pwrdm.ptr);return -ERANGE;}(*c).usecount-=1;if (*c).usecount>0{pwrdm_unlock((*c).pwrdm.ptr);return 0;}((*ARCH_CLKDM).clkdm_clk_disable.unwrap())(c);pwrdm_state_switch_nolock((*c).pwrdm.ptr);pwrdm_unlock((*c).pwrdm.ptr);0}
pub unsafe fn clkdm_hwmod_enable(c:*mut Clockdomain,oh:*mut OmapHwmod)->i32 {if cpu_is_omap24xx()||cpu_is_omap34xx(){0}else if oh.is_null(){-EINVAL}else{clkdm_clk_enable(c,core::ptr::null_mut())}}
pub unsafe fn clkdm_hwmod_disable(c:*mut Clockdomain,oh:*mut OmapHwmod)->i32 {if cpu_is_omap24xx()||cpu_is_omap34xx(){0}else if oh.is_null(){-EINVAL}else{clkdm_clk_disable(c,core::ptr::null_mut())}}
unsafe fn _clkdm_save_context(c:*mut Clockdomain,_:*mut core::ffi::c_void)->i32 {if ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_save_context.is_none(){-EINVAL}else{((*ARCH_CLKDM).clkdm_save_context.unwrap())(c)}}
unsafe fn _clkdm_restore_context(c:*mut Clockdomain,_:*mut core::ffi::c_void)->i32 {if ARCH_CLKDM.is_null()||(*ARCH_CLKDM).clkdm_restore_context.is_none(){-EINVAL}else{((*ARCH_CLKDM).clkdm_restore_context.unwrap())(c)}}
pub unsafe fn clkdm_save_context(){clkdm_for_each(Some(_clkdm_save_context),core::ptr::null_mut());}
pub unsafe fn clkdm_restore_context(){clkdm_for_each(Some(_clkdm_restore_context),core::ptr::null_mut());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
