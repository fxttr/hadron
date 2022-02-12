ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

BASE =  0xFFFFFFFF80000000;

SECTIONS {
    . = 0x100000;
    . += SIZEOF_HEADERS;

    .init : AT(ADDR(.init)) {
	KEEP( *(.multiboot) )
	*(.itext)
    }

    . += BASE;

    .text ALIGN(0x1000) : AT(ADDR(.text) - BASE)
    {
        *(.text .text.*)
    }

    .rodata ALIGN(0x1000) : AT(ADDR(.rodata) - BASE)
    {
	*(.rodata .rodata.*)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - BASE)
    {
	*(.data .data.*)
    }

    .bss : AT(ADDR(.bss) - BASE)
    {
	*(.bss .bss.*)
    }

    kernel_end = .;
}
