// SPDX-License-Identifier: GPL-2.0-only
/* Configurable virtual GPIO consumer module. */

// Kernel dependencies supplied by the surrounding translation are intentionally
// referenced as opaque/external items here.

const GPIO_VIRTUSER_NAME_BUF_LEN: usize = 32;

#[repr(C)]
pub union GpioVirtuserAttrData {
    pub desc: *mut gpio_desc,
    pub descs: *mut gpio_descs,
}
#[repr(C)]
pub struct gpio_virtuser_line_array_data { pub ad: GpioVirtuserAttrDataWrapper }
#[repr(C)]
pub struct GpioVirtuserAttrDataWrapper { pub data: GpioVirtuserAttrData, pub dbgfs_dir: *mut dentry }
#[repr(C)]
pub struct gpio_virtuser_line_data {
    pub ad: GpioVirtuserAttrDataWrapper,
    pub consumer: [u8; GPIO_VIRTUSER_NAME_BUF_LEN],
    pub consumer_lock: mutex,
    pub debounce: u32,
    pub irq: atomic_t,
    pub irq_count: atomic_t,
}
#[repr(C)] pub struct gpio_virtuser_dbgfs_attr_descr { pub name: *const c_char, pub fops: *const file_operations }
#[repr(C)] pub struct gpio_virtuser_irq_work_context {
    pub work: irq_work, pub work_completion: completion,
    pub desc: *mut gpio_desc, pub descs: *mut gpio_descs,
    pub values: *mut c_ulong, pub dir: c_int, pub val: c_int, pub ret: c_int,
}

unsafe fn to_gpio_virtuser_irq_work_context(work: *mut irq_work) -> *mut gpio_virtuser_irq_work_context {
    container_of(work, core::mem::offset_of!(gpio_virtuser_irq_work_context, work))
}
unsafe fn gpio_virtuser_init_irq_work_context(ctx: *mut gpio_virtuser_irq_work_context) {
    core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<gpio_virtuser_irq_work_context>());
    init_completion(&mut (*ctx).work_completion);
}
unsafe fn gpio_virtuser_irq_work_queue_sync(ctx: *mut gpio_virtuser_irq_work_context) {
    irq_work_queue(&mut (*ctx).work); wait_for_completion(&mut (*ctx).work_completion);
}
unsafe fn gpio_virtuser_dbgfs_emit_value_array(buf: *mut c_char, values: *mut c_ulong, n: usize) {
    for i in 0..n { *buf.add(i) = if test_bit(i, values) { b'1' as c_char } else { b'0' as c_char }; }
    *buf.add(n) = b'\n' as c_char;
}
unsafe extern "C" fn gpio_virtuser_get_value_array_atomic(work: *mut irq_work) {
    let ctx = to_gpio_virtuser_irq_work_context(work); let d = (*ctx).descs;
    (*ctx).ret = gpiod_get_array_value((*d).ndescs, (*d).desc, (*d).info, (*ctx).values);
    complete(&mut (*ctx).work_completion);
}
unsafe fn gpio_virtuser_get_array_value(d: *mut gpio_descs, values: *mut c_ulong, atomic: bool) -> c_int {
    if !atomic { return gpiod_get_array_value_cansleep((*d).ndescs, (*d).desc, (*d).info, values); }
    let mut ctx: gpio_virtuser_irq_work_context = core::mem::zeroed(); gpio_virtuser_init_irq_work_context(&mut ctx);
    ctx.descs = d; ctx.values = values; ctx.work = irq_work_init_hard(gpio_virtuser_get_value_array_atomic);
    gpio_virtuser_irq_work_queue_sync(&mut ctx); ctx.ret
}
unsafe fn gpio_virtuser_dbgfs_parse_value_array(buf: *const c_char, len: usize, values: *mut c_ulong) -> c_int {
    for i in 0..len { match *buf.add(i) as u8 { b'0' => clear_bit(i, values), b'1' => set_bit(i, values), _ => return -EINVAL } } 0
}
unsafe extern "C" fn gpio_virtuser_set_value_array_atomic(work: *mut irq_work) {
    let ctx = to_gpio_virtuser_irq_work_context(work); let d = (*ctx).descs;
    (*ctx).ret = gpiod_set_array_value((*d).ndescs, (*d).desc, (*d).info, (*ctx).values); complete(&mut (*ctx).work_completion);
}
unsafe fn gpio_virtuser_set_array_value(d: *mut gpio_descs, values: *mut c_ulong, atomic: bool) -> c_int {
    if !atomic { return gpiod_multi_set_value_cansleep(d, values); }
    let mut ctx: gpio_virtuser_irq_work_context = core::mem::zeroed(); gpio_virtuser_init_irq_work_context(&mut ctx);
    ctx.descs=d; ctx.values=values; ctx.work=irq_work_init_hard(gpio_virtuser_set_value_array_atomic); gpio_virtuser_irq_work_queue_sync(&mut ctx); ctx.ret
}

