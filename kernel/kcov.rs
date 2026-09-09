// SPDX-License-Identifier: GPL-2.0
// Linux-kernel dependencies are supplied by the surrounding translation unit.

const KCOV_WORDS_PER_CMP: usize = 4;

#[repr(C)]
pub struct Kcov {
    pub refcount: refcount_t,
    pub lock: spinlock_t,
    pub mode: kcov_mode,
    pub size: u32,
    pub area: *mut core::ffi::c_void,
    pub t: *mut task_struct,
    pub remote: bool,
    pub remote_size: u32,
    pub sequence: i32,
}
#[repr(C)] pub struct KcovRemoteArea { pub list: list_head, pub size: u32 }
#[repr(C)] pub struct KcovRemote { pub handle: u64, pub kcov: *mut Kcov, pub hnode: hlist_node }
#[repr(C)] pub struct KcovPercpuData { pub lock: local_lock_t }

static mut KCOV_REMOTE_LOCK: spinlock_t = DEFINE_SPINLOCK!();
static mut KCOV_REMOTE_MAP: hashtable = DEFINE_HASHTABLE!(4);
static mut KCOV_REMOTE_AREAS: [list_head; 2] = [LIST_HEAD_INIT!(), LIST_HEAD_INIT!()];
static mut KCOV_PERCPU_DATA: KcovPercpuData = KcovPercpuData { lock: INIT_LOCAL_LOCK!() };

unsafe fn kcov_remote_find(handle: u64) -> *mut KcovRemote {
    let mut remote: *mut KcovRemote = core::ptr::null_mut();
    hash_for_each_possible!(KCOV_REMOTE_MAP, remote, hnode, handle, {
        if (*remote).handle == handle { return remote; }
    });
    core::ptr::null_mut()
}
unsafe fn kcov_remote_add(kcov: *mut Kcov, handle: u64) -> *mut KcovRemote {
    if !kcov_remote_find(handle).is_null() { return ERR_PTR!(-EEXIST); }
    let remote = kmalloc_obj!(KcovRemote, GFP_ATOMIC);
    if remote.is_null() { return ERR_PTR!(-ENOMEM); }
    (*remote).handle = handle; (*remote).kcov = kcov;
    hash_add!(KCOV_REMOTE_MAP, &mut (*remote).hnode, handle); remote
}
unsafe fn kcov_remote_area_get(size: u32, irq: bool) -> *mut KcovRemoteArea {
    let list = &mut KCOV_REMOTE_AREAS[irq as usize]; let mut pos = (*list).next;
    while pos != list as *mut _ {
        let area = list_entry!(pos, KcovRemoteArea, list);
        if (*area).size == size { list_del!(&mut (*area).list); return area; } pos = (*pos).next;
    } core::ptr::null_mut()
}
unsafe fn kcov_remote_area_put(area:*mut KcovRemoteArea,size:u32,irq:bool) {
    INIT_LIST_HEAD!(&mut (*area).list); (*area).size=size; list_add!(&mut (*area).list,&mut KCOV_REMOTE_AREAS[irq as usize]);
    kmsan_unpoison_memory!(&mut (*area).list as *mut _,core::mem::size_of::<list_head>());
}
#[inline(always)] unsafe fn in_softirq_really()->bool { in_serving_softirq()&&!in_hardirq()&&!in_nmi() }
unsafe fn check_kcov_mode(needed:kcov_mode,t:*mut task_struct)->bool { if !in_task()&&!(in_softirq_really()&&(*t).kcov_softirq!=0){return false} let m=READ_ONCE!((*t).kcov_mode); barrier!(); m==needed }
unsafe fn canonicalize_ip(ip:usize)->usize { ip /* CONFIG_RANDOMIZE_BASE subtracts kaslr_offset(). */ }

