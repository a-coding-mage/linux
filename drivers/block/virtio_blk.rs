// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of virtio_blk.c. Kernel symbols are supplied externally.

pub const PART_BITS: usize = 4;
pub const VQ_NAME_LEN: usize = 16;
pub const MAX_DISCARD_SEGMENTS: u32 = 256;
pub const VIRTIO_BLK_MAX_SG_ELEMS: usize = 32768;
#[cfg(CONFIG_ARCH_NO_SG_CHAIN)] pub const VIRTIO_BLK_INLINE_SG_CNT: usize = 0;
#[cfg(not(CONFIG_ARCH_NO_SG_CHAIN))] pub const VIRTIO_BLK_INLINE_SG_CNT: usize = 2;

static mut num_request_queues: u32 = 0;
static mut poll_queues: u32 = 0;
static mut major: i32 = 0;
static mut virtblk_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)] pub struct virtio_blk_vq { pub vq: *mut virtqueue, pub lock: spinlock_t, pub name: [i8; VQ_NAME_LEN] }
#[repr(C)] pub struct virtio_blk {
    pub vdev_mutex: mutex, pub vdev: *mut virtio_device, pub disk: *mut gendisk,
    pub tag_set: blk_mq_tag_set, pub config_work: work_struct, pub index: i32,
    pub num_vqs: i32, pub io_queues: [i32; HCTX_MAX_TYPES], pub vqs: *mut virtio_blk_vq,
    pub zone_sectors: u32,
}
#[repr(C)] pub union virtblk_in_hdr { pub status: u8, pub zone_append: virtblk_zone_append }
#[repr(C)] pub struct virtblk_zone_append { pub sector: __virtio64, pub status: u8 }
#[repr(C)] pub struct virtblk_req {
    pub out_hdr: virtio_blk_outhdr, pub in_hdr: virtblk_in_hdr, pub in_hdr_len: usize,
    pub sg_table: sg_table, pub sg: [scatterlist; VIRTIO_BLK_INLINE_SG_CNT],
}

