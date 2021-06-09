MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  FLASH : ORIGIN = 0x00000000, LENGTH = 0x00008000
  FLASH_CONFIG : ORIGIN = 0x00000400, LENGTH = 0x00000010
  RAM : ORIGIN = 0x1FFFFC00, LENGTH = 0x00001000
  EEPROM : ORIGIN = 0x10000000, LENGTH = 0x000000FF
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
  _stext = ORIGIN(FLASH) + 0x410;

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/

SECTIONS {
  .eeprom (NOLOAD):
  {
    KEEP(*(.eeprom))
  } > EEPROM

  .flash_config :
  {
    . = ALIGN(4);
    KEEP(*(.FlashConfig))
    . = ALIGN(4);
  } > FLASH_CONFIG
}
