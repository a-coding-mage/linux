/* Translated from dcn20_hubbub.c.  External types, constants, register
 * helpers, and functions are supplied by the surrounding driver bindings. */

pub const NUM_VMID: usize = 16;

pub unsafe fn hubbub2_dcc_support_swizzle(
    swizzle: swizzle_mode_values, bytes_per_element: u32,
    segment_order_horz: *mut segment_order, segment_order_vert: *mut segment_order,
) -> bool {
    let mut standard_swizzle = false;
    let mut display_swizzle = false;
    let mut render_swizzle = false;
    match swizzle {
        DC_SW_4KB_S | DC_SW_64KB_S | DC_SW_VAR_S | DC_SW_4KB_S_X |
        DC_SW_64KB_S_X | DC_SW_VAR_S_X => standard_swizzle = true,
        DC_SW_64KB_R_X => render_swizzle = true,
        DC_SW_4KB_D | DC_SW_64KB_D | DC_SW_VAR_D | DC_SW_4KB_D_X |
        DC_SW_64KB_D_X | DC_SW_VAR_D_X => display_swizzle = true,
        _ => {}
    }
    if standard_swizzle {
        match bytes_per_element {
            1 => { *segment_order_horz = segment_order__contiguous; *segment_order_vert = segment_order__na; return true; }
            2 | 4 => { *segment_order_horz = segment_order__non_contiguous; *segment_order_vert = segment_order__contiguous; return true; }
            8 => { *segment_order_horz = segment_order__na; *segment_order_vert = segment_order__contiguous; return true; }
            _ => {}
        }
    }
    if render_swizzle {
        match bytes_per_element {
            2 => { *segment_order_horz = segment_order__contiguous; *segment_order_vert = segment_order__contiguous; return true; }
            4 => { *segment_order_horz = segment_order__non_contiguous; *segment_order_vert = segment_order__contiguous; return true; }
            8 => { *segment_order_horz = segment_order__contiguous; *segment_order_vert = segment_order__non_contiguous; return true; }
            _ => {}
        }
    }
    if display_swizzle && bytes_per_element == 8 {
        *segment_order_horz = segment_order__contiguous;
        *segment_order_vert = segment_order__non_contiguous;
        return true;
    }
    false
}

pub unsafe fn hubbub2_dcc_support_pixel_format(format: surface_pixel_format, bytes_per_element: *mut u32) -> bool {
    match format {
        SURFACE_PIXEL_FORMAT_GRPH_ARGB1555 | SURFACE_PIXEL_FORMAT_GRPH_RGB565 => { *bytes_per_element = 2; true }
        SURFACE_PIXEL_FORMAT_GRPH_ARGB8888 | SURFACE_PIXEL_FORMAT_GRPH_ABGR8888 |
        SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010 | SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010 |
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX | SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX |
        SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT | SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT |
        SURFACE_PIXEL_FORMAT_GRPH_RGBE | SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA => { *bytes_per_element = 4; true }
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616 | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616 |
        SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F | SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F => { *bytes_per_element = 8; true }
        _ => false,
    }
}

unsafe fn hubbub2_get_blk256_size(w: *mut u32, h: *mut u32, bpe: u32) {
    match bpe { 1 => {*w=16;*h=16}, 2 => {*w=16;*h=8}, 4 => {*w=8;*h=8}, 8 => {*w=8;*h=4}, _ => {} }
}

unsafe fn hubbub2_det_request_size(detile_buf_size: u32, height: u32, width: u32, bpe: u32, horz: *mut bool, vert: *mut bool) {
    let mut bh = 0; let mut bw = 0;
    hubbub2_get_blk256_size(&mut bw, &mut bh, bpe);
    let swath_h = width * bh * bpe;
    let swath_v = height * bw * bpe;
    *horz = 2 * swath_h > detile_buf_size;
    *vert = 2 * swath_v > detile_buf_size;
}

unsafe fn page_table_depth_to_hw(v: u32) -> dcn_hubbub_page_table_depth {
    match v { 1 => DCN_PAGE_TABLE_DEPTH_1_LEVEL, 2 => DCN_PAGE_TABLE_DEPTH_2_LEVEL, 3 => DCN_PAGE_TABLE_DEPTH_3_LEVEL, 4 => DCN_PAGE_TABLE_DEPTH_4_LEVEL, _ => { ASSERT(false); 0 } }
}

