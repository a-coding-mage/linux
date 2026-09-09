/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */


#[repr(C)]
pub struct v10_gfx_mqd {
	u32 reserved_0; // offset: 0  (0x0)
	u32 reserved_1; // offset: 1  (0x1)
	u32 reserved_2; // offset: 2  (0x2)
	u32 reserved_3; // offset: 3  (0x3)
	u32 reserved_4; // offset: 4  (0x4)
	u32 reserved_5; // offset: 5  (0x5)
	u32 reserved_6; // offset: 6  (0x6)
	u32 reserved_7; // offset: 7  (0x7)
	u32 reserved_8; // offset: 8  (0x8)
	u32 reserved_9; // offset: 9  (0x9)
	u32 reserved_10; // offset: 10  (0xA)
	u32 reserved_11; // offset: 11  (0xB)
	u32 reserved_12; // offset: 12  (0xC)
	u32 reserved_13; // offset: 13  (0xD)
	u32 reserved_14; // offset: 14  (0xE)
	u32 reserved_15; // offset: 15  (0xF)
	u32 reserved_16; // offset: 16  (0x10)
	u32 reserved_17; // offset: 17  (0x11)
	u32 reserved_18; // offset: 18  (0x12)
	u32 reserved_19; // offset: 19  (0x13)
	u32 reserved_20; // offset: 20  (0x14)
	u32 reserved_21; // offset: 21  (0x15)
	u32 reserved_22; // offset: 22  (0x16)
	u32 reserved_23; // offset: 23  (0x17)
	u32 reserved_24; // offset: 24  (0x18)
	u32 reserved_25; // offset: 25  (0x19)
	u32 reserved_26; // offset: 26  (0x1A)
	u32 reserved_27; // offset: 27  (0x1B)
	u32 reserved_28; // offset: 28  (0x1C)
	u32 reserved_29; // offset: 29  (0x1D)
	u32 reserved_30; // offset: 30  (0x1E)
	u32 reserved_31; // offset: 31  (0x1F)
	u32 reserved_32; // offset: 32  (0x20)
	u32 reserved_33; // offset: 33  (0x21)
	u32 reserved_34; // offset: 34  (0x22)
	u32 reserved_35; // offset: 35  (0x23)
	u32 reserved_36; // offset: 36  (0x24)
	u32 reserved_37; // offset: 37  (0x25)
	u32 reserved_38; // offset: 38  (0x26)
	u32 reserved_39; // offset: 39  (0x27)
	u32 reserved_40; // offset: 40  (0x28)
	u32 reserved_41; // offset: 41  (0x29)
	u32 reserved_42; // offset: 42  (0x2A)
	u32 reserved_43; // offset: 43  (0x2B)
	u32 reserved_44; // offset: 44  (0x2C)
	u32 reserved_45; // offset: 45  (0x2D)
	u32 reserved_46; // offset: 46  (0x2E)
	u32 reserved_47; // offset: 47  (0x2F)
	u32 reserved_48; // offset: 48  (0x30)
	u32 reserved_49; // offset: 49  (0x31)
	u32 reserved_50; // offset: 50  (0x32)
	u32 reserved_51; // offset: 51  (0x33)
	u32 reserved_52; // offset: 52  (0x34)
	u32 reserved_53; // offset: 53  (0x35)
	u32 reserved_54; // offset: 54  (0x36)
	u32 reserved_55; // offset: 55  (0x37)
	u32 reserved_56; // offset: 56  (0x38)
	u32 reserved_57; // offset: 57  (0x39)
	u32 reserved_58; // offset: 58  (0x3A)
	u32 reserved_59; // offset: 59  (0x3B)
	u32 reserved_60; // offset: 60  (0x3C)
	u32 reserved_61; // offset: 61  (0x3D)
	u32 reserved_62; // offset: 62  (0x3E)
	u32 reserved_63; // offset: 63  (0x3F)
	u32 reserved_64; // offset: 64  (0x40)
	u32 reserved_65; // offset: 65  (0x41)
	u32 reserved_66; // offset: 66  (0x42)
	u32 reserved_67; // offset: 67  (0x43)
	u32 reserved_68; // offset: 68  (0x44)
	u32 reserved_69; // offset: 69  (0x45)
	u32 reserved_70; // offset: 70  (0x46)
	u32 reserved_71; // offset: 71  (0x47)
	u32 reserved_72; // offset: 72  (0x48)
	u32 reserved_73; // offset: 73  (0x49)
	u32 reserved_74; // offset: 74  (0x4A)
	u32 reserved_75; // offset: 75  (0x4B)
	u32 reserved_76; // offset: 76  (0x4C)
	u32 reserved_77; // offset: 77  (0x4D)
	u32 reserved_78; // offset: 78  (0x4E)
	u32 reserved_79; // offset: 79  (0x4F)
	u32 reserved_80; // offset: 80  (0x50)
	u32 reserved_81; // offset: 81  (0x51)
	u32 reserved_82; // offset: 82  (0x52)
	u32 reserved_83; // offset: 83  (0x53)
	u32 reserved_84; // offset: 84  (0x54)
	u32 reserved_85; // offset: 85  (0x55)
	u32 reserved_86; // offset: 86  (0x56)
	u32 reserved_87; // offset: 87  (0x57)
	u32 reserved_88; // offset: 88  (0x58)
	u32 reserved_89; // offset: 89  (0x59)
	u32 reserved_90; // offset: 90  (0x5A)
	u32 reserved_91; // offset: 91  (0x5B)
	u32 reserved_92; // offset: 92  (0x5C)
	u32 reserved_93; // offset: 93  (0x5D)
	u32 reserved_94; // offset: 94  (0x5E)
	u32 reserved_95; // offset: 95  (0x5F)
	u32 reserved_96; // offset: 96  (0x60)
	u32 reserved_97; // offset: 97  (0x61)
	u32 reserved_98; // offset: 98  (0x62)
	u32 reserved_99; // offset: 99  (0x63)
	u32 reserved_100; // offset: 100  (0x64)
	u32 reserved_101; // offset: 101  (0x65)
	u32 reserved_102; // offset: 102  (0x66)
	u32 reserved_103; // offset: 103  (0x67)
	u32 reserved_104; // offset: 104  (0x68)
	u32 reserved_105; // offset: 105  (0x69)
	u32 disable_queue; // offset: 106  (0x6A)
	u32 reserved_107; // offset: 107  (0x6B)
	u32 reserved_108; // offset: 108  (0x6C)
	u32 reserved_109; // offset: 109  (0x6D)
	u32 reserved_110; // offset: 110  (0x6E)
	u32 reserved_111; // offset: 111  (0x6F)
	u32 reserved_112; // offset: 112  (0x70)
	u32 reserved_113; // offset: 113  (0x71)
	u32 reserved_114; // offset: 114  (0x72)
	u32 reserved_115; // offset: 115  (0x73)
	u32 reserved_116; // offset: 116  (0x74)
	u32 reserved_117; // offset: 117  (0x75)
	u32 reserved_118; // offset: 118  (0x76)
	u32 reserved_119; // offset: 119  (0x77)
	u32 reserved_120; // offset: 120  (0x78)
	u32 reserved_121; // offset: 121  (0x79)
	u32 reserved_122; // offset: 122  (0x7A)
	u32 reserved_123; // offset: 123  (0x7B)
	u32 reserved_124; // offset: 124  (0x7C)
	u32 reserved_125; // offset: 125  (0x7D)
	u32 reserved_126; // offset: 126  (0x7E)
	u32 reserved_127; // offset: 127  (0x7F)
	u32 cp_mqd_base_addr; // offset: 128  (0x80)
	u32 cp_mqd_base_addr_hi; // offset: 129  (0x81)
	u32 cp_gfx_hqd_active; // offset: 130  (0x82)
	u32 cp_gfx_hqd_vmid; // offset: 131  (0x83)
	u32 reserved_131; // offset: 132  (0x84)
	u32 reserved_132; // offset: 133  (0x85)
	u32 cp_gfx_hqd_queue_priority; // offset: 134  (0x86)
	u32 cp_gfx_hqd_quantum; // offset: 135  (0x87)
	u32 cp_gfx_hqd_base; // offset: 136  (0x88)
	u32 cp_gfx_hqd_base_hi; // offset: 137  (0x89)
	u32 cp_gfx_hqd_rptr; // offset: 138  (0x8A)
	u32 cp_gfx_hqd_rptr_addr; // offset: 139  (0x8B)
	u32 cp_gfx_hqd_rptr_addr_hi; // offset: 140  (0x8C)
	u32 cp_rb_wptr_poll_addr_lo; // offset: 141  (0x8D)
	u32 cp_rb_wptr_poll_addr_hi; // offset: 142  (0x8E)
	u32 cp_rb_doorbell_control; // offset: 143  (0x8F)
	u32 cp_gfx_hqd_offset; // offset: 144  (0x90)
	u32 cp_gfx_hqd_cntl; // offset: 145  (0x91)
	u32 reserved_146; // offset: 146  (0x92)
	u32 reserved_147; // offset: 147  (0x93)
	u32 cp_gfx_hqd_csmd_rptr; // offset: 148  (0x94)
	u32 cp_gfx_hqd_wptr; // offset: 149  (0x95)
	u32 cp_gfx_hqd_wptr_hi; // offset: 150  (0x96)
	u32 reserved_151; // offset: 151  (0x97)
	u32 reserved_152; // offset: 152  (0x98)
	u32 reserved_153; // offset: 153  (0x99)
	u32 reserved_154; // offset: 154  (0x9A)
	u32 reserved_155; // offset: 155  (0x9B)
	u32 cp_gfx_hqd_mapped; // offset: 156  (0x9C)
	u32 cp_gfx_hqd_que_mgr_control; // offset: 157  (0x9D)
	u32 reserved_158; // offset: 158  (0x9E)
	u32 reserved_159; // offset: 159  (0x9F)
	u32 cp_gfx_hqd_hq_status0; // offset: 160  (0xA0)
	u32 cp_gfx_hqd_hq_control0; // offset: 161  (0xA1)
	u32 cp_gfx_mqd_control; // offset: 162  (0xA2)
	u32 reserved_163; // offset: 163  (0xA3)
	u32 reserved_164; // offset: 164  (0xA4)
	u32 reserved_165; // offset: 165  (0xA5)
	u32 reserved_166; // offset: 166  (0xA6)
	u32 reserved_167; // offset: 167  (0xA7)
	u32 reserved_168; // offset: 168  (0xA8)
	u32 reserved_169; // offset: 169  (0xA9)
	u32 cp_num_prim_needed_count0_lo; // offset: 170  (0xAA)
	u32 cp_num_prim_needed_count0_hi; // offset: 171  (0xAB)
	u32 cp_num_prim_needed_count1_lo; // offset: 172  (0xAC)
	u32 cp_num_prim_needed_count1_hi; // offset: 173  (0xAD)
	u32 cp_num_prim_needed_count2_lo; // offset: 174  (0xAE)
	u32 cp_num_prim_needed_count2_hi; // offset: 175  (0xAF)
	u32 cp_num_prim_needed_count3_lo; // offset: 176  (0xB0)
	u32 cp_num_prim_needed_count3_hi; // offset: 177  (0xB1)
	u32 cp_num_prim_written_count0_lo; // offset: 178  (0xB2)
	u32 cp_num_prim_written_count0_hi; // offset: 179  (0xB3)
	u32 cp_num_prim_written_count1_lo; // offset: 180  (0xB4)
	u32 cp_num_prim_written_count1_hi; // offset: 181  (0xB5)
	u32 cp_num_prim_written_count2_lo; // offset: 182  (0xB6)
	u32 cp_num_prim_written_count2_hi; // offset: 183  (0xB7)
	u32 cp_num_prim_written_count3_lo; // offset: 184  (0xB8)
	u32 cp_num_prim_written_count3_hi; // offset: 185  (0xB9)
	u32 reserved_186; // offset: 186  (0xBA)
	u32 reserved_187; // offset: 187  (0xBB)
	u32 reserved_188; // offset: 188  (0xBC)
	u32 reserved_189; // offset: 189  (0xBD)
	u32 mp1_smn_fps_cnt; // offset: 190  (0xBE)
	u32 sq_thread_trace_buf0_base; // offset: 191  (0xBF)
	u32 sq_thread_trace_buf0_size; // offset: 192  (0xC0)
	u32 sq_thread_trace_buf1_base; // offset: 193  (0xC1)
	u32 sq_thread_trace_buf1_size; // offset: 194  (0xC2)
	u32 sq_thread_trace_wptr; // offset: 195  (0xC3)
	u32 sq_thread_trace_mask; // offset: 196  (0xC4)
	u32 sq_thread_trace_token_mask; // offset: 197  (0xC5)
	u32 sq_thread_trace_ctrl; // offset: 198  (0xC6)
	u32 sq_thread_trace_status; // offset: 199  (0xC7)
	u32 sq_thread_trace_dropped_cntr; // offset: 200  (0xC8)
	u32 sq_thread_trace_finish_done_debug; // offset: 201  (0xC9)
	u32 sq_thread_trace_gfx_draw_cntr; // offset: 202  (0xCA)
	u32 sq_thread_trace_gfx_marker_cntr; // offset: 203  (0xCB)
	u32 sq_thread_trace_hp3d_draw_cntr; // offset: 204  (0xCC)
	u32 sq_thread_trace_hp3d_marker_cntr; // offset: 205  (0xCD)
	u32 reserved_206; // offset: 206  (0xCE)
	u32 reserved_207; // offset: 207  (0xCF)
	u32 cp_sc_psinvoc_count0_lo; // offset: 208  (0xD0)
	u32 cp_sc_psinvoc_count0_hi; // offset: 209  (0xD1)
	u32 cp_pa_cprim_count_lo; // offset: 210  (0xD2)
	u32 cp_pa_cprim_count_hi; // offset: 211  (0xD3)
	u32 cp_pa_cinvoc_count_lo; // offset: 212  (0xD4)
	u32 cp_pa_cinvoc_count_hi; // offset: 213  (0xD5)
	u32 cp_vgt_vsinvoc_count_lo; // offset: 214  (0xD6)
	u32 cp_vgt_vsinvoc_count_hi; // offset: 215  (0xD7)
	u32 cp_vgt_gsinvoc_count_lo; // offset: 216  (0xD8)
	u32 cp_vgt_gsinvoc_count_hi; // offset: 217  (0xD9)
	u32 cp_vgt_gsprim_count_lo; // offset: 218  (0xDA)
	u32 cp_vgt_gsprim_count_hi; // offset: 219  (0xDB)
	u32 cp_vgt_iaprim_count_lo; // offset: 220  (0xDC)
	u32 cp_vgt_iaprim_count_hi; // offset: 221  (0xDD)
	u32 cp_vgt_iavert_count_lo; // offset: 222  (0xDE)
	u32 cp_vgt_iavert_count_hi; // offset: 223  (0xDF)
	u32 cp_vgt_hsinvoc_count_lo; // offset: 224  (0xE0)
	u32 cp_vgt_hsinvoc_count_hi; // offset: 225  (0xE1)
	u32 cp_vgt_dsinvoc_count_lo; // offset: 226  (0xE2)
	u32 cp_vgt_dsinvoc_count_hi; // offset: 227  (0xE3)
	u32 cp_vgt_csinvoc_count_lo; // offset: 228  (0xE4)
	u32 cp_vgt_csinvoc_count_hi; // offset: 229  (0xE5)
	u32 reserved_230; // offset: 230  (0xE6)
	u32 reserved_231; // offset: 231  (0xE7)
	u32 reserved_232; // offset: 232  (0xE8)
	u32 reserved_233; // offset: 233  (0xE9)
	u32 reserved_234; // offset: 234  (0xEA)
	u32 reserved_235; // offset: 235  (0xEB)
	u32 reserved_236; // offset: 236  (0xEC)
	u32 reserved_237; // offset: 237  (0xED)
	u32 reserved_238; // offset: 238  (0xEE)
	u32 reserved_239; // offset: 239  (0xEF)
	u32 reserved_240; // offset: 240  (0xF0)
	u32 reserved_241; // offset: 241  (0xF1)
	u32 reserved_242; // offset: 242  (0xF2)
	u32 reserved_243; // offset: 243  (0xF3)
	u32 reserved_244; // offset: 244  (0xF4)
	u32 reserved_245; // offset: 245  (0xF5)
	u32 reserved_246; // offset: 246  (0xF6)
	u32 reserved_247; // offset: 247  (0xF7)
	u32 reserved_248; // offset: 248  (0xF8)
	u32 reserved_249; // offset: 249  (0xF9)
	u32 reserved_250; // offset: 250  (0xFA)
	u32 reserved_251; // offset: 251  (0xFB)
	u32 reserved_252; // offset: 252  (0xFC)
	u32 reserved_253; // offset: 253  (0xFD)
	u32 reserved_254; // offset: 254  (0xFE)
	u32 reserved_255; // offset: 255  (0xFF)
	u32 reserved_256; // offset: 256  (0x100)
	u32 reserved_257; // offset: 257  (0x101)
	u32 reserved_258; // offset: 258  (0x102)
	u32 reserved_259; // offset: 259  (0x103)
	u32 reserved_260; // offset: 260  (0x104)
	u32 reserved_261; // offset: 261  (0x105)
	u32 reserved_262; // offset: 262  (0x106)
	u32 reserved_263; // offset: 263  (0x107)
	u32 reserved_264; // offset: 264  (0x108)
	u32 reserved_265; // offset: 265  (0x109)
	u32 reserved_266; // offset: 266  (0x10A)
	u32 reserved_267; // offset: 267  (0x10B)
	u32 vgt_strmout_buffer_filled_size_0; // offset: 268  (0x10C)
	u32 vgt_strmout_buffer_filled_size_1; // offset: 269  (0x10D)
	u32 vgt_strmout_buffer_filled_size_2; // offset: 270  (0x10E)
	u32 vgt_strmout_buffer_filled_size_3; // offset: 271  (0x10F)
	u32 reserved_272; // offset: 272  (0x110)
	u32 reserved_273; // offset: 273  (0x111)
	u32 reserved_274; // offset: 274  (0x112)
	u32 reserved_275; // offset: 275  (0x113)
	u32 vgt_dma_max_size; // offset: 276  (0x114)
	u32 vgt_dma_num_instances; // offset: 277  (0x115)
	u32 reserved_278; // offset: 278  (0x116)
	u32 reserved_279; // offset: 279  (0x117)
	u32 reserved_280; // offset: 280  (0x118)
	u32 reserved_281; // offset: 281  (0x119)
	u32 reserved_282; // offset: 282  (0x11A)
	u32 reserved_283; // offset: 283  (0x11B)
	u32 reserved_284; // offset: 284  (0x11C)
	u32 reserved_285; // offset: 285  (0x11D)
	u32 reserved_286; // offset: 286  (0x11E)
	u32 reserved_287; // offset: 287  (0x11F)
	u32 it_set_base_ib_addr_lo; // offset: 288  (0x120)
	u32 it_set_base_ib_addr_hi; // offset: 289  (0x121)
	u32 reserved_290; // offset: 290  (0x122)
	u32 reserved_291; // offset: 291  (0x123)
	u32 reserved_292; // offset: 292  (0x124)
	u32 reserved_293; // offset: 293  (0x125)
	u32 reserved_294; // offset: 294  (0x126)
	u32 reserved_295; // offset: 295  (0x127)
	u32 reserved_296; // offset: 296  (0x128)
	u32 reserved_297; // offset: 297  (0x129)
	u32 reserved_298; // offset: 298  (0x12A)
	u32 reserved_299; // offset: 299  (0x12B)
	u32 reserved_300; // offset: 300  (0x12C)
	u32 reserved_301; // offset: 301  (0x12D)
	u32 reserved_302; // offset: 302  (0x12E)
	u32 reserved_303; // offset: 303  (0x12F)
	u32 reserved_304; // offset: 304  (0x130)
	u32 reserved_305; // offset: 305  (0x131)
	u32 reserved_306; // offset: 306  (0x132)
	u32 reserved_307; // offset: 307  (0x133)
	u32 reserved_308; // offset: 308  (0x134)
	u32 reserved_309; // offset: 309  (0x135)
	u32 reserved_310; // offset: 310  (0x136)
	u32 reserved_311; // offset: 311  (0x137)
	u32 reserved_312; // offset: 312  (0x138)
	u32 reserved_313; // offset: 313  (0x139)
	u32 reserved_314; // offset: 314  (0x13A)
	u32 reserved_315; // offset: 315  (0x13B)
	u32 reserved_316; // offset: 316  (0x13C)
	u32 reserved_317; // offset: 317  (0x13D)
	u32 reserved_318; // offset: 318  (0x13E)
	u32 reserved_319; // offset: 319  (0x13F)
	u32 reserved_320; // offset: 320  (0x140)
	u32 reserved_321; // offset: 321  (0x141)
	u32 reserved_322; // offset: 322  (0x142)
	u32 reserved_323; // offset: 323  (0x143)
	u32 reserved_324; // offset: 324  (0x144)
	u32 reserved_325; // offset: 325  (0x145)
	u32 reserved_326; // offset: 326  (0x146)
	u32 reserved_327; // offset: 327  (0x147)
	u32 reserved_328; // offset: 328  (0x148)
	u32 reserved_329; // offset: 329  (0x149)
	u32 reserved_330; // offset: 330  (0x14A)
	u32 reserved_331; // offset: 331  (0x14B)
	u32 reserved_332; // offset: 332  (0x14C)
	u32 reserved_333; // offset: 333  (0x14D)
	u32 reserved_334; // offset: 334  (0x14E)
	u32 reserved_335; // offset: 335  (0x14F)
	u32 reserved_336; // offset: 336  (0x150)
	u32 reserved_337; // offset: 337  (0x151)
	u32 reserved_338; // offset: 338  (0x152)
	u32 reserved_339; // offset: 339  (0x153)
	u32 reserved_340; // offset: 340  (0x154)
	u32 reserved_341; // offset: 341  (0x155)
	u32 reserved_342; // offset: 342  (0x156)
	u32 reserved_343; // offset: 343  (0x157)
	u32 reserved_344; // offset: 344  (0x158)
	u32 reserved_345; // offset: 345  (0x159)
	u32 reserved_346; // offset: 346  (0x15A)
	u32 reserved_347; // offset: 347  (0x15B)
	u32 reserved_348; // offset: 348  (0x15C)
	u32 reserved_349; // offset: 349  (0x15D)
	u32 reserved_350; // offset: 350  (0x15E)
	u32 reserved_351; // offset: 351  (0x15F)
	u32 reserved_352; // offset: 352  (0x160)
	u32 reserved_353; // offset: 353  (0x161)
	u32 reserved_354; // offset: 354  (0x162)
	u32 reserved_355; // offset: 355  (0x163)
	u32 spi_shader_pgm_rsrc3_ps; // offset: 356  (0x164)
	u32 spi_shader_pgm_rsrc3_vs; // offset: 357  (0x165)
	u32 spi_shader_pgm_rsrc3_gs; // offset: 358  (0x166)
	u32 spi_shader_pgm_rsrc3_hs; // offset: 359  (0x167)
	u32 spi_shader_pgm_rsrc4_ps; // offset: 360  (0x168)
	u32 spi_shader_pgm_rsrc4_vs; // offset: 361  (0x169)
	u32 spi_shader_pgm_rsrc4_gs; // offset: 362  (0x16A)
	u32 spi_shader_pgm_rsrc4_hs; // offset: 363  (0x16B)
	u32 db_occlusion_count0_low_00; // offset: 364  (0x16C)
	u32 db_occlusion_count0_hi_00; // offset: 365  (0x16D)
	u32 db_occlusion_count1_low_00; // offset: 366  (0x16E)
	u32 db_occlusion_count1_hi_00; // offset: 367  (0x16F)
	u32 db_occlusion_count2_low_00; // offset: 368  (0x170)
	u32 db_occlusion_count2_hi_00; // offset: 369  (0x171)
	u32 db_occlusion_count3_low_00; // offset: 370  (0x172)
	u32 db_occlusion_count3_hi_00; // offset: 371  (0x173)
	u32 db_occlusion_count0_low_01; // offset: 372  (0x174)
	u32 db_occlusion_count0_hi_01; // offset: 373  (0x175)
	u32 db_occlusion_count1_low_01; // offset: 374  (0x176)
	u32 db_occlusion_count1_hi_01; // offset: 375  (0x177)
	u32 db_occlusion_count2_low_01; // offset: 376  (0x178)
	u32 db_occlusion_count2_hi_01; // offset: 377  (0x179)
	u32 db_occlusion_count3_low_01; // offset: 378  (0x17A)
	u32 db_occlusion_count3_hi_01; // offset: 379  (0x17B)
	u32 db_occlusion_count0_low_02; // offset: 380  (0x17C)
	u32 db_occlusion_count0_hi_02; // offset: 381  (0x17D)
	u32 db_occlusion_count1_low_02; // offset: 382  (0x17E)
	u32 db_occlusion_count1_hi_02; // offset: 383  (0x17F)
	u32 db_occlusion_count2_low_02; // offset: 384  (0x180)
	u32 db_occlusion_count2_hi_02; // offset: 385  (0x181)
	u32 db_occlusion_count3_low_02; // offset: 386  (0x182)
	u32 db_occlusion_count3_hi_02; // offset: 387  (0x183)
	u32 db_occlusion_count0_low_03; // offset: 388  (0x184)
	u32 db_occlusion_count0_hi_03; // offset: 389  (0x185)
	u32 db_occlusion_count1_low_03; // offset: 390  (0x186)
	u32 db_occlusion_count1_hi_03; // offset: 391  (0x187)
	u32 db_occlusion_count2_low_03; // offset: 392  (0x188)
	u32 db_occlusion_count2_hi_03; // offset: 393  (0x189)
	u32 db_occlusion_count3_low_03; // offset: 394  (0x18A)
	u32 db_occlusion_count3_hi_03; // offset: 395  (0x18B)
	u32 db_occlusion_count0_low_04; // offset: 396  (0x18C)
	u32 db_occlusion_count0_hi_04; // offset: 397  (0x18D)
	u32 db_occlusion_count1_low_04; // offset: 398  (0x18E)
	u32 db_occlusion_count1_hi_04; // offset: 399  (0x18F)
	u32 db_occlusion_count2_low_04; // offset: 400  (0x190)
	u32 db_occlusion_count2_hi_04; // offset: 401  (0x191)
	u32 db_occlusion_count3_low_04; // offset: 402  (0x192)
	u32 db_occlusion_count3_hi_04; // offset: 403  (0x193)
	u32 db_occlusion_count0_low_05; // offset: 404  (0x194)
	u32 db_occlusion_count0_hi_05; // offset: 405  (0x195)
	u32 db_occlusion_count1_low_05; // offset: 406  (0x196)
	u32 db_occlusion_count1_hi_05; // offset: 407  (0x197)
	u32 db_occlusion_count2_low_05; // offset: 408  (0x198)
	u32 db_occlusion_count2_hi_05; // offset: 409  (0x199)
	u32 db_occlusion_count3_low_05; // offset: 410  (0x19A)
	u32 db_occlusion_count3_hi_05; // offset: 411  (0x19B)
	u32 db_occlusion_count0_low_06; // offset: 412  (0x19C)
	u32 db_occlusion_count0_hi_06; // offset: 413  (0x19D)
	u32 db_occlusion_count1_low_06; // offset: 414  (0x19E)
	u32 db_occlusion_count1_hi_06; // offset: 415  (0x19F)
	u32 db_occlusion_count2_low_06; // offset: 416  (0x1A0)
	u32 db_occlusion_count2_hi_06; // offset: 417  (0x1A1)
	u32 db_occlusion_count3_low_06; // offset: 418  (0x1A2)
	u32 db_occlusion_count3_hi_06; // offset: 419  (0x1A3)
	u32 db_occlusion_count0_low_07; // offset: 420  (0x1A4)
	u32 db_occlusion_count0_hi_07; // offset: 421  (0x1A5)
	u32 db_occlusion_count1_low_07; // offset: 422  (0x1A6)
	u32 db_occlusion_count1_hi_07; // offset: 423  (0x1A7)
	u32 db_occlusion_count2_low_07; // offset: 424  (0x1A8)
	u32 db_occlusion_count2_hi_07; // offset: 425  (0x1A9)
	u32 db_occlusion_count3_low_07; // offset: 426  (0x1AA)
	u32 db_occlusion_count3_hi_07; // offset: 427  (0x1AB)
	u32 db_occlusion_count0_low_10; // offset: 428  (0x1AC)
	u32 db_occlusion_count0_hi_10; // offset: 429  (0x1AD)
	u32 db_occlusion_count1_low_10; // offset: 430  (0x1AE)
	u32 db_occlusion_count1_hi_10; // offset: 431  (0x1AF)
	u32 db_occlusion_count2_low_10; // offset: 432  (0x1B0)
	u32 db_occlusion_count2_hi_10; // offset: 433  (0x1B1)
	u32 db_occlusion_count3_low_10; // offset: 434  (0x1B2)
	u32 db_occlusion_count3_hi_10; // offset: 435  (0x1B3)
	u32 db_occlusion_count0_low_11; // offset: 436  (0x1B4)
	u32 db_occlusion_count0_hi_11; // offset: 437  (0x1B5)
	u32 db_occlusion_count1_low_11; // offset: 438  (0x1B6)
	u32 db_occlusion_count1_hi_11; // offset: 439  (0x1B7)
	u32 db_occlusion_count2_low_11; // offset: 440  (0x1B8)
	u32 db_occlusion_count2_hi_11; // offset: 441  (0x1B9)
	u32 db_occlusion_count3_low_11; // offset: 442  (0x1BA)
	u32 db_occlusion_count3_hi_11; // offset: 443  (0x1BB)
	u32 db_occlusion_count0_low_12; // offset: 444  (0x1BC)
	u32 db_occlusion_count0_hi_12; // offset: 445  (0x1BD)
	u32 db_occlusion_count1_low_12; // offset: 446  (0x1BE)
	u32 db_occlusion_count1_hi_12; // offset: 447  (0x1BF)
	u32 db_occlusion_count2_low_12; // offset: 448  (0x1C0)
	u32 db_occlusion_count2_hi_12; // offset: 449  (0x1C1)
	u32 db_occlusion_count3_low_12; // offset: 450  (0x1C2)
	u32 db_occlusion_count3_hi_12; // offset: 451  (0x1C3)
	u32 db_occlusion_count0_low_13; // offset: 452  (0x1C4)
	u32 db_occlusion_count0_hi_13; // offset: 453  (0x1C5)
	u32 db_occlusion_count1_low_13; // offset: 454  (0x1C6)
	u32 db_occlusion_count1_hi_13; // offset: 455  (0x1C7)
	u32 db_occlusion_count2_low_13; // offset: 456  (0x1C8)
	u32 db_occlusion_count2_hi_13; // offset: 457  (0x1C9)
	u32 db_occlusion_count3_low_13; // offset: 458  (0x1CA)
	u32 db_occlusion_count3_hi_13; // offset: 459  (0x1CB)
	u32 db_occlusion_count0_low_14; // offset: 460  (0x1CC)
	u32 db_occlusion_count0_hi_14; // offset: 461  (0x1CD)
	u32 db_occlusion_count1_low_14; // offset: 462  (0x1CE)
	u32 db_occlusion_count1_hi_14; // offset: 463  (0x1CF)
	u32 db_occlusion_count2_low_14; // offset: 464  (0x1D0)
	u32 db_occlusion_count2_hi_14; // offset: 465  (0x1D1)
	u32 db_occlusion_count3_low_14; // offset: 466  (0x1D2)
	u32 db_occlusion_count3_hi_14; // offset: 467  (0x1D3)
	u32 db_occlusion_count0_low_15; // offset: 468  (0x1D4)
	u32 db_occlusion_count0_hi_15; // offset: 469  (0x1D5)
	u32 db_occlusion_count1_low_15; // offset: 470  (0x1D6)
	u32 db_occlusion_count1_hi_15; // offset: 471  (0x1D7)
	u32 db_occlusion_count2_low_15; // offset: 472  (0x1D8)
	u32 db_occlusion_count2_hi_15; // offset: 473  (0x1D9)
	u32 db_occlusion_count3_low_15; // offset: 474  (0x1DA)
	u32 db_occlusion_count3_hi_15; // offset: 475  (0x1DB)
	u32 db_occlusion_count0_low_16; // offset: 476  (0x1DC)
	u32 db_occlusion_count0_hi_16; // offset: 477  (0x1DD)
	u32 db_occlusion_count1_low_16; // offset: 478  (0x1DE)
	u32 db_occlusion_count1_hi_16; // offset: 479  (0x1DF)
	u32 db_occlusion_count2_low_16; // offset: 480  (0x1E0)
	u32 db_occlusion_count2_hi_16; // offset: 481  (0x1E1)
	u32 db_occlusion_count3_low_16; // offset: 482  (0x1E2)
	u32 db_occlusion_count3_hi_16; // offset: 483  (0x1E3)
	u32 db_occlusion_count0_low_17; // offset: 484  (0x1E4)
	u32 db_occlusion_count0_hi_17; // offset: 485  (0x1E5)
	u32 db_occlusion_count1_low_17; // offset: 486  (0x1E6)
	u32 db_occlusion_count1_hi_17; // offset: 487  (0x1E7)
	u32 db_occlusion_count2_low_17; // offset: 488  (0x1E8)
	u32 db_occlusion_count2_hi_17; // offset: 489  (0x1E9)
	u32 db_occlusion_count3_low_17; // offset: 490  (0x1EA)
	u32 db_occlusion_count3_hi_17; // offset: 491  (0x1EB)
	u32 reserved_492; // offset: 492  (0x1EC)
	u32 reserved_493; // offset: 493  (0x1ED)
	u32 reserved_494; // offset: 494  (0x1EE)
	u32 reserved_495; // offset: 495  (0x1EF)
	u32 reserved_496; // offset: 496  (0x1F0)
	u32 reserved_497; // offset: 497  (0x1F1)
	u32 reserved_498; // offset: 498  (0x1F2)
	u32 reserved_499; // offset: 499  (0x1F3)
	u32 reserved_500; // offset: 500  (0x1F4)
	u32 reserved_501; // offset: 501  (0x1F5)
	u32 reserved_502; // offset: 502  (0x1F6)
	u32 reserved_503; // offset: 503  (0x1F7)
	u32 reserved_504; // offset: 504  (0x1F8)
	u32 reserved_505; // offset: 505  (0x1F9)
	u32 reserved_506; // offset: 506  (0x1FA)
	u32 reserved_507; // offset: 507  (0x1FB)
	u32 reserved_508; // offset: 508  (0x1FC)
	u32 reserved_509; // offset: 509  (0x1FD)
	u32 reserved_510; // offset: 510  (0x1FE)
	u32 reserved_511; // offset: 511  (0x1FF)
}