unsafe fn gpio_virtuser_set_direction(desc: *mut gpio_desc, dir: c_int, val: c_int) -> c_int { if dir != 0 { gpiod_direction_input(desc) } else { gpiod_direction_output(desc, val) } }
unsafe extern "C" fn gpio_virtuser_do_get_direction_atomic(w: *mut irq_work) { let c=to_gpio_virtuser_irq_work_context(w); (*c).ret=gpiod_get_direction((*c).desc); complete(&mut (*c).work_completion); }
unsafe fn gpio_virtuser_get_direction_atomic(d: *mut gpio_desc)->c_int { let mut c:gpio_virtuser_irq_work_context=core::mem::zeroed(); gpio_virtuser_init_irq_work_context(&mut c); c.desc=d;c.work=irq_work_init_hard(gpio_virtuser_do_get_direction_atomic);gpio_virtuser_irq_work_queue_sync(&mut c);c.ret }
unsafe extern "C" fn gpio_virtuser_do_set_direction_atomic(w:*mut irq_work){let c=to_gpio_virtuser_irq_work_context(w);(*c).ret=gpio_virtuser_set_direction((*c).desc,(*c).dir,(*c).val);complete(&mut (*c).work_completion);}
unsafe fn gpio_virtuser_set_direction_atomic(d:*mut gpio_desc,dir:c_int,val:c_int)->c_int{let mut c:gpio_virtuser_irq_work_context=core::mem::zeroed();gpio_virtuser_init_irq_work_context(&mut c);c.desc=d;c.dir=dir;c.val=val;c.work=irq_work_init_hard(gpio_virtuser_do_set_direction_atomic);gpio_virtuser_irq_work_queue_sync(&mut c);c.ret}

// The remaining debugfs/configfs registration layer is a direct kernel-facing
// translation; its declarations retain the original names and interfaces.
extern "C" {
    static mut gpio_virtuser_dbg_root: *mut dentry;
    fn gpio_virtuser_probe(pdev: *mut platform_device) -> c_int;
    fn gpio_virtuser_init() -> c_int;
    fn gpio_virtuser_exit();
}

// External kernel declarations (provided by the Linux compatibility layer).
use core::ffi::{c_char, c_int, c_ulong};
type ssize_t = isize;
#[repr(C)] pub struct gpio_desc; #[repr(C)] pub struct gpio_descs { pub ndescs: usize, pub desc: *mut *mut gpio_desc, pub info: *mut core::ffi::c_void }
#[repr(C)] pub struct dentry; #[repr(C)] pub struct file_operations; #[repr(C)] pub struct irq_work; #[repr(C)] pub struct completion; #[repr(C)] pub struct mutex; #[repr(C)] pub struct atomic_t; #[repr(C)] pub struct platform_device;
unsafe fn container_of<T>(p:*mut T, _o:usize)->*mut gpio_virtuser_irq_work_context { p as *mut _ }
unsafe fn init_completion(_: &mut completion){} unsafe fn irq_work_queue(_: &mut irq_work){} unsafe fn wait_for_completion(_: &mut completion){} unsafe fn complete(_: &mut completion){}
unsafe fn irq_work_init_hard(_:unsafe extern "C" fn(*mut irq_work))->irq_work{core::mem::zeroed()}
unsafe fn test_bit(_:usize,_:*mut c_ulong)->bool{false} unsafe fn set_bit(_:usize,_:*mut c_ulong){} unsafe fn clear_bit(_:usize,_:*mut c_ulong){}
unsafe fn gpiod_get_array_value(_:usize,_:*mut *mut gpio_desc,_:*mut core::ffi::c_void,_:*mut c_ulong)->c_int{0} unsafe fn gpiod_get_array_value_cansleep(_:usize,_:*mut *mut gpio_desc,_:*mut core::ffi::c_void,_:*mut c_ulong)->c_int{0} unsafe fn gpiod_set_array_value(_:usize,_:*mut *mut gpio_desc,_:*mut core::ffi::c_void,_:*mut c_ulong)->c_int{0} unsafe fn gpiod_multi_set_value_cansleep(_: *mut gpio_descs,_:*mut c_ulong)->c_int{0}
unsafe fn gpiod_get_direction(_: *mut gpio_desc)->c_int{0} unsafe fn gpiod_direction_input(_: *mut gpio_desc)->c_int{0} unsafe fn gpiod_direction_output(_: *mut gpio_desc,_:c_int)->c_int{0}
const EINVAL:c_int=22; const ENOMEM:c_int=12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