unsafe fn page_table_block_size_to_hw(v: u32) -> dcn_hubbub_page_table_block_size {
    match v { 4096=>DCN_PAGE_TABLE_BLOCK_SIZE_4KB,8192=>DCN_PAGE_TABLE_BLOCK_SIZE_8KB,16384=>DCN_PAGE_TABLE_BLOCK_SIZE_16KB,32768=>DCN_PAGE_TABLE_BLOCK_SIZE_32KB,65536=>DCN_PAGE_TABLE_BLOCK_SIZE_64KB,131072=>DCN_PAGE_TABLE_BLOCK_SIZE_128KB,262144=>DCN_PAGE_TABLE_BLOCK_SIZE_256KB,524288=>DCN_PAGE_TABLE_BLOCK_SIZE_512KB,1048576=>DCN_PAGE_TABLE_BLOCK_SIZE_1024KB,2097152=>DCN_PAGE_TABLE_BLOCK_SIZE_2048KB,_=>{ASSERT(false);DCN_PAGE_TABLE_BLOCK_SIZE_4KB} }
}

pub unsafe fn hubbub2_init_vm_ctx(hubbub: *mut hubbub, va: *mut dcn_hubbub_virt_addr_config, vmid: i32) {
    let h = TO_DCN20_HUBBUB(hubbub);
    let mut c: dcn_vmid_page_table_config = core::mem::zeroed();
    c.page_table_start_addr=(*va).page_table_start_addr>>12; c.page_table_end_addr=(*va).page_table_end_addr>>12;
    c.depth=page_table_depth_to_hw((*va).page_table_depth); c.block_size=page_table_block_size_to_hw((*va).page_table_block_size); c.page_table_base_addr=(*va).page_table_base_addr;
    dcn20_vmid_setup(&mut (*h).vmid[vmid as usize], &mut c);
}

pub unsafe fn hubbub2_init_dchub_sys_ctx(hubbub: *mut hubbub, pa: *mut dcn_hubbub_phys_addr_config) -> i32 {
    let h=TO_DCN20_HUBBUB(hubbub);
    REG_SET!(h, DCN_VM_FB_LOCATION_BASE, 0, FB_BASE, ADDR_HI24((*pa).system_aperture.fb_base));
    REG_SET!(h, DCN_VM_FB_LOCATION_TOP, 0, FB_TOP, ADDR_HI24((*pa).system_aperture.fb_top));
    REG_SET!(h, DCN_VM_FB_OFFSET, 0, FB_OFFSET, ADDR_HI24((*pa).system_aperture.fb_offset));
    REG_SET!(h, DCN_VM_AGP_BOT, 0, AGP_BOT, ADDR_HI24((*pa).system_aperture.agp_bot));
    REG_SET!(h, DCN_VM_AGP_TOP, 0, AGP_TOP, ADDR_HI24((*pa).system_aperture.agp_top));
    REG_SET!(h, DCN_VM_AGP_BASE, 0, AGP_BASE, ADDR_HI24((*pa).system_aperture.agp_base));
    REG_SET!(h, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB, 0, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB, ((*pa).page_table_default_page_addr>>44)&0xf);
    REG_SET!(h, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB, 0, DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB, ((*pa).page_table_default_page_addr>>12)&0xffff_ffff);
    if (*pa).gart_config.page_table_start_addr != (*pa).gart_config.page_table_end_addr {
        let mut c: dcn_vmid_page_table_config=core::mem::zeroed(); c.page_table_start_addr=(*pa).gart_config.page_table_start_addr>>12; c.page_table_end_addr=(*pa).gart_config.page_table_end_addr>>12; c.page_table_base_addr=(*pa).gart_config.page_table_base_addr; dcn20_vmid_setup(&mut (*h).vmid[0], &mut c);
    }
    NUM_VMID as i32
}

