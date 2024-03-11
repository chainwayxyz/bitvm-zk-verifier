ENTRY(_start);

MEMORY
{
    code (RX) : ORIGIN = 0x10000, LENGTH = 0x1000000
    ram (RW!X) : ORIGIN = 0x110000, LENGTH = 0x1000000
}
 
SECTIONS {
         .text : ALIGN(4096) {
    		*(.text._start)
    		*(.text.__start)
		*(.init)
    		*(.text*)
	} >code
	
	. = 0x20000;
	.rodata : ALIGN(4096) {
    		*(.rodata*)
    		*(.srodata*)
	} >ram
	.eh_frame : { KEEP (*(.eh_frame)) } >ram
	.data : ALIGN(4096) {
	    *(.data .data.*)
	    *(.gnu.linkonce.d.*)
	    __global_pointer$ = . + 0x800;
	    *(.sdata .sdata.* .sdata2.*)
	    *(.gnu.linkonce.s.*)
	} >ram
	.got : ALIGN(4096) {
		*(.got .got.*);
	} >ram
	/* empty space here */
	.bss : ALIGN(4096) {
	    __bss_begin = .;
	    *(.sbss*)
	    *(.gnu.linkonce.sb.*)
	    *(.bss .bss.*)
	    *(.gnu.linkonce.b.*)
	    __bss_end = .;
	} >ram
	 __bss_size = __bss_end - __bss_begin;
	.input : ALIGN(4096) {
		__input_begin = .;
		. = . + 300*1024;
		__input_end = .;
	} >ram

	__input_maxlength = __input_end - __input_begin;

	.stack : ALIGN(4096) {
		. += 8*1024;
		__stack_init$ = . - 8;
        } >ram
	__heap_start = .;
	

	/* Include entry point at start of binary */
	 .stab          0 : { *(.stab) }
	 .stabstr       0 : { *(.stabstr) }
	 .stab.excl     0 : { *(.stab.excl) }
  	 .stab.exclstr  0 : { *(.stab.exclstr) }
  	 .stab.index    0 : { *(.stab.index) }
  	 .stab.indexstr 0 : { *(.stab.indexstr) }
  	 .comment       0 : { *(.comment) }
  	/* DWARF debug sections.
     Symbols in the DWARF debugging sections are relative to the beginning
     of the section so we begin them at 0.  */
  /* DWARF 1 */
  	.debug          0 : { *(.debug) }
  	.line           0 : { *(.line) }
  /* GNU DWARF 1 extensions */
  	.debug_srcinfo  0 : { *(.debug_srcinfo) }
  	.debug_sfnames  0 : { *(.debug_sfnames) }
  /* DWARF 1.1 and DWARF 2 */
  	.debug_aranges  0 : { *(.debug_aranges) }
  	.debug_pubnames 0 : { *(.debug_pubnames) }
  /* DWARF 2 */
  	.debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
  	.debug_abbrev   0 : { *(.debug_abbrev) }
  	.debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end ) }
 	.debug_frame    0 : { *(.debug_frame) }
  	.debug_str      0 : { *(.debug_str) }
  	.debug_loc      0 : { *(.debug_loc) }
  	.debug_macinfo  0 : { *(.debug_macinfo) }
  /* SGI/MIPS DWARF 2 extensions */
  	.debug_weaknames 0 : { *(.debug_weaknames) }
  	.debug_funcnames 0 : { *(.debug_funcnames) }
  	.debug_typenames 0 : { *(.debug_typenames) }
  	.debug_varnames  0 : { *(.debug_varnames) }
  /* DWARF 3 */
  	.debug_pubtypes 0 : { *(.debug_pubtypes) }
  	.debug_ranges   0 : { *(.debug_ranges) }
  /* DWARF Extension.  */
  	.debug_macro    0 : { *(.debug_macro) }
  	.gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }
	/DISCARD/ : {
		/* ABI crap starts here */
		*(.MIPS.abiflags)
		*(.MIPS.options)
		*(.options)
		*(.pdr)
		*(.reginfo)
		*(.eh_frame)
    *(.rel*)
    *(.comment)
    *(.riscv.attributes)
	}
}