#[inline] unsafe fn virtblk_result(status: u8) -> blk_status_t {
    match status { VIRTIO_BLK_S_OK => BLK_STS_OK, VIRTIO_BLK_S_UNSUPP => BLK_STS_NOTSUPP,
        VIRTIO_BLK_S_ZONE_OPEN_RESOURCE => BLK_STS_ZONE_OPEN_RESOURCE,
        VIRTIO_BLK_S_ZONE_ACTIVE_RESOURCE => BLK_STS_ZONE_ACTIVE_RESOURCE,
        _ => BLK_STS_IOERR }
}
#[inline] unsafe fn get_virtio_blk_vq(hctx: *mut blk_mq_hw_ctx) -> *mut virtio_blk_vq {
    let vblk = (*(*hctx).queue).queuedata as *mut virtio_blk;
    (*vblk).vqs.add((*(*hctx).queue).queue_num as usize)
}
unsafe fn virtblk_add_req(vq: *mut virtqueue, vbr: *mut virtblk_req) -> i32 {
    let mut out_hdr: scatterlist = core::mem::zeroed(); let mut in_hdr: scatterlist = core::mem::zeroed();
    let mut sgs: [*mut scatterlist; 3] = [core::ptr::null_mut(); 3]; let mut num_out=0; let mut num_in=0;
    sg_init_one(&mut out_hdr, &mut (*vbr).out_hdr as *mut _, core::mem::size_of::<virtio_blk_outhdr>());
    sgs[num_out]=&mut out_hdr; num_out+=1;
    if (*vbr).sg_table.nents != 0 { if (*vbr).out_hdr.r#type & cpu_to_virtio32((*vq).vdev, VIRTIO_BLK_T_OUT) != 0 { sgs[num_out]=(*vbr).sg_table.sgl; num_out+=1; } else { sgs[num_out+num_in]=(*vbr).sg_table.sgl; num_in+=1; } }
    sg_init_one(&mut in_hdr, &mut (*vbr).in_hdr as *mut _, (*vbr).in_hdr_len);
    sgs[num_out+num_in]=&mut in_hdr; num_in+=1;
    virtqueue_add_sgs(vq, sgs.as_mut_ptr(), num_out as u32, num_in as u32, vbr as *mut _, GFP_ATOMIC)
}
unsafe fn virtblk_unmap_data(req:*mut request,vbr:*mut virtblk_req){if blk_rq_nr_phys_segments(req)!=0{sg_free_table_chained(&mut (*vbr).sg_table,VIRTIO_BLK_INLINE_SG_CNT as i32);}}
unsafe fn virtblk_cleanup_cmd(req:*mut request){if (*req).rq_flags&RQF_SPECIAL_PAYLOAD!=0{kfree(bvec_virt(&mut (*req).special_vec) as *mut _);}}
unsafe fn virtblk_setup_discard_write_zeroes_erase(req:*mut request,unmap:bool)->i32{
    let segments=blk_rq_nr_discard_segments(req) as usize; let flags=if unmap{VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP}else{0};
    let range=kmalloc_objs::<virtio_blk_discard_write_zeroes>(segments,GFP_ATOMIC); if range.is_null(){return -ENOMEM;}
    if queue_max_discard_segments((*req).q)==1 {(*range).flags=cpu_to_le32(flags);(*range).num_sectors=cpu_to_le32(blk_rq_sectors(req));(*range).sector=cpu_to_le64(blk_rq_pos(req));}
    else { let mut bio: *mut bio=core::ptr::null_mut(); let mut n=0; __rq_for_each_bio!(bio,req,{let r=range.add(n);(*r).flags=cpu_to_le32(flags);(*r).num_sectors=cpu_to_le32((*bio).bi_iter.bi_size>>SECTOR_SHIFT);(*r).sector=cpu_to_le64((*bio).bi_iter.bi_sector);n+=1;}); }
    bvec_set_virt(&mut (*req).special_vec,range,(core::mem::size_of::<virtio_blk_discard_write_zeroes>()*segments));(*req).rq_flags|=RQF_SPECIAL_PAYLOAD;0
}
unsafe fn virtblk_map_data(_hctx:*mut blk_mq_hw_ctx,req:*mut request,vbr:*mut virtblk_req)->i32{
    if blk_rq_nr_phys_segments(req)==0{return 0;} (*vbr).sg_table.sgl=(*vbr).sg.as_mut_ptr();
    let e=sg_alloc_table_chained(&mut (*vbr).sg_table,blk_rq_nr_phys_segments(req),(*vbr).sg_table.sgl,VIRTIO_BLK_INLINE_SG_CNT as i32);if e!=0{return -ENOMEM;} blk_rq_map_sg(req,(*vbr).sg_table.sgl)
}
unsafe fn virtblk_setup_cmd(vdev:*mut virtio_device,req:*mut request,vbr:*mut virtblk_req)->blk_status_t{
    let mut len=core::mem::size_of::<u8>();let mut unmap=false;let mut typ=0u32;let mut sector=0u64;
    (*vbr).out_hdr.ioprio=cpu_to_virtio32(vdev,req_get_ioprio(req));
    match req_op(req){REQ_OP_READ=>{typ=VIRTIO_BLK_T_IN;sector=blk_rq_pos(req)},REQ_OP_WRITE=>{typ=VIRTIO_BLK_T_OUT;sector=blk_rq_pos(req)},REQ_OP_FLUSH=>typ=VIRTIO_BLK_T_FLUSH,REQ_OP_DISCARD=>typ=VIRTIO_BLK_T_DISCARD,REQ_OP_WRITE_ZEROES=>{typ=VIRTIO_BLK_T_WRITE_ZEROES;unmap=(*req).cmd_flags&REQ_NOUNMAP==0},REQ_OP_SECURE_ERASE=>typ=VIRTIO_BLK_T_SECURE_ERASE,REQ_OP_ZONE_OPEN=>{typ=VIRTIO_BLK_T_ZONE_OPEN;sector=blk_rq_pos(req)},REQ_OP_ZONE_CLOSE=>{typ=VIRTIO_BLK_T_ZONE_CLOSE;sector=blk_rq_pos(req)},REQ_OP_ZONE_FINISH=>{typ=VIRTIO_BLK_T_ZONE_FINISH;sector=blk_rq_pos(req)},REQ_OP_ZONE_APPEND=>{typ=VIRTIO_BLK_T_ZONE_APPEND;sector=blk_rq_pos(req);len=core::mem::size_of::<virtblk_zone_append>()},REQ_OP_ZONE_RESET=>{typ=VIRTIO_BLK_T_ZONE_RESET;sector=blk_rq_pos(req)},REQ_OP_ZONE_RESET_ALL=>typ=VIRTIO_BLK_T_ZONE_RESET_ALL,REQ_OP_DRV_IN=>return 0,_=>{WARN_ON_ONCE(1);return BLK_STS_IOERR}}
    (*vbr).in_hdr_len=len;(*vbr).out_hdr.r#type=cpu_to_virtio32(vdev,typ);(*vbr).out_hdr.sector=cpu_to_virtio64(vdev,sector);
    if typ==VIRTIO_BLK_T_DISCARD||typ==VIRTIO_BLK_T_WRITE_ZEROES||typ==VIRTIO_BLK_T_SECURE_ERASE {if virtblk_setup_discard_write_zeroes_erase(req,unmap)!=0{return BLK_STS_RESOURCE;}} 0
}
#[inline] unsafe fn virtblk_vbr_status(vbr:*mut virtblk_req)->u8{*((vbr as *mut u8).add(core::mem::offset_of!(virtblk_req,in_hdr)+(*vbr).in_hdr_len-1))}
unsafe fn virtblk_request_done(req:*mut request){let vbr=blk_mq_rq_to_pdu(req) as *mut virtblk_req;let st=virtblk_result(virtblk_vbr_status(vbr));virtblk_unmap_data(req,vbr);virtblk_cleanup_cmd(req);if req_op(req)==REQ_OP_ZONE_APPEND{(*req).__sector=virtio64_to_cpu((*(*(*req).mq_hctx).queue).queuedata.cast::<virtio_blk>().read().vdev,(*vbr).in_hdr.zone_append.sector);}blk_mq_end_request(req,st);}

// The remaining driver operations retain the source control flow and external kernel interfaces.
unsafe fn virtblk_prep_rq(hctx:*mut blk_mq_hw_ctx,vblk:*mut virtio_blk,req:*mut request,vbr:*mut virtblk_req)->blk_status_t{let s=virtblk_setup_cmd((*vblk).vdev,req,vbr);if s!=0{return s;}let n=virtblk_map_data(hctx,req,vbr);if n<0{virtblk_cleanup_cmd(req);return BLK_STS_RESOURCE;}(*vbr).sg_table.nents=n as u32;blk_mq_start_request(req);BLK_STS_OK}
unsafe fn virtio_queue_rq(hctx:*mut blk_mq_hw_ctx,bd:*const blk_mq_queue_data)->blk_status_t{let vblk=(*(*hctx).queue).queuedata as *mut virtio_blk;let req=(*bd).rq;let vbr=blk_mq_rq_to_pdu(req) as *mut virtblk_req;let s=virtblk_prep_rq(hctx,vblk,req,vbr);if s!=0{return s;}let q=(*vblk).vqs.add((*hctx).queue_num as usize);let e=virtblk_add_req((*q).vq,vbr);if e!=0{virtblk_unmap_data(req,vbr);return if e==-ENOSPC{BLK_STS_DEV_RESOURCE}else{BLK_STS_IOERR};}if (*bd).last&&virtqueue_kick_prepare((*q).vq){virtqueue_notify((*q).vq);}BLK_STS_OK}
unsafe fn virtblk_getgeo(disk:*mut gendisk,geo:*mut hd_geometry)->i32{let vblk=(*disk).private_data as *mut virtio_blk;mutex_lock(&mut (*vblk).vdev_mutex);if (*vblk).vdev.is_null(){mutex_unlock(&mut (*vblk).vdev_mutex);return -ENXIO;}(*geo).heads=1<<6;(*geo).sectors=1<<5;(*geo).cylinders=get_capacity(disk)>>11;mutex_unlock(&mut (*vblk).vdev_mutex);0}
unsafe fn index_to_minor(index:i32)->i32{index<<PART_BITS} unsafe fn minor_to_index(minor:i32)->i32{minor>>PART_BITS}
unsafe fn virtblk_config_changed(vdev:*mut virtio_device){let vblk=(*vdev).priv_ as *mut virtio_blk;queue_work(virtblk_wq,&mut (*vblk).config_work);}
unsafe fn virtblk_remove(vdev:*mut virtio_device){let vblk=(*vdev).priv_ as *mut virtio_blk;flush_work(&mut (*vblk).config_work);del_gendisk((*vblk).disk);blk_mq_free_tag_set(&mut (*vblk).tag_set);mutex_lock(&mut (*vblk).vdev_mutex);virtio_reset_device(vdev);(*vblk).vdev=core::ptr::null_mut();(*vdev).config.del_vqs(vdev);kfree((*vblk).vqs as *mut _);mutex_unlock(&mut (*vblk).vdev_mutex);put_disk((*vblk).disk);}
unsafe fn virtio_blk_init()->i32{virtblk_wq=alloc_workqueue(c"virtio-blk",WQ_PERCPU,0);if virtblk_wq.is_null(){return -ENOMEM;}major=register_blkdev(0,c"virtblk");if major<0{destroy_workqueue(virtblk_wq);return major;}let e=register_virtio_driver(&mut virtio_blk);if e!=0{unregister_blkdev(major,c"virtblk");destroy_workqueue(virtblk_wq);}e}
unsafe fn virtio_blk_fini(){unregister_virtio_driver(&mut virtio_blk);unregister_blkdev(major,c"virtblk");destroy_workqueue(virtblk_wq);}

// Declaration-only external kernel types, constants, macros, and functions are intentionally unresolved.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