#[repr(C)]
pub struct v10_sdma_mqd {
	u32 sdmax_rlcx_rb_cntl;
	u32 sdmax_rlcx_rb_base;
	u32 sdmax_rlcx_rb_base_hi;
	u32 sdmax_rlcx_rb_rptr;
	u32 sdmax_rlcx_rb_rptr_hi;
	u32 sdmax_rlcx_rb_wptr;
	u32 sdmax_rlcx_rb_wptr_hi;
	u32 sdmax_rlcx_rb_wptr_poll_cntl;
	u32 sdmax_rlcx_rb_rptr_addr_hi;
	u32 sdmax_rlcx_rb_rptr_addr_lo;
	u32 sdmax_rlcx_ib_cntl;
	u32 sdmax_rlcx_ib_rptr;
	u32 sdmax_rlcx_ib_offset;
	u32 sdmax_rlcx_ib_base_lo;
	u32 sdmax_rlcx_ib_base_hi;
	u32 sdmax_rlcx_ib_size;
	u32 sdmax_rlcx_skip_cntl;
	u32 sdmax_rlcx_context_status;
	u32 sdmax_rlcx_doorbell;
	u32 sdmax_rlcx_status;
	u32 sdmax_rlcx_doorbell_log;
	u32 sdmax_rlcx_watermark;
	u32 sdmax_rlcx_doorbell_offset;
	u32 sdmax_rlcx_csa_addr_lo;
	u32 sdmax_rlcx_csa_addr_hi;
	u32 sdmax_rlcx_ib_sub_remain;
	u32 sdmax_rlcx_preempt;
	u32 sdmax_rlcx_dummy_reg;
	u32 sdmax_rlcx_rb_wptr_poll_addr_hi;
	u32 sdmax_rlcx_rb_wptr_poll_addr_lo;
	u32 sdmax_rlcx_rb_aql_cntl;
	u32 sdmax_rlcx_minor_ptr_update;
	u32 sdmax_rlcx_midcmd_data0;
	u32 sdmax_rlcx_midcmd_data1;
	u32 sdmax_rlcx_midcmd_data2;
	u32 sdmax_rlcx_midcmd_data3;
	u32 sdmax_rlcx_midcmd_data4;
	u32 sdmax_rlcx_midcmd_data5;
	u32 sdmax_rlcx_midcmd_data6;
	u32 sdmax_rlcx_midcmd_data7;
	u32 sdmax_rlcx_midcmd_data8;
	u32 sdmax_rlcx_midcmd_cntl;
	u32 reserved_42;
	u32 reserved_43;
	u32 reserved_44;
	u32 reserved_45;
	u32 reserved_46;
	u32 reserved_47;
	u32 reserved_48;
	u32 reserved_49;
	u32 reserved_50;
	u32 reserved_51;
	u32 reserved_52;
	u32 reserved_53;
	u32 reserved_54;
	u32 reserved_55;
	u32 reserved_56;
	u32 reserved_57;
	u32 reserved_58;
	u32 reserved_59;
	u32 reserved_60;
	u32 reserved_61;
	u32 reserved_62;
	u32 reserved_63;
	u32 reserved_64;
	u32 reserved_65;
	u32 reserved_66;
	u32 reserved_67;
	u32 reserved_68;
	u32 reserved_69;
	u32 reserved_70;
	u32 reserved_71;
	u32 reserved_72;
	u32 reserved_73;
	u32 reserved_74;
	u32 reserved_75;
	u32 reserved_76;
	u32 reserved_77;
	u32 reserved_78;
	u32 reserved_79;
	u32 reserved_80;
	u32 reserved_81;
	u32 reserved_82;
	u32 reserved_83;
	u32 reserved_84;
	u32 reserved_85;
	u32 reserved_86;
	u32 reserved_87;
	u32 reserved_88;
	u32 reserved_89;
	u32 reserved_90;
	u32 reserved_91;
	u32 reserved_92;
	u32 reserved_93;
	u32 reserved_94;
	u32 reserved_95;
	u32 reserved_96;
	u32 reserved_97;
	u32 reserved_98;
	u32 reserved_99;
	u32 reserved_100;
	u32 reserved_101;
	u32 reserved_102;
	u32 reserved_103;
	u32 reserved_104;
	u32 reserved_105;
	u32 reserved_106;
	u32 reserved_107;
	u32 reserved_108;
	u32 reserved_109;
	u32 reserved_110;
	u32 reserved_111;
	u32 reserved_112;
	u32 reserved_113;
	u32 reserved_114;
	u32 reserved_115;
	u32 reserved_116;
	u32 reserved_117;
	u32 reserved_118;
	u32 reserved_119;
	u32 reserved_120;
	u32 reserved_121;
	u32 reserved_122;
	u32 reserved_123;
	u32 reserved_124;
	u32 reserved_125;
	u32 reserved_126;
	u32 reserved_127;
	u32 sdma_engine_id;
	u32 sdma_queue_id;
}

