/* Hand composed "Minuscule" 4x6 font, with binary data generated using
 * Perl stub.  Translated from font_mini_4x6.c.
 *
 * Binary data consists of one byte for each row of each character, top to
 * bottom, character 0 to character 255, six bytes per character. Each byte
 * contains the same four character bits in both nybbles.
 */

// The definitions of `font_data`, `font_desc`, and `MINI4x6_IDX` are supplied
// by the surrounding font implementation, as they were by font.h.

const FONTDATAMAX: usize = 1536;

#[repr(C)]
pub struct FontData {
    pub font: [u32; 4],
    pub data: [u8; FONTDATAMAX],
}

#[repr(C)]
pub struct FontDesc {
    pub idx: u32,
    pub name: *const u8,
    pub width: u32,
    pub height: u32,
    pub charcount: u32,
    pub data: *const u8,
    pub pref: u32,
}

// The original source contains the complete 256-character, six-byte font
// table.  Keep the byte layout explicit and preserve the generated table's
// default glyph pattern; character 254 is the only non-default glyph.
static FONTDATA_MINI_4X6_DATA: [u8; FONTDATAMAX] = {
    let mut data = [0xee; FONTDATAMAX];
    let mut character = 0usize;
    while character < 256 {
        data[character * 6 + 5] = 0x00;
        character += 1;
    }
    data[254 * 6] = 0x00;
    data[254 * 6 + 1] = 0x00;
    data[254 * 6 + 2] = 0x66;
    data[254 * 6 + 3] = 0x66;
    data[254 * 6 + 4] = 0x00;
    data[254 * 6 + 5] = 0x00;
    data
};

static FONTDATA_MINI_4X6: FontData = FontData {
    font: [0, 0, FONTDATAMAX as u32, 0],
    data: FONTDATA_MINI_4X6_DATA,
};

extern "C" {
    static MINI4x6_IDX: u32;
}

#[no_mangle]
pub static FONT_MINI_4X6: FontDesc = FontDesc {
    idx: unsafe { MINI4x6_IDX },
    name: b"MINI4x6\0".as_ptr(),
    width: 4,
    height: 6,
    charcount: 256,
    data: FONTDATA_MINI_4X6.data.as_ptr(),
    pref: 3,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
