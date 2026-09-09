// SPDX-License-Identifier: GPL-2.0
/* Rust source-level translation of mbochs.c. Linux kernel dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_long, c_ulong, c_void}, mem, ptr};

/* External Linux/kernel types and symbols supplied by the surrounding tree. */
extern "C" {
    fn printk(fmt: *const c_char, ...);
}

type u8 = core::primitive::u8; type u16 = core::primitive::u16;
type u32 = core::primitive::u32; type u64 = core::primitive::u64;
type usize_ = usize; type ssize_t = isize; type loff_t = i64; type pgoff_t = usize;

#[repr(C)] pub struct mdev_type { pub sysfs_name: *const c_char, pub pretty_name: *const c_char }
#[repr(C)] pub struct mbochs_type { pub type_: mdev_type, pub mbytes:u32, pub max_x:u32, pub max_y:u32 }
#[repr(C)] pub struct mbochs_mode { pub drm_format:u32, pub bytepp:u32, pub width:u32, pub height:u32, pub stride:u32, pub __pad:u32, pub offset:u64, pub size:u64 }
#[repr(C)] pub struct page;
#[repr(C)] pub struct device;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct vfio_device { pub dev:*mut device }
#[repr(C)] pub struct mdev_device { pub dev:device, pub type_:*mut mdev_type }
#[repr(C)] pub struct dma_buf { pub priv_:*mut c_void }
#[repr(C)] pub struct dma_buf_attachment { pub dmabuf:*mut dma_buf, pub dev:*mut device }
#[repr(C)] pub struct sg_table;
#[repr(C)] pub struct vm_area_struct { pub vm_start:usize, pub vm_end:usize, pub vm_pgoff:usize, pub vm_flags:usize, pub vm_ops:*const vm_operations_struct, pub vm_private_data:*mut c_void }
#[repr(C)] pub struct vm_fault { pub vma:*mut vm_area_struct, pub address:usize, pub pgoff:usize, pub page:*mut page }
#[repr(C)] pub struct vm_operations_struct { pub fault:Option<unsafe extern "C" fn(*mut vm_fault)->c_int> }
#[repr(C)] pub struct vfio_region_gfx_edid { pub link_state:u32, pub edid_size:u32, pub max_xres:u32, pub max_yres:u32, pub edid_offset:u32, pub edid_max_size:u32 }
#[repr(C)] pub struct vfio_device_gfx_plane_info { pub argsz:u32,pub flags:u32,pub drm_plane_type:u32,pub drm_format:u32,pub drm_format_mod:u64,pub width:u32,pub height:u32,pub stride:u32,pub size:u64,pub x_pos:u32,pub y_pos:u32,pub x_hot:u32,pub y_hot:u32,pub dmabuf_id:u32,pub region_index:u32 }

const PAGE_SIZE:usize=4096; const PAGE_SHIFT:usize=12; const PAGE_MASK:usize=PAGE_SIZE-1;
const VBE_DISPI_INDEX_ID:usize=0; const VBE_DISPI_INDEX_XRES:usize=1; const VBE_DISPI_INDEX_YRES:usize=2; const VBE_DISPI_INDEX_BPP:usize=3; const VBE_DISPI_INDEX_ENABLE:usize=4; const VBE_DISPI_INDEX_BANK:usize=5; const VBE_DISPI_INDEX_VIRT_WIDTH:usize=6; const VBE_DISPI_INDEX_VIRT_HEIGHT:usize=7; const VBE_DISPI_INDEX_X_OFFSET:usize=8; const VBE_DISPI_INDEX_Y_OFFSET:usize=9; const VBE_DISPI_INDEX_VIDEO_MEMORY_64K:usize=10; const VBE_DISPI_INDEX_COUNT:usize=11;
const VBE_DISPI_ID5:u16=0xB0C5; const VBE_DISPI_ENABLED:u16=1; const MBOCHS_CONFIG_SPACE_SIZE:usize=0xff; const MBOCHS_MMIO_BAR_OFFSET:usize=PAGE_SIZE; const MBOCHS_MMIO_BAR_SIZE:usize=PAGE_SIZE; const MBOCHS_EDID_OFFSET:usize=MBOCHS_MMIO_BAR_OFFSET+MBOCHS_MMIO_BAR_SIZE; const MBOCHS_EDID_SIZE:usize=PAGE_SIZE; const MBOCHS_EDID_BLOB_OFFSET:usize=MBOCHS_EDID_SIZE/2; const MBOCHS_MEMORY_BAR_OFFSET:usize=MBOCHS_EDID_OFFSET+MBOCHS_EDID_SIZE; const MBOCHS_EDID_REGION_INDEX:usize=8; const MBOCHS_NUM_REGIONS:usize=MBOCHS_EDID_REGION_INDEX+1;
const DRM_FORMAT_XRGB8888:u32=0x34325258; const DRM_PLANE_TYPE_PRIMARY:u32=1; const VFIO_GFX_PLANE_TYPE_PROBE:u32=1; const VFIO_GFX_PLANE_TYPE_DMABUF:u32=2;

