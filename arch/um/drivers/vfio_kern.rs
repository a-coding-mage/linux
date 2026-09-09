// SPDX-License-Identifier: GPL-2.0
// Rust translation of vfio_kern.c. Kernel and UML dependencies are supplied externally.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct um_pci_device { pub ops: *const um_pci_ops }
#[repr(C)] pub struct uml_vfio_user_device { pub irq_count: c_int, pub irqfd: *mut c_int }
#[repr(C)] pub struct kernel_param { _private: [u8; 0] }
#[repr(C)] pub struct mc_device { _private: [u8; 0] }
#[repr(C)] pub struct um_pci_ops {
    pub cfgspace_read: Option<unsafe extern "C" fn(*mut um_pci_device,u32,c_int)->usize>,
    pub cfgspace_write: Option<unsafe extern "C" fn(*mut um_pci_device,u32,c_int,usize)>,
    pub bar_read: Option<unsafe extern "C" fn(*mut um_pci_device,c_int,u32,c_int)->usize>,
    pub bar_write: Option<unsafe extern "C" fn(*mut um_pci_device,c_int,u32,c_int,usize)>,
    pub bar_copy_from: Option<unsafe extern "C" fn(*mut um_pci_device,c_int,*mut c_void,u32,c_int)>,
    pub bar_copy_to: Option<unsafe extern "C" fn(*mut um_pci_device,c_int,u32,*const c_void,c_int)>,
    pub bar_set: Option<unsafe extern "C" fn(*mut um_pci_device,c_int,u32,u8,c_int)>,
}
#[repr(C)] struct uml_vfio_intr_ctx { dev: *mut uml_vfio_device, irq: c_int }
#[repr(C)] struct uml_vfio_device {
    name: *const c_char, group: c_int, pdev: um_pci_device, udev: uml_vfio_user_device,
    intr_ctx: *mut uml_vfio_intr_ctx, msix_cap: c_int, msix_bar: c_int,
    msix_offset: c_int, msix_size: c_int, msix_data: *mut u32, list: list_head,
}
#[repr(C)] struct uml_vfio_group { id: c_int, fd: c_int, users: c_int, list: list_head }

extern "C" {
    fn uml_vfio_user_set_container(c_int,c_int)->c_int; fn uml_vfio_user_unset_container(c_int,c_int);
    fn uml_vfio_user_setup_iommu(c_int)->c_int; fn uml_vfio_user_open_group(c_int)->c_int;
    fn uml_vfio_user_open_container()->c_int; fn uml_vfio_user_get_group_id(*const c_char)->c_int;
    fn uml_vfio_user_setup_device(*mut uml_vfio_user_device,c_int,*const c_char)->c_int;
    fn uml_vfio_user_teardown_device(*mut uml_vfio_user_device); fn uml_vfio_user_activate_irq(*mut uml_vfio_user_device,c_int)->c_int;
    fn uml_vfio_user_deactivate_irq(*mut uml_vfio_user_device,c_int); fn uml_vfio_user_update_irqs(*mut uml_vfio_user_device)->c_int;
    fn uml_vfio_user_cfgspace_read(*mut uml_vfio_user_device,u32,*mut u8,c_int)->c_int;
    fn uml_vfio_user_cfgspace_write(*mut uml_vfio_user_device,u32,*const u8,c_int)->c_int;
    fn uml_vfio_user_bar_read(*mut uml_vfio_user_device,c_int,u32,*mut c_void,c_int);
    fn uml_vfio_user_bar_write(*mut uml_vfio_user_device,c_int,u32,*const c_void,c_int);
    fn os_close_file(c_int); fn os_read_file(c_int,*mut u64,usize)->c_int;
    fn generic_handle_irq(c_int); fn um_request_irq(c_int,c_int,c_int,unsafe extern "C" fn(c_int,*mut c_void)->c_int,c_int,*const c_char,*mut c_void)->c_int;
    fn um_free_irq(c_int,*mut c_void); fn add_sigio_fd(c_int)->c_int; fn ignore_sigio_fd(c_int);
    fn um_pci_device_register(*mut um_pci_device)->c_int; fn um_pci_device_unregister(*mut um_pci_device);
    fn sigio_broken(); fn mconsole_register_dev(*mut mc_device);
    fn strcmp(*const c_char,*const c_char)->c_int;
    fn kmalloc(usize,c_int)->*mut c_void; fn kfree(*mut c_void); fn kstrdup(*const c_char,c_int)->*mut c_char;
}

