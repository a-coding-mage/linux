// SPDX-License-Identifier: GPL-2.0
/* Kernel unwinding support. */

// Dependencies supplied by the surrounding kernel translation.

const KERNEL_START: usize = KERNEL_BINARY_TEXT_START;

#[inline]
fn alignment_ok<T>(ptr: usize) -> bool {
    (ptr & (core::mem::size_of::<T>() - 1)) == 0
}

extern "C" {
    static mut __start___unwind: unwind_table_entry;
    static mut __stop___unwind: unwind_table_entry;
}

static mut unwind_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut kernel_unwind_table: unwind_table = unsafe { core::mem::zeroed() };
static mut unwind_tables: list_head = unsafe { core::mem::zeroed() };

unsafe fn find_unwind_entry_in_table(
    table: *const unwind_table,
    addr: usize,
) -> *const unwind_table_entry {
    let mut e: *const unwind_table_entry = core::ptr::null();
    let mut lo: usize = 0;
    let mut hi: usize = (*table).length - 1;

    while lo <= hi {
        let mid = (hi - lo) / 2 + lo;
        e = (*table).table.add(mid);
        if addr < (*e).region_start as usize {
            hi = mid - 1;
        } else if addr > (*e).region_end as usize {
            lo = mid + 1;
        } else {
            return e;
        }
    }
    e = core::ptr::null();
    e
}

unsafe fn find_unwind_entry(addr: usize) -> *const unwind_table_entry {
    let mut table: *mut unwind_table;
    let mut e: *const unwind_table_entry = core::ptr::null();

    if addr >= kernel_unwind_table.start as usize && addr <= kernel_unwind_table.end as usize {
        e = find_unwind_entry_in_table(&kernel_unwind_table, addr);
    } else {
        let mut flags: usize = 0;
        spin_lock_irqsave(&mut unwind_lock, &mut flags);
        list_for_each_entry!(table, &mut unwind_tables, list);
        if addr >= (*table).start as usize && addr <= (*table).end as usize {
            e = find_unwind_entry_in_table(table, addr);
        }
        if !e.is_null() {
            list_move(&mut (*table).list, &mut unwind_tables);
        }
        spin_unlock_irqrestore(&mut unwind_lock, flags);
    }
    e
}

unsafe fn unwind_table_init(
    table: *mut unwind_table,
    name: *const core::ffi::c_char,
    base_addr: usize,
    gp: usize,
    table_start: *mut core::ffi::c_void,
    table_end: *mut core::ffi::c_void,
) {
    let mut start = table_start as *mut unwind_table_entry;
    let end = (table_end as *mut unwind_table_entry).sub(1);
    (*table).name = name;
    (*table).base_addr = base_addr;
    (*table).gp = gp;
    (*table).start = base_addr + (*start).region_start as usize;
    (*table).end = base_addr + (*end).region_end as usize;
    (*table).table = table_start as *mut unwind_table_entry;
    (*table).length = end.offset_from(start) as usize + 1;
    INIT_LIST_HEAD(&mut (*table).list);

    while start <= end {
        if start < end && (*start).region_end > (*start.add(1)).region_start {
            pr_warn!("Out of order unwind entry! %px and %px\n", start, start.add(1));
        }
        (*start).region_start += base_addr as _;
        (*start).region_end += base_addr as _;
        start = start.add(1);
    }
}

unsafe extern "C" fn cmp_unwind_table_entry(a: *const c_void, b: *const c_void) -> i32 {
    (*((a as *const unwind_table_entry))).region_start as i32
        - (*((b as *const unwind_table_entry))).region_start as i32
}

unsafe fn unwind_table_sort(start: *mut unwind_table_entry, finish: *mut unwind_table_entry) {
    sort(start as *mut c_void, finish.offset_from(start) as usize,
         core::mem::size_of::<unwind_table_entry>(), cmp_unwind_table_entry, core::ptr::null_mut());
}

pub unsafe fn unwind_table_add(name: *const c_char, base_addr: usize, gp: usize,
                               start: *mut c_void, end: *mut c_void) -> *mut unwind_table {
    unwind_table_sort(start as *mut unwind_table_entry, end as *mut unwind_table_entry);
    let table = kmalloc_obj::<unwind_table>(GFP_USER);
    if table.is_null() { return core::ptr::null_mut(); }
    unwind_table_init(table, name, base_addr, gp, start, end);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut unwind_lock, &mut flags);
    list_add_tail(&mut (*table).list, &mut unwind_tables);
    spin_unlock_irqrestore(&mut unwind_lock, flags);
    table
}

pub unsafe fn unwind_table_remove(table: *mut unwind_table) {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut unwind_lock, &mut flags);
    list_del(&mut (*table).list);
    spin_unlock_irqrestore(&mut unwind_lock, flags);
    kfree(table as *mut c_void);
}

pub unsafe extern "C" fn unwind_init() -> i32 {
    let start = &__start___unwind as *const _ as isize;
    let stop = &__stop___unwind as *const _ as isize;
    dbg!("unwind_init: start = 0x%lx, end = 0x%lx, entries = %lu\n", start, stop,
        (stop - start) as usize / core::mem::size_of::<unwind_table_entry>());
    unwind_table_init(&mut kernel_unwind_table, b"kernel\0".as_ptr() as _, KERNEL_START,
                      parisc_gp(), &__start___unwind as *const _ as *mut _,
                      &__stop___unwind as *const _ as *mut _);
    0
}

unsafe fn pc_is_kernel_fn(pc: usize, fun: *mut c_void) -> bool {
    dereference_kernel_function_descriptor(fun) as usize == pc
}