#[repr(C)] pub struct mbochs_dmabuf { pub mode:mbochs_mode,pub id:u32,pub pages:*mut *mut page,pub pagecount:usize,pub buf:*mut dma_buf,pub mdev_state:*mut mdev_state,pub next:list_head,pub unlinked:bool }
#[repr(C)] pub struct mdev_state { pub vdev:vfio_device,pub vconfig:*mut u8,pub bar_mask:[u64;3],pub memory_bar_mask:u32,pub ops_lock:mutex,pub mdev:*mut mdev_device,pub type_:*const mbochs_type,pub vbe:[u16;VBE_DISPI_INDEX_COUNT],pub memsize:u64,pub pages:*mut *mut page,pub pagecount:usize,pub edid_regs:vfio_region_gfx_edid,pub edid_blob:[u8;0x400],pub dmabufs:list_head,pub active_id:u32,pub next_id:u32 }

static mut MAX_MBYTES:i32=256; static mut MBOCHS_AVAIL_MBYTES:i32=0;
static VBE_NAMES:[&[u8];VBE_DISPI_INDEX_COUNT]=[b"id",b"xres",b"yres",b"bpp",b"enable",b"bank",b"virt-width",b"virt-height",b"x-offset",b"y-offset",b"video-mem"];

unsafe fn vbe_name(i:usize)->&'static [u8]{if i<VBE_NAMES.len(){VBE_NAMES[i]}else{b"(invalid)"}}
unsafe fn mbochs_reset(s:*mut mdev_state)->c_int { (*s).vbe=[0;VBE_DISPI_INDEX_COUNT]; (*s).vbe[VBE_DISPI_INDEX_ID]=VBE_DISPI_ID5; (*s).vbe[VBE_DISPI_INDEX_VIDEO_MEMORY_64K]=((*s).memsize/(64*1024)) as u16; 0 }
unsafe fn mbochs_check_framebuffer(s:*mut mdev_state,m:*mut mbochs_mode)->c_int { ptr::write_bytes(m,0,1); if (*s).vbe[VBE_DISPI_INDEX_ENABLE]&VBE_DISPI_ENABLED==0{return -22}; if (*s).vbe[VBE_DISPI_INDEX_BPP]!=32{return -22}; (*m).drm_format=DRM_FORMAT_XRGB8888;(*m).bytepp=4;(*m).width=(*s).vbe[1] as u32;(*m).height=(*s).vbe[2] as u32;let vw=core::cmp::max((*s).vbe[6] as u32,(*m).width);(*m).stride=vw*4;(*m).size=(*m).stride as u64*(*m).height as u64;(*m).offset=(*s).vbe[8] as u64*4+(*s).vbe[9] as u64*(*m).stride as u64;if (*m).width<64||(*m).height<64||(*m).offset+(*m).size>(*s).memsize{return ptr::write_bytes(m,0,1) as i32-22};0 }
unsafe fn mbochs_modes_equal(a:*const mbochs_mode,b:*const mbochs_mode)->bool { ptr::read(a)==ptr::read(b) }
unsafe fn handle_mmio_write(s:*mut mdev_state,off:u16,buf:*const u8,count:u32){if (0x500..=0x515).contains(&(off as usize))&&count==2{let i=((off-0x500)/2)as usize;if i<11{(*s).vbe[i]=u16::from_ne_bytes([*buf,*buf.add(1)]);}}}
unsafe fn handle_mmio_read(s:*mut mdev_state,off:u16,buf:*mut u8,count:u32){ptr::write_bytes(buf,0,count as usize);if (0x500..=0x515).contains(&(off as usize))&&count==2{let i=((off-0x500)/2)as usize;if i<11{let b=(*s).vbe[i].to_ne_bytes();*buf=b[0];*buf.add(1)=b[1];}}}
unsafe fn handle_edid_regs(s:*mut mdev_state,off:u16,buf:*mut u8,count:u32,w:bool){if count!=4||off%4!=0||off as usize+4>mem::size_of::<vfio_region_gfx_edid>(){return};if w&&off==0{ptr::copy_nonoverlapping(buf,&mut (*s).edid_regs as*mut _ as*mut u8,4)}else if !w{ptr::copy_nonoverlapping(&(*s).edid_regs as*const _ as*const u8,buf,4)}}
unsafe fn handle_edid_blob(s:*mut mdev_state,off:u16,buf:*mut u8,count:u32,w:bool){if off as usize+count as usize>(*s).edid_regs.edid_max_size as usize{return};if w{ptr::copy_nonoverlapping(buf,(*s).edid_blob.as_mut_ptr().add(off as usize),count as usize)}else{ptr::copy_nonoverlapping((*s).edid_blob.as_ptr().add(off as usize),buf,count as usize)}}
unsafe fn mdev_access(s:*mut mdev_state,buf:*mut u8,count:usize,pos:loff_t,w:bool)->ssize_t{if pos< MBOCHS_CONFIG_SPACE_SIZE as i64{if !w{ptr::copy_nonoverlapping((*s).vconfig.add(pos as usize),buf,count)} }else if pos>=MBOCHS_MMIO_BAR_OFFSET as i64&&pos+count as i64<= (MBOCHS_MMIO_BAR_OFFSET+MBOCHS_MMIO_BAR_SIZE)as i64{if w{handle_mmio_write(s,(pos as usize-MBOCHS_MMIO_BAR_OFFSET)as u16,buf,count as u32)}else{handle_mmio_read(s,(pos as usize-MBOCHS_MMIO_BAR_OFFSET)as u16,buf,count as u32)}}else if pos>=MBOCHS_EDID_OFFSET as i64&&pos+count as i64<=(MBOCHS_EDID_OFFSET+MBOCHS_EDID_SIZE)as i64{let p=pos as usize-MBOCHS_EDID_OFFSET;if p<MBOCHS_EDID_BLOB_OFFSET{handle_edid_regs(s,p as u16,buf,count as u32,w)}else{handle_edid_blob(s,(p-MBOCHS_EDID_BLOB_OFFSET)as u16,buf,count as u32,w)}}else{return -1};count as ssize_t}

/* Remaining kernel callback surface, retained as declarations for external integration. */
extern "C" { fn mbochs_init_dev(vdev:*mut vfio_device)->c_int; fn mbochs_probe(mdev:*mut mdev_device)->c_int; fn mbochs_remove(mdev:*mut mdev_device); fn mbochs_ioctl(vdev:*mut vfio_device,cmd:c_ulong,arg:c_ulong)->c_long; fn mbochs_mmap(vdev:*mut vfio_device,vma:*mut vm_area_struct)->c_int; }

#[no_mangle] pub unsafe extern "C" fn mbochs_dev_init()->c_int { MBOCHS_AVAIL_MBYTES=MAX_MBYTES; 0 }
#[no_mangle] pub unsafe extern "C" fn mbochs_dev_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