#[no_mangle] pub unsafe extern "C" fn __sanitizer_cov_trace_pc() {
    let t=current; let ip=canonicalize_ip(_RET_IP!()); if !check_kcov_mode(KCOV_MODE_TRACE_PC,t){return}
    let area=(*t).kcov_area as *mut usize; let pos=READ_ONCE!(area)+1;
    if likely!(pos<(*t).kcov_size as usize){WRITE_ONCE!(area,pos);barrier!();*area.add(pos)=ip}
}
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] unsafe fn write_comp_data(typ:u64,a:u64,b:u64,ip:u64) {
    let t=current;if !check_kcov_mode(KCOV_MODE_TRACE_CMP,t){return} let area=(*t).kcov_area as *mut u64;let c=READ_ONCE!(area);
    let s=1+c*KCOV_WORDS_PER_CMP as u64;let max=(*t).kcov_size as u64*core::mem::size_of::<usize>() as u64;
    if (s+KCOV_WORDS_PER_CMP as u64)*8<=max {WRITE_ONCE!(area,c+1);barrier!();*area.add(s as usize)=typ;*area.add(s as usize+1)=a;*area.add(s as usize+2)=b;*area.add(s as usize+3)=canonicalize_ip(ip as usize) as u64}
}
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] macro_rules! cmp_fn {($n:ident,$t:ty,$s:expr,$c:expr)=>{#[no_mangle]pub unsafe extern "C" fn $n(a:$t,b:$t){write_comp_data(KCOV_CMP_SIZE!($s)|$c,a as u64,b as u64,_RET_IP!() as u64)}}}
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_cmp1,u8,0,0);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_cmp2,u16,1,0);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_cmp4,u32,2,0);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_cmp8,u64,3,0);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_const_cmp1,u8,0,KCOV_CMP_CONST);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_const_cmp2,u16,1,KCOV_CMP_CONST);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_const_cmp4,u32,2,KCOV_CMP_CONST);
#[cfg(CONFIG_KCOV_ENABLE_COMPARISONS)] cmp_fn!(__sanitizer_cov_trace_const_cmp8,u64,3,KCOV_CMP_CONST);

unsafe fn kcov_start(t:*mut task_struct,k:*mut Kcov,s:u32,a:*mut core::ffi::c_void,m:kcov_mode,q:i32){(*t).kcov=k;(*t).kcov_size=s;(*t).kcov_area=a;(*t).kcov_sequence=q;barrier!();WRITE_ONCE!((*t).kcov_mode,m)}
unsafe fn kcov_stop(t:*mut task_struct){WRITE_ONCE!((*t).kcov_mode,KCOV_MODE_DISABLED);barrier!();(*t).kcov=core::ptr::null_mut();(*t).kcov_size=0;(*t).kcov_area=core::ptr::null_mut()}
unsafe fn kcov_task_reset(t:*mut task_struct){kcov_stop(t);(*t).kcov_sequence=0}
#[no_mangle]pub unsafe extern "C" fn kcov_task_init(t:*mut task_struct){kcov_task_reset(t);(*t).kcov_remote=core::ptr::null_mut();(*t).kcov_handle=(*current).kcov_handle;(*t).kcov_softirq=0;(*t).kcov_saved_mode=0;(*t).kcov_saved_size=0;(*t).kcov_saved_area=core::ptr::null_mut();(*t).kcov_saved_kcov=core::ptr::null_mut();(*t).kcov_saved_sequence=0}
unsafe fn kcov_reset(k:*mut Kcov){(*k).t=core::ptr::null_mut();(*k).mode=KCOV_MODE_INIT;(*k).remote=false;(*k).remote_size=0;(*k).sequence+=1}
unsafe fn kcov_get(k:*mut Kcov){refcount_inc!(&mut(*k).refcount)}
unsafe fn kcov_put(k:*mut Kcov){if refcount_dec_and_test!(&mut(*k).refcount){vfree!((*k).area);kfree!(k)}}

extern "C"{fn kcov_remote_start(handle:u64);fn kcov_remote_stop();fn kcov_common_handle()->kcov_common_handle_id;}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