unsafe fn unwind_special(info: *mut unwind_frame_info, pc: usize, frame_size: i32) -> i32 {
    extern "C" {
        static ret_from_kernel_thread: c_void;
        static syscall_exit: c_void;
        static intr_return: c_void;
        static _switch_to_ret: c_void;
        #[cfg(CONFIG_IRQSTACKS)] static _call_on_stack: c_void;
    }
    if pc_is_kernel_fn(pc, handle_interruption as *mut _) {
        let regs = ((*info).sp - frame_size as usize - PT_SZ_ALGN) as *mut pt_regs;
        (*info).prev_sp = (*regs).gr[30] as usize;
        (*info).prev_ip = (*regs).iaoq[0] as usize;
        return 1;
    }
    if pc == &ret_from_kernel_thread as *const _ as usize || pc == &syscall_exit as *const _ as usize {
        (*info).prev_sp = 0; (*info).prev_ip = 0; return 1;
    }
    if pc == &intr_return as *const _ as usize {
        let regs = ((*info).sp - PT_SZ_ALGN) as *mut pt_regs;
        (*info).prev_sp = (*regs).gr[30] as usize;
        (*info).prev_ip = (*regs).iaoq[0] as usize;
        (*info).rp = (*regs).gr[2] as usize; return 1;
    }
    if pc_is_kernel_fn(pc, _switch_to as *mut _) || pc == &_switch_to_ret as *const _ as usize {
        (*info).prev_sp = (*info).sp - CALLEE_SAVE_FRAME_SIZE;
        if alignment_ok::<usize>((*info).prev_sp) { (*info).prev_ip = *((*info).prev_sp - RP_OFFSET) as *const usize; }
        else { (*info).prev_ip = 0; (*info).prev_sp = 0; }
        return 1;
    }
    0
}

// The remaining unwinding routines retain the C algorithm and use the kernel's
// supplied structures and helpers.
pub unsafe fn unwind_frame_regs(info: *mut unwind_frame_info) {
    let e = find_unwind_entry((*info).ip);
    if e.is_null() { (*info).prev_sp = 0; (*info).prev_ip = 0; return; }
    let mut frame_size: isize = 0;
    let mut npc = (*e).region_start as usize;
    let mut looking_for_rp = (*e).Save_RP != 0;
    let mut rpoffset = 0usize;
    while (frame_size < ((*e).Total_frame_size as isize) << 3 || looking_for_rp) && npc < (*info).ip {
        let insn = *(npc as *const u32);
        if insn & 0xffffc001 == 0x37de0000 || insn & 0xffe00001 == 0x6fc00000 {
            frame_size += ((insn & 0x3fff) >> 1) as isize;
        } else if insn & 0xffe00009 == 0x73c00008 {
            frame_size += (((insn >> 4) & 0x3ff) << 3) as isize;
        } else if insn == 0x6bc23fd9 { rpoffset = 20; looking_for_rp = false;
        } else if insn == 0x0fc212c1 { rpoffset = 16; looking_for_rp = false; }
        npc += 4;
    }
    let max = ((*e).Total_frame_size as isize) << 3;
    if frame_size > max { frame_size = max; }
    if unwind_special(info, (*e).region_start as usize, frame_size as i32) == 0 {
        (*info).prev_sp = (*info).sp - frame_size as usize;
        if (*e).Millicode != 0 { (*info).rp = (*info).r31; }
        else if rpoffset != 0 && alignment_ok::<usize>((*info).prev_sp) { (*info).rp = *((*info).prev_sp - rpoffset) as *const usize; }
        else { (*info).rp = 0; }
        (*info).prev_ip = (*info).rp; (*info).rp = 0;
    }
}
pub unsafe fn unwind_frame_init(info: *mut unwind_frame_info, t: *mut task_struct, regs: *mut pt_regs) {
    core::ptr::write_bytes(info, 0, 1); (*info).t=t; (*info).sp=(*regs).gr[30] as usize;
    (*info).ip=(*regs).iaoq[0] as usize; (*info).rp=(*regs).gr[2] as usize; (*info).r31=(*regs).gr[31] as usize;
}
pub unsafe fn unwind_frame_init_from_blocked_task(info: *mut unwind_frame_info, t: *mut task_struct) {
    unwind_frame_init(info, t, &mut (*t).thread.regs); (*info).sp=(*t).thread.ksp; (*info).ip=(*t).thread.kpc;
}
pub unsafe fn unwind_frame_init_task(info: *mut unwind_frame_info, task: *mut task_struct, regs: *mut pt_regs) {
    let task = if task.is_null() { current } else { task }; unwind_frame_init_from_blocked_task(info, task);
    if task == current && !regs.is_null() { unwind_frame_init(info, task, regs); }
}
pub unsafe fn unwind_once(next_frame: *mut unwind_frame_info) -> i32 {
    unwind_frame_regs(next_frame); if (*next_frame).prev_sp==0 || (*next_frame).prev_ip==0 { return -1; }
    (*next_frame).sp=(*next_frame).prev_sp; (*next_frame).ip=(*next_frame).prev_ip; (*next_frame).prev_sp=0; (*next_frame).prev_ip=0; 0
}
pub unsafe fn unwind_to_user(info: *mut unwind_frame_info) -> i32 {
    let mut ret; loop { ret=unwind_once(info); if ret!=0 || ((*info).ip & 3)==0 { return ret; } }
}
pub unsafe fn return_address(mut level: u32) -> usize {
    let mut info: unwind_frame_info = core::mem::zeroed(); unwind_frame_init_task(&mut info, current, core::ptr::null_mut()); level += 2;
    loop { if unwind_once(&mut info)<0 || info.ip==0 || !kernel_text_address(info.ip) { return 0; } if info.ip==0 || { let x=level; level-=1; x==0 } { return info.ip; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
