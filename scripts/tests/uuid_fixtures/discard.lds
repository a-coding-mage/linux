/* Private userspace link: discard the same non-runtime kernel metadata. */
SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .text;