#[repr(C)]
pub struct v10_compute_mqd {
	u32 header;
	u32 compute_dispatch_initiator;
	u32 compute_dim_x;
	u32 compute_dim_y;
	u32 compute_dim_z;
	u32 compute_start_x;
	u32 compute_start_y;
	u32 compute_start_z;
	u32 compute_num_thread_x;
	u32 compute_num_thread_y;
	u32 compute_num_thread_z;
	u32 compute_pipelinestat_enable;
	u32 compute_perfcount_enable;
	u32 compute_pgm_lo;
	u32 compute_pgm_hi;
	u32 compute_tba_lo;
	u32 compute_tba_hi;
	u32 compute_tma_lo;
	u32 compute_tma_hi;
	u32 compute_pgm_rsrc1;
	u32 compute_pgm_rsrc2;
	u32 compute_vmid;
	u32 compute_resource_limits;
	u32 compute_static_thread_mgmt_se0;
	u32 compute_static_thread_mgmt_se1;
	u32 compute_tmpring_size;
	u32 compute_static_thread_mgmt_se2;
	u32 compute_static_thread_mgmt_se3;
	u32 compute_restart_x;
	u32 compute_restart_y;
	u32 compute_restart_z;
	u32 compute_thread_trace_enable;
	u32 compute_misc_reserved;
	u32 compute_dispatch_id;
	u32 compute_threadgroup_id;
	u32 compute_relaunch;
	u32 compute_wave_restore_addr_lo;
	u32 compute_wave_restore_addr_hi;
	u32 compute_wave_restore_control;
	u32 reserved_39;
	u32 reserved_40;
	u32 reserved_41;
	u32 reserved_42;
	u32 reserved_43;
	u32 reserved_44;
	u32 reserved_45;
	u32 reserved_46;
	u32 reserved_47;
	u32 reserved_48;
	u32 reserved_49;
	u32 reserved_50;
	u32 reserved_51;
	u32 reserved_52;
	u32 reserved_53;
	u32 reserved_54;
	u32 reserved_55;
	u32 reserved_56;
	u32 reserved_57;
	u32 reserved_58;
	u32 reserved_59;
	u32 reserved_60;
	u32 reserved_61;
	u32 reserved_62;
	u32 reserved_63;
	u32 reserved_64;
	u32 compute_user_data_0;
	u32 compute_user_data_1;
	u32 compute_user_data_2;
	u32 compute_user_data_3;
	u32 compute_user_data_4;
	u32 compute_user_data_5;
	u32 compute_user_data_6;
	u32 compute_user_data_7;
	u32 compute_user_data_8;
	u32 compute_user_data_9;
	u32 compute_user_data_10;
	u32 compute_user_data_11;
	u32 compute_user_data_12;
	u32 compute_user_data_13;
	u32 compute_user_data_14;
	u32 compute_user_data_15;
	u32 cp_compute_csinvoc_count_lo;
	u32 cp_compute_csinvoc_count_hi;
	u32 reserved_83;
	u32 reserved_84;
	u32 reserved_85;
	u32 cp_mqd_query_time_lo;
	u32 cp_mqd_query_time_hi;
	u32 cp_mqd_connect_start_time_lo;
	u32 cp_mqd_connect_start_time_hi;
	u32 cp_mqd_connect_end_time_lo;
	u32 cp_mqd_connect_end_time_hi;
	u32 cp_mqd_connect_end_wf_count;
	u32 cp_mqd_connect_end_pq_rptr;
	u32 cp_mqd_connect_end_pq_wptr;
	u32 cp_mqd_connect_end_ib_rptr;
	u32 cp_mqd_readindex_lo;
	u32 cp_mqd_readindex_hi;
	u32 cp_mqd_save_start_time_lo;
	u32 cp_mqd_save_start_time_hi;
	u32 cp_mqd_save_end_time_lo;
	u32 cp_mqd_save_end_time_hi;
	u32 cp_mqd_restore_start_time_lo;
	u32 cp_mqd_restore_start_time_hi;
	u32 cp_mqd_restore_end_time_lo;
	u32 cp_mqd_restore_end_time_hi;
	u32 disable_queue;
	u32 reserved_107;
	u32 gds_cs_ctxsw_cnt0;
	u32 gds_cs_ctxsw_cnt1;
	u32 gds_cs_ctxsw_cnt2;
	u32 gds_cs_ctxsw_cnt3;
	u32 reserved_112;
	u32 reserved_113;
	u32 cp_pq_exe_status_lo;
	u32 cp_pq_exe_status_hi;
	u32 cp_packet_id_lo;
	u32 cp_packet_id_hi;
	u32 cp_packet_exe_status_lo;
	u32 cp_packet_exe_status_hi;
	u32 gds_save_base_addr_lo;
	u32 gds_save_base_addr_hi;
	u32 gds_save_mask_lo;
	u32 gds_save_mask_hi;
	u32 ctx_save_base_addr_lo;
	u32 ctx_save_base_addr_hi;
	u32 reserved_126;
	u32 reserved_127;
	u32 cp_mqd_base_addr_lo;
	u32 cp_mqd_base_addr_hi;
	u32 cp_hqd_active;
	u32 cp_hqd_vmid;
	u32 cp_hqd_persistent_state;
	u32 cp_hqd_pipe_priority;
	u32 cp_hqd_queue_priority;
	u32 cp_hqd_quantum;
	u32 cp_hqd_pq_base_lo;
	u32 cp_hqd_pq_base_hi;
	u32 cp_hqd_pq_rptr;
	u32 cp_hqd_pq_rptr_report_addr_lo;
	u32 cp_hqd_pq_rptr_report_addr_hi;
	u32 cp_hqd_pq_wptr_poll_addr_lo;
	u32 cp_hqd_pq_wptr_poll_addr_hi;
	u32 cp_hqd_pq_doorbell_control;
	u32 reserved_144;
	u32 cp_hqd_pq_control;
	u32 cp_hqd_ib_base_addr_lo;
	u32 cp_hqd_ib_base_addr_hi;
	u32 cp_hqd_ib_rptr;
	u32 cp_hqd_ib_control;
	u32 cp_hqd_iq_timer;
	u32 cp_hqd_iq_rptr;
	u32 cp_hqd_dequeue_request;
	u32 cp_hqd_dma_offload;
	u32 cp_hqd_sema_cmd;
	u32 cp_hqd_msg_type;
	u32 cp_hqd_atomic0_preop_lo;
	u32 cp_hqd_atomic0_preop_hi;
	u32 cp_hqd_atomic1_preop_lo;
	u32 cp_hqd_atomic1_preop_hi;
	u32 cp_hqd_hq_scheduler0;
	u32 cp_hqd_hq_scheduler1;
	u32 cp_mqd_control;
	u32 cp_hqd_hq_status1;
	u32 cp_hqd_hq_control1;
	u32 cp_hqd_eop_base_addr_lo;
	u32 cp_hqd_eop_base_addr_hi;
	u32 cp_hqd_eop_control;
	u32 cp_hqd_eop_rptr;
	u32 cp_hqd_eop_wptr;
	u32 cp_hqd_eop_done_events;
	u32 cp_hqd_ctx_save_base_addr_lo;
	u32 cp_hqd_ctx_save_base_addr_hi;
	u32 cp_hqd_ctx_save_control;
	u32 cp_hqd_cntl_stack_offset;
	u32 cp_hqd_cntl_stack_size;
	u32 cp_hqd_wg_state_offset;
	u32 cp_hqd_ctx_save_size;
	u32 cp_hqd_gds_resource_state;
	u32 cp_hqd_error;
	u32 cp_hqd_eop_wptr_mem;
	u32 cp_hqd_aql_control;
	u32 cp_hqd_pq_wptr_lo;
	u32 cp_hqd_pq_wptr_hi;
	u32 cp_hqd_suspend_cntl_stack_offset;
	u32 cp_hqd_suspend_cntl_stack_dw_cnt;
	u32 cp_hqd_suspend_wg_state_offset;
	u32 reserved_187;
	u32 reserved_188;
	u32 reserved_189;
	u32 reserved_190;
	u32 reserved_191;
	u32 iqtimer_pkt_header;
	u32 iqtimer_pkt_dw0;
	u32 iqtimer_pkt_dw1;
	u32 iqtimer_pkt_dw2;
	u32 iqtimer_pkt_dw3;
	u32 iqtimer_pkt_dw4;
	u32 iqtimer_pkt_dw5;
	u32 iqtimer_pkt_dw6;
	u32 iqtimer_pkt_dw7;
	u32 iqtimer_pkt_dw8;
	u32 iqtimer_pkt_dw9;
	u32 iqtimer_pkt_dw10;
	u32 iqtimer_pkt_dw11;
	u32 iqtimer_pkt_dw12;
	u32 iqtimer_pkt_dw13;
	u32 iqtimer_pkt_dw14;
	u32 iqtimer_pkt_dw15;
	u32 iqtimer_pkt_dw16;
	u32 iqtimer_pkt_dw17;
	u32 iqtimer_pkt_dw18;
	u32 iqtimer_pkt_dw19;
	u32 iqtimer_pkt_dw20;
	u32 iqtimer_pkt_dw21;
	u32 iqtimer_pkt_dw22;
	u32 iqtimer_pkt_dw23;
	u32 iqtimer_pkt_dw24;
	u32 iqtimer_pkt_dw25;
	u32 iqtimer_pkt_dw26;
	u32 iqtimer_pkt_dw27;
	u32 iqtimer_pkt_dw28;
	u32 iqtimer_pkt_dw29;
	u32 iqtimer_pkt_dw30;
	u32 iqtimer_pkt_dw31;
	u32 reserved_225;
	u32 reserved_226;
	u32 reserved_227;
	u32 set_resources_header;
	u32 set_resources_dw1;
	u32 set_resources_dw2;
	u32 set_resources_dw3;
	u32 set_resources_dw4;
	u32 set_resources_dw5;
	u32 set_resources_dw6;
	u32 set_resources_dw7;
	u32 reserved_236;
	u32 reserved_237;
	u32 reserved_238;
	u32 reserved_239;
	u32 queue_doorbell_id0;
	u32 queue_doorbell_id1;
	u32 queue_doorbell_id2;
	u32 queue_doorbell_id3;
	u32 queue_doorbell_id4;
	u32 queue_doorbell_id5;
	u32 queue_doorbell_id6;
	u32 queue_doorbell_id7;
	u32 queue_doorbell_id8;
	u32 queue_doorbell_id9;
	u32 queue_doorbell_id10;
	u32 queue_doorbell_id11;
	u32 queue_doorbell_id12;
	u32 queue_doorbell_id13;
	u32 queue_doorbell_id14;
	u32 queue_doorbell_id15;
	u32 reserved_256;
	u32 reserved_257;
	u32 reserved_258;
	u32 reserved_259;
	u32 reserved_260;
	u32 reserved_261;
	u32 reserved_262;
	u32 reserved_263;
	u32 reserved_264;
	u32 reserved_265;
	u32 reserved_266;
	u32 reserved_267;
	u32 reserved_268;
	u32 reserved_269;
	u32 reserved_270;
	u32 reserved_271;
	u32 reserved_272;
	u32 reserved_273;
	u32 reserved_274;
	u32 reserved_275;
	u32 reserved_276;
	u32 reserved_277;
	u32 reserved_278;
	u32 reserved_279;
	u32 reserved_280;
	u32 reserved_281;
	u32 reserved_282;
	u32 reserved_283;
	u32 reserved_284;
	u32 reserved_285;
	u32 reserved_286;
	u32 reserved_287;
	u32 reserved_288;
	u32 reserved_289;
	u32 reserved_290;
	u32 reserved_291;
	u32 reserved_292;
	u32 reserved_293;
	u32 reserved_294;
	u32 reserved_295;
	u32 reserved_296;
	u32 reserved_297;
	u32 reserved_298;
	u32 reserved_299;
	u32 reserved_300;
	u32 reserved_301;
	u32 reserved_302;
	u32 reserved_303;
	u32 reserved_304;
	u32 reserved_305;
	u32 reserved_306;
	u32 reserved_307;
	u32 reserved_308;
	u32 reserved_309;
	u32 reserved_310;
	u32 reserved_311;
	u32 reserved_312;
	u32 reserved_313;
	u32 reserved_314;
	u32 reserved_315;
	u32 reserved_316;
	u32 reserved_317;
	u32 reserved_318;
	u32 reserved_319;
	u32 reserved_320;
	u32 reserved_321;
	u32 reserved_322;
	u32 reserved_323;
	u32 reserved_324;
	u32 reserved_325;
	u32 reserved_326;
	u32 reserved_327;
	u32 reserved_328;
	u32 reserved_329;
	u32 reserved_330;
	u32 reserved_331;
	u32 reserved_332;
	u32 reserved_333;
	u32 reserved_334;
	u32 reserved_335;
	u32 reserved_336;
	u32 reserved_337;
	u32 reserved_338;
	u32 reserved_339;
	u32 reserved_340;
	u32 reserved_341;
	u32 reserved_342;
	u32 reserved_343;
	u32 reserved_344;
	u32 reserved_345;
	u32 reserved_346;
	u32 reserved_347;
	u32 reserved_348;
	u32 reserved_349;
	u32 reserved_350;
	u32 reserved_351;
	u32 reserved_352;
	u32 reserved_353;
	u32 reserved_354;
	u32 reserved_355;
	u32 reserved_356;
	u32 reserved_357;
	u32 reserved_358;
	u32 reserved_359;
	u32 reserved_360;
	u32 reserved_361;
	u32 reserved_362;
	u32 reserved_363;
	u32 reserved_364;
	u32 reserved_365;
	u32 reserved_366;
	u32 reserved_367;
	u32 reserved_368;
	u32 reserved_369;
	u32 reserved_370;
	u32 reserved_371;
	u32 reserved_372;
	u32 reserved_373;
	u32 reserved_374;
	u32 reserved_375;
	u32 reserved_376;
	u32 reserved_377;
	u32 reserved_378;
	u32 reserved_379;
	u32 reserved_380;
	u32 reserved_381;
	u32 reserved_382;
	u32 reserved_383;
	u32 reserved_384;
	u32 reserved_385;
	u32 reserved_386;
	u32 reserved_387;
	u32 reserved_388;
	u32 reserved_389;
	u32 reserved_390;
	u32 reserved_391;
	u32 reserved_392;
	u32 reserved_393;
	u32 reserved_394;
	u32 reserved_395;
	u32 reserved_396;
	u32 reserved_397;
	u32 reserved_398;
	u32 reserved_399;
	u32 reserved_400;
	u32 reserved_401;
	u32 reserved_402;
	u32 reserved_403;
	u32 reserved_404;
	u32 reserved_405;
	u32 reserved_406;
	u32 reserved_407;
	u32 reserved_408;
	u32 reserved_409;
	u32 reserved_410;
	u32 reserved_411;
	u32 reserved_412;
	u32 reserved_413;
	u32 reserved_414;
	u32 reserved_415;
	u32 reserved_416;
	u32 reserved_417;
	u32 reserved_418;
	u32 reserved_419;
	u32 reserved_420;
	u32 reserved_421;
	u32 reserved_422;
	u32 reserved_423;
	u32 reserved_424;
	u32 reserved_425;
	u32 reserved_426;
	u32 reserved_427;
	u32 reserved_428;
	u32 reserved_429;
	u32 reserved_430;
	u32 reserved_431;
	u32 reserved_432;
	u32 reserved_433;
	u32 reserved_434;
	u32 reserved_435;
	u32 reserved_436;
	u32 reserved_437;
	u32 reserved_438;
	u32 reserved_439;
	u32 reserved_440;
	u32 reserved_441;
	u32 reserved_442;
	u32 reserved_443;
	u32 reserved_444;
	u32 reserved_445;
	u32 reserved_446;
	u32 reserved_447;
	u32 reserved_448;
	u32 reserved_449;
	u32 reserved_450;
	u32 reserved_451;
	u32 reserved_452;
	u32 reserved_453;
	u32 reserved_454;
	u32 reserved_455;
	u32 reserved_456;
	u32 reserved_457;
	u32 reserved_458;
	u32 reserved_459;
	u32 reserved_460;
	u32 reserved_461;
	u32 reserved_462;
	u32 reserved_463;
	u32 reserved_464;
	u32 reserved_465;
	u32 reserved_466;
	u32 reserved_467;
	u32 reserved_468;
	u32 reserved_469;
	u32 reserved_470;
	u32 reserved_471;
	u32 reserved_472;
	u32 reserved_473;
	u32 reserved_474;
	u32 reserved_475;
	u32 reserved_476;
	u32 reserved_477;
	u32 reserved_478;
	u32 reserved_479;
	u32 reserved_480;
	u32 reserved_481;
	u32 reserved_482;
	u32 reserved_483;
	u32 reserved_484;
	u32 reserved_485;
	u32 reserved_486;
	u32 reserved_487;
	u32 reserved_488;
	u32 reserved_489;
	u32 reserved_490;
	u32 reserved_491;
	u32 reserved_492;
	u32 reserved_493;
	u32 reserved_494;
	u32 reserved_495;
	u32 reserved_496;
	u32 reserved_497;
	u32 reserved_498;
	u32 reserved_499;
	u32 reserved_500;
	u32 reserved_501;
	u32 reserved_502;
	u32 reserved_503;
	u32 reserved_504;
	u32 reserved_505;
	u32 reserved_506;
	u32 reserved_507;
	u32 reserved_508;
	u32 reserved_509;
	u32 reserved_510;
	u32 reserved_511;
}