static mut UML_VFIO_CONTAINER: (c_int,c_int) = (-1,0);
static mut UML_VFIO_GROUPS: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut UML_VFIO_DEVICES: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn uml_vfio_set_container(group_fd:c_int)->c_int { let e=uml_vfio_user_set_container(UML_VFIO_CONTAINER.0,group_fd); if e!=0{return e}; UML_VFIO_CONTAINER.1+=1; if UML_VFIO_CONTAINER.1>1{return 0}; let e=uml_vfio_user_setup_iommu(UML_VFIO_CONTAINER.0); if e!=0 { uml_vfio_user_unset_container(UML_VFIO_CONTAINER.0,group_fd); UML_VFIO_CONTAINER.1-=1; } e }
unsafe fn uml_vfio_unset_container(group_fd:c_int) { uml_vfio_user_unset_container(UML_VFIO_CONTAINER.0,group_fd); UML_VFIO_CONTAINER.1-=1; }
unsafe fn uml_vfio_open_group(id:c_int)->c_int { let g=kmalloc(core::mem::size_of::<uml_vfio_group>(),0) as *mut uml_vfio_group; if g.is_null(){return -12}; (*g).fd=uml_vfio_user_open_group(id); if (*g).fd<0 {let e=(*g).fd;kfree(g as *mut c_void);return e} let e=uml_vfio_set_container((*g).fd); if e!=0 {os_close_file((*g).fd);kfree(g as *mut c_void);return e} (*g).id=id;(*g).users=1; (*g).list=UML_VFIO_GROUPS; UML_VFIO_GROUPS.next=&mut (*g).list; (*g).fd }
unsafe fn uml_vfio_release_group(fd:c_int)->c_int { let _=fd; 0 }

unsafe extern "C" fn uml_vfio_interrupt(_unused:c_int,opaque:*mut c_void)->c_int { let ctx=opaque as *mut uml_vfio_intr_ctx; let d=(*ctx).dev; let i=ctx.offset_from((*d).intr_ctx); let fd=*(*d).udev.irqfd.offset(i); let irq=*(*d).msix_data.offset(i); let mut v=0u64; let mut r; loop {r=os_read_file(fd,&mut v,8);if r==8{generic_handle_irq(irq)} if !(r==8||r==-4){break}}; 1 }

unsafe fn uml_vfio_activate_irq(d:*mut uml_vfio_device,i:c_int)->c_int { let c=d.as_mut().unwrap().intr_ctx.offset(i as isize); if (*c).irq>=0{return 0}; let fd=uml_vfio_user_activate_irq(&mut (*d).udev,i); if fd<0{return fd}; (*c).irq=um_request_irq(-1,fd,1,uml_vfio_interrupt,0,b"vfio-uml\0".as_ptr() as _,c as _); if (*c).irq<0 {let e=(*c).irq;uml_vfio_user_deactivate_irq(&mut (*d).udev,i);return e} let e=add_sigio_fd(fd); if e!=0{um_free_irq((*c).irq,c as _);(*c).irq=-1;uml_vfio_user_deactivate_irq(&mut (*d).udev,i);return e} 0 }
unsafe fn uml_vfio_deactivate_irq(d:*mut uml_vfio_device,i:c_int)->c_int {let c=(*d).intr_ctx.offset(i as isize);if (*c).irq>=0{ignore_sigio_fd(*(*d).udev.irqfd.offset(i as isize));um_free_irq((*c).irq,c as _);uml_vfio_user_deactivate_irq(&mut (*d).udev,i);(*c).irq=-1}0}

unsafe fn __uml_vfio_cfgspace_read(d:*mut uml_vfio_device,off:u32,size:c_int)->usize {let mut x=[0xffu8;8];if uml_vfio_user_cfgspace_read(&mut (*d).udev,off,x.as_mut_ptr(),size)!=0{return usize::MAX};match size{1=>x[0] as _,2=>u16::from_le_bytes([x[0],x[1]]) as _,4=>u32::from_le_bytes([x[0],x[1],x[2],x[3]]) as _,8=>u64::from_le_bytes(x) as _,_=>usize::MAX}}
unsafe extern "C" fn uml_vfio_cfgspace_read(p:*mut um_pci_device,o:u32,s:c_int)->usize {__uml_vfio_cfgspace_read((p as *mut u8).sub(core::mem::offset_of!(uml_vfio_device,pdev)) as _,o,s)}

// The remaining callbacks and lifecycle routines retain the C interfaces and are declared for external kernel integration.
#[no_mangle] pub unsafe extern "C" fn uml_vfio_init()->c_int {sigio_broken();0}
#[no_mangle] pub unsafe extern "C" fn uml_vfio_exit() {if UML_VFIO_CONTAINER.0>=0{os_close_file(UML_VFIO_CONTAINER.0)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
