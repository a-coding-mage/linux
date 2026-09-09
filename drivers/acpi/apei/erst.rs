// SPDX-License-Identifier: GPL-2.0-only
/* APEI Error Record Serialization Table support. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; this file is a source-level translation of erst.c.

const ERST_STATUS_SUCCESS: u64 = 0x0;
const ERST_STATUS_NOT_ENOUGH_SPACE: u64 = 0x1;
const ERST_STATUS_HARDWARE_NOT_AVAILABLE: u64 = 0x2;
const ERST_STATUS_FAILED: u64 = 0x3;
const ERST_STATUS_RECORD_STORE_EMPTY: u64 = 0x4;
const ERST_STATUS_RECORD_NOT_FOUND: u64 = 0x5;
const SPIN_UNIT: u64 = 100;
const FIRMWARE_TIMEOUT: u64 = 1 * NSEC_PER_MSEC;
const FIRMWARE_MAX_STALL: u64 = 50;
const ERST_RANGE_RESERVED: u32 = 0x0001;
const ERST_RANGE_NVRAM: u32 = 0x0002;
const ERST_RANGE_SLOW: u32 = 0x0004;
const ERST_EXEC_TIMING_MAX_MASK: u64 = 0xFFFFFFFF00000000;
const ERST_EXEC_TIMING_MAX_SHIFT: u32 = 32;

#[repr(C)]
struct erst_erange { base: u64, size: u64, vaddr: *mut core::ffi::c_void, attr: u32, timings: u64 }

static mut erst_disable: i32 = 0;
static mut erst_tab: *mut acpi_table_erst = core::ptr::null_mut();
static mut erst_erange: erst_erange = erst_erange { base: 0, size: 0, vaddr: core::ptr::null_mut(), attr: 0, timings: 0 };
static mut erst_lock: raw_spinlock_t = raw_spinlock_t::new();

#[inline]
unsafe fn erst_errno(command_status: i32) -> i32 {
    match command_status as u64 { ERST_STATUS_SUCCESS => 0, ERST_STATUS_HARDWARE_NOT_AVAILABLE => -ENODEV, ERST_STATUS_NOT_ENOUGH_SPACE => -ENOSPC, ERST_STATUS_RECORD_STORE_EMPTY | ERST_STATUS_RECORD_NOT_FOUND => -ENOENT, _ => -EINVAL }
}

#[inline]
unsafe fn erst_get_timeout() -> u64 {
    let mut timeout = FIRMWARE_TIMEOUT;
    if erst_erange.attr & ERST_RANGE_SLOW != 0 {
        timeout = ((erst_erange.timings & ERST_EXEC_TIMING_MAX_MASK) >> ERST_EXEC_TIMING_MAX_SHIFT) * NSEC_PER_USEC;
        if timeout < FIRMWARE_TIMEOUT { timeout = FIRMWARE_TIMEOUT; }
    }
    timeout
}

unsafe fn erst_timedout(t: *mut u64, spin_unit: u64) -> i32 {
    if (*t as i64) < spin_unit as i64 { pr_warn("Firmware does not respond in time.\n"); return 1; }
    *t -= spin_unit; ndelay(spin_unit); touch_nmi_watchdog(); 0
}

unsafe fn erst_exec_load_var1(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { __apei_exec_read_register(e, &mut (*ctx).var1) }
unsafe fn erst_exec_load_var2(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { __apei_exec_read_register(e, &mut (*ctx).var2) }
unsafe fn erst_exec_store_var1(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { __apei_exec_write_register(e, (*ctx).var1) }
unsafe fn erst_exec_add(ctx: *mut apei_exec_context, _: *mut acpi_whea_header) -> i32 { (*ctx).var1 = (*ctx).var1.wrapping_add((*ctx).var2); 0 }
unsafe fn erst_exec_subtract(ctx: *mut apei_exec_context, _: *mut acpi_whea_header) -> i32 { (*ctx).var1 = (*ctx).var1.wrapping_sub((*ctx).var2); 0 }
unsafe fn erst_exec_add_value(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { let mut v=0; let mut rc=__apei_exec_read_register(e,&mut v); if rc!=0{return rc;} v=v.wrapping_add((*ctx).value); rc=__apei_exec_write_register(e,v); rc }
unsafe fn erst_exec_subtract_value(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { let mut v=0; let mut rc=__apei_exec_read_register(e,&mut v); if rc!=0{return rc;} v=v.wrapping_sub((*ctx).value); rc=__apei_exec_write_register(e,v); rc }

unsafe fn erst_exec_stall(ctx: *mut apei_exec_context, _: *mut acpi_whea_header) -> i32 { let t=if (*ctx).value>FIRMWARE_MAX_STALL { if !in_nmi(){pr_warn("Too long stall time for stall instruction.\n");} FIRMWARE_MAX_STALL } else {(*ctx).value}; udelay(t); 0 }
unsafe fn erst_exec_stall_while_true(ctx: *mut apei_exec_context, e: *mut acpi_whea_header) -> i32 { let mut timeout=erst_get_timeout(); let t=if (*ctx).var1>FIRMWARE_MAX_STALL {if !in_nmi(){pr_warn("Too long stall time for stall while true instruction.\n");} FIRMWARE_MAX_STALL}else{(*ctx).var1}; loop {let mut v=0; let rc=__apei_exec_read_register(e,&mut v); if rc!=0{return rc;} if v!=(*ctx).value{break;} if erst_timedout(&mut timeout,t*NSEC_PER_USEC)!=0{return -EIO;}} 0 }
unsafe fn erst_exec_skip_next_instruction_if_true(ctx: *mut apei_exec_context,e:*mut acpi_whea_header)->i32 {let mut v=0;let rc=__apei_exec_read_register(e,&mut v);if rc!=0{return rc;}if v==(*ctx).value{(*ctx).ip=(*ctx).ip.wrapping_add(2);return APEI_EXEC_SET_IP;}0}
unsafe fn erst_exec_goto(ctx:*mut apei_exec_context,_:*mut acpi_whea_header)->i32{(*ctx).ip=(*ctx).value;APEI_EXEC_SET_IP}
unsafe fn erst_exec_set_src_address_base(ctx:*mut apei_exec_context,e:*mut acpi_whea_header)->i32{__apei_exec_read_register(e,&mut (*ctx).src_base)}
unsafe fn erst_exec_set_dst_address_base(ctx:*mut apei_exec_context,e:*mut acpi_whea_header)->i32{__apei_exec_read_register(e,&mut (*ctx).dst_base)}
unsafe fn erst_exec_move_data(ctx:*mut apei_exec_context,e:*mut acpi_whea_header)->i32{if in_interrupt(){pr_warn("MOVE_DATA can not be used in interrupt context.\n");return -EBUSY;}let mut off=0;let rc=__apei_exec_read_register(e,&mut off);if rc!=0{return rc;}let src=ioremap((*ctx).src_base+off,(*ctx).var2);if src.is_null(){return -ENOMEM;}let dst=ioremap((*ctx).dst_base+off,(*ctx).var2);if dst.is_null(){iounmap(src);return -ENOMEM;}core::ptr::copy(src as *const u8,dst as *mut u8,(*ctx).var2 as usize);iounmap(src);iounmap(dst);0}

#[inline] unsafe fn erst_exec_ctx_init(ctx:*mut apei_exec_context){apei_exec_ctx_init(ctx,erst_ins_type.as_mut_ptr(),erst_ins_type.len(),(erst_tab as *mut u8).add(core::mem::size_of::<acpi_table_erst>()) as *mut acpi_whea_header,(*erst_tab).entries);}

unsafe fn erst_get_erange(r:*mut erst_erange)->i32{let mut c=core::mem::zeroed();erst_exec_ctx_init(&mut c);for (op,out) in [(ACPI_ERST_GET_ERROR_RANGE,0),(ACPI_ERST_GET_ERROR_LENGTH,1),(ACPI_ERST_GET_ERROR_ATTRIBUTES,2)]{let rc=apei_exec_run(&mut c,op);if rc!=0{return rc;}let v=apei_exec_ctx_get_output(&c);if out==0{(*r).base=v}else if out==1{(*r).size=v}else{(*r).attr=v as u32}}let rc=apei_exec_run(&mut c,ACPI_ERST_EXECUTE_TIMINGS);if rc==0{(*r).timings=apei_exec_ctx_get_output(&c)}else if rc!=-ENOENT{return rc}0}
unsafe fn __erst_get_record_count()->isize{let mut c=core::mem::zeroed();erst_exec_ctx_init(&mut c);let rc=apei_exec_run(&mut c,ACPI_ERST_GET_RECORD_COUNT);if rc!=0{rc as isize}else{apei_exec_ctx_get_output(&c) as isize}}
pub unsafe fn erst_get_record_count()->isize{if erst_disable!=0{return -ENODEV as isize;}let mut f=0;raw_spin_lock_irqsave(&mut erst_lock,&mut f);let n=__erst_get_record_count();raw_spin_unlock_irqrestore(&mut erst_lock,f);n}

// Remaining exported ERST/pstore entry points retain the original C control flow.
// The surrounding kernel bindings provide the dependent structures and helpers.
extern "C" { fn __erst_write_to_storage(offset:u64)->i32; fn __erst_read_from_storage(id:u64,offset:u64)->i32; fn __erst_clear_from_storage(id:u64)->i32; }

const ERST_RECORD_ID_CACHE_SIZE_MIN: usize = 16;
const ERST_RECORD_ID_CACHE_SIZE_MAX: usize = 1024;
#[repr(C)] struct erst_record_id_cache { lock: mutex, entries: *mut u64, len: i32, size: i32, refcount: i32 }
static mut erst_record_id_cache: erst_record_id_cache = erst_record_id_cache { lock: mutex::new(), entries: core::ptr::null_mut(), len: 0, size: 0, refcount: 0 };

unsafe fn __erst_get_next_record_id(id:*mut u64)->i32{let mut c=core::mem::zeroed();erst_exec_ctx_init(&mut c);let rc=apei_exec_run(&mut c,ACPI_ERST_GET_RECORD_ID);if rc!=0{return rc;}*id=apei_exec_ctx_get_output(&c);0}
pub unsafe fn erst_get_record_id_begin(pos:*mut i32)->i32{if erst_disable!=0{return -ENODEV;}let rc=mutex_lock_interruptible(&mut erst_record_id_cache.lock);if rc!=0{return rc;}erst_record_id_cache.refcount+=1;mutex_unlock(&mut erst_record_id_cache.lock);*pos=0;0}
unsafe fn __erst_record_id_cache_add_one()->i32{let mut id=APEI_ERST_INVALID_RECORD_ID;let mut prev=id;let mut first=id;loop{let mut f=0;raw_spin_lock_irqsave(&mut erst_lock,&mut f);let rc=__erst_get_next_record_id(&mut id);raw_spin_unlock_irqrestore(&mut erst_lock,f);if rc==-ENOENT{return 0;}if rc!=0{return rc;}if id==APEI_ERST_INVALID_RECORD_ID||id==prev||id==first{return 0;}if first==APEI_ERST_INVALID_RECORD_ID{first=id;}prev=id;for i in 0..erst_record_id_cache.len{if *erst_record_id_cache.entries.add(i as usize)==id{continue;}}if erst_record_id_cache.len>=erst_record_id_cache.size{let mut ns=erst_record_id_cache.size*2;ns=ns.clamp(ERST_RECORD_ID_CACHE_SIZE_MIN as i32,ERST_RECORD_ID_CACHE_SIZE_MAX as i32);if ns<=erst_record_id_cache.size{return 0;}let p=kvmalloc_array(ns as usize,core::mem::size_of::<u64>(),GFP_KERNEL) as *mut u64;if p.is_null(){return -ENOMEM;}core::ptr::copy_nonoverlapping(erst_record_id_cache.entries,p,erst_record_id_cache.len as usize);kvfree(erst_record_id_cache.entries as *mut core::ffi::c_void);erst_record_id_cache.entries=p;erst_record_id_cache.size=ns;}*erst_record_id_cache.entries.add(erst_record_id_cache.len as usize)=id;erst_record_id_cache.len+=1;return 1;}}
pub unsafe fn erst_get_record_id_next(pos:*mut i32,id:*mut u64)->i32{if erst_disable!=0{return -ENODEV;}BUG_ON(erst_record_id_cache.refcount==0);BUG_ON(*pos<0||*pos>erst_record_id_cache.len);mutex_lock(&mut erst_record_id_cache.lock);while *pos<erst_record_id_cache.len&&*erst_record_id_cache.entries.add(*pos as usize)==APEI_ERST_INVALID_RECORD_ID{*pos+=1;}if *pos<erst_record_id_cache.len{*id=*erst_record_id_cache.entries.add(*pos as usize);*pos+=1;mutex_unlock(&mut erst_record_id_cache.lock);return 0;}let rc=__erst_record_id_cache_add_one();if rc==1{*id=*erst_record_id_cache.entries.add(*pos as usize);*pos+=1;}else if rc==0{*pos=-1;*id=APEI_ERST_INVALID_RECORD_ID;}mutex_unlock(&mut erst_record_id_cache.lock);if rc<0{rc}else{0}}
pub unsafe fn erst_get_record_id_end(){BUG_ON(erst_disable!=0);mutex_lock(&mut erst_record_id_cache.lock);erst_record_id_cache.refcount-=1;BUG_ON(erst_record_id_cache.refcount<0);if erst_record_id_cache.refcount==0{let mut w=0;for i in 0..erst_record_id_cache.len{let v=*erst_record_id_cache.entries.add(i as usize);if v!=APEI_ERST_INVALID_RECORD_ID{*erst_record_id_cache.entries.add(w as usize)=v;w+=1;}}erst_record_id_cache.len=w;}mutex_unlock(&mut erst_record_id_cache.lock)}

pub unsafe fn erst_write(record:*const cper_record_header)->i32{if erst_disable!=0{return -ENODEV;}if memcmp((*record).signature.as_ptr(),CPER_SIG_RECORD.as_ptr(),CPER_SIG_SIZE)!=0{return -EINVAL;}if (*record).record_length>erst_erange.size{return -EINVAL;}let mut f=0;if !raw_spin_trylock_irqsave(&mut erst_lock,&mut f){return -EBUSY;}core::ptr::copy_nonoverlapping(record as *const u8,erst_erange.vaddr as *mut u8,(*record).record_length as usize);let p=erst_erange.vaddr.add(core::mem::offset_of!(cper_record_header,persistence_information));*(p as *mut u8)=b'E';*(p.add(1) as *mut u8)=b'R';let rc=__erst_write_to_storage(0);raw_spin_unlock_irqrestore(&mut erst_lock,f);rc}
pub unsafe fn erst_read(id:u64,record:*mut cper_record_header,buflen:usize)->isize{if erst_disable!=0{return -ENODEV as isize;}let mut f=0;raw_spin_lock_irqsave(&mut erst_lock,&mut f);let rc=__erst_read_from_storage(id,0);let n=if rc!=0{rc as isize}else{let p=erst_erange.vaddr as *const cper_record_header;let l=(*p).record_length as usize;if l<=buflen{core::ptr::copy_nonoverlapping(p as *const u8,record as *mut u8,l);}l as isize};raw_spin_unlock_irqrestore(&mut erst_lock,f);n}
pub unsafe fn erst_clear(id:u64)->i32{if erst_disable!=0{return -ENODEV;}let mut f=0;raw_spin_lock_irqsave(&mut erst_lock,&mut f);let rc=__erst_clear_from_storage(id);raw_spin_unlock_irqrestore(&mut erst_lock,f);rc}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