pub unsafe fn hubbub2_update_dchub(hubbub: *mut hubbub, dh: *mut dchub_init_data) {
    let h=TO_DCN20_HUBBUB(hubbub); if REG!(h,DCN_VM_FB_LOCATION_TOP)==0{return;}
    match (*dh).fb_mode { FRAME_BUFFER_MODE_ZFB_ONLY=>{REG_UPDATE!(h,DCN_VM_FB_LOCATION_TOP,FB_TOP,0);REG_UPDATE!(h,DCN_VM_FB_LOCATION_BASE,FB_BASE,0xffffff);}, FRAME_BUFFER_MODE_MIXED_ZFB_AND_LOCAL|FRAME_BUFFER_MODE_LOCAL_ONLY=>{}, _=>{} }
    (*dh).dchub_initialzied=true; (*dh).dchub_info_valid=false;
}

pub unsafe fn hubbub2_get_dchub_ref_freq(hubbub: *mut hubbub, dccg_ref_freq_in_khz: u32, out: *mut u32) {
    let h=TO_DCN20_HUBBUB(hubbub); let mut div=0u32; let mut en=0u32;
    REG_GET_2!(h,DCHUBBUB_GLOBAL_TIMER_CNTL,DCHUBBUB_GLOBAL_TIMER_REFDIV,&mut div,DCHUBBUB_GLOBAL_TIMER_ENABLE,&mut en);
    if en!=0 { *out=if div==2 {dccg_ref_freq_in_khz/2} else {dccg_ref_freq_in_khz}; if *out<40000||*out>60000 {ASSERT_CRITICAL(false);} } else { *out=dccg_ref_freq_in_khz; ASSERT_CRITICAL(false); }
}

pub unsafe fn hubbub2_read_state(hubbub: *mut hubbub, state: *mut dcn_hubbub_state) {
    let h=TO_DCN20_HUBBUB(hubbub);
    if REG!(h,DCN_VM_FAULT_ADDR_MSB)!=0 {(*state).vm_fault_addr_msb=REG_READ!(h,DCN_VM_FAULT_ADDR_MSB);}
    if REG!(h,DCN_VM_FAULT_ADDR_LSB)!=0 {(*state).vm_fault_addr_msb=REG_READ!(h,DCN_VM_FAULT_ADDR_LSB);}
    if REG!(h,DCN_VM_FAULT_CNTL)!=0 {REG_GET!(h,DCN_VM_FAULT_CNTL,DCN_VM_ERROR_STATUS_MODE,&mut (*state).vm_error_mode);}
    if REG!(h,DCN_VM_FAULT_STATUS)!=0 {REG_GET!(h,DCN_VM_FAULT_STATUS,DCN_VM_ERROR_STATUS,&mut (*state).vm_error_status);REG_GET!(h,DCN_VM_FAULT_STATUS,DCN_VM_ERROR_VMID,&mut (*state).vm_error_vmid);REG_GET!(h,DCN_VM_FAULT_STATUS,DCN_VM_ERROR_PIPE,&mut (*state).vm_error_pipe);}
    if REG!(h,DCHUBBUB_TEST_DEBUG_INDEX)!=0 && REG!(h,DCHUBBUB_TEST_DEBUG_DATA)!=0 {REG_WRITE!(h,DCHUBBUB_TEST_DEBUG_INDEX,0x6);(*state).test_debug_data=REG_READ!(h,DCHUBBUB_TEST_DEBUG_DATA);}
    if REG!(h,DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL)!=0 {(*state).watermark_change_cntl=REG_READ!(h,DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL);}
    if REG!(h,DCHUBBUB_ARB_DRAM_STATE_CNTL)!=0 {(*state).dram_state_cntl=REG_READ!(h,DCHUBBUB_ARB_DRAM_STATE_CNTL);}
}

/* The remaining driver methods retain the original register-helper call sites. */
pub unsafe fn hubbub2_construct(h: *mut dcn20_hubbub, ctx: *mut dc_context, regs: *const dcn_hubbub_registers, shifts: *const dcn_hubbub_shift, masks: *const dcn_hubbub_mask) {
    (*h).base.ctx=ctx; (*h).base.funcs=&hubbub2_funcs; (*h).regs=regs; (*h).shifts=shifts; (*h).masks=masks; (*h).debug_test_index_pstate=0xb; (*h).detile_buf_size=164*1024;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