#[repr(C)]
pub struct v10_ce_ib_state {
	/* section of non chained ib part */
	u32 ce_ib_completion_status;
	u32 ce_constegnine_count;
	u32 ce_ibOffset_ib1;
	u32 ce_ibOffset_ib2;

	/* section of chained ib */
	u32 ce_chainib_addrlo_ib1;
	u32 ce_chainib_addrlo_ib2;
	u32 ce_chainib_addrhi_ib1;
	u32 ce_chainib_addrhi_ib2;
	u32 ce_chainib_size_ib1;
	u32 ce_chainib_size_ib2;
} /* total 10 DWORD */

#[repr(C)]
pub struct v10_de_ib_state {
	/* section of non chained ib part */
	u32 ib_completion_status;
	u32 de_constEngine_count;
	u32 ib_offset_ib1;
	u32 ib_offset_ib2;

	/* section of chained ib */
	u32 chain_ib_addrlo_ib1;
	u32 chain_ib_addrlo_ib2;
	u32 chain_ib_addrhi_ib1;
	u32 chain_ib_addrhi_ib2;
	u32 chain_ib_size_ib1;
	u32 chain_ib_size_ib2;

	/* section of non chained ib part */
	u32 preamble_begin_ib1;
	u32 preamble_begin_ib2;
	u32 preamble_end_ib1;
	u32 preamble_end_ib2;

	/* section of chained ib */
	u32 chain_ib_pream_addrlo_ib1;
	u32 chain_ib_pream_addrlo_ib2;
	u32 chain_ib_pream_addrhi_ib1;
	u32 chain_ib_pream_addrhi_ib2;

	/* section of non chained ib part */
	u32 draw_indirect_baseLo;
	u32 draw_indirect_baseHi;
	u32 disp_indirect_baseLo;
	u32 disp_indirect_baseHi;
	u32 gds_backup_addrlo;
	u32 gds_backup_addrhi;
	u32 index_base_addrlo;
	u32 index_base_addrhi;
	u32 sample_cntl;
} /* Total of 27 DWORD */

#[repr(C)]
pub struct v10_gfx_meta_data {
	/* 10 DWORD, address must be 4KB aligned */
	ce_payload: v10_ce_ib_state;
	u32 reserved1[54];
	/* 27 DWORD, address must be 64B aligned */
	de_payload: v10_de_ib_state;
	/* PFP IB base address which get pre-empted */
	u32 DeIbBaseAddrLo;
	u32 DeIbBaseAddrHi;
	u32 reserved2[931];
} /* Total of 4K Bytes */



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
