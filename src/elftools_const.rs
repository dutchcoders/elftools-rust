// AUTO-GENERATED FILE, DO NOT EDIT [elftools_const.rs]

extern crate libc;

// This file defines standard ELF types, structures, and macros.
// Copyright (C) 1995-2012 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// The GNU C Library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// The GNU C Library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with the GNU C Library; if not, see
// <http://www.gnu.org/licenses/>.






#[warn(non_camel_case_types)]
pub const _ELF_H: i32 = 1;










// Standard ELF types.








// Type for a 16-bit quantity.


#[allow(non_camel_case_types)]
pub type Elf32_Half = u16;


#[allow(non_camel_case_types)]
pub type Elf64_Half = u16;




// Types for signed and unsigned 32-bit quantities.


#[allow(non_camel_case_types)]
pub type Elf32_Word = u32;


#[allow(non_camel_case_types)]
pub type Elf32_Sword = i32;


#[allow(non_camel_case_types)]
pub type Elf64_Word = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Sword = i32;




// Types for signed and unsigned 64-bit quantities.


#[allow(non_camel_case_types)]
pub type Elf32_Xword = u64;


#[allow(non_camel_case_types)]
pub type Elf32_Sxword = i64;


#[allow(non_camel_case_types)]
pub type Elf64_Xword = u64;


#[allow(non_camel_case_types)]
pub type Elf64_Sxword = i64;




// Type of addresses.


#[allow(non_camel_case_types)]
pub type Elf32_Addr = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Addr = u64;




// Type of file offsets.


#[allow(non_camel_case_types)]
pub type Elf32_Off = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Off = u64;




// Type for section indices, which are 16-bit quantities.


#[allow(non_camel_case_types)]
pub type Elf32_Section = u16;


#[allow(non_camel_case_types)]
pub type Elf64_Section = u16;




// Type for version symbol information.


#[allow(non_camel_case_types)]
pub type Elf32_Versym = Elf32_Half;


#[allow(non_camel_case_types)]
pub type Elf64_Versym = Elf64_Half;






// The ELF file header.  This appears at the start of every ELF file.




#[warn(non_camel_case_types)]
pub const EI_NIDENT: usize = (16);


#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; EI_NIDENT], // Magic number and other info
    pub e_type: Elf32_Half, // Object file type
    pub e_machine: Elf32_Half, // Architecture
    pub e_version: Elf32_Word, // Object file version
    pub e_entry: Elf32_Addr, // Entry point virtual address
    pub e_phoff: Elf32_Off, // Program header table file offset
    pub e_shoff: Elf32_Off, // Section header table file offset
    pub e_flags: Elf32_Word, // Processor-specific flags
    pub e_ehsize: Elf32_Half, // ELF header size in bytes
    pub e_phentsize: Elf32_Half, // Program header table entry size
    pub e_phnum: Elf32_Half, // Program header table entry count
    pub e_shentsize: Elf32_Half, // Section header table entry size
    pub e_shnum: Elf32_Half, // Section header table entry count
    pub e_shstrndx: Elf32_Half, // Section header string table index
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; EI_NIDENT], // Magic number and other info
    pub e_type: Elf64_Half, // Object file type
    pub e_machine: Elf64_Half, // Architecture
    pub e_version: Elf64_Word, // Object file version
    pub e_entry: Elf64_Addr, // Entry point virtual address
    pub e_phoff: Elf64_Off, // Program header table file offset
    pub e_shoff: Elf64_Off, // Section header table file offset
    pub e_flags: Elf64_Word, // Processor-specific flags
    pub e_ehsize: Elf64_Half, // ELF header size in bytes
    pub e_phentsize: Elf64_Half, // Program header table entry size
    pub e_phnum: Elf64_Half, // Program header table entry count
    pub e_shentsize: Elf64_Half, // Section header table entry size
    pub e_shnum: Elf64_Half, // Section header table entry count
    pub e_shstrndx: Elf64_Half, // Section header string table index
}




// Fields in the e_ident array.  The EI_* macros are indices into the
// array.  The macros under each EI_* macro are the values the byte
// may have.




#[warn(non_camel_case_types)]
pub const EI_MAG0: i32 = 0; /* File identification byte 0 index */
#[warn(non_camel_case_types)]
pub const ELFMAG0: i32 = 0x7f; /* Magic number byte 0 */


#[warn(non_camel_case_types)]
pub const EI_MAG1: i32 = 1; /* File identification byte 1 index */




#[warn(non_camel_case_types)]
pub const EI_MAG2: i32 = 2; /* File identification byte 2 index */




#[warn(non_camel_case_types)]
pub const EI_MAG3: i32 = 3; /* File identification byte 3 index */




// Conglomeration of the identification bytes, for easy testing as a word.




#[warn(non_camel_case_types)]
pub const SELFMAG: i32 = 4;


#[warn(non_camel_case_types)]
pub const EI_CLASS: i32 = 4; /* File class byte index */
#[warn(non_camel_case_types)]
pub const ELFCLASSNONE: i32 = 0; /* Invalid class */
#[warn(non_camel_case_types)]
pub const ELFCLASS32: i32 = 1; /* 32-bit objects */
#[warn(non_camel_case_types)]
pub const ELFCLASS64: i32 = 2; /* 64-bit objects */
#[warn(non_camel_case_types)]
pub const ELFCLASSNUM: i32 = 3;


#[warn(non_camel_case_types)]
pub const EI_DATA: i32 = 5; /* Data encoding byte index */
#[warn(non_camel_case_types)]
pub const ELFDATANONE: i32 = 0; /* Invalid data encoding */
#[warn(non_camel_case_types)]
pub const ELFDATA2LSB: i32 = 1; /* 2's complement, little endian */
#[warn(non_camel_case_types)]
pub const ELFDATA2MSB: i32 = 2; /* 2's complement, big endian */
#[warn(non_camel_case_types)]
pub const ELFDATANUM: i32 = 3;


#[warn(non_camel_case_types)]
pub const EI_VERSION: i32 = 6; /* File version byte index */




#[warn(non_camel_case_types)]
pub const EI_OSABI: i32 = 7; /* OS ABI identification */
#[warn(non_camel_case_types)]
pub const ELFOSABI_NONE: i32 = 0; /* UNIX System V ABI */
#[warn(non_camel_case_types)]
pub const ELFOSABI_SYSV: i32 = 0; /* Alias.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_HPUX: i32 = 1; /* HP-UX */
#[warn(non_camel_case_types)]
pub const ELFOSABI_NETBSD: i32 = 2; /* NetBSD.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_GNU: i32 = 3; /* Object uses GNU ELF extensions.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_LINUX: i32 = ELFOSABI_GNU; /* Compatibility alias.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_SOLARIS: i32 = 6; /* Sun Solaris.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_AIX: i32 = 7; /* IBM AIX.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_IRIX: i32 = 8; /* SGI Irix.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_FREEBSD: i32 = 9; /* FreeBSD.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_TRU64: i32 = 10; /* Compaq TRU64 UNIX.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_MODESTO: i32 = 11; /* Novell Modesto.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_OPENBSD: i32 = 12; /* OpenBSD.  */
#[warn(non_camel_case_types)]
pub const ELFOSABI_ARM_AEABI: i32 = 64; /* ARM EABI */
#[warn(non_camel_case_types)]
pub const ELFOSABI_ARM: i32 = 97; /* ARM */
#[warn(non_camel_case_types)]
pub const ELFOSABI_STANDALONE: i32 = 255; /* Standalone (embedded) application */


#[warn(non_camel_case_types)]
pub const EI_ABIVERSION: i32 = 8; /* ABI version */


#[warn(non_camel_case_types)]
pub const EI_PAD: i32 = 9; /* Byte index of padding bytes */


// Legal values for e_type (object file type).




#[warn(non_camel_case_types)]
pub const ET_NONE: i32 = 0; /* No file type */
#[warn(non_camel_case_types)]
pub const ET_REL: i32 = 1; /* Relocatable file */
#[warn(non_camel_case_types)]
pub const ET_EXEC: i32 = 2; /* Executable file */
#[warn(non_camel_case_types)]
pub const ET_DYN: i32 = 3; /* Shared object file */
#[warn(non_camel_case_types)]
pub const ET_CORE: i32 = 4; /* Core file */
#[warn(non_camel_case_types)]
pub const ET_NUM: i32 = 5; /* Number of defined types */
#[warn(non_camel_case_types)]
pub const ET_LOOS: i32 = 0xfe00; /* OS-specific range start */
#[warn(non_camel_case_types)]
pub const ET_HIOS: i32 = 0xfeff; /* OS-specific range end */
#[warn(non_camel_case_types)]
pub const ET_LOPROC: i32 = 0xff00; /* Processor-specific range start */
#[warn(non_camel_case_types)]
pub const ET_HIPROC: i32 = 0xffff; /* Processor-specific range end */


// Legal values for e_machine (architecture).




#[warn(non_camel_case_types)]
pub const EM_NONE: i32 = 0; /* No machine */
#[warn(non_camel_case_types)]
pub const EM_M32: i32 = 1; /* AT&T WE 32100 */
#[warn(non_camel_case_types)]
pub const EM_SPARC: i32 = 2; /* SUN SPARC */
#[warn(non_camel_case_types)]
pub const EM_386: i32 = 3; /* Intel 80386 */
#[warn(non_camel_case_types)]
pub const EM_68K: i32 = 4; /* Motorola m68k family */
#[warn(non_camel_case_types)]
pub const EM_88K: i32 = 5; /* Motorola m88k family */
#[warn(non_camel_case_types)]
pub const EM_860: i32 = 7; /* Intel 80860 */
#[warn(non_camel_case_types)]
pub const EM_MIPS: i32 = 8; /* MIPS R3000 big-endian */
#[warn(non_camel_case_types)]
pub const EM_S370: i32 = 9; /* IBM System/370 */
#[warn(non_camel_case_types)]
pub const EM_MIPS_RS3_LE: i32 = 10; /* MIPS R3000 little-endian */


#[warn(non_camel_case_types)]
pub const EM_PARISC: i32 = 15; /* HPPA */
#[warn(non_camel_case_types)]
pub const EM_VPP500: i32 = 17; /* Fujitsu VPP500 */
#[warn(non_camel_case_types)]
pub const EM_SPARC32PLUS: i32 = 18; /* Sun's "v8plus" */
#[warn(non_camel_case_types)]
pub const EM_960: i32 = 19; /* Intel 80960 */
#[warn(non_camel_case_types)]
pub const EM_PPC: i32 = 20; /* PowerPC */
#[warn(non_camel_case_types)]
pub const EM_PPC64: i32 = 21; /* PowerPC 64-bit */
#[warn(non_camel_case_types)]
pub const EM_S390: i32 = 22; /* IBM S390 */


#[warn(non_camel_case_types)]
pub const EM_V800: i32 = 36; /* NEC V800 series */
#[warn(non_camel_case_types)]
pub const EM_FR20: i32 = 37; /* Fujitsu FR20 */
#[warn(non_camel_case_types)]
pub const EM_RH32: i32 = 38; /* TRW RH-32 */
#[warn(non_camel_case_types)]
pub const EM_RCE: i32 = 39; /* Motorola RCE */
#[warn(non_camel_case_types)]
pub const EM_ARM: i32 = 40; /* ARM */
#[warn(non_camel_case_types)]
pub const EM_FAKE_ALPHA: i32 = 41; /* Digital Alpha */
#[warn(non_camel_case_types)]
pub const EM_SH: i32 = 42; /* Hitachi SH */
#[warn(non_camel_case_types)]
pub const EM_SPARCV9: i32 = 43; /* SPARC v9 64-bit */
#[warn(non_camel_case_types)]
pub const EM_TRICORE: i32 = 44; /* Siemens Tricore */
#[warn(non_camel_case_types)]
pub const EM_ARC: i32 = 45; /* Argonaut RISC Core */
#[warn(non_camel_case_types)]
pub const EM_H8_300: i32 = 46; /* Hitachi H8/300 */
#[warn(non_camel_case_types)]
pub const EM_H8_300H: i32 = 47; /* Hitachi H8/300H */
#[warn(non_camel_case_types)]
pub const EM_H8S: i32 = 48; /* Hitachi H8S */
#[warn(non_camel_case_types)]
pub const EM_H8_500: i32 = 49; /* Hitachi H8/500 */
#[warn(non_camel_case_types)]
pub const EM_IA_64: i32 = 50; /* Intel Merced */
#[warn(non_camel_case_types)]
pub const EM_MIPS_X: i32 = 51; /* Stanford MIPS-X */
#[warn(non_camel_case_types)]
pub const EM_COLDFIRE: i32 = 52; /* Motorola Coldfire */
#[warn(non_camel_case_types)]
pub const EM_68HC12: i32 = 53; /* Motorola M68HC12 */
#[warn(non_camel_case_types)]
pub const EM_MMA: i32 = 54; /* Fujitsu MMA Multimedia Accelerator*/
#[warn(non_camel_case_types)]
pub const EM_PCP: i32 = 55; /* Siemens PCP */
#[warn(non_camel_case_types)]
pub const EM_NCPU: i32 = 56; /* Sony nCPU embeeded RISC */
#[warn(non_camel_case_types)]
pub const EM_NDR1: i32 = 57; /* Denso NDR1 microprocessor */
#[warn(non_camel_case_types)]
pub const EM_STARCORE: i32 = 58; /* Motorola Start*Core processor */
#[warn(non_camel_case_types)]
pub const EM_ME16: i32 = 59; /* Toyota ME16 processor */
#[warn(non_camel_case_types)]
pub const EM_ST100: i32 = 60; /* STMicroelectronic ST100 processor */
#[warn(non_camel_case_types)]
pub const EM_TINYJ: i32 = 61; /* Advanced Logic Corp. Tinyj emb.fam*/
#[warn(non_camel_case_types)]
pub const EM_X86_64: i32 = 62; /* AMD x86-64 architecture */
#[warn(non_camel_case_types)]
pub const EM_PDSP: i32 = 63; /* Sony DSP Processor */


#[warn(non_camel_case_types)]
pub const EM_FX66: i32 = 66; /* Siemens FX66 microcontroller */
#[warn(non_camel_case_types)]
pub const EM_ST9PLUS: i32 = 67; /* STMicroelectronics ST9+ 8/16 mc */
#[warn(non_camel_case_types)]
pub const EM_ST7: i32 = 68; /* STmicroelectronics ST7 8 bit mc */
#[warn(non_camel_case_types)]
pub const EM_68HC16: i32 = 69; /* Motorola MC68HC16 microcontroller */
#[warn(non_camel_case_types)]
pub const EM_68HC11: i32 = 70; /* Motorola MC68HC11 microcontroller */
#[warn(non_camel_case_types)]
pub const EM_68HC08: i32 = 71; /* Motorola MC68HC08 microcontroller */
#[warn(non_camel_case_types)]
pub const EM_68HC05: i32 = 72; /* Motorola MC68HC05 microcontroller */
#[warn(non_camel_case_types)]
pub const EM_SVX: i32 = 73; /* Silicon Graphics SVx */
#[warn(non_camel_case_types)]
pub const EM_ST19: i32 = 74; /* STMicroelectronics ST19 8 bit mc */
#[warn(non_camel_case_types)]
pub const EM_VAX: i32 = 75; /* Digital VAX */
#[warn(non_camel_case_types)]
pub const EM_CRIS: i32 = 76; /* Axis Communications 32-bit embedded processor */
#[warn(non_camel_case_types)]
pub const EM_JAVELIN: i32 = 77; /* Infineon Technologies 32-bit embedded processor */
#[warn(non_camel_case_types)]
pub const EM_FIREPATH: i32 = 78; /* Element 14 64-bit DSP Processor */
#[warn(non_camel_case_types)]
pub const EM_ZSP: i32 = 79; /* LSI Logic 16-bit DSP Processor */
#[warn(non_camel_case_types)]
pub const EM_MMIX: i32 = 80; /* Donald Knuth's educational 64-bit processor */
#[warn(non_camel_case_types)]
pub const EM_HUANY: i32 = 81; /* Harvard University machine-independent object files */
#[warn(non_camel_case_types)]
pub const EM_PRISM: i32 = 82; /* SiTera Prism */
#[warn(non_camel_case_types)]
pub const EM_AVR: i32 = 83; /* Atmel AVR 8-bit microcontroller */
#[warn(non_camel_case_types)]
pub const EM_FR30: i32 = 84; /* Fujitsu FR30 */
#[warn(non_camel_case_types)]
pub const EM_D10V: i32 = 85; /* Mitsubishi D10V */
#[warn(non_camel_case_types)]
pub const EM_D30V: i32 = 86; /* Mitsubishi D30V */
#[warn(non_camel_case_types)]
pub const EM_V850: i32 = 87; /* NEC v850 */
#[warn(non_camel_case_types)]
pub const EM_M32R: i32 = 88; /* Mitsubishi M32R */
#[warn(non_camel_case_types)]
pub const EM_MN10300: i32 = 89; /* Matsushita MN10300 */
#[warn(non_camel_case_types)]
pub const EM_MN10200: i32 = 90; /* Matsushita MN10200 */
#[warn(non_camel_case_types)]
pub const EM_PJ: i32 = 91; /* picoJava */
#[warn(non_camel_case_types)]
pub const EM_OPENRISC: i32 = 92; /* OpenRISC 32-bit embedded processor */
#[warn(non_camel_case_types)]
pub const EM_ARC_A5: i32 = 93; /* ARC Cores Tangent-A5 */
#[warn(non_camel_case_types)]
pub const EM_XTENSA: i32 = 94; /* Tensilica Xtensa Architecture */
#[warn(non_camel_case_types)]
pub const EM_TILEPRO: i32 = 188; /* Tilera TILEPro */
#[warn(non_camel_case_types)]
pub const EM_TILEGX: i32 = 191; /* Tilera TILE-Gx */
#[warn(non_camel_case_types)]
pub const EM_NUM: i32 = 192;


// If it is necessary to assign new unofficial EM_* values, please
// pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
// chances of collision with official or non-GNU unofficial values.




#[warn(non_camel_case_types)]
pub const EM_ALPHA: i32 = 0x9026;


// Legal values for e_version (version).




#[warn(non_camel_case_types)]
pub const EV_NONE: i32 = 0; /* Invalid ELF version */
#[warn(non_camel_case_types)]
pub const EV_CURRENT: i32 = 1; /* Current version */
#[warn(non_camel_case_types)]
pub const EV_NUM: i32 = 2;


// Section header.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Shdr {
    pub sh_name: Elf32_Word, // Section name (string tbl index)
    pub sh_type: Elf32_Word, // Section type
    pub sh_flags: Elf32_Word, // Section flags
    pub sh_addr: Elf32_Addr, // Section virtual addr at execution
    pub sh_offset: Elf32_Off, // Section file offset
    pub sh_size: Elf32_Word, // Section size in bytes
    pub sh_link: Elf32_Word, // Link to another section
    pub sh_info: Elf32_Word, // Additional section information
    pub sh_addralign: Elf32_Word, // Section alignment
    pub sh_entsize: Elf32_Word, // Entry size if section holds table
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word, // Section name (string tbl index)
    pub sh_type: Elf64_Word, // Section type
    pub sh_flags: Elf64_Xword, // Section flags
    pub sh_addr: Elf64_Addr, // Section virtual addr at execution
    pub sh_offset: Elf64_Off, // Section file offset
    pub sh_size: Elf64_Xword, // Section size in bytes
    pub sh_link: Elf64_Word, // Link to another section
    pub sh_info: Elf64_Word, // Additional section information
    pub sh_addralign: Elf64_Xword, // Section alignment
    pub sh_entsize: Elf64_Xword, // Entry size if section holds table
}




// Special section indices.




#[warn(non_camel_case_types)]
pub const SHN_UNDEF: i32 = 0; /* Undefined section */
#[warn(non_camel_case_types)]
pub const SHN_LORESERVE: i32 = 0xff00; /* Start of reserved indices */
#[warn(non_camel_case_types)]
pub const SHN_LOPROC: i32 = 0xff00; /* Start of processor-specific */








#[warn(non_camel_case_types)]
pub const SHN_HIPROC: i32 = 0xff1f; /* End of processor-specific */
#[warn(non_camel_case_types)]
pub const SHN_LOOS: i32 = 0xff20; /* Start of OS-specific */
#[warn(non_camel_case_types)]
pub const SHN_HIOS: i32 = 0xff3f; /* End of OS-specific */
#[warn(non_camel_case_types)]
pub const SHN_ABS: i32 = 0xfff1; /* Associated symbol is absolute */
#[warn(non_camel_case_types)]
pub const SHN_COMMON: i32 = 0xfff2; /* Associated symbol is common */
#[warn(non_camel_case_types)]
pub const SHN_XINDEX: i32 = 0xffff; /* Index is in extra table.  */
#[warn(non_camel_case_types)]
pub const SHN_HIRESERVE: i32 = 0xffff; /* End of reserved indices */


// Legal values for sh_type (section type).




#[warn(non_camel_case_types)]
pub const SHT_NULL: i32 = 0; /* Section header table entry unused */
#[warn(non_camel_case_types)]
pub const SHT_PROGBITS: i32 = 1; /* Program data */
#[warn(non_camel_case_types)]
pub const SHT_SYMTAB: i32 = 2; /* Symbol table */
#[warn(non_camel_case_types)]
pub const SHT_STRTAB: i32 = 3; /* String table */
#[warn(non_camel_case_types)]
pub const SHT_RELA: i32 = 4; /* Relocation entries with addends */
#[warn(non_camel_case_types)]
pub const SHT_HASH: i32 = 5; /* Symbol hash table */
#[warn(non_camel_case_types)]
pub const SHT_DYNAMIC: i32 = 6; /* Dynamic linking information */
#[warn(non_camel_case_types)]
pub const SHT_NOTE: i32 = 7; /* Notes */
#[warn(non_camel_case_types)]
pub const SHT_NOBITS: i32 = 8; /* Program space with no data (bss) */
#[warn(non_camel_case_types)]
pub const SHT_REL: i32 = 9; /* Relocation entries, no addends */
#[warn(non_camel_case_types)]
pub const SHT_SHLIB: i32 = 10; /* Reserved */
#[warn(non_camel_case_types)]
pub const SHT_DYNSYM: i32 = 11; /* Dynamic linker symbol table */
#[warn(non_camel_case_types)]
pub const SHT_INIT_ARRAY: i32 = 14; /* Array of constructors */
#[warn(non_camel_case_types)]
pub const SHT_FINI_ARRAY: i32 = 15; /* Array of destructors */
#[warn(non_camel_case_types)]
pub const SHT_PREINIT_ARRAY: i32 = 16; /* Array of pre-constructors */
#[warn(non_camel_case_types)]
pub const SHT_GROUP: i32 = 17; /* Section group */
#[warn(non_camel_case_types)]
pub const SHT_SYMTAB_SHNDX: i32 = 18; /* Extended section indeces */
#[warn(non_camel_case_types)]
pub const SHT_NUM: i32 = 19; /* Number of defined types.  */
#[warn(non_camel_case_types)]
pub const SHT_LOOS: i32 = 0x60000000; /* Start OS-specific.  */
#[warn(non_camel_case_types)]
pub const SHT_GNU_ATTRIBUTES: i32 = 0x6ffffff5; /* Object attributes.  */
#[warn(non_camel_case_types)]
pub const SHT_GNU_HASH: i32 = 0x6ffffff6; /* GNU-style hash table.  */
#[warn(non_camel_case_types)]
pub const SHT_GNU_LIBLIST: i32 = 0x6ffffff7; /* Prelink library list */
#[warn(non_camel_case_types)]
pub const SHT_CHECKSUM: i32 = 0x6ffffff8; /* Checksum for DSO content.  */
#[warn(non_camel_case_types)]
pub const SHT_LOSUNW: i32 = 0x6ffffffa; /* Sun-specific low bound.  */
#[warn(non_camel_case_types)]
pub const SHT_SUNW_move: i32 = 0x6ffffffa;
#[warn(non_camel_case_types)]
pub const SHT_SUNW_COMDAT: i32 = 0x6ffffffb;
#[warn(non_camel_case_types)]
pub const SHT_SUNW_syminfo: i32 = 0x6ffffffc;
#[warn(non_camel_case_types)]
pub const SHT_GNU_verdef: i32 = 0x6ffffffd; /* Version definition section.  */
#[warn(non_camel_case_types)]
pub const SHT_GNU_verneed: i32 = 0x6ffffffe; /* Version needs section.  */
#[warn(non_camel_case_types)]
pub const SHT_GNU_versym: i32 = 0x6fffffff; /* Version symbol table.  */
#[warn(non_camel_case_types)]
pub const SHT_HISUNW: i32 = 0x6fffffff; /* Sun-specific high bound.  */
#[warn(non_camel_case_types)]
pub const SHT_HIOS: i32 = 0x6fffffff; /* End OS-specific type */
#[warn(non_camel_case_types)]
pub const SHT_LOPROC: i32 = 0x70000000; /* Start of processor-specific */
#[warn(non_camel_case_types)]
pub const SHT_HIPROC: i32 = 0x7fffffff; /* End of processor-specific */
#[warn(non_camel_case_types)]
pub const SHT_LOUSER: i32 = 0x80000000; /* Start of application-specific */
#[warn(non_camel_case_types)]
pub const SHT_HIUSER: i32 = 0x8fffffff; /* End of application-specific */


// Legal values for sh_flags (section flags).




#[warn(non_camel_case_types)]
pub const SHF_WRITE: i32 = (1 << 0); /* Writable */
#[warn(non_camel_case_types)]
pub const SHF_ALLOC: i32 = (1 << 1); /* Occupies memory during execution */
#[warn(non_camel_case_types)]
pub const SHF_EXECINSTR: i32 = (1 << 2); /* Executable */
#[warn(non_camel_case_types)]
pub const SHF_MERGE: i32 = (1 << 4); /* Might be merged */
#[warn(non_camel_case_types)]
pub const SHF_STRINGS: i32 = (1 << 5); /* Contains nul-terminated strings */
#[warn(non_camel_case_types)]
pub const SHF_INFO_LINK: i32 = (1 << 6); /* `sh_info' contains SHT index */
#[warn(non_camel_case_types)]
pub const SHF_LINK_ORDER: i32 = (1 << 7); /* Preserve order after combining */




#[warn(non_camel_case_types)]
pub const SHF_GROUP: i32 = (1 << 9); /* Section is member of a group.  */
#[warn(non_camel_case_types)]
pub const SHF_TLS: i32 = (1 << 10); /* Section hold thread-local data.  */
#[warn(non_camel_case_types)]
pub const SHF_MASKOS: i32 = 0x0ff00000; /* OS-specific.  */
#[warn(non_camel_case_types)]
pub const SHF_MASKPROC: i32 = 0xf0000000; /* Processor-specific */










// Section group handling.


#[warn(non_camel_case_types)]
pub const GRP_COMDAT: i32 = 0x1; /* Mark group as COMDAT.  */


// Symbol table entry.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Sym {
    pub st_name: Elf32_Word, // Symbol name (string tbl index)
    pub st_value: Elf32_Addr, // Symbol value
    pub st_size: Elf32_Word, // Symbol size
    pub st_info: u8, // Symbol type and binding
    pub st_other: u8, // Symbol visibility
    pub st_shndx: Elf32_Section, // Section index
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Sym {
    pub st_name: Elf64_Word, // Symbol name (string tbl index)
    pub st_info: u8, // Symbol type and binding
    pub st_other: u8, // Symbol visibility
    pub st_shndx: Elf64_Section, // Section index
    pub st_value: Elf64_Addr, // Symbol value
    pub st_size: Elf64_Xword, // Symbol size
}




// The syminfo section if available contains additional information about
// every dynamic symbol.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Syminfo {
    pub si_boundto: Elf32_Half, // Direct bindings, symbol bound to
    pub si_flags: Elf32_Half, // Per symbol flags
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Syminfo {
    pub si_boundto: Elf64_Half, // Direct bindings, symbol bound to
    pub si_flags: Elf64_Half, // Per symbol flags
}




// Possible values for si_boundto.


#[warn(non_camel_case_types)]
pub const SYMINFO_BT_SELF: i32 = 0xffff; /* Symbol bound to self */
#[warn(non_camel_case_types)]
pub const SYMINFO_BT_PARENT: i32 = 0xfffe; /* Symbol bound to parent */
#[warn(non_camel_case_types)]
pub const SYMINFO_BT_LOWRESERVE: i32 = 0xff00; /* Beginning of reserved entries */


// Possible bitmasks for si_flags.


#[warn(non_camel_case_types)]
pub const SYMINFO_FLG_DIRECT: i32 = 0x0001; /* Direct bound symbol */
#[warn(non_camel_case_types)]
pub const SYMINFO_FLG_PASSTHRU: i32 = 0x0002; /* Pass-thru symbol for translator */
#[warn(non_camel_case_types)]
pub const SYMINFO_FLG_COPY: i32 = 0x0004; /* Symbol is a copy-reloc */




// Syminfo version values.


#[warn(non_camel_case_types)]
pub const SYMINFO_NONE: i32 = 0;
#[warn(non_camel_case_types)]
pub const SYMINFO_CURRENT: i32 = 1;
#[warn(non_camel_case_types)]
pub const SYMINFO_NUM: i32 = 2;




// How to extract and insert information held in the st_info field.












// Both Elf32_Sym and Elf64_Sym use the same one-byte st_info field.










// Legal values for ST_BIND subfield of st_info (symbol binding).




#[warn(non_camel_case_types)]
pub const STB_LOCAL: i32 = 0; /* Local symbol */
#[warn(non_camel_case_types)]
pub const STB_GLOBAL: i32 = 1; /* Global symbol */
#[warn(non_camel_case_types)]
pub const STB_WEAK: i32 = 2; /* Weak symbol */
#[warn(non_camel_case_types)]
pub const STB_NUM: i32 = 3; /* Number of defined types.  */
#[warn(non_camel_case_types)]
pub const STB_LOOS: i32 = 10; /* Start of OS-specific */
#[warn(non_camel_case_types)]
pub const STB_GNU_UNIQUE: i32 = 10; /* Unique symbol.  */
#[warn(non_camel_case_types)]
pub const STB_HIOS: i32 = 12; /* End of OS-specific */
#[warn(non_camel_case_types)]
pub const STB_LOPROC: i32 = 13; /* Start of processor-specific */
#[warn(non_camel_case_types)]
pub const STB_HIPROC: i32 = 15; /* End of processor-specific */


// Legal values for ST_TYPE subfield of st_info (symbol type).




#[warn(non_camel_case_types)]
pub const STT_NOTYPE: i32 = 0; /* Symbol type is unspecified */
#[warn(non_camel_case_types)]
pub const STT_OBJECT: i32 = 1; /* Symbol is a data object */
#[warn(non_camel_case_types)]
pub const STT_FUNC: i32 = 2; /* Symbol is a code object */
#[warn(non_camel_case_types)]
pub const STT_SECTION: i32 = 3; /* Symbol associated with a section */
#[warn(non_camel_case_types)]
pub const STT_FILE: i32 = 4; /* Symbol's name is file name */
#[warn(non_camel_case_types)]
pub const STT_COMMON: i32 = 5; /* Symbol is a common data object */
#[warn(non_camel_case_types)]
pub const STT_TLS: i32 = 6; /* Symbol is thread-local data object*/
#[warn(non_camel_case_types)]
pub const STT_NUM: i32 = 7; /* Number of defined types.  */
#[warn(non_camel_case_types)]
pub const STT_LOOS: i32 = 10; /* Start of OS-specific */
#[warn(non_camel_case_types)]
pub const STT_GNU_IFUNC: i32 = 10; /* Symbol is indirect code object */
#[warn(non_camel_case_types)]
pub const STT_HIOS: i32 = 12; /* End of OS-specific */
#[warn(non_camel_case_types)]
pub const STT_LOPROC: i32 = 13; /* Start of processor-specific */
#[warn(non_camel_case_types)]
pub const STT_HIPROC: i32 = 15; /* End of processor-specific */




// Symbol table indices are found in the hash buckets and chain table
// of a symbol hash table section.  This special index value indicates
// the end of a chain, meaning no further symbols are found in that bucket.




#[warn(non_camel_case_types)]
pub const STN_UNDEF: i32 = 0; /* End of a chain.  */




// How to extract and insert information held in the st_other field.








// For ELF64 the definitions are the same.






// Symbol visibility specification encoded in the st_other field.


#[warn(non_camel_case_types)]
pub const STV_DEFAULT: i32 = 0; /* Default symbol visibility rules */
#[warn(non_camel_case_types)]
pub const STV_INTERNAL: i32 = 1; /* Processor specific hidden class */
#[warn(non_camel_case_types)]
pub const STV_HIDDEN: i32 = 2; /* Sym unavailable in other modules */
#[warn(non_camel_case_types)]
pub const STV_PROTECTED: i32 = 3; /* Not preemptible, not exported */




// Relocation table entry without addend (in section of type SHT_REL).




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Rel {
    pub r_offset: Elf32_Addr, // Address
    pub r_info: Elf32_Word, // Relocation type and symbol index
}




// I have seen two different definitions of the Elf64_Rel and
// Elf64_Rela structures, so we'll leave them out until Novell (or
// whoever) gets their act together.


// The following, at least, is used on Sparc v9, MIPS, and Alpha.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Rel {
    pub r_offset: Elf64_Addr, // Address
    pub r_info: Elf64_Xword, // Relocation type and symbol index
}




// Relocation table entry with addend (in section of type SHT_RELA).




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Rela {
    pub r_offset: Elf32_Addr, // Address
    pub r_info: Elf32_Word, // Relocation type and symbol index
    pub r_addend: Elf32_Sword, // Addend
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr, // Address
    pub r_info: Elf64_Xword, // Relocation type and symbol index
    pub r_addend: Elf64_Sxword, // Addend
}




// How to extract and insert information held in the r_info field.




















// Program segment header.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Phdr {
    pub p_type: Elf32_Word, // Segment type
    pub p_offset: Elf32_Off, // Segment file offset
    pub p_vaddr: Elf32_Addr, // Segment virtual address
    pub p_paddr: Elf32_Addr, // Segment physical address
    pub p_filesz: Elf32_Word, // Segment size in file
    pub p_memsz: Elf32_Word, // Segment size in memory
    pub p_flags: Elf32_Word, // Segment flags
    pub p_align: Elf32_Word, // Segment alignment
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word, // Segment type
    pub p_flags: Elf64_Word, // Segment flags
    pub p_offset: Elf64_Off, // Segment file offset
    pub p_vaddr: Elf64_Addr, // Segment virtual address
    pub p_paddr: Elf64_Addr, // Segment physical address
    pub p_filesz: Elf64_Xword, // Segment size in file
    pub p_memsz: Elf64_Xword, // Segment size in memory
    pub p_align: Elf64_Xword, // Segment alignment
}




// Special value for e_phnum.  This indicates that the real number of
// program headers is too large to fit into e_phnum.  Instead the real
// value is in the field sh_info of section 0.




#[warn(non_camel_case_types)]
pub const PN_XNUM: i32 = 0xffff;


// Legal values for p_type (segment type).




#[warn(non_camel_case_types)]
pub const PT_NULL: i32 = 0; /* Program header table entry unused */
#[warn(non_camel_case_types)]
pub const PT_LOAD: i32 = 1; /* Loadable program segment */
#[warn(non_camel_case_types)]
pub const PT_DYNAMIC: i32 = 2; /* Dynamic linking information */
#[warn(non_camel_case_types)]
pub const PT_INTERP: i32 = 3; /* Program interpreter */
#[warn(non_camel_case_types)]
pub const PT_NOTE: i32 = 4; /* Auxiliary information */
#[warn(non_camel_case_types)]
pub const PT_SHLIB: i32 = 5; /* Reserved */
#[warn(non_camel_case_types)]
pub const PT_PHDR: i32 = 6; /* Entry for header table itself */
#[warn(non_camel_case_types)]
pub const PT_TLS: i32 = 7; /* Thread-local storage segment */
#[warn(non_camel_case_types)]
pub const PT_NUM: i32 = 8; /* Number of defined types */
#[warn(non_camel_case_types)]
pub const PT_LOOS: i32 = 0x60000000; /* Start of OS-specific */
#[warn(non_camel_case_types)]
pub const PT_GNU_EH_FRAME: i32 = 0x6474e550; /* GCC .eh_frame_hdr segment */
#[warn(non_camel_case_types)]
pub const PT_GNU_STACK: i32 = 0x6474e551; /* Indicates stack executability */
#[warn(non_camel_case_types)]
pub const PT_GNU_RELRO: i32 = 0x6474e552; /* Read-only after relocation */
#[warn(non_camel_case_types)]
pub const PT_LOSUNW: i32 = 0x6ffffffa;
#[warn(non_camel_case_types)]
pub const PT_SUNWBSS: i32 = 0x6ffffffa; /* Sun Specific segment */
#[warn(non_camel_case_types)]
pub const PT_SUNWSTACK: i32 = 0x6ffffffb; /* Stack segment */
#[warn(non_camel_case_types)]
pub const PT_HISUNW: i32 = 0x6fffffff;
#[warn(non_camel_case_types)]
pub const PT_HIOS: i32 = 0x6fffffff; /* End of OS-specific */
#[warn(non_camel_case_types)]
pub const PT_LOPROC: i32 = 0x70000000; /* Start of processor-specific */
#[warn(non_camel_case_types)]
pub const PT_HIPROC: i32 = 0x7fffffff; /* End of processor-specific */


// Legal values for p_flags (segment flags).




#[warn(non_camel_case_types)]
pub const PF_X: i32 = (1 << 0); /* Segment is executable */
#[warn(non_camel_case_types)]
pub const PF_W: i32 = (1 << 1); /* Segment is writable */
#[warn(non_camel_case_types)]
pub const PF_R: i32 = (1 << 2); /* Segment is readable */
#[warn(non_camel_case_types)]
pub const PF_MASKOS: i32 = 0x0ff00000; /* OS-specific */
#[warn(non_camel_case_types)]
pub const PF_MASKPROC: i32 = 0xf0000000; /* Processor-specific */


// Legal values for note segment descriptor types for core files.




#[warn(non_camel_case_types)]
pub const NT_PRSTATUS: i32 = 1; /* Contains copy of prstatus struct */
#[warn(non_camel_case_types)]
pub const NT_FPREGSET: i32 = 2; /* Contains copy of fpregset struct */
#[warn(non_camel_case_types)]
pub const NT_PRPSINFO: i32 = 3; /* Contains copy of prpsinfo struct */
#[warn(non_camel_case_types)]
pub const NT_PRXREG: i32 = 4; /* Contains copy of prxregset struct */
#[warn(non_camel_case_types)]
pub const NT_TASKSTRUCT: i32 = 4; /* Contains copy of task structure */
#[warn(non_camel_case_types)]
pub const NT_PLATFORM: i32 = 5; /* String from sysinfo(SI_PLATFORM) */
#[warn(non_camel_case_types)]
pub const NT_AUXV: i32 = 6; /* Contains copy of auxv array */
#[warn(non_camel_case_types)]
pub const NT_GWINDOWS: i32 = 7; /* Contains copy of gwindows struct */
#[warn(non_camel_case_types)]
pub const NT_ASRS: i32 = 8; /* Contains copy of asrset struct */
#[warn(non_camel_case_types)]
pub const NT_PSTATUS: i32 = 10; /* Contains copy of pstatus struct */
#[warn(non_camel_case_types)]
pub const NT_PSINFO: i32 = 13; /* Contains copy of psinfo struct */
#[warn(non_camel_case_types)]
pub const NT_PRCRED: i32 = 14; /* Contains copy of prcred struct */
#[warn(non_camel_case_types)]
pub const NT_UTSNAME: i32 = 15; /* Contains copy of utsname struct */
#[warn(non_camel_case_types)]
pub const NT_LWPSTATUS: i32 = 16; /* Contains copy of lwpstatus struct */
#[warn(non_camel_case_types)]
pub const NT_LWPSINFO: i32 = 17; /* Contains copy of lwpinfo struct */
#[warn(non_camel_case_types)]
pub const NT_PRFPXREG: i32 = 20; /* Contains copy of fprxregset struct */
#[warn(non_camel_case_types)]
pub const NT_PRXFPREG: i32 = 0x46e62b7f; /* Contains copy of user_fxsr_struct */
#[warn(non_camel_case_types)]
pub const NT_PPC_VMX: i32 = 0x100; /* PowerPC Altivec/VMX registers */
#[warn(non_camel_case_types)]
pub const NT_PPC_SPE: i32 = 0x101; /* PowerPC SPE/EVR registers */
#[warn(non_camel_case_types)]
pub const NT_PPC_VSX: i32 = 0x102; /* PowerPC VSX registers */
#[warn(non_camel_case_types)]
pub const NT_386_TLS: i32 = 0x200; /* i386 TLS slots (struct user_desc) */
#[warn(non_camel_case_types)]
pub const NT_386_IOPERM: i32 = 0x201; /* x86 io permission bitmap (1=deny) */
#[warn(non_camel_case_types)]
pub const NT_X86_XSTATE: i32 = 0x202; /* x86 extended state using xsave */


// Legal values for the note segment descriptor types for object files.




#[warn(non_camel_case_types)]
pub const NT_VERSION: i32 = 1; /* Contains a version string.  */




// Dynamic section entry.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Dyn {
    pub d_tag: Elf32_Sword, // Dynamic entry type
    // pub d_val: Elf32_Word, /* Integer value */
    pub d_ptr: Elf32_Addr, // Address value
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sxword, // Dynamic entry type
    // pub d_val: Elf64_Xword, /* Integer value */
    pub d_ptr: Elf64_Addr, // Address value
}




// Legal values for d_tag (dynamic entry type).




#[warn(non_camel_case_types)]
pub const DT_NULL: i32 = 0; /* Marks end of dynamic section */
#[warn(non_camel_case_types)]
pub const DT_NEEDED: i32 = 1; /* Name of needed library */
#[warn(non_camel_case_types)]
pub const DT_PLTRELSZ: i32 = 2; /* Size in bytes of PLT relocs */
#[warn(non_camel_case_types)]
pub const DT_PLTGOT: i32 = 3; /* Processor defined value */
#[warn(non_camel_case_types)]
pub const DT_HASH: i32 = 4; /* Address of symbol hash table */
#[warn(non_camel_case_types)]
pub const DT_STRTAB: i32 = 5; /* Address of string table */
#[warn(non_camel_case_types)]
pub const DT_SYMTAB: i32 = 6; /* Address of symbol table */
#[warn(non_camel_case_types)]
pub const DT_RELA: i32 = 7; /* Address of Rela relocs */
#[warn(non_camel_case_types)]
pub const DT_RELASZ: i32 = 8; /* Total size of Rela relocs */
#[warn(non_camel_case_types)]
pub const DT_RELAENT: i32 = 9; /* Size of one Rela reloc */
#[warn(non_camel_case_types)]
pub const DT_STRSZ: i32 = 10; /* Size of string table */
#[warn(non_camel_case_types)]
pub const DT_SYMENT: i32 = 11; /* Size of one symbol table entry */
#[warn(non_camel_case_types)]
pub const DT_INIT: i32 = 12; /* Address of init function */
#[warn(non_camel_case_types)]
pub const DT_FINI: i32 = 13; /* Address of termination function */
#[warn(non_camel_case_types)]
pub const DT_SONAME: i32 = 14; /* Name of shared object */
#[warn(non_camel_case_types)]
pub const DT_RPATH: i32 = 15; /* Library search path (deprecated) */
#[warn(non_camel_case_types)]
pub const DT_SYMBOLIC: i32 = 16; /* Start symbol search here */
#[warn(non_camel_case_types)]
pub const DT_REL: i32 = 17; /* Address of Rel relocs */
#[warn(non_camel_case_types)]
pub const DT_RELSZ: i32 = 18; /* Total size of Rel relocs */
#[warn(non_camel_case_types)]
pub const DT_RELENT: i32 = 19; /* Size of one Rel reloc */
#[warn(non_camel_case_types)]
pub const DT_PLTREL: i32 = 20; /* Type of reloc in PLT */
#[warn(non_camel_case_types)]
pub const DT_DEBUG: i32 = 21; /* For debugging; unspecified */
#[warn(non_camel_case_types)]
pub const DT_TEXTREL: i32 = 22; /* Reloc might modify .text */
#[warn(non_camel_case_types)]
pub const DT_JMPREL: i32 = 23; /* Address of PLT relocs */
#[warn(non_camel_case_types)]
pub const DT_BIND_NOW: i32 = 24; /* Process relocations of object */
#[warn(non_camel_case_types)]
pub const DT_INIT_ARRAY: i32 = 25; /* Array with addresses of init fct */
#[warn(non_camel_case_types)]
pub const DT_FINI_ARRAY: i32 = 26; /* Array with addresses of fini fct */
#[warn(non_camel_case_types)]
pub const DT_INIT_ARRAYSZ: i32 = 27; /* Size in bytes of DT_INIT_ARRAY */
#[warn(non_camel_case_types)]
pub const DT_FINI_ARRAYSZ: i32 = 28; /* Size in bytes of DT_FINI_ARRAY */
#[warn(non_camel_case_types)]
pub const DT_RUNPATH: i32 = 29; /* Library search path */
#[warn(non_camel_case_types)]
pub const DT_FLAGS: i32 = 30; /* Flags for the object being loaded */
#[warn(non_camel_case_types)]
pub const DT_ENCODING: i32 = 32; /* Start of encoded range */
#[warn(non_camel_case_types)]
pub const DT_PREINIT_ARRAY: i32 = 32; /* Array with addresses of preinit fct*/
#[warn(non_camel_case_types)]
pub const DT_PREINIT_ARRAYSZ: i32 = 33; /* size in bytes of DT_PREINIT_ARRAY */
#[warn(non_camel_case_types)]
pub const DT_NUM: i32 = 34; /* Number used */
#[warn(non_camel_case_types)]
pub const DT_LOOS: i32 = 0x6000000d; /* Start of OS-specific */
#[warn(non_camel_case_types)]
pub const DT_HIOS: i32 = 0x6ffff000; /* End of OS-specific */
#[warn(non_camel_case_types)]
pub const DT_LOPROC: i32 = 0x70000000; /* Start of processor-specific */
#[warn(non_camel_case_types)]
pub const DT_HIPROC: i32 = 0x7fffffff; /* End of processor-specific */
#[warn(non_camel_case_types)]
pub const DT_PROCNUM: i32 = DT_MIPS_NUM; /* Most used by any processor */


// DT_* entries which fall between DT_VALRNGHI & DT_VALRNGLO use the
// Dyn.d_un.d_val field of the Elf*_Dyn structure.  This follows Sun's
// approach.


#[warn(non_camel_case_types)]
pub const DT_VALRNGLO: i32 = 0x6ffffd00;
#[warn(non_camel_case_types)]
pub const DT_GNU_PRELINKED: i32 = 0x6ffffdf5; /* Prelinking timestamp */
#[warn(non_camel_case_types)]
pub const DT_GNU_CONFLICTSZ: i32 = 0x6ffffdf6; /* Size of conflict section */
#[warn(non_camel_case_types)]
pub const DT_GNU_LIBLISTSZ: i32 = 0x6ffffdf7; /* Size of library list */
#[warn(non_camel_case_types)]
pub const DT_CHECKSUM: i32 = 0x6ffffdf8;
#[warn(non_camel_case_types)]
pub const DT_PLTPADSZ: i32 = 0x6ffffdf9;
#[warn(non_camel_case_types)]
pub const DT_MOVEENT: i32 = 0x6ffffdfa;
#[warn(non_camel_case_types)]
pub const DT_MOVESZ: i32 = 0x6ffffdfb;
#[warn(non_camel_case_types)]
pub const DT_FEATURE_1: i32 = 0x6ffffdfc; /* Feature selection (DTF_*).  */




#[warn(non_camel_case_types)]
pub const DT_SYMINSZ: i32 = 0x6ffffdfe; /* Size of syminfo table (in bytes) */
#[warn(non_camel_case_types)]
pub const DT_SYMINENT: i32 = 0x6ffffdff; /* Entry size of syminfo */
#[warn(non_camel_case_types)]
pub const DT_VALRNGHI: i32 = 0x6ffffdff;


#[warn(non_camel_case_types)]
pub const DT_VALNUM: i32 = 12;


// DT_* entries which fall between DT_ADDRRNGHI & DT_ADDRRNGLO use the
// Dyn.d_un.d_ptr field of the Elf*_Dyn structure.
//
// If any adjustment is made to the ELF object after it has been
// built these entries will need to be adjusted.


#[warn(non_camel_case_types)]
pub const DT_ADDRRNGLO: i32 = 0x6ffffe00;
#[warn(non_camel_case_types)]
pub const DT_GNU_HASH: i32 = 0x6ffffef5; /* GNU-style hash table.  */
#[warn(non_camel_case_types)]
pub const DT_TLSDESC_PLT: i32 = 0x6ffffef6;
#[warn(non_camel_case_types)]
pub const DT_TLSDESC_GOT: i32 = 0x6ffffef7;
#[warn(non_camel_case_types)]
pub const DT_GNU_CONFLICT: i32 = 0x6ffffef8; /* Start of conflict section */
#[warn(non_camel_case_types)]
pub const DT_GNU_LIBLIST: i32 = 0x6ffffef9; /* Library list */
#[warn(non_camel_case_types)]
pub const DT_CONFIG: i32 = 0x6ffffefa; /* Configuration information.  */
#[warn(non_camel_case_types)]
pub const DT_DEPAUDIT: i32 = 0x6ffffefb; /* Dependency auditing.  */
#[warn(non_camel_case_types)]
pub const DT_AUDIT: i32 = 0x6ffffefc; /* Object auditing.  */
#[warn(non_camel_case_types)]
pub const DT_PLTPAD: i32 = 0x6ffffefd; /* PLT padding.  */
#[warn(non_camel_case_types)]
pub const DT_MOVETAB: i32 = 0x6ffffefe; /* Move table.  */
#[warn(non_camel_case_types)]
pub const DT_SYMINFO: i32 = 0x6ffffeff; /* Syminfo table.  */
#[warn(non_camel_case_types)]
pub const DT_ADDRRNGHI: i32 = 0x6ffffeff;


#[warn(non_camel_case_types)]
pub const DT_ADDRNUM: i32 = 11;


// The versioning entry types.  The next are defined as part of the
// GNU extension.


#[warn(non_camel_case_types)]
pub const DT_VERSYM: i32 = 0x6ffffff0;


#[warn(non_camel_case_types)]
pub const DT_RELACOUNT: i32 = 0x6ffffff9;
#[warn(non_camel_case_types)]
pub const DT_RELCOUNT: i32 = 0x6ffffffa;


// These were chosen by Sun.


#[warn(non_camel_case_types)]
pub const DT_FLAGS_1: i32 = 0x6ffffffb; /* State flags, see DF_1_* below.  */




#[warn(non_camel_case_types)]
pub const DT_VERDEFNUM: i32 = 0x6ffffffd; /* Number of version definitions */




#[warn(non_camel_case_types)]
pub const DT_VERNEEDNUM: i32 = 0x6fffffff; /* Number of needed versions */


#[warn(non_camel_case_types)]
pub const DT_VERSIONTAGNUM: i32 = 16;


// Sun added these machine-independent extensions in the "processor-specific"
// range.  Be compatible.


#[warn(non_camel_case_types)]
pub const DT_AUXILIARY: i32 = 0x7ffffffd; /* Shared object to load before self */
#[warn(non_camel_case_types)]
pub const DT_FILTER: i32 = 0x7fffffff; /* Shared object to get values from */


#[warn(non_camel_case_types)]
pub const DT_EXTRANUM: i32 = 3;


// Values of `d_un.d_val' in the DT_FLAGS entry.


#[warn(non_camel_case_types)]
pub const DF_ORIGIN: i32 = 0x00000001; /* Object may use DF_ORIGIN */
#[warn(non_camel_case_types)]
pub const DF_SYMBOLIC: i32 = 0x00000002; /* Symbol resolutions starts here */
#[warn(non_camel_case_types)]
pub const DF_TEXTREL: i32 = 0x00000004; /* Object contains text relocations */
#[warn(non_camel_case_types)]
pub const DF_BIND_NOW: i32 = 0x00000008; /* No lazy binding for this object */
#[warn(non_camel_case_types)]
pub const DF_STATIC_TLS: i32 = 0x00000010; /* Module uses the static TLS model */


// State flags selectable in the `d_un.d_val' element of the DT_FLAGS_1
// entry in the dynamic section.


#[warn(non_camel_case_types)]
pub const DF_1_NOW: i32 = 0x00000001; /* Set RTLD_NOW for this object.  */
#[warn(non_camel_case_types)]
pub const DF_1_GLOBAL: i32 = 0x00000002; /* Set RTLD_GLOBAL for this object.  */
#[warn(non_camel_case_types)]
pub const DF_1_GROUP: i32 = 0x00000004; /* Set RTLD_GROUP for this object.  */
#[warn(non_camel_case_types)]
pub const DF_1_NODELETE: i32 = 0x00000008; /* Set RTLD_NODELETE for this object.*/
#[warn(non_camel_case_types)]
pub const DF_1_LOADFLTR: i32 = 0x00000010; /* Trigger filtee loading at runtime.*/
#[warn(non_camel_case_types)]
pub const DF_1_INITFIRST: i32 = 0x00000020; /* Set RTLD_INITFIRST for this object*/
#[warn(non_camel_case_types)]
pub const DF_1_NOOPEN: i32 = 0x00000040; /* Set RTLD_NOOPEN for this object.  */
#[warn(non_camel_case_types)]
pub const DF_1_ORIGIN: i32 = 0x00000080; /* $ORIGIN must be handled.  */
#[warn(non_camel_case_types)]
pub const DF_1_DIRECT: i32 = 0x00000100; /* Direct binding enabled.  */
#[warn(non_camel_case_types)]
pub const DF_1_TRANS: i32 = 0x00000200;
#[warn(non_camel_case_types)]
pub const DF_1_INTERPOSE: i32 = 0x00000400; /* Object is used to interpose.  */
#[warn(non_camel_case_types)]
pub const DF_1_NODEFLIB: i32 = 0x00000800; /* Ignore default lib search path.  */
#[warn(non_camel_case_types)]
pub const DF_1_NODUMP: i32 = 0x00001000; /* Object can't be dldump'ed.  */
#[warn(non_camel_case_types)]
pub const DF_1_CONFALT: i32 = 0x00002000; /* Configuration alternative created.*/
#[warn(non_camel_case_types)]
pub const DF_1_ENDFILTEE: i32 = 0x00004000; /* Filtee terminates filters search. */
#[warn(non_camel_case_types)]
pub const DF_1_DISPRELDNE: i32 = 0x00008000; /* Disp reloc applied at build time. */
#[warn(non_camel_case_types)]
pub const DF_1_DISPRELPND: i32 = 0x00010000; /* Disp reloc applied at run-time.  */


// Flags for the feature selection in DT_FEATURE_1.


#[warn(non_camel_case_types)]
pub const DTF_1_PARINIT: i32 = 0x00000001;
#[warn(non_camel_case_types)]
pub const DTF_1_CONFEXP: i32 = 0x00000002;


// Flags in the DT_POSFLAG_1 entry effecting only the next DT_* entry.


#[warn(non_camel_case_types)]
pub const DF_P1_LAZYLOAD: i32 = 0x00000001; /* Lazyload following object.  */






// Version definition sections.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Verdef {
    pub vd_version: Elf32_Half, // Version revision
    pub vd_flags: Elf32_Half, // Version information
    pub vd_ndx: Elf32_Half, // Version Index
    pub vd_cnt: Elf32_Half, // Number of associated aux entries
    pub vd_hash: Elf32_Word, // Version name hash value
    pub vd_aux: Elf32_Word, // Offset in bytes to verdaux array
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Verdef {
    pub vd_version: Elf64_Half, // Version revision
    pub vd_flags: Elf64_Half, // Version information
    pub vd_ndx: Elf64_Half, // Version Index
    pub vd_cnt: Elf64_Half, // Number of associated aux entries
    pub vd_hash: Elf64_Word, // Version name hash value
    pub vd_aux: Elf64_Word, // Offset in bytes to verdaux array
}






// Legal values for vd_version (version revision).


#[warn(non_camel_case_types)]
pub const VER_DEF_NONE: i32 = 0; /* No version */
#[warn(non_camel_case_types)]
pub const VER_DEF_CURRENT: i32 = 1; /* Current version */
#[warn(non_camel_case_types)]
pub const VER_DEF_NUM: i32 = 2; /* Given version number */


// Legal values for vd_flags (version information flags).


#[warn(non_camel_case_types)]
pub const VER_FLG_BASE: i32 = 0x1; /* Version definition of file itself */
#[warn(non_camel_case_types)]
pub const VER_FLG_WEAK: i32 = 0x2; /* Weak version identifier */


// Versym symbol index values.


#[warn(non_camel_case_types)]
pub const VER_NDX_LOCAL: i32 = 0; /* Symbol is local.  */
#[warn(non_camel_case_types)]
pub const VER_NDX_GLOBAL: i32 = 1; /* Symbol is global.  */
#[warn(non_camel_case_types)]
pub const VER_NDX_LORESERVE: i32 = 0xff00; /* Beginning of reserved entries.  */
#[warn(non_camel_case_types)]
pub const VER_NDX_ELIMINATE: i32 = 0xff01; /* Symbol is to be eliminated.  */


// Auxialiary version information.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Verdaux {
    pub vda_name: Elf32_Word, // Version or dependency names
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Verdaux {
    pub vda_name: Elf64_Word, // Version or dependency names
}






// Version dependency section.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Verneed {
    pub vn_version: Elf32_Half, // Version of structure
    pub vn_cnt: Elf32_Half, // Number of associated aux entries
    pub vn_aux: Elf32_Word, // Offset in bytes to vernaux array
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Verneed {
    pub vn_version: Elf64_Half, // Version of structure
    pub vn_cnt: Elf64_Half, // Number of associated aux entries
    pub vn_aux: Elf64_Word, // Offset in bytes to vernaux array
}






// Legal values for vn_version (version revision).


#[warn(non_camel_case_types)]
pub const VER_NEED_NONE: i32 = 0; /* No version */
#[warn(non_camel_case_types)]
pub const VER_NEED_CURRENT: i32 = 1; /* Current version */
#[warn(non_camel_case_types)]
pub const VER_NEED_NUM: i32 = 2; /* Given version number */


// Auxiliary needed version information.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Vernaux {
    pub vna_hash: Elf32_Word, // Hash value of dependency name
    pub vna_flags: Elf32_Half, // Dependency specific information
    pub vna_other: Elf32_Half, // Unused
    pub vna_name: Elf32_Word, // Dependency name string offset
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Vernaux {
    pub vna_hash: Elf64_Word, // Hash value of dependency name
    pub vna_flags: Elf64_Half, // Dependency specific information
    pub vna_other: Elf64_Half, // Unused
    pub vna_name: Elf64_Word, // Dependency name string offset
}








// Auxiliary vector.




// This vector is normally only used by the program interpreter.  The
// usual definition in an ABI supplement uses the name auxv_t.  The
// vector is not usually defined in a standard <elf.h> file, but it
// can't hurt.  We rename it to avoid conflicts.  The sizes of these
// types are an arrangement between the exec server and the program
// interpreter, so we don't fully specify them here.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_auxv_t {
    pub a_type: u32, // Entry type
    pub a_val: u32, // Integer value
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_auxv_t {
    pub a_type: u64, // Entry type
    pub a_val: u64, // Integer value
}




// Legal values for a_type (entry type).




#[warn(non_camel_case_types)]
pub const AT_NULL: i32 = 0; /* End of vector */
#[warn(non_camel_case_types)]
pub const AT_IGNORE: i32 = 1; /* Entry should be ignored */
#[warn(non_camel_case_types)]
pub const AT_EXECFD: i32 = 2; /* File descriptor of program */
#[warn(non_camel_case_types)]
pub const AT_PHDR: i32 = 3; /* Program headers for program */
#[warn(non_camel_case_types)]
pub const AT_PHENT: i32 = 4; /* Size of program header entry */
#[warn(non_camel_case_types)]
pub const AT_PHNUM: i32 = 5; /* Number of program headers */
#[warn(non_camel_case_types)]
pub const AT_PAGESZ: i32 = 6; /* System page size */
#[warn(non_camel_case_types)]
pub const AT_BASE: i32 = 7; /* Base address of interpreter */
#[warn(non_camel_case_types)]
pub const AT_FLAGS: i32 = 8; /* Flags */
#[warn(non_camel_case_types)]
pub const AT_ENTRY: i32 = 9; /* Entry point of program */
#[warn(non_camel_case_types)]
pub const AT_NOTELF: i32 = 10; /* Program is not ELF */
#[warn(non_camel_case_types)]
pub const AT_UID: i32 = 11; /* Real uid */
#[warn(non_camel_case_types)]
pub const AT_EUID: i32 = 12; /* Effective uid */
#[warn(non_camel_case_types)]
pub const AT_GID: i32 = 13; /* Real gid */
#[warn(non_camel_case_types)]
pub const AT_EGID: i32 = 14; /* Effective gid */
#[warn(non_camel_case_types)]
pub const AT_CLKTCK: i32 = 17; /* Frequency of times() */


// Some more special a_type values describing the hardware.


#[warn(non_camel_case_types)]
pub const AT_PLATFORM: i32 = 15; /* String identifying platform.  */






// This entry gives some information about the FPU initialization
// performed by the kernel.


#[warn(non_camel_case_types)]
pub const AT_FPUCW: i32 = 18; /* Used FPU control word.  */


// Cache block sizes.


#[warn(non_camel_case_types)]
pub const AT_DCACHEBSIZE: i32 = 19; /* Data cache block size.  */
#[warn(non_camel_case_types)]
pub const AT_ICACHEBSIZE: i32 = 20; /* Instruction cache block size.  */
#[warn(non_camel_case_types)]
pub const AT_UCACHEBSIZE: i32 = 21; /* Unified cache block size.  */


// A special ignored value for PPC, used by the kernel to control the
// interpretation of the AUXV. Must be > 16.


#[warn(non_camel_case_types)]
pub const AT_IGNOREPPC: i32 = 22; /* Entry should be ignored.  */


#[warn(non_camel_case_types)]
pub const AT_SECURE: i32 = 23; /* Boolean, was exec setuid-like?  */


#[warn(non_camel_case_types)]
pub const AT_BASE_PLATFORM: i32 = 24; /* String identifying real platforms.*/


#[warn(non_camel_case_types)]
pub const AT_RANDOM: i32 = 25; /* Address of 16 random bytes.  */


#[warn(non_camel_case_types)]
pub const AT_EXECFN: i32 = 31; /* Filename of executable.  */


// Pointer to the global system page used for system calls and other
// nice things.


#[warn(non_camel_case_types)]
pub const AT_SYSINFO: i32 = 32;
#[warn(non_camel_case_types)]
pub const AT_SYSINFO_EHDR: i32 = 33;


// Shapes of the caches.  Bits 0-3 contains associativity; bits 4-7 contains
// log2 of line size; mask those to get cache size.


#[warn(non_camel_case_types)]
pub const AT_L1I_CACHESHAPE: i32 = 34;
#[warn(non_camel_case_types)]
pub const AT_L1D_CACHESHAPE: i32 = 35;
#[warn(non_camel_case_types)]
pub const AT_L2_CACHESHAPE: i32 = 36;
#[warn(non_camel_case_types)]
pub const AT_L3_CACHESHAPE: i32 = 37;


// Note section contents.  Each entry in the note section begins with
// a header of a fixed form.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Nhdr {
    pub n_namesz: Elf32_Word, // Length of the note's name.
    pub n_descsz: Elf32_Word, // Length of the note's descriptor.
    pub n_type: Elf32_Word, // Type of the note.
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Nhdr {
    pub n_namesz: Elf64_Word, // Length of the note's name.
    pub n_descsz: Elf64_Word, // Length of the note's descriptor.
    pub n_type: Elf64_Word, // Type of the note.
}




// Known names of notes.




// Solaris entries in the note section have this name.






// Note entries for GNU systems have this name.








// Defined types of notes for Solaris.




// Value of descriptor (one word) is desired pagesize for the binary.


#[warn(non_camel_case_types)]
pub const ELF_NOTE_PAGESIZE_HINT: i32 = 1;




// Defined note types for GNU systems.




// ABI information.  The descriptor consists of words:
// word 0: OS descriptor
// word 1: major version of the ABI
// word 2: minor version of the ABI
// word 3: subminor version of the ABI
//


#[warn(non_camel_case_types)]
pub const NT_GNU_ABI_TAG: i32 = 1;
#[warn(non_camel_case_types)]
pub const ELF_NOTE_ABI: i32 = NT_GNU_ABI_TAG; /* Old name.  */


// Known OSes.  These values can appear in word 0 of an
// NT_GNU_ABI_TAG note section entry.


#[warn(non_camel_case_types)]
pub const ELF_NOTE_OS_LINUX: i32 = 0;
#[warn(non_camel_case_types)]
pub const ELF_NOTE_OS_GNU: i32 = 1;
#[warn(non_camel_case_types)]
pub const ELF_NOTE_OS_SOLARIS2: i32 = 2;
#[warn(non_camel_case_types)]
pub const ELF_NOTE_OS_FREEBSD: i32 = 3;


// Synthetic hwcap information.  The descriptor begins with two words:
// word 0: number of entries
// word 1: bitmask of enabled entries
// Then follow variable-length entries, one byte followed by a
// '\0'-terminated hwcap name string.  The byte gives the bit
// number to test if enabled, (1U << bit) & bitmask.


#[warn(non_camel_case_types)]
pub const NT_GNU_HWCAP: i32 = 2;


// Build ID bits as generated by ld --build-id.
// The descriptor consists of any nonzero number of bytes.


#[warn(non_camel_case_types)]
pub const NT_GNU_BUILD_ID: i32 = 3;


// Version note generated by GNU gold containing a version string.


#[warn(non_camel_case_types)]
pub const NT_GNU_GOLD_VERSION: i32 = 4;




// Move records.


#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Move {
    pub m_value: Elf32_Xword, // Symbol value.
    pub m_info: Elf32_Word, // Size and index.
    pub m_poffset: Elf32_Word, // Symbol offset.
    pub m_repeat: Elf32_Half, // Repeat count.
    pub m_stride: Elf32_Half, // Stride info.
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Move {
    pub m_value: Elf64_Xword, // Symbol value.
    pub m_info: Elf64_Xword, // Size and index.
    pub m_poffset: Elf64_Xword, // Symbol offset.
    pub m_repeat: Elf64_Half, // Repeat count.
    pub m_stride: Elf64_Half, // Stride info.
}




// Macro to construct move records.




















// Motorola 68k specific definitions.




// Values for Elf32_Ehdr.e_flags.


#[warn(non_camel_case_types)]
pub const EF_CPU32: i32 = 0x00810000;


// m68k relocs.




#[warn(non_camel_case_types)]
pub const R_68K_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_68K_32: i32 = 1; /* Direct 32 bit  */
#[warn(non_camel_case_types)]
pub const R_68K_16: i32 = 2; /* Direct 16 bit  */
#[warn(non_camel_case_types)]
pub const R_68K_8: i32 = 3; /* Direct 8 bit  */
#[warn(non_camel_case_types)]
pub const R_68K_PC32: i32 = 4; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_68K_PC16: i32 = 5; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_68K_PC8: i32 = 6; /* PC relative 8 bit */
#[warn(non_camel_case_types)]
pub const R_68K_GOT32: i32 = 7; /* 32 bit PC relative GOT entry */
#[warn(non_camel_case_types)]
pub const R_68K_GOT16: i32 = 8; /* 16 bit PC relative GOT entry */
#[warn(non_camel_case_types)]
pub const R_68K_GOT8: i32 = 9; /* 8 bit PC relative GOT entry */
#[warn(non_camel_case_types)]
pub const R_68K_GOT32O: i32 = 10; /* 32 bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_68K_GOT16O: i32 = 11; /* 16 bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_68K_GOT8O: i32 = 12; /* 8 bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_68K_PLT32: i32 = 13; /* 32 bit PC relative PLT address */
#[warn(non_camel_case_types)]
pub const R_68K_PLT16: i32 = 14; /* 16 bit PC relative PLT address */
#[warn(non_camel_case_types)]
pub const R_68K_PLT8: i32 = 15; /* 8 bit PC relative PLT address */
#[warn(non_camel_case_types)]
pub const R_68K_PLT32O: i32 = 16; /* 32 bit PLT offset */
#[warn(non_camel_case_types)]
pub const R_68K_PLT16O: i32 = 17; /* 16 bit PLT offset */
#[warn(non_camel_case_types)]
pub const R_68K_PLT8O: i32 = 18; /* 8 bit PLT offset */
#[warn(non_camel_case_types)]
pub const R_68K_COPY: i32 = 19; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_68K_GLOB_DAT: i32 = 20; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_68K_JMP_SLOT: i32 = 21; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_68K_RELATIVE: i32 = 22; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_GD32: i32 = 25; /* 32 bit GOT offset for GD */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_GD16: i32 = 26; /* 16 bit GOT offset for GD */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_GD8: i32 = 27; /* 8 bit GOT offset for GD */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDM32: i32 = 28; /* 32 bit GOT offset for LDM */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDM16: i32 = 29; /* 16 bit GOT offset for LDM */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDM8: i32 = 30; /* 8 bit GOT offset for LDM */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDO32: i32 = 31; /* 32 bit module-relative offset */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDO16: i32 = 32; /* 16 bit module-relative offset */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_LDO8: i32 = 33; /* 8 bit module-relative offset */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_IE32: i32 = 34; /* 32 bit GOT offset for IE */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_IE16: i32 = 35; /* 16 bit GOT offset for IE */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_IE8: i32 = 36; /* 8 bit GOT offset for IE */












#[warn(non_camel_case_types)]
pub const R_68K_TLS_DTPMOD32: i32 = 40; /* 32 bit module number */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_DTPREL32: i32 = 41; /* 32 bit module-relative offset */
#[warn(non_camel_case_types)]
pub const R_68K_TLS_TPREL32: i32 = 42; /* 32 bit TP-relative offset */
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_68K_NUM: i32 = 43;


// Intel 80386 specific definitions.




// i386 relocs.




#[warn(non_camel_case_types)]
pub const R_386_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_386_32: i32 = 1; /* Direct 32 bit  */
#[warn(non_camel_case_types)]
pub const R_386_PC32: i32 = 2; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_386_GOT32: i32 = 3; /* 32 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_386_PLT32: i32 = 4; /* 32 bit PLT address */
#[warn(non_camel_case_types)]
pub const R_386_COPY: i32 = 5; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_386_GLOB_DAT: i32 = 6; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_386_JMP_SLOT: i32 = 7; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_386_RELATIVE: i32 = 8; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_386_GOTOFF: i32 = 9; /* 32 bit offset to GOT */
#[warn(non_camel_case_types)]
pub const R_386_GOTPC: i32 = 10; /* 32 bit PC relative offset to GOT */
#[warn(non_camel_case_types)]
pub const R_386_32PLT: i32 = 11;
#[warn(non_camel_case_types)]
pub const R_386_TLS_TPOFF: i32 = 14; /* Offset in static TLS block */






















#[warn(non_camel_case_types)]
pub const R_386_16: i32 = 20;
#[warn(non_camel_case_types)]
pub const R_386_PC16: i32 = 21;
#[warn(non_camel_case_types)]
pub const R_386_8: i32 = 22;
#[warn(non_camel_case_types)]
pub const R_386_PC8: i32 = 23;




#[warn(non_camel_case_types)]
pub const R_386_TLS_GD_PUSH: i32 = 25; /* Tag for pushl in GD TLS code */




#[warn(non_camel_case_types)]
pub const R_386_TLS_GD_POP: i32 = 27; /* Tag for popl in GD TLS code */




#[warn(non_camel_case_types)]
pub const R_386_TLS_LDM_PUSH: i32 = 29; /* Tag for pushl in LDM TLS code */




#[warn(non_camel_case_types)]
pub const R_386_TLS_LDM_POP: i32 = 31; /* Tag for popl in LDM TLS code */
#[warn(non_camel_case_types)]
pub const R_386_TLS_LDO_32: i32 = 32; /* Offset relative to TLS block */








#[warn(non_camel_case_types)]
pub const R_386_TLS_DTPMOD32: i32 = 35; /* ID of module containing symbol */
#[warn(non_camel_case_types)]
pub const R_386_TLS_DTPOFF32: i32 = 36; /* Offset in TLS block */
#[warn(non_camel_case_types)]
pub const R_386_TLS_TPOFF32: i32 = 37; /* Negated offset in static TLS block */
// 38?


#[warn(non_camel_case_types)]
pub const R_386_TLS_GOTDESC: i32 = 39; /* GOT offset for TLS descriptor.  */














#[warn(non_camel_case_types)]
pub const R_386_IRELATIVE: i32 = 42; /* Adjust indirectly by program base */
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_386_NUM: i32 = 43;


// SUN SPARC specific definitions.




// Legal values for ST_TYPE subfield of st_info (symbol type).




#[warn(non_camel_case_types)]
pub const STT_SPARC_REGISTER: i32 = 13; /* Global register reserved to app. */


// Values for Elf64_Ehdr.e_flags.




#[warn(non_camel_case_types)]
pub const EF_SPARCV9_MM: i32 = 3;
#[warn(non_camel_case_types)]
pub const EF_SPARCV9_TSO: i32 = 0;
#[warn(non_camel_case_types)]
pub const EF_SPARCV9_PSO: i32 = 1;
#[warn(non_camel_case_types)]
pub const EF_SPARCV9_RMO: i32 = 2;
#[warn(non_camel_case_types)]
pub const EF_SPARC_LEDATA: i32 = 0x800000; /* little endian data */
#[warn(non_camel_case_types)]
pub const EF_SPARC_EXT_MASK: i32 = 0xFFFF00;
#[warn(non_camel_case_types)]
pub const EF_SPARC_32PLUS: i32 = 0x000100; /* generic V8+ features */
#[warn(non_camel_case_types)]
pub const EF_SPARC_SUN_US1: i32 = 0x000200; /* Sun UltraSPARC1 extensions */
#[warn(non_camel_case_types)]
pub const EF_SPARC_HAL_R1: i32 = 0x000400; /* HAL R1 extensions */
#[warn(non_camel_case_types)]
pub const EF_SPARC_SUN_US3: i32 = 0x000800; /* Sun UltraSPARCIII extensions */


// SPARC relocs.




#[warn(non_camel_case_types)]
pub const R_SPARC_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_SPARC_8: i32 = 1; /* Direct 8 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_16: i32 = 2; /* Direct 16 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_32: i32 = 3; /* Direct 32 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_DISP8: i32 = 4; /* PC relative 8 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_DISP16: i32 = 5; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_DISP32: i32 = 6; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_WDISP30: i32 = 7; /* PC relative 30 bit shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_WDISP22: i32 = 8; /* PC relative 22 bit shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_HI22: i32 = 9; /* High 22 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_22: i32 = 10; /* Direct 22 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_13: i32 = 11; /* Direct 13 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_LO10: i32 = 12; /* Truncated 10 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_GOT10: i32 = 13; /* Truncated 10 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_GOT13: i32 = 14; /* 13 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_GOT22: i32 = 15; /* 22 bit GOT entry shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_PC10: i32 = 16; /* PC relative 10 bit truncated */
#[warn(non_camel_case_types)]
pub const R_SPARC_PC22: i32 = 17; /* PC relative 22 bit shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_WPLT30: i32 = 18; /* 30 bit PC relative PLT address */
#[warn(non_camel_case_types)]
pub const R_SPARC_COPY: i32 = 19; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_SPARC_GLOB_DAT: i32 = 20; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_JMP_SLOT: i32 = 21; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_RELATIVE: i32 = 22; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_SPARC_UA32: i32 = 23; /* Direct 32 bit unaligned */


// Additional Sparc64 relocs.




#[warn(non_camel_case_types)]
pub const R_SPARC_PLT32: i32 = 24; /* Direct 32 bit ref to PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_HIPLT22: i32 = 25; /* High 22 bit PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_LOPLT10: i32 = 26; /* Truncated 10 bit PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_PCPLT32: i32 = 27; /* PC rel 32 bit ref to PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_PCPLT22: i32 = 28; /* PC rel high 22 bit PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_PCPLT10: i32 = 29; /* PC rel trunc 10 bit PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_10: i32 = 30; /* Direct 10 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_11: i32 = 31; /* Direct 11 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_64: i32 = 32; /* Direct 64 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_OLO10: i32 = 33; /* 10bit with secondary 13bit addend */
#[warn(non_camel_case_types)]
pub const R_SPARC_HH22: i32 = 34; /* Top 22 bits of direct 64 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_HM10: i32 = 35; /* High middle 10 bits of ... */
#[warn(non_camel_case_types)]
pub const R_SPARC_LM22: i32 = 36; /* Low middle 22 bits of ... */
#[warn(non_camel_case_types)]
pub const R_SPARC_PC_HH22: i32 = 37; /* Top 22 bits of pc rel 64 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_PC_HM10: i32 = 38; /* High middle 10 bit of ... */
#[warn(non_camel_case_types)]
pub const R_SPARC_PC_LM22: i32 = 39; /* Low miggle 22 bits of ... */
#[warn(non_camel_case_types)]
pub const R_SPARC_WDISP16: i32 = 40; /* PC relative 16 bit shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_WDISP19: i32 = 41; /* PC relative 19 bit shifted */
#[warn(non_camel_case_types)]
pub const R_SPARC_GLOB_JMP: i32 = 42; /* was part of v9 ABI but was removed */
#[warn(non_camel_case_types)]
pub const R_SPARC_7: i32 = 43; /* Direct 7 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_5: i32 = 44; /* Direct 5 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_6: i32 = 45; /* Direct 6 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_DISP64: i32 = 46; /* PC relative 64 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_PLT64: i32 = 47; /* Direct 64 bit ref to PLT entry */
#[warn(non_camel_case_types)]
pub const R_SPARC_HIX22: i32 = 48; /* High 22 bit complemented */
#[warn(non_camel_case_types)]
pub const R_SPARC_LOX10: i32 = 49; /* Truncated 11 bit complemented */
#[warn(non_camel_case_types)]
pub const R_SPARC_H44: i32 = 50; /* Direct high 12 of 44 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_M44: i32 = 51; /* Direct mid 22 of 44 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_L44: i32 = 52; /* Direct low 10 of 44 bit */
#[warn(non_camel_case_types)]
pub const R_SPARC_REGISTER: i32 = 53; /* Global register usage */
#[warn(non_camel_case_types)]
pub const R_SPARC_UA64: i32 = 54; /* Direct 64 bit unaligned */
#[warn(non_camel_case_types)]
pub const R_SPARC_UA16: i32 = 55; /* Direct 16 bit unaligned */
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_GD_HI22: i32 = 56;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_GD_LO10: i32 = 57;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_GD_ADD: i32 = 58;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_GD_CALL: i32 = 59;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDM_HI22: i32 = 60;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDM_LO10: i32 = 61;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDM_ADD: i32 = 62;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDM_CALL: i32 = 63;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDO_HIX22: i32 = 64;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDO_LOX10: i32 = 65;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LDO_ADD: i32 = 66;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_IE_HI22: i32 = 67;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_IE_LO10: i32 = 68;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_IE_LD: i32 = 69;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_IE_LDX: i32 = 70;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_IE_ADD: i32 = 71;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LE_HIX22: i32 = 72;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_LE_LOX10: i32 = 73;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_DTPMOD32: i32 = 74;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_DTPMOD64: i32 = 75;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_DTPOFF32: i32 = 76;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_DTPOFF64: i32 = 77;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_TPOFF32: i32 = 78;
#[warn(non_camel_case_types)]
pub const R_SPARC_TLS_TPOFF64: i32 = 79;
#[warn(non_camel_case_types)]
pub const R_SPARC_GOTDATA_HIX22: i32 = 80;
#[warn(non_camel_case_types)]
pub const R_SPARC_GOTDATA_LOX10: i32 = 81;
#[warn(non_camel_case_types)]
pub const R_SPARC_GOTDATA_OP_HIX22: i32 = 82;
#[warn(non_camel_case_types)]
pub const R_SPARC_GOTDATA_OP_LOX10: i32 = 83;
#[warn(non_camel_case_types)]
pub const R_SPARC_GOTDATA_OP: i32 = 84;
#[warn(non_camel_case_types)]
pub const R_SPARC_H34: i32 = 85;
#[warn(non_camel_case_types)]
pub const R_SPARC_SIZE32: i32 = 86;
#[warn(non_camel_case_types)]
pub const R_SPARC_SIZE64: i32 = 87;
#[warn(non_camel_case_types)]
pub const R_SPARC_WDISP10: i32 = 88;
#[warn(non_camel_case_types)]
pub const R_SPARC_JMP_IREL: i32 = 248;
#[warn(non_camel_case_types)]
pub const R_SPARC_IRELATIVE: i32 = 249;
#[warn(non_camel_case_types)]
pub const R_SPARC_GNU_VTINHERIT: i32 = 250;
#[warn(non_camel_case_types)]
pub const R_SPARC_GNU_VTENTRY: i32 = 251;
#[warn(non_camel_case_types)]
pub const R_SPARC_REV32: i32 = 252;
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_SPARC_NUM: i32 = 253;


// For Sparc64, legal values for d_tag of Elf64_Dyn.




#[warn(non_camel_case_types)]
pub const DT_SPARC_REGISTER: i32 = 0x70000001;
#[warn(non_camel_case_types)]
pub const DT_SPARC_NUM: i32 = 2;


// MIPS R3000 specific definitions.




// Legal values for e_flags field of Elf32_Ehdr.




#[warn(non_camel_case_types)]
pub const EF_MIPS_NOREORDER: i32 = 1; /* A .noreorder directive was used */
#[warn(non_camel_case_types)]
pub const EF_MIPS_PIC: i32 = 2; /* Contains PIC code */
#[warn(non_camel_case_types)]
pub const EF_MIPS_CPIC: i32 = 4; /* Uses PIC calling sequence */
#[warn(non_camel_case_types)]
pub const EF_MIPS_XGOT: i32 = 8;
#[warn(non_camel_case_types)]
pub const EF_MIPS_64BIT_WHIRL: i32 = 16;
#[warn(non_camel_case_types)]
pub const EF_MIPS_ABI2: i32 = 32;
#[warn(non_camel_case_types)]
pub const EF_MIPS_ABI_ON32: i32 = 64;
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH: i32 = 0xf0000000; /* MIPS architecture level */


// Legal values for MIPS architecture level.




#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_1: i32 = 0x00000000; /* -mips1 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_2: i32 = 0x10000000; /* -mips2 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_3: i32 = 0x20000000; /* -mips3 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_4: i32 = 0x30000000; /* -mips4 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_5: i32 = 0x40000000; /* -mips5 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_32: i32 = 0x60000000; /* MIPS32 code.  */
#[warn(non_camel_case_types)]
pub const EF_MIPS_ARCH_64: i32 = 0x70000000; /* MIPS64 code.  */


// The following are non-official names and should not be used.




#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_1: i32 = 0x00000000; /* -mips1 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_2: i32 = 0x10000000; /* -mips2 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_3: i32 = 0x20000000; /* -mips3 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_4: i32 = 0x30000000; /* -mips4 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_5: i32 = 0x40000000; /* -mips5 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_32: i32 = 0x60000000; /* MIPS32 code.  */
#[warn(non_camel_case_types)]
pub const E_MIPS_ARCH_64: i32 = 0x70000000; /* MIPS64 code.  */


// Special section indices.




#[warn(non_camel_case_types)]
pub const SHN_MIPS_ACOMMON: i32 = 0xff00; /* Allocated common symbols */
#[warn(non_camel_case_types)]
pub const SHN_MIPS_TEXT: i32 = 0xff01; /* Allocated test symbols.  */
#[warn(non_camel_case_types)]
pub const SHN_MIPS_DATA: i32 = 0xff02; /* Allocated data symbols.  */
#[warn(non_camel_case_types)]
pub const SHN_MIPS_SCOMMON: i32 = 0xff03; /* Small common symbols */
#[warn(non_camel_case_types)]
pub const SHN_MIPS_SUNDEFINED: i32 = 0xff04; /* Small undefined symbols */


// Legal values for sh_type field of Elf32_Shdr.




#[warn(non_camel_case_types)]
pub const SHT_MIPS_LIBLIST: i32 = 0x70000000; /* Shared objects used in link */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_MSYM: i32 = 0x70000001;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_CONFLICT: i32 = 0x70000002; /* Conflicting symbols */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_GPTAB: i32 = 0x70000003; /* Global data area sizes */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_UCODE: i32 = 0x70000004; /* Reserved for SGI/MIPS compilers */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DEBUG: i32 = 0x70000005; /* MIPS ECOFF debugging information*/
#[warn(non_camel_case_types)]
pub const SHT_MIPS_REGINFO: i32 = 0x70000006; /* Register usage information */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_PACKAGE: i32 = 0x70000007;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_PACKSYM: i32 = 0x70000008;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_RELD: i32 = 0x70000009;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_IFACE: i32 = 0x7000000b;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_CONTENT: i32 = 0x7000000c;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_OPTIONS: i32 = 0x7000000d; /* Miscellaneous options.  */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_SHDR: i32 = 0x70000010;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_FDESC: i32 = 0x70000011;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_EXTSYM: i32 = 0x70000012;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DENSE: i32 = 0x70000013;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_PDESC: i32 = 0x70000014;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_LOCSYM: i32 = 0x70000015;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_AUXSYM: i32 = 0x70000016;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_OPTSYM: i32 = 0x70000017;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_LOCSTR: i32 = 0x70000018;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_LINE: i32 = 0x70000019;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_RFDESC: i32 = 0x7000001a;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DELTASYM: i32 = 0x7000001b;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DELTAINST: i32 = 0x7000001c;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DELTACLASS: i32 = 0x7000001d;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DWARF: i32 = 0x7000001e; /* DWARF debugging information.  */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_DELTADECL: i32 = 0x7000001f;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_SYMBOL_LIB: i32 = 0x70000020;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_EVENTS: i32 = 0x70000021; /* Event section.  */
#[warn(non_camel_case_types)]
pub const SHT_MIPS_TRANSLATE: i32 = 0x70000022;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_PIXIE: i32 = 0x70000023;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_XLATE: i32 = 0x70000024;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_XLATE_DEBUG: i32 = 0x70000025;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_WHIRL: i32 = 0x70000026;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_EH_REGION: i32 = 0x70000027;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_XLATE_OLD: i32 = 0x70000028;
#[warn(non_camel_case_types)]
pub const SHT_MIPS_PDR_EXCEPTION: i32 = 0x70000029;


// Legal values for sh_flags field of Elf32_Shdr.




#[warn(non_camel_case_types)]
pub const SHF_MIPS_GPREL: i32 = 0x10000000; /* Must be part of global data area */
#[warn(non_camel_case_types)]
pub const SHF_MIPS_MERGE: i32 = 0x20000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_ADDR: i32 = 0x40000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_STRINGS: i32 = 0x80000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_NOSTRIP: i32 = 0x08000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_LOCAL: i32 = 0x04000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_NAMES: i32 = 0x02000000;
#[warn(non_camel_case_types)]
pub const SHF_MIPS_NODUPE: i32 = 0x01000000;




// Symbol tables.




// MIPS specific values for `st_other'.


#[warn(non_camel_case_types)]
pub const STO_MIPS_DEFAULT: i32 = 0x0;
#[warn(non_camel_case_types)]
pub const STO_MIPS_INTERNAL: i32 = 0x1;
#[warn(non_camel_case_types)]
pub const STO_MIPS_HIDDEN: i32 = 0x2;
#[warn(non_camel_case_types)]
pub const STO_MIPS_PROTECTED: i32 = 0x3;
#[warn(non_camel_case_types)]
pub const STO_MIPS_PLT: i32 = 0x8;
#[warn(non_camel_case_types)]
pub const STO_MIPS_SC_ALIGN_UNUSED: i32 = 0xff;


// MIPS specific values for `st_info'.


#[warn(non_camel_case_types)]
pub const STB_MIPS_SPLIT_COMMON: i32 = 13;


// Entries found in sections of type SHT_MIPS_GPTAB.
































// Entry found in sections of type SHT_MIPS_REGINFO.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_RegInfo {
    pub ri_gprmask: Elf32_Word, // General registers used
    pub ri_cprmask: [Elf32_Word; 4], // Coprocessor registers used
    pub ri_gp_value: Elf32_Sword, // $gp register value
}




// Entries found in sections of type SHT_MIPS_OPTIONS.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf_Options {
    pub size: u8, // Size of descriptor, including header.
    pub info: Elf32_Word, // Kind-specific information.
}




// Values for `kind' field in Elf_Options.




#[warn(non_camel_case_types)]
pub const ODK_NULL: i32 = 0; /* Undefined.  */
#[warn(non_camel_case_types)]
pub const ODK_REGINFO: i32 = 1; /* Register usage information.  */
#[warn(non_camel_case_types)]
pub const ODK_EXCEPTIONS: i32 = 2; /* Exception processing options.  */
#[warn(non_camel_case_types)]
pub const ODK_PAD: i32 = 3; /* Section padding options.  */
#[warn(non_camel_case_types)]
pub const ODK_HWPATCH: i32 = 4; /* Hardware workarounds performed */
#[warn(non_camel_case_types)]
pub const ODK_FILL: i32 = 5; /* record the fill value used by the linker. */
#[warn(non_camel_case_types)]
pub const ODK_TAGS: i32 = 6; /* reserve space for desktop tools to write. */
#[warn(non_camel_case_types)]
pub const ODK_HWAND: i32 = 7; /* HW workarounds.  'AND' bits when merging. */
#[warn(non_camel_case_types)]
pub const ODK_HWOR: i32 = 8; /* HW workarounds.  'OR' bits when merging.  */


// Values for `info' in Elf_Options for ODK_EXCEPTIONS entries.




#[warn(non_camel_case_types)]
pub const OEX_FPU_MIN: i32 = 0x1f; /* FPE's which MUST be enabled.  */
#[warn(non_camel_case_types)]
pub const OEX_FPU_MAX: i32 = 0x1f00; /* FPE's which MAY be enabled.  */
#[warn(non_camel_case_types)]
pub const OEX_PAGE0: i32 = 0x10000; /* page zero must be mapped.  */
#[warn(non_camel_case_types)]
pub const OEX_SMM: i32 = 0x20000; /* Force sequential memory mode?  */
#[warn(non_camel_case_types)]
pub const OEX_FPDBUG: i32 = 0x40000; /* Force floating point debug mode?  */
#[warn(non_camel_case_types)]
pub const OEX_PRECISEFP: i32 = OEX_FPDBUG;
#[warn(non_camel_case_types)]
pub const OEX_DISMISS: i32 = 0x80000; /* Dismiss invalid address faults?  */


#[warn(non_camel_case_types)]
pub const OEX_FPU_INVAL: i32 = 0x10;
#[warn(non_camel_case_types)]
pub const OEX_FPU_DIV0: i32 = 0x08;
#[warn(non_camel_case_types)]
pub const OEX_FPU_OFLO: i32 = 0x04;
#[warn(non_camel_case_types)]
pub const OEX_FPU_UFLO: i32 = 0x02;
#[warn(non_camel_case_types)]
pub const OEX_FPU_INEX: i32 = 0x01;


// Masks for `info' in Elf_Options for an ODK_HWPATCH entry.




#[warn(non_camel_case_types)]
pub const OHW_R4KEOP: i32 = 0x1; /* R4000 end-of-page patch.  */
#[warn(non_camel_case_types)]
pub const OHW_R8KPFETCH: i32 = 0x2; /* may need R8000 prefetch patch.  */
#[warn(non_camel_case_types)]
pub const OHW_R5KEOP: i32 = 0x4; /* R5000 end-of-page patch.  */
#[warn(non_camel_case_types)]
pub const OHW_R5KCVTL: i32 = 0x8; /* R5000 cvt.[ds].l bug.  clean=1.  */


#[warn(non_camel_case_types)]
pub const OPAD_PREFIX: i32 = 0x1;
#[warn(non_camel_case_types)]
pub const OPAD_POSTFIX: i32 = 0x2;
#[warn(non_camel_case_types)]
pub const OPAD_SYMBOL: i32 = 0x4;


// Entry found in `.options' section.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf_Options_Hw {
    pub hwp_flags1: Elf32_Word, // Extra flags.
    pub hwp_flags2: Elf32_Word, // Extra flags.
}




// Masks for `info' in ElfOptions for ODK_HWAND and ODK_HWOR entries.




#[warn(non_camel_case_types)]
pub const OHWA0_R4KEOP_CHECKED: i32 = 0x00000001;
#[warn(non_camel_case_types)]
pub const OHWA1_R4KEOP_CLEAN: i32 = 0x00000002;


// MIPS relocs.




#[warn(non_camel_case_types)]
pub const R_MIPS_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_MIPS_16: i32 = 1; /* Direct 16 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_32: i32 = 2; /* Direct 32 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_REL32: i32 = 3; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_26: i32 = 4; /* Direct 26 bit shifted */
#[warn(non_camel_case_types)]
pub const R_MIPS_HI16: i32 = 5; /* High 16 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_LO16: i32 = 6; /* Low 16 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_GPREL16: i32 = 7; /* GP relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_LITERAL: i32 = 8; /* 16 bit literal entry */
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT16: i32 = 9; /* 16 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_MIPS_PC16: i32 = 10; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_CALL16: i32 = 11; /* 16 bit GOT entry for function */
#[warn(non_camel_case_types)]
pub const R_MIPS_GPREL32: i32 = 12; /* GP relative 32 bit */


#[warn(non_camel_case_types)]
pub const R_MIPS_SHIFT5: i32 = 16;
#[warn(non_camel_case_types)]
pub const R_MIPS_SHIFT6: i32 = 17;
#[warn(non_camel_case_types)]
pub const R_MIPS_64: i32 = 18;
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT_DISP: i32 = 19;
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT_PAGE: i32 = 20;
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT_OFST: i32 = 21;
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT_HI16: i32 = 22;
#[warn(non_camel_case_types)]
pub const R_MIPS_GOT_LO16: i32 = 23;
#[warn(non_camel_case_types)]
pub const R_MIPS_SUB: i32 = 24;
#[warn(non_camel_case_types)]
pub const R_MIPS_INSERT_A: i32 = 25;
#[warn(non_camel_case_types)]
pub const R_MIPS_INSERT_B: i32 = 26;
#[warn(non_camel_case_types)]
pub const R_MIPS_DELETE: i32 = 27;
#[warn(non_camel_case_types)]
pub const R_MIPS_HIGHER: i32 = 28;
#[warn(non_camel_case_types)]
pub const R_MIPS_HIGHEST: i32 = 29;
#[warn(non_camel_case_types)]
pub const R_MIPS_CALL_HI16: i32 = 30;
#[warn(non_camel_case_types)]
pub const R_MIPS_CALL_LO16: i32 = 31;
#[warn(non_camel_case_types)]
pub const R_MIPS_SCN_DISP: i32 = 32;
#[warn(non_camel_case_types)]
pub const R_MIPS_REL16: i32 = 33;
#[warn(non_camel_case_types)]
pub const R_MIPS_ADD_IMMEDIATE: i32 = 34;
#[warn(non_camel_case_types)]
pub const R_MIPS_PJUMP: i32 = 35;
#[warn(non_camel_case_types)]
pub const R_MIPS_RELGOT: i32 = 36;
#[warn(non_camel_case_types)]
pub const R_MIPS_JALR: i32 = 37;
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPMOD32: i32 = 38; /* Module number 32 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPREL32: i32 = 39; /* Module-relative offset 32 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPMOD64: i32 = 40; /* Module number 64 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPREL64: i32 = 41; /* Module-relative offset 64 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_GD: i32 = 42; /* 16 bit GOT offset for GD */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_LDM: i32 = 43; /* 16 bit GOT offset for LDM */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPREL_HI16: i32 = 44; /* Module-relative offset, high 16 bits */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_DTPREL_LO16: i32 = 45; /* Module-relative offset, low 16 bits */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_GOTTPREL: i32 = 46; /* 16 bit GOT offset for IE */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_TPREL32: i32 = 47; /* TP-relative offset, 32 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_TPREL64: i32 = 48; /* TP-relative offset, 64 bit */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_TPREL_HI16: i32 = 49; /* TP-relative offset, high 16 bits */
#[warn(non_camel_case_types)]
pub const R_MIPS_TLS_TPREL_LO16: i32 = 50; /* TP-relative offset, low 16 bits */
#[warn(non_camel_case_types)]
pub const R_MIPS_GLOB_DAT: i32 = 51;
#[warn(non_camel_case_types)]
pub const R_MIPS_COPY: i32 = 126;
#[warn(non_camel_case_types)]
pub const R_MIPS_JUMP_SLOT: i32 = 127;
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_MIPS_NUM: i32 = 128;


// Legal values for p_type field of Elf32_Phdr.




#[warn(non_camel_case_types)]
pub const PT_MIPS_REGINFO: i32 = 0x70000000; /* Register usage information */
#[warn(non_camel_case_types)]
pub const PT_MIPS_RTPROC: i32 = 0x70000001; /* Runtime procedure table. */
#[warn(non_camel_case_types)]
pub const PT_MIPS_OPTIONS: i32 = 0x70000002;


// Special program header types.




#[warn(non_camel_case_types)]
pub const PF_MIPS_LOCAL: i32 = 0x10000000;


// Legal values for d_tag field of Elf32_Dyn.




#[warn(non_camel_case_types)]
pub const DT_MIPS_RLD_VERSION: i32 = 0x70000001; /* Runtime linker interface version */
#[warn(non_camel_case_types)]
pub const DT_MIPS_TIME_STAMP: i32 = 0x70000002; /* Timestamp */
#[warn(non_camel_case_types)]
pub const DT_MIPS_ICHECKSUM: i32 = 0x70000003; /* Checksum */
#[warn(non_camel_case_types)]
pub const DT_MIPS_IVERSION: i32 = 0x70000004; /* Version string (string tbl index) */
#[warn(non_camel_case_types)]
pub const DT_MIPS_FLAGS: i32 = 0x70000005; /* Flags */
#[warn(non_camel_case_types)]
pub const DT_MIPS_BASE_ADDRESS: i32 = 0x70000006; /* Base address */
#[warn(non_camel_case_types)]
pub const DT_MIPS_MSYM: i32 = 0x70000007;
#[warn(non_camel_case_types)]
pub const DT_MIPS_CONFLICT: i32 = 0x70000008; /* Address of CONFLICT section */
#[warn(non_camel_case_types)]
pub const DT_MIPS_LIBLIST: i32 = 0x70000009; /* Address of LIBLIST section */
#[warn(non_camel_case_types)]
pub const DT_MIPS_LOCAL_GOTNO: i32 = 0x7000000a; /* Number of local GOT entries */
#[warn(non_camel_case_types)]
pub const DT_MIPS_CONFLICTNO: i32 = 0x7000000b; /* Number of CONFLICT entries */
#[warn(non_camel_case_types)]
pub const DT_MIPS_LIBLISTNO: i32 = 0x70000010; /* Number of LIBLIST entries */
#[warn(non_camel_case_types)]
pub const DT_MIPS_SYMTABNO: i32 = 0x70000011; /* Number of DYNSYM entries */
#[warn(non_camel_case_types)]
pub const DT_MIPS_UNREFEXTNO: i32 = 0x70000012; /* First external DYNSYM */
#[warn(non_camel_case_types)]
pub const DT_MIPS_GOTSYM: i32 = 0x70000013; /* First GOT entry in DYNSYM */
#[warn(non_camel_case_types)]
pub const DT_MIPS_HIPAGENO: i32 = 0x70000014; /* Number of GOT page table entries */
#[warn(non_camel_case_types)]
pub const DT_MIPS_RLD_MAP: i32 = 0x70000016; /* Address of run time loader map.  */
#[warn(non_camel_case_types)]
pub const DT_MIPS_DELTA_CLASS: i32 = 0x70000017; /* Delta C++ class definition.  */




#[warn(non_camel_case_types)]
pub const DT_MIPS_DELTA_INSTANCE: i32 = 0x70000019; /* Delta C++ class instances.  */




#[warn(non_camel_case_types)]
pub const DT_MIPS_DELTA_RELOC: i32 = 0x7000001b; /* Delta relocations.  */




















#[warn(non_camel_case_types)]
pub const DT_MIPS_CXX_FLAGS: i32 = 0x70000022; /* Flags indicating for C++ flavor.  */
#[warn(non_camel_case_types)]
pub const DT_MIPS_PIXIE_INIT: i32 = 0x70000023;
#[warn(non_camel_case_types)]
pub const DT_MIPS_SYMBOL_LIB: i32 = 0x70000024;
#[warn(non_camel_case_types)]
pub const DT_MIPS_LOCALPAGE_GOTIDX: i32 = 0x70000025;
#[warn(non_camel_case_types)]
pub const DT_MIPS_LOCAL_GOTIDX: i32 = 0x70000026;
#[warn(non_camel_case_types)]
pub const DT_MIPS_HIDDEN_GOTIDX: i32 = 0x70000027;
#[warn(non_camel_case_types)]
pub const DT_MIPS_PROTECTED_GOTIDX: i32 = 0x70000028;
#[warn(non_camel_case_types)]
pub const DT_MIPS_OPTIONS: i32 = 0x70000029; /* Address of .options.  */
#[warn(non_camel_case_types)]
pub const DT_MIPS_INTERFACE: i32 = 0x7000002a; /* Address of .interface.  */
#[warn(non_camel_case_types)]
pub const DT_MIPS_DYNSTR_ALIGN: i32 = 0x7000002b;
#[warn(non_camel_case_types)]
pub const DT_MIPS_INTERFACE_SIZE: i32 = 0x7000002c; /* Size of the .interface section. */








#[warn(non_camel_case_types)]
pub const DT_MIPS_COMPACT_SIZE: i32 = 0x7000002f; /* (O32)Size of compact rel section. */
#[warn(non_camel_case_types)]
pub const DT_MIPS_GP_VALUE: i32 = 0x70000030; /* GP value for aux GOTs.  */
#[warn(non_camel_case_types)]
pub const DT_MIPS_AUX_DYNAMIC: i32 = 0x70000031; /* Address of aux .dynamic.  */
// The address of .got.plt in an executable using the new non-PIC ABI.


#[warn(non_camel_case_types)]
pub const DT_MIPS_PLTGOT: i32 = 0x70000032;
// The base of the PLT in an executable using the new non-PIC ABI if that
// PLT is writable.  For a non-writable PLT, this is omitted or has a zero
// value.


#[warn(non_camel_case_types)]
pub const DT_MIPS_RWPLT: i32 = 0x70000034;
#[warn(non_camel_case_types)]
pub const DT_MIPS_NUM: i32 = 0x35;


// Legal values for DT_MIPS_FLAGS Elf32_Dyn entry.




#[warn(non_camel_case_types)]
pub const RHF_NONE: i32 = 0; /* No flags */
#[warn(non_camel_case_types)]
pub const RHF_QUICKSTART: i32 = (1 << 0); /* Use quickstart */
#[warn(non_camel_case_types)]
pub const RHF_NOTPOT: i32 = (1 << 1); /* Hash size not power of 2 */
#[warn(non_camel_case_types)]
pub const RHF_NO_LIBRARY_REPLACEMENT: i32 = (1 << 2); /* Ignore LD_LIBRARY_PATH */
#[warn(non_camel_case_types)]
pub const RHF_NO_MOVE: i32 = (1 << 3);
#[warn(non_camel_case_types)]
pub const RHF_SGI_ONLY: i32 = (1 << 4);
#[warn(non_camel_case_types)]
pub const RHF_GUARANTEE_INIT: i32 = (1 << 5);
#[warn(non_camel_case_types)]
pub const RHF_DELTA_C_PLUS_PLUS: i32 = (1 << 6);
#[warn(non_camel_case_types)]
pub const RHF_GUARANTEE_START_INIT: i32 = (1 << 7);
#[warn(non_camel_case_types)]
pub const RHF_PIXIE: i32 = (1 << 8);
#[warn(non_camel_case_types)]
pub const RHF_DEFAULT_DELAY_LOAD: i32 = (1 << 9);
#[warn(non_camel_case_types)]
pub const RHF_REQUICKSTART: i32 = (1 << 10);
#[warn(non_camel_case_types)]
pub const RHF_REQUICKSTARTED: i32 = (1 << 11);
#[warn(non_camel_case_types)]
pub const RHF_CORD: i32 = (1 << 12);
#[warn(non_camel_case_types)]
pub const RHF_NO_UNRES_UNDEF: i32 = (1 << 13);
#[warn(non_camel_case_types)]
pub const RHF_RLD_ORDER_SAFE: i32 = (1 << 14);


// Entries found in sections of type SHT_MIPS_LIBLIST.




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf32_Lib {
    pub l_name: Elf32_Word, // Name (string table index)
    pub l_time_stamp: Elf32_Word, // Timestamp
    pub l_checksum: Elf32_Word, // Checksum
    pub l_version: Elf32_Word, // Interface version
    pub l_flags: Elf32_Word, // Flags
}




#[repr(C)]
#[derive(Clone, Debug)]
pub struct Elf64_Lib {
    pub l_name: Elf64_Word, // Name (string table index)
    pub l_time_stamp: Elf64_Word, // Timestamp
    pub l_checksum: Elf64_Word, // Checksum
    pub l_version: Elf64_Word, // Interface version
    pub l_flags: Elf64_Word, // Flags
}






// Legal values for l_flags.




#[warn(non_camel_case_types)]
pub const LL_NONE: i32 = 0;
#[warn(non_camel_case_types)]
pub const LL_EXACT_MATCH: i32 = (1 << 0); /* Require exact match */
#[warn(non_camel_case_types)]
pub const LL_IGNORE_INT_VER: i32 = (1 << 1); /* Ignore interface version */
#[warn(non_camel_case_types)]
pub const LL_REQUIRE_MINOR: i32 = (1 << 2);
#[warn(non_camel_case_types)]
pub const LL_EXPORTS: i32 = (1 << 3);
#[warn(non_camel_case_types)]
pub const LL_DELAY_LOAD: i32 = (1 << 4);
#[warn(non_camel_case_types)]
pub const LL_DELTA: i32 = (1 << 5);


// Entries found in sections of type SHT_MIPS_CONFLICT.




#[allow(non_camel_case_types)]
pub type Elf32_Conflict = Elf32_Addr;






// HPPA specific definitions.




// Legal values for e_flags field of Elf32_Ehdr.




#[warn(non_camel_case_types)]
pub const EF_PARISC_TRAPNIL: i32 = 0x00010000; /* Trap nil pointer dereference.  */
#[warn(non_camel_case_types)]
pub const EF_PARISC_EXT: i32 = 0x00020000; /* Program uses arch. extensions. */
#[warn(non_camel_case_types)]
pub const EF_PARISC_LSB: i32 = 0x00040000; /* Program expects little endian. */
#[warn(non_camel_case_types)]
pub const EF_PARISC_WIDE: i32 = 0x00080000; /* Program expects wide mode.  */




#[warn(non_camel_case_types)]
pub const EF_PARISC_LAZYSWAP: i32 = 0x00400000; /* Allow lazy swapping.  */
#[warn(non_camel_case_types)]
pub const EF_PARISC_ARCH: i32 = 0x0000ffff; /* Architecture version.  */


// Defined values for `e_flags & EF_PARISC_ARCH' are:




#[warn(non_camel_case_types)]
pub const EFA_PARISC_1_0: i32 = 0x020b; /* PA-RISC 1.0 big-endian.  */
#[warn(non_camel_case_types)]
pub const EFA_PARISC_1_1: i32 = 0x0210; /* PA-RISC 1.1 big-endian.  */
#[warn(non_camel_case_types)]
pub const EFA_PARISC_2_0: i32 = 0x0214; /* PA-RISC 2.0 big-endian.  */


// Additional section indeces.








#[warn(non_camel_case_types)]
pub const SHN_PARISC_HUGE_COMMON: i32 = 0xff01; /* Common blocks in huge model.  */


// Legal values for sh_type field of Elf32_Shdr.




#[warn(non_camel_case_types)]
pub const SHT_PARISC_EXT: i32 = 0x70000000; /* Contains product specific ext. */
#[warn(non_camel_case_types)]
pub const SHT_PARISC_UNWIND: i32 = 0x70000001; /* Unwind information.  */
#[warn(non_camel_case_types)]
pub const SHT_PARISC_DOC: i32 = 0x70000002; /* Debug info for optimized code. */


// Legal values for sh_flags field of Elf32_Shdr.




#[warn(non_camel_case_types)]
pub const SHF_PARISC_SHORT: i32 = 0x20000000; /* Section with short addressing. */
#[warn(non_camel_case_types)]
pub const SHF_PARISC_HUGE: i32 = 0x40000000; /* Section far from gp.  */
#[warn(non_camel_case_types)]
pub const SHF_PARISC_SBP: i32 = 0x80000000; /* Static branch prediction code. */


// Legal values for ST_TYPE subfield of st_info (symbol type).




#[warn(non_camel_case_types)]
pub const STT_PARISC_MILLICODE: i32 = 13; /* Millicode function entry point.  */


#[warn(non_camel_case_types)]
pub const STT_HP_OPAQUE: i32 = (STT_LOOS + 0x1);
#[warn(non_camel_case_types)]
pub const STT_HP_STUB: i32 = (STT_LOOS + 0x2);


// HPPA relocs.




#[warn(non_camel_case_types)]
pub const R_PARISC_NONE: i32 = 0; /* No reloc.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR32: i32 = 1; /* Direct 32-bit reference.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR21L: i32 = 2; /* Left 21 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR17R: i32 = 3; /* Right 17 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR17F: i32 = 4; /* 17 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR14R: i32 = 6; /* Right 14 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL32: i32 = 9; /* 32-bit rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL21L: i32 = 10; /* Left 21 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL17R: i32 = 11; /* Right 17 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL17F: i32 = 12; /* 17 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL14R: i32 = 14; /* Right 14 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DPREL21L: i32 = 18; /* Left 21 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DPREL14R: i32 = 22; /* Right 14 bits of rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL21L: i32 = 26; /* GP-relative, left 21 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL14R: i32 = 30; /* GP-relative, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF21L: i32 = 34; /* LT-relative, left 21 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF14R: i32 = 38; /* LT-relative, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_SECREL32: i32 = 41; /* 32 bits section rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_SEGBASE: i32 = 48; /* No relocation, set segment base.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_SEGREL32: i32 = 49; /* 32 bits segment rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF21L: i32 = 50; /* PLT rel. address, left 21 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF14R: i32 = 54; /* PLT rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR32: i32 = 57; /* 32 bits LT-rel. function pointer. */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR21L: i32 = 58; /* LT-rel. fct ptr, left 21 bits. */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR14R: i32 = 62; /* LT-rel. fct ptr, right 14 bits. */
#[warn(non_camel_case_types)]
pub const R_PARISC_FPTR64: i32 = 64; /* 64 bits function address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLABEL32: i32 = 65; /* 32 bits function address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLABEL21L: i32 = 66; /* Left 21 bits of fdesc address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLABEL14R: i32 = 70; /* Right 14 bits of fdesc address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL64: i32 = 72; /* 64 bits PC-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL22F: i32 = 74; /* 22 bits PC-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL14WR: i32 = 75; /* PC-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL14DR: i32 = 76; /* PC rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL16F: i32 = 77; /* 16 bits PC-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL16WF: i32 = 78; /* 16 bits PC-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PCREL16DF: i32 = 79; /* 16 bits PC-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR64: i32 = 80; /* 64 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR14WR: i32 = 83; /* 14 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR14DR: i32 = 84; /* 14 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR16F: i32 = 85; /* 16 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR16WF: i32 = 86; /* 16 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_DIR16DF: i32 = 87; /* 16 bits of eff. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL64: i32 = 88; /* 64 bits of GP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL14WR: i32 = 91; /* GP-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL14DR: i32 = 92; /* GP-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL16F: i32 = 93; /* 16 bits GP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL16WF: i32 = 94; /* 16 bits GP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GPREL16DF: i32 = 95; /* 16 bits GP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF64: i32 = 96; /* 64 bits LT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF14WR: i32 = 99; /* LT-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF14DR: i32 = 100; /* LT-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF16F: i32 = 101; /* 16 bits LT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF16WF: i32 = 102; /* 16 bits LT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF16DF: i32 = 103; /* 16 bits LT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_SECREL64: i32 = 104; /* 64 bits section rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_SEGREL64: i32 = 112; /* 64 bits segment rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF14WR: i32 = 115; /* PLT-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF14DR: i32 = 116; /* PLT-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF16F: i32 = 117; /* 16 bits LT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF16WF: i32 = 118; /* 16 bits PLT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_PLTOFF16DF: i32 = 119; /* 16 bits PLT-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR64: i32 = 120; /* 64 bits LT-rel. function ptr.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR14WR: i32 = 123; /* LT-rel. fct. ptr., right 14 bits. */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR14DR: i32 = 124; /* LT-rel. fct. ptr., right 14 bits. */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR16F: i32 = 125; /* 16 bits LT-rel. function ptr.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR16WF: i32 = 126; /* 16 bits LT-rel. function ptr.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_FPTR16DF: i32 = 127; /* 16 bits LT-rel. function ptr.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LORESERVE: i32 = 128;
#[warn(non_camel_case_types)]
pub const R_PARISC_COPY: i32 = 128; /* Copy relocation.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_IPLT: i32 = 129; /* Dynamic reloc, imported PLT */
#[warn(non_camel_case_types)]
pub const R_PARISC_EPLT: i32 = 130; /* Dynamic reloc, exported PLT */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL32: i32 = 153; /* 32 bits TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL21L: i32 = 154; /* TP-rel. address, left 21 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL14R: i32 = 158; /* TP-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP21L: i32 = 162; /* LT-TP-rel. address, left 21 bits. */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP14R: i32 = 166; /* LT-TP-rel. address, right 14 bits.*/
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP14F: i32 = 167; /* 14 bits LT-TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL64: i32 = 216; /* 64 bits TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL14WR: i32 = 219; /* TP-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL14DR: i32 = 220; /* TP-rel. address, right 14 bits.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL16F: i32 = 221; /* 16 bits TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL16WF: i32 = 222; /* 16 bits TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TPREL16DF: i32 = 223; /* 16 bits TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP64: i32 = 224; /* 64 bits LT-TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP14WR: i32 = 227; /* LT-TP-rel. address, right 14 bits.*/
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP14DR: i32 = 228; /* LT-TP-rel. address, right 14 bits.*/
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP16F: i32 = 229; /* 16 bits LT-TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP16WF: i32 = 230; /* 16 bits LT-TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_LTOFF_TP16DF: i32 = 231; /* 16 bits LT-TP-rel. address.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_GNU_VTENTRY: i32 = 232;
#[warn(non_camel_case_types)]
pub const R_PARISC_GNU_VTINHERIT: i32 = 233;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_GD21L: i32 = 234; /* GD 21-bit left.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_GD14R: i32 = 235; /* GD 14-bit right.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_GDCALL: i32 = 236; /* GD call to __t_g_a.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LDM21L: i32 = 237; /* LD module 21-bit left.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LDM14R: i32 = 238; /* LD module 14-bit right.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LDMCALL: i32 = 239; /* LD module call to __t_g_a.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LDO21L: i32 = 240; /* LD offset 21-bit left.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LDO14R: i32 = 241; /* LD offset 14-bit right.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_DTPMOD32: i32 = 242; /* DTP module 32-bit.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_DTPMOD64: i32 = 243; /* DTP module 64-bit.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_DTPOFF32: i32 = 244; /* DTP offset 32-bit.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_DTPOFF64: i32 = 245; /* DTP offset 32-bit.  */
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LE21L: i32 = R_PARISC_TPREL21L;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_LE14R: i32 = R_PARISC_TPREL14R;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_IE21L: i32 = R_PARISC_LTOFF_TP21L;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_IE14R: i32 = R_PARISC_LTOFF_TP14R;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_TPREL32: i32 = R_PARISC_TPREL32;
#[warn(non_camel_case_types)]
pub const R_PARISC_TLS_TPREL64: i32 = R_PARISC_TPREL64;
#[warn(non_camel_case_types)]
pub const R_PARISC_HIRESERVE: i32 = 255;


// Legal values for p_type field of Elf32_Phdr/Elf64_Phdr.




#[warn(non_camel_case_types)]
pub const PT_HP_TLS: i32 = (PT_LOOS + 0x0);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_NONE: i32 = (PT_LOOS + 0x1);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_VERSION: i32 = (PT_LOOS + 0x2);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_KERNEL: i32 = (PT_LOOS + 0x3);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_COMM: i32 = (PT_LOOS + 0x4);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_PROC: i32 = (PT_LOOS + 0x5);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_LOADABLE: i32 = (PT_LOOS + 0x6);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_STACK: i32 = (PT_LOOS + 0x7);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_SHM: i32 = (PT_LOOS + 0x8);
#[warn(non_camel_case_types)]
pub const PT_HP_CORE_MMF: i32 = (PT_LOOS + 0x9);
#[warn(non_camel_case_types)]
pub const PT_HP_PARALLEL: i32 = (PT_LOOS + 0x10);
#[warn(non_camel_case_types)]
pub const PT_HP_FASTBIND: i32 = (PT_LOOS + 0x11);
#[warn(non_camel_case_types)]
pub const PT_HP_OPT_ANNOT: i32 = (PT_LOOS + 0x12);
#[warn(non_camel_case_types)]
pub const PT_HP_HSL_ANNOT: i32 = (PT_LOOS + 0x13);
#[warn(non_camel_case_types)]
pub const PT_HP_STACK: i32 = (PT_LOOS + 0x14);


#[warn(non_camel_case_types)]
pub const PT_PARISC_ARCHEXT: i32 = 0x70000000;
#[warn(non_camel_case_types)]
pub const PT_PARISC_UNWIND: i32 = 0x70000001;


// Legal values for p_flags field of Elf32_Phdr/Elf64_Phdr.




#[warn(non_camel_case_types)]
pub const PF_PARISC_SBP: i32 = 0x08000000;


#[warn(non_camel_case_types)]
pub const PF_HP_PAGE_SIZE: i32 = 0x00100000;
#[warn(non_camel_case_types)]
pub const PF_HP_FAR_SHARED: i32 = 0x00200000;
#[warn(non_camel_case_types)]
pub const PF_HP_NEAR_SHARED: i32 = 0x00400000;
#[warn(non_camel_case_types)]
pub const PF_HP_CODE: i32 = 0x01000000;
#[warn(non_camel_case_types)]
pub const PF_HP_MODIFY: i32 = 0x02000000;
#[warn(non_camel_case_types)]
pub const PF_HP_LAZYSWAP: i32 = 0x04000000;
#[warn(non_camel_case_types)]
pub const PF_HP_SBP: i32 = 0x08000000;




// Alpha specific definitions.




// Legal values for e_flags field of Elf64_Ehdr.




#[warn(non_camel_case_types)]
pub const EF_ALPHA_32BIT: i32 = 1; /* All addresses must be < 2GB.  */
#[warn(non_camel_case_types)]
pub const EF_ALPHA_CANRELAX: i32 = 2; /* Relocations for relaxing exist.  */


// Legal values for sh_type field of Elf64_Shdr.




// These two are primerily concerned with ECOFF debugging info.


#[warn(non_camel_case_types)]
pub const SHT_ALPHA_DEBUG: i32 = 0x70000001;
#[warn(non_camel_case_types)]
pub const SHT_ALPHA_REGINFO: i32 = 0x70000002;


// Legal values for sh_flags field of Elf64_Shdr.




#[warn(non_camel_case_types)]
pub const SHF_ALPHA_GPREL: i32 = 0x10000000;


// Legal values for st_other field of Elf64_Sym.


#[warn(non_camel_case_types)]
pub const STO_ALPHA_NOPV: i32 = 0x80; /* No PV required.  */
#[warn(non_camel_case_types)]
pub const STO_ALPHA_STD_GPLOAD: i32 = 0x88; /* PV only used for initial ldgp.  */


// Alpha relocs.




#[warn(non_camel_case_types)]
pub const R_ALPHA_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_ALPHA_REFLONG: i32 = 1; /* Direct 32 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_REFQUAD: i32 = 2; /* Direct 64 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GPREL32: i32 = 3; /* GP relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_LITERAL: i32 = 4; /* GP relative 16 bit w/optimization */
#[warn(non_camel_case_types)]
pub const R_ALPHA_LITUSE: i32 = 5; /* Optimization hint for LITERAL */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GPDISP: i32 = 6; /* Add displacement to GP */
#[warn(non_camel_case_types)]
pub const R_ALPHA_BRADDR: i32 = 7; /* PC+4 relative 23 bit shifted */
#[warn(non_camel_case_types)]
pub const R_ALPHA_HINT: i32 = 8; /* PC+4 relative 16 bit shifted */
#[warn(non_camel_case_types)]
pub const R_ALPHA_SREL16: i32 = 9; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_SREL32: i32 = 10; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_SREL64: i32 = 11; /* PC relative 64 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GPRELHIGH: i32 = 17; /* GP relative 32 bit, high 16 bits */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GPRELLOW: i32 = 18; /* GP relative 32 bit, low 16 bits */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GPREL16: i32 = 19; /* GP relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_ALPHA_COPY: i32 = 24; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_ALPHA_GLOB_DAT: i32 = 25; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_ALPHA_JMP_SLOT: i32 = 26; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_ALPHA_RELATIVE: i32 = 27; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_ALPHA_TLS_GD_HI: i32 = 28;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TLSGD: i32 = 29;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TLS_LDM: i32 = 30;
#[warn(non_camel_case_types)]
pub const R_ALPHA_DTPMOD64: i32 = 31;
#[warn(non_camel_case_types)]
pub const R_ALPHA_GOTDTPREL: i32 = 32;
#[warn(non_camel_case_types)]
pub const R_ALPHA_DTPREL64: i32 = 33;
#[warn(non_camel_case_types)]
pub const R_ALPHA_DTPRELHI: i32 = 34;
#[warn(non_camel_case_types)]
pub const R_ALPHA_DTPRELLO: i32 = 35;
#[warn(non_camel_case_types)]
pub const R_ALPHA_DTPREL16: i32 = 36;
#[warn(non_camel_case_types)]
pub const R_ALPHA_GOTTPREL: i32 = 37;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TPREL64: i32 = 38;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TPRELHI: i32 = 39;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TPRELLO: i32 = 40;
#[warn(non_camel_case_types)]
pub const R_ALPHA_TPREL16: i32 = 41;
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_ALPHA_NUM: i32 = 46;


// Magic values of the LITUSE relocation addend.


#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_ADDR: i32 = 0;
#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_BASE: i32 = 1;
#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_BYTOFF: i32 = 2;
#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_JSR: i32 = 3;
#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_TLS_GD: i32 = 4;
#[warn(non_camel_case_types)]
pub const LITUSE_ALPHA_TLS_LDM: i32 = 5;


// Legal values for d_tag of Elf64_Dyn.


#[warn(non_camel_case_types)]
pub const DT_ALPHA_PLTRO: i32 = (DT_LOPROC + 0);
#[warn(non_camel_case_types)]
pub const DT_ALPHA_NUM: i32 = 1;


// PowerPC specific declarations




// Values for Elf32/64_Ehdr.e_flags.


#[warn(non_camel_case_types)]
pub const EF_PPC_EMB: i32 = 0x80000000; /* PowerPC embedded flag */


// Cygnus local bits below


#[warn(non_camel_case_types)]
pub const EF_PPC_RELOCATABLE: i32 = 0x00010000; /* PowerPC -mrelocatable flag*/






// PowerPC relocations defined by the ABIs


#[warn(non_camel_case_types)]
pub const R_PPC_NONE: i32 = 0;
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR32: i32 = 1; /* 32bit absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR24: i32 = 2; /* 26bit address, 2 bits ignored.  */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR16: i32 = 3; /* 16bit absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR16_LO: i32 = 4; /* lower 16bit of absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR16_HI: i32 = 5; /* high 16bit of absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR16_HA: i32 = 6; /* adjusted high 16bit */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR14: i32 = 7; /* 16bit address, 2 bits ignored */
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR14_BRTAKEN: i32 = 8;
#[warn(non_camel_case_types)]
pub const R_PPC_ADDR14_BRNTAKEN: i32 = 9;
#[warn(non_camel_case_types)]
pub const R_PPC_REL24: i32 = 10; /* PC relative 26 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_REL14: i32 = 11; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_REL14_BRTAKEN: i32 = 12;
#[warn(non_camel_case_types)]
pub const R_PPC_REL14_BRNTAKEN: i32 = 13;
#[warn(non_camel_case_types)]
pub const R_PPC_GOT16: i32 = 14;
#[warn(non_camel_case_types)]
pub const R_PPC_GOT16_LO: i32 = 15;
#[warn(non_camel_case_types)]
pub const R_PPC_GOT16_HI: i32 = 16;
#[warn(non_camel_case_types)]
pub const R_PPC_GOT16_HA: i32 = 17;
#[warn(non_camel_case_types)]
pub const R_PPC_PLTREL24: i32 = 18;
#[warn(non_camel_case_types)]
pub const R_PPC_COPY: i32 = 19;
#[warn(non_camel_case_types)]
pub const R_PPC_GLOB_DAT: i32 = 20;
#[warn(non_camel_case_types)]
pub const R_PPC_JMP_SLOT: i32 = 21;
#[warn(non_camel_case_types)]
pub const R_PPC_RELATIVE: i32 = 22;
#[warn(non_camel_case_types)]
pub const R_PPC_LOCAL24PC: i32 = 23;
#[warn(non_camel_case_types)]
pub const R_PPC_UADDR32: i32 = 24;
#[warn(non_camel_case_types)]
pub const R_PPC_UADDR16: i32 = 25;
#[warn(non_camel_case_types)]
pub const R_PPC_REL32: i32 = 26;
#[warn(non_camel_case_types)]
pub const R_PPC_PLT32: i32 = 27;
#[warn(non_camel_case_types)]
pub const R_PPC_PLTREL32: i32 = 28;
#[warn(non_camel_case_types)]
pub const R_PPC_PLT16_LO: i32 = 29;
#[warn(non_camel_case_types)]
pub const R_PPC_PLT16_HI: i32 = 30;
#[warn(non_camel_case_types)]
pub const R_PPC_PLT16_HA: i32 = 31;
#[warn(non_camel_case_types)]
pub const R_PPC_SDAREL16: i32 = 32;
#[warn(non_camel_case_types)]
pub const R_PPC_SECTOFF: i32 = 33;
#[warn(non_camel_case_types)]
pub const R_PPC_SECTOFF_LO: i32 = 34;
#[warn(non_camel_case_types)]
pub const R_PPC_SECTOFF_HI: i32 = 35;
#[warn(non_camel_case_types)]
pub const R_PPC_SECTOFF_HA: i32 = 36;


// PowerPC relocations defined for the TLS access ABI.


#[warn(non_camel_case_types)]
pub const R_PPC_TLS: i32 = 67; /* none	(sym+add)@tls */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPMOD32: i32 = 68; /* word32	(sym+add)@dtpmod */
#[warn(non_camel_case_types)]
pub const R_PPC_TPREL16: i32 = 69; /* half16*	(sym+add)@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC_TPREL16_LO: i32 = 70; /* half16	(sym+add)@tprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC_TPREL16_HI: i32 = 71; /* half16	(sym+add)@tprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC_TPREL16_HA: i32 = 72; /* half16	(sym+add)@tprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC_TPREL32: i32 = 73; /* word32	(sym+add)@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPREL16: i32 = 74; /* half16*	(sym+add)@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPREL16_LO: i32 = 75; /* half16	(sym+add)@dtprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPREL16_HI: i32 = 76; /* half16	(sym+add)@dtprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPREL16_HA: i32 = 77; /* half16	(sym+add)@dtprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC_DTPREL32: i32 = 78; /* word32	(sym+add)@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSGD16: i32 = 79; /* half16*	(sym+add)@got@tlsgd */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSGD16_LO: i32 = 80; /* half16	(sym+add)@got@tlsgd@l */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSGD16_HI: i32 = 81; /* half16	(sym+add)@got@tlsgd@h */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSGD16_HA: i32 = 82; /* half16	(sym+add)@got@tlsgd@ha */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSLD16: i32 = 83; /* half16*	(sym+add)@got@tlsld */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSLD16_LO: i32 = 84; /* half16	(sym+add)@got@tlsld@l */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSLD16_HI: i32 = 85; /* half16	(sym+add)@got@tlsld@h */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TLSLD16_HA: i32 = 86; /* half16	(sym+add)@got@tlsld@ha */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TPREL16: i32 = 87; /* half16*	(sym+add)@got@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TPREL16_LO: i32 = 88; /* half16	(sym+add)@got@tprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TPREL16_HI: i32 = 89; /* half16	(sym+add)@got@tprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_TPREL16_HA: i32 = 90; /* half16	(sym+add)@got@tprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_DTPREL16: i32 = 91; /* half16*	(sym+add)@got@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_DTPREL16_LO: i32 = 92; /* half16*	(sym+add)@got@dtprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_DTPREL16_HI: i32 = 93; /* half16*	(sym+add)@got@dtprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC_GOT_DTPREL16_HA: i32 = 94; /* half16*	(sym+add)@got@dtprel@ha */


// The remaining relocs are from the Embedded ELF ABI, and are not
// in the SVR4 ELF ABI.


#[warn(non_camel_case_types)]
pub const R_PPC_EMB_NADDR32: i32 = 101;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_NADDR16: i32 = 102;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_NADDR16_LO: i32 = 103;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_NADDR16_HI: i32 = 104;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_NADDR16_HA: i32 = 105;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_SDAI16: i32 = 106;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_SDA2I16: i32 = 107;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_SDA2REL: i32 = 108;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_SDA21: i32 = 109; /* 16 bit offset in SDA */
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_MRKREF: i32 = 110;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_RELSEC16: i32 = 111;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_RELST_LO: i32 = 112;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_RELST_HI: i32 = 113;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_RELST_HA: i32 = 114;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_BIT_FLD: i32 = 115;
#[warn(non_camel_case_types)]
pub const R_PPC_EMB_RELSDA: i32 = 116; /* 16 bit relative offset in SDA */


// Diab tool relocations.


#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_SDA21_LO: i32 = 180; /* like EMB_SDA21, but lower 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_SDA21_HI: i32 = 181; /* like EMB_SDA21, but high 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_SDA21_HA: i32 = 182; /* like EMB_SDA21, adjusted high 16 */
#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_RELSDA_LO: i32 = 183; /* like EMB_RELSDA, but lower 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_RELSDA_HI: i32 = 184; /* like EMB_RELSDA, but high 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC_DIAB_RELSDA_HA: i32 = 185; /* like EMB_RELSDA, adjusted high 16 */


// GNU extension to support local ifunc.


#[warn(non_camel_case_types)]
pub const R_PPC_IRELATIVE: i32 = 248;


// GNU relocs used in PIC code sequences.


#[warn(non_camel_case_types)]
pub const R_PPC_REL16: i32 = 249; /* half16   (sym+add-.) */
#[warn(non_camel_case_types)]
pub const R_PPC_REL16_LO: i32 = 250; /* half16   (sym+add-.)@l */
#[warn(non_camel_case_types)]
pub const R_PPC_REL16_HI: i32 = 251; /* half16   (sym+add-.)@h */
#[warn(non_camel_case_types)]
pub const R_PPC_REL16_HA: i32 = 252; /* half16   (sym+add-.)@ha */


// This is a phony reloc to handle any old fashioned TOC16 references
// that may still be in object files.


#[warn(non_camel_case_types)]
pub const R_PPC_TOC16: i32 = 255;


// PowerPC specific values for the Dyn d_tag field.


#[warn(non_camel_case_types)]
pub const DT_PPC_GOT: i32 = (DT_LOPROC + 0);
#[warn(non_camel_case_types)]
pub const DT_PPC_NUM: i32 = 1;


// PowerPC64 relocations defined by the ABIs


#[warn(non_camel_case_types)]
pub const R_PPC64_NONE: i32 = R_PPC_NONE;
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR32: i32 = R_PPC_ADDR32; /* 32bit absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR24: i32 = R_PPC_ADDR24; /* 26bit address, word aligned */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16: i32 = R_PPC_ADDR16; /* 16bit absolute address */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_LO: i32 = R_PPC_ADDR16_LO; /* lower 16bits of address */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HI: i32 = R_PPC_ADDR16_HI; /* high 16bits of address. */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HA: i32 = R_PPC_ADDR16_HA; /* adjusted high 16bits.  */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR14: i32 = R_PPC_ADDR14; /* 16bit address, word aligned */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR14_BRTAKEN: i32 = R_PPC_ADDR14_BRTAKEN;
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR14_BRNTAKEN: i32 = R_PPC_ADDR14_BRNTAKEN;
#[warn(non_camel_case_types)]
pub const R_PPC64_REL24: i32 = R_PPC_REL24; /* PC-rel. 26 bit, word aligned */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL14: i32 = R_PPC_REL14; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL14_BRTAKEN: i32 = R_PPC_REL14_BRTAKEN;
#[warn(non_camel_case_types)]
pub const R_PPC64_REL14_BRNTAKEN: i32 = R_PPC_REL14_BRNTAKEN;
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16: i32 = R_PPC_GOT16;
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16_LO: i32 = R_PPC_GOT16_LO;
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16_HI: i32 = R_PPC_GOT16_HI;
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16_HA: i32 = R_PPC_GOT16_HA;


#[warn(non_camel_case_types)]
pub const R_PPC64_COPY: i32 = R_PPC_COPY;
#[warn(non_camel_case_types)]
pub const R_PPC64_GLOB_DAT: i32 = R_PPC_GLOB_DAT;
#[warn(non_camel_case_types)]
pub const R_PPC64_JMP_SLOT: i32 = R_PPC_JMP_SLOT;
#[warn(non_camel_case_types)]
pub const R_PPC64_RELATIVE: i32 = R_PPC_RELATIVE;


#[warn(non_camel_case_types)]
pub const R_PPC64_UADDR32: i32 = R_PPC_UADDR32;
#[warn(non_camel_case_types)]
pub const R_PPC64_UADDR16: i32 = R_PPC_UADDR16;
#[warn(non_camel_case_types)]
pub const R_PPC64_REL32: i32 = R_PPC_REL32;
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT32: i32 = R_PPC_PLT32;
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTREL32: i32 = R_PPC_PLTREL32;
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT16_LO: i32 = R_PPC_PLT16_LO;
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT16_HI: i32 = R_PPC_PLT16_HI;
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT16_HA: i32 = R_PPC_PLT16_HA;


#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF: i32 = R_PPC_SECTOFF;
#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF_LO: i32 = R_PPC_SECTOFF_LO;
#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF_HI: i32 = R_PPC_SECTOFF_HI;
#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF_HA: i32 = R_PPC_SECTOFF_HA;
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR30: i32 = 37; /* word30 (S + A - P) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR64: i32 = 38; /* doubleword64 S + A */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HIGHER: i32 = 39; /* half16 #higher(S + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HIGHERA: i32 = 40; /* half16 #highera(S + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HIGHEST: i32 = 41; /* half16 #highest(S + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_HIGHESTA: i32 = 42; /* half16 #highesta(S + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_UADDR64: i32 = 43; /* doubleword64 S + A */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL64: i32 = 44; /* doubleword64 S + A - P */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT64: i32 = 45; /* doubleword64 L + A */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTREL64: i32 = 46; /* doubleword64 L + A - P */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16: i32 = 47; /* half16* S + A - .TOC */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16_LO: i32 = 48; /* half16 #lo(S + A - .TOC.) */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16_HI: i32 = 49; /* half16 #hi(S + A - .TOC.) */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16_HA: i32 = 50; /* half16 #ha(S + A - .TOC.) */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC: i32 = 51; /* doubleword64 .TOC */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16: i32 = 52; /* half16* M + A */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16_LO: i32 = 53; /* half16 #lo(M + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16_HI: i32 = 54; /* half16 #hi(M + A) */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16_HA: i32 = 55; /* half16 #ha(M + A) */


#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_DS: i32 = 56; /* half16ds* (S + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_ADDR16_LO_DS: i32 = 57; /* half16ds  #lo(S + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16_DS: i32 = 58; /* half16ds* (G + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT16_LO_DS: i32 = 59; /* half16ds  #lo(G + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLT16_LO_DS: i32 = 60; /* half16ds  #lo(L + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF_DS: i32 = 61; /* half16ds* (R + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_SECTOFF_LO_DS: i32 = 62; /* half16ds  #lo(R + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16_DS: i32 = 63; /* half16ds* (S + A - .TOC.) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_TOC16_LO_DS: i32 = 64; /* half16ds  #lo(S + A - .TOC.) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16_DS: i32 = 65; /* half16ds* (M + A) >> 2 */
#[warn(non_camel_case_types)]
pub const R_PPC64_PLTGOT16_LO_DS: i32 = 66; /* half16ds  #lo(M + A) >> 2 */


// PowerPC64 relocations defined for the TLS access ABI.


#[warn(non_camel_case_types)]
pub const R_PPC64_TLS: i32 = 67; /* none	(sym+add)@tls */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPMOD64: i32 = 68; /* doubleword64 (sym+add)@dtpmod */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16: i32 = 69; /* half16*	(sym+add)@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_LO: i32 = 70; /* half16	(sym+add)@tprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HI: i32 = 71; /* half16	(sym+add)@tprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HA: i32 = 72; /* half16	(sym+add)@tprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL64: i32 = 73; /* doubleword64 (sym+add)@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16: i32 = 74; /* half16*	(sym+add)@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_LO: i32 = 75; /* half16	(sym+add)@dtprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HI: i32 = 76; /* half16	(sym+add)@dtprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HA: i32 = 77; /* half16	(sym+add)@dtprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL64: i32 = 78; /* doubleword64 (sym+add)@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSGD16: i32 = 79; /* half16*	(sym+add)@got@tlsgd */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSGD16_LO: i32 = 80; /* half16	(sym+add)@got@tlsgd@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSGD16_HI: i32 = 81; /* half16	(sym+add)@got@tlsgd@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSGD16_HA: i32 = 82; /* half16	(sym+add)@got@tlsgd@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSLD16: i32 = 83; /* half16*	(sym+add)@got@tlsld */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSLD16_LO: i32 = 84; /* half16	(sym+add)@got@tlsld@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSLD16_HI: i32 = 85; /* half16	(sym+add)@got@tlsld@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TLSLD16_HA: i32 = 86; /* half16	(sym+add)@got@tlsld@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TPREL16_DS: i32 = 87; /* half16ds*	(sym+add)@got@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TPREL16_LO_DS: i32 = 88; /* half16ds (sym+add)@got@tprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TPREL16_HI: i32 = 89; /* half16	(sym+add)@got@tprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_TPREL16_HA: i32 = 90; /* half16	(sym+add)@got@tprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_DTPREL16_DS: i32 = 91; /* half16ds*	(sym+add)@got@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_DTPREL16_LO_DS: i32 = 92; /* half16ds (sym+add)@got@dtprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_DTPREL16_HI: i32 = 93; /* half16	(sym+add)@got@dtprel@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_GOT_DTPREL16_HA: i32 = 94; /* half16	(sym+add)@got@dtprel@ha */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_DS: i32 = 95; /* half16ds*	(sym+add)@tprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_LO_DS: i32 = 96; /* half16ds	(sym+add)@tprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HIGHER: i32 = 97; /* half16	(sym+add)@tprel@higher */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HIGHERA: i32 = 98; /* half16	(sym+add)@tprel@highera */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HIGHEST: i32 = 99; /* half16	(sym+add)@tprel@highest */
#[warn(non_camel_case_types)]
pub const R_PPC64_TPREL16_HIGHESTA: i32 = 100; /* half16	(sym+add)@tprel@highesta */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_DS: i32 = 101; /* half16ds* (sym+add)@dtprel */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_LO_DS: i32 = 102; /* half16ds	(sym+add)@dtprel@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HIGHER: i32 = 103; /* half16	(sym+add)@dtprel@higher */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HIGHERA: i32 = 104; /* half16	(sym+add)@dtprel@highera */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HIGHEST: i32 = 105; /* half16	(sym+add)@dtprel@highest */
#[warn(non_camel_case_types)]
pub const R_PPC64_DTPREL16_HIGHESTA: i32 = 106; /* half16	(sym+add)@dtprel@highesta */


// GNU extension to support local ifunc.


#[warn(non_camel_case_types)]
pub const R_PPC64_JMP_IREL: i32 = 247;
#[warn(non_camel_case_types)]
pub const R_PPC64_IRELATIVE: i32 = 248;
#[warn(non_camel_case_types)]
pub const R_PPC64_REL16: i32 = 249; /* half16   (sym+add-.) */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL16_LO: i32 = 250; /* half16   (sym+add-.)@l */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL16_HI: i32 = 251; /* half16   (sym+add-.)@h */
#[warn(non_camel_case_types)]
pub const R_PPC64_REL16_HA: i32 = 252; /* half16   (sym+add-.)@ha */


// PowerPC64 specific values for the Dyn d_tag field.


#[warn(non_camel_case_types)]
pub const DT_PPC64_GLINK: i32 = (DT_LOPROC + 0);
#[warn(non_camel_case_types)]
pub const DT_PPC64_OPD: i32 = (DT_LOPROC + 1);
#[warn(non_camel_case_types)]
pub const DT_PPC64_OPDSZ: i32 = (DT_LOPROC + 2);
#[warn(non_camel_case_types)]
pub const DT_PPC64_NUM: i32 = 3;




// ARM specific declarations




// Processor specific flags for the ELF header e_flags field.


#[warn(non_camel_case_types)]
pub const EF_ARM_RELEXEC: i32 = 0x01;
#[warn(non_camel_case_types)]
pub const EF_ARM_HASENTRY: i32 = 0x02;
#[warn(non_camel_case_types)]
pub const EF_ARM_INTERWORK: i32 = 0x04;
#[warn(non_camel_case_types)]
pub const EF_ARM_APCS_26: i32 = 0x08;
#[warn(non_camel_case_types)]
pub const EF_ARM_APCS_FLOAT: i32 = 0x10;
#[warn(non_camel_case_types)]
pub const EF_ARM_PIC: i32 = 0x20;
#[warn(non_camel_case_types)]
pub const EF_ARM_ALIGN8: i32 = 0x40; /* 8-bit structure alignment is in use */
#[warn(non_camel_case_types)]
pub const EF_ARM_NEW_ABI: i32 = 0x80;
#[warn(non_camel_case_types)]
pub const EF_ARM_OLD_ABI: i32 = 0x100;
#[warn(non_camel_case_types)]
pub const EF_ARM_SOFT_FLOAT: i32 = 0x200;
#[warn(non_camel_case_types)]
pub const EF_ARM_VFP_FLOAT: i32 = 0x400;
#[warn(non_camel_case_types)]
pub const EF_ARM_MAVERICK_FLOAT: i32 = 0x800;




// Other constants defined in the ARM ELF spec. version B-01.


// NB. These conflict with values defined above.


#[warn(non_camel_case_types)]
pub const EF_ARM_SYMSARESORTED: i32 = 0x04;
#[warn(non_camel_case_types)]
pub const EF_ARM_DYNSYMSUSESEGIDX: i32 = 0x08;
#[warn(non_camel_case_types)]
pub const EF_ARM_MAPSYMSFIRST: i32 = 0x10;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABIMASK: i32 = 0xFF000000;


// Constants defined in AAELF.


#[warn(non_camel_case_types)]
pub const EF_ARM_BE8: i32 = 0x00800000;
#[warn(non_camel_case_types)]
pub const EF_ARM_LE8: i32 = 0x00400000;




#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_UNKNOWN: i32 = 0x00000000;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_VER1: i32 = 0x01000000;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_VER2: i32 = 0x02000000;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_VER3: i32 = 0x03000000;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_VER4: i32 = 0x04000000;
#[warn(non_camel_case_types)]
pub const EF_ARM_EABI_VER5: i32 = 0x05000000;


// Additional symbol types for Thumb.


#[warn(non_camel_case_types)]
pub const STT_ARM_TFUNC: i32 = STT_LOPROC; /* A Thumb function.  */
#[warn(non_camel_case_types)]
pub const STT_ARM_16BIT: i32 = STT_HIPROC; /* A Thumb label.  */


// ARM-specific values for sh_flags


#[warn(non_camel_case_types)]
pub const SHF_ARM_ENTRYSECT: i32 = 0x10000000; /* Section contains an entry point */






// ARM-specific program header flags






#[warn(non_camel_case_types)]
pub const PF_ARM_PI: i32 = 0x20000000; /* Position-independent segment.  */
#[warn(non_camel_case_types)]
pub const PF_ARM_ABS: i32 = 0x40000000; /* Absolute segment.  */


// Processor specific values for the Phdr p_type field.


#[warn(non_camel_case_types)]
pub const PT_ARM_EXIDX: i32 = (PT_LOPROC + 1); /* ARM unwind segment.  */


// Processor specific values for the Shdr sh_type field.


#[warn(non_camel_case_types)]
pub const SHT_ARM_EXIDX: i32 = (SHT_LOPROC + 1); /* ARM unwind section.  */
#[warn(non_camel_case_types)]
pub const SHT_ARM_PREEMPTMAP: i32 = (SHT_LOPROC + 2); /* Preemption details.  */
#[warn(non_camel_case_types)]
pub const SHT_ARM_ATTRIBUTES: i32 = (SHT_LOPROC + 3); /* ARM attributes section.  */




// ARM relocs.




#[warn(non_camel_case_types)]
pub const R_ARM_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_ARM_PC24: i32 = 1; /* PC relative 26 bit branch */
#[warn(non_camel_case_types)]
pub const R_ARM_ABS32: i32 = 2; /* Direct 32 bit  */
#[warn(non_camel_case_types)]
pub const R_ARM_REL32: i32 = 3; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_ARM_PC13: i32 = 4;
#[warn(non_camel_case_types)]
pub const R_ARM_ABS16: i32 = 5; /* Direct 16 bit */
#[warn(non_camel_case_types)]
pub const R_ARM_ABS12: i32 = 6; /* Direct 12 bit */
#[warn(non_camel_case_types)]
pub const R_ARM_THM_ABS5: i32 = 7;
#[warn(non_camel_case_types)]
pub const R_ARM_ABS8: i32 = 8; /* Direct 8 bit */
#[warn(non_camel_case_types)]
pub const R_ARM_SBREL32: i32 = 9;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_PC22: i32 = 10;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_PC8: i32 = 11;
#[warn(non_camel_case_types)]
pub const R_ARM_AMP_VCALL9: i32 = 12;
#[warn(non_camel_case_types)]
pub const R_ARM_SWI24: i32 = 13; /* Obsolete static relocation.  */
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_DESC: i32 = 13; /* Dynamic relocation.  */
#[warn(non_camel_case_types)]
pub const R_ARM_THM_SWI8: i32 = 14;
#[warn(non_camel_case_types)]
pub const R_ARM_XPC25: i32 = 15;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_XPC22: i32 = 16;
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_DTPMOD32: i32 = 17; /* ID of module containing symbol */
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_DTPOFF32: i32 = 18; /* Offset in TLS block */
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_TPOFF32: i32 = 19; /* Offset in static TLS block */
#[warn(non_camel_case_types)]
pub const R_ARM_COPY: i32 = 20; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_ARM_GLOB_DAT: i32 = 21; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_ARM_JUMP_SLOT: i32 = 22; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_ARM_RELATIVE: i32 = 23; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_ARM_GOTOFF: i32 = 24; /* 32 bit offset to GOT */
#[warn(non_camel_case_types)]
pub const R_ARM_GOTPC: i32 = 25; /* 32 bit PC relative offset to GOT */
#[warn(non_camel_case_types)]
pub const R_ARM_GOT32: i32 = 26; /* 32 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_ARM_PLT32: i32 = 27; /* 32 bit PLT address */
#[warn(non_camel_case_types)]
pub const R_ARM_ALU_PCREL_7_0: i32 = 32;
#[warn(non_camel_case_types)]
pub const R_ARM_ALU_PCREL_15_8: i32 = 33;
#[warn(non_camel_case_types)]
pub const R_ARM_ALU_PCREL_23_15: i32 = 34;
#[warn(non_camel_case_types)]
pub const R_ARM_LDR_SBREL_11_0: i32 = 35;
#[warn(non_camel_case_types)]
pub const R_ARM_ALU_SBREL_19_12: i32 = 36;
#[warn(non_camel_case_types)]
pub const R_ARM_ALU_SBREL_27_20: i32 = 37;
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_GOTDESC: i32 = 90;
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_CALL: i32 = 91;
#[warn(non_camel_case_types)]
pub const R_ARM_TLS_DESCSEQ: i32 = 92;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_TLS_CALL: i32 = 93;
#[warn(non_camel_case_types)]
pub const R_ARM_GNU_VTENTRY: i32 = 100;
#[warn(non_camel_case_types)]
pub const R_ARM_GNU_VTINHERIT: i32 = 101;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_PC11: i32 = 102; /* thumb unconditional branch */
#[warn(non_camel_case_types)]
pub const R_ARM_THM_PC9: i32 = 103; /* thumb conditional branch */




















#[warn(non_camel_case_types)]
pub const R_ARM_THM_TLS_DESCSEQ: i32 = 129;
#[warn(non_camel_case_types)]
pub const R_ARM_IRELATIVE: i32 = 160;
#[warn(non_camel_case_types)]
pub const R_ARM_RXPC25: i32 = 249;
#[warn(non_camel_case_types)]
pub const R_ARM_RSBREL32: i32 = 250;
#[warn(non_camel_case_types)]
pub const R_ARM_THM_RPC22: i32 = 251;
#[warn(non_camel_case_types)]
pub const R_ARM_RREL32: i32 = 252;
#[warn(non_camel_case_types)]
pub const R_ARM_RABS22: i32 = 253;
#[warn(non_camel_case_types)]
pub const R_ARM_RPC24: i32 = 254;
#[warn(non_camel_case_types)]
pub const R_ARM_RBASE: i32 = 255;
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_ARM_NUM: i32 = 256;


// IA-64 specific declarations.




// Processor specific flags for the Ehdr e_flags field.


#[warn(non_camel_case_types)]
pub const EF_IA_64_MASKOS: i32 = 0x0000000f; /* os-specific flags */
#[warn(non_camel_case_types)]
pub const EF_IA_64_ABI64: i32 = 0x00000010; /* 64-bit ABI */
#[warn(non_camel_case_types)]
pub const EF_IA_64_ARCH: i32 = 0xff000000; /* arch. version mask */


// Processor specific values for the Phdr p_type field.


#[warn(non_camel_case_types)]
pub const PT_IA_64_ARCHEXT: i32 = (PT_LOPROC + 0); /* arch extension bits */
#[warn(non_camel_case_types)]
pub const PT_IA_64_UNWIND: i32 = (PT_LOPROC + 1); /* ia64 unwind bits */
#[warn(non_camel_case_types)]
pub const PT_IA_64_HP_OPT_ANOT: i32 = (PT_LOOS + 0x12);
#[warn(non_camel_case_types)]
pub const PT_IA_64_HP_HSL_ANOT: i32 = (PT_LOOS + 0x13);
#[warn(non_camel_case_types)]
pub const PT_IA_64_HP_STACK: i32 = (PT_LOOS + 0x14);


// Processor specific flags for the Phdr p_flags field.


#[warn(non_camel_case_types)]
pub const PF_IA_64_NORECOV: i32 = 0x80000000; /* spec insns w/o recovery */


// Processor specific values for the Shdr sh_type field.


#[warn(non_camel_case_types)]
pub const SHT_IA_64_EXT: i32 = (SHT_LOPROC + 0); /* extension bits */
#[warn(non_camel_case_types)]
pub const SHT_IA_64_UNWIND: i32 = (SHT_LOPROC + 1); /* unwind bits */


// Processor specific flags for the Shdr sh_flags field.


#[warn(non_camel_case_types)]
pub const SHF_IA_64_SHORT: i32 = 0x10000000; /* section near gp */
#[warn(non_camel_case_types)]
pub const SHF_IA_64_NORECOV: i32 = 0x20000000; /* spec insns w/o recovery */


// Processor specific values for the Dyn d_tag field.


#[warn(non_camel_case_types)]
pub const DT_IA_64_PLT_RESERVE: i32 = (DT_LOPROC + 0);
#[warn(non_camel_case_types)]
pub const DT_IA_64_NUM: i32 = 1;


// IA-64 relocations.


#[warn(non_camel_case_types)]
pub const R_IA64_NONE: i32 = 0x00; /* none */
#[warn(non_camel_case_types)]
pub const R_IA64_IMM14: i32 = 0x21; /* symbol + addend, add imm14 */
#[warn(non_camel_case_types)]
pub const R_IA64_IMM22: i32 = 0x22; /* symbol + addend, add imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_IMM64: i32 = 0x23; /* symbol + addend, mov imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_DIR32MSB: i32 = 0x24; /* symbol + addend, data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DIR32LSB: i32 = 0x25; /* symbol + addend, data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DIR64MSB: i32 = 0x26; /* symbol + addend, data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DIR64LSB: i32 = 0x27; /* symbol + addend, data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL22: i32 = 0x2a; /* @gprel(sym + add), add imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL64I: i32 = 0x2b; /* @gprel(sym + add), mov imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL32MSB: i32 = 0x2c; /* @gprel(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL32LSB: i32 = 0x2d; /* @gprel(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL64MSB: i32 = 0x2e; /* @gprel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_GPREL64LSB: i32 = 0x2f; /* @gprel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF22: i32 = 0x32; /* @ltoff(sym + add), add imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF64I: i32 = 0x33; /* @ltoff(sym + add), mov imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_PLTOFF22: i32 = 0x3a; /* @pltoff(sym + add), add imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_PLTOFF64I: i32 = 0x3b; /* @pltoff(sym + add), mov imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_PLTOFF64MSB: i32 = 0x3e; /* @pltoff(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PLTOFF64LSB: i32 = 0x3f; /* @pltoff(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_FPTR64I: i32 = 0x43; /* @fptr(sym + add), mov imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_FPTR32MSB: i32 = 0x44; /* @fptr(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_FPTR32LSB: i32 = 0x45; /* @fptr(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_FPTR64MSB: i32 = 0x46; /* @fptr(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_FPTR64LSB: i32 = 0x47; /* @fptr(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL60B: i32 = 0x48; /* @pcrel(sym + add), brl */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL21B: i32 = 0x49; /* @pcrel(sym + add), ptb, call */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL21M: i32 = 0x4a; /* @pcrel(sym + add), chk.s */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL21F: i32 = 0x4b; /* @pcrel(sym + add), fchkf */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL32MSB: i32 = 0x4c; /* @pcrel(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL32LSB: i32 = 0x4d; /* @pcrel(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL64MSB: i32 = 0x4e; /* @pcrel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL64LSB: i32 = 0x4f; /* @pcrel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR22: i32 = 0x52; /* @ltoff(@fptr(s+a)), imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR64I: i32 = 0x53; /* @ltoff(@fptr(s+a)), imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR32MSB: i32 = 0x54; /* @ltoff(@fptr(s+a)), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR32LSB: i32 = 0x55; /* @ltoff(@fptr(s+a)), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR64MSB: i32 = 0x56; /* @ltoff(@fptr(s+a)), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_FPTR64LSB: i32 = 0x57; /* @ltoff(@fptr(s+a)), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SEGREL32MSB: i32 = 0x5c; /* @segrel(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SEGREL32LSB: i32 = 0x5d; /* @segrel(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SEGREL64MSB: i32 = 0x5e; /* @segrel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SEGREL64LSB: i32 = 0x5f; /* @segrel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SECREL32MSB: i32 = 0x64; /* @secrel(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SECREL32LSB: i32 = 0x65; /* @secrel(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SECREL64MSB: i32 = 0x66; /* @secrel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_SECREL64LSB: i32 = 0x67; /* @secrel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_REL32MSB: i32 = 0x6c; /* data 4 + REL */
#[warn(non_camel_case_types)]
pub const R_IA64_REL32LSB: i32 = 0x6d; /* data 4 + REL */
#[warn(non_camel_case_types)]
pub const R_IA64_REL64MSB: i32 = 0x6e; /* data 8 + REL */
#[warn(non_camel_case_types)]
pub const R_IA64_REL64LSB: i32 = 0x6f; /* data 8 + REL */
#[warn(non_camel_case_types)]
pub const R_IA64_LTV32MSB: i32 = 0x74; /* symbol + addend, data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTV32LSB: i32 = 0x75; /* symbol + addend, data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTV64MSB: i32 = 0x76; /* symbol + addend, data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTV64LSB: i32 = 0x77; /* symbol + addend, data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL21BI: i32 = 0x79; /* @pcrel(sym + add), 21bit inst */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL22: i32 = 0x7a; /* @pcrel(sym + add), 22bit inst */
#[warn(non_camel_case_types)]
pub const R_IA64_PCREL64I: i32 = 0x7b; /* @pcrel(sym + add), 64bit inst */
#[warn(non_camel_case_types)]
pub const R_IA64_IPLTMSB: i32 = 0x80; /* dynamic reloc, imported PLT, MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_IPLTLSB: i32 = 0x81; /* dynamic reloc, imported PLT, LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_COPY: i32 = 0x84; /* copy relocation */
#[warn(non_camel_case_types)]
pub const R_IA64_SUB: i32 = 0x85; /* Addend and symbol difference */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF22X: i32 = 0x86; /* LTOFF22, relaxable.  */
#[warn(non_camel_case_types)]
pub const R_IA64_LDXMOV: i32 = 0x87; /* Use of LTOFF22X.  */
#[warn(non_camel_case_types)]
pub const R_IA64_TPREL14: i32 = 0x91; /* @tprel(sym + add), imm14 */
#[warn(non_camel_case_types)]
pub const R_IA64_TPREL22: i32 = 0x92; /* @tprel(sym + add), imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_TPREL64I: i32 = 0x93; /* @tprel(sym + add), imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_TPREL64MSB: i32 = 0x96; /* @tprel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_TPREL64LSB: i32 = 0x97; /* @tprel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_TPREL22: i32 = 0x9a; /* @ltoff(@tprel(s+a)), imm2 */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPMOD64MSB: i32 = 0xa6; /* @dtpmod(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPMOD64LSB: i32 = 0xa7; /* @dtpmod(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_DTPMOD22: i32 = 0xaa; /* @ltoff(@dtpmod(sym + add)), imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL14: i32 = 0xb1; /* @dtprel(sym + add), imm14 */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL22: i32 = 0xb2; /* @dtprel(sym + add), imm22 */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL64I: i32 = 0xb3; /* @dtprel(sym + add), imm64 */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL32MSB: i32 = 0xb4; /* @dtprel(sym + add), data4 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL32LSB: i32 = 0xb5; /* @dtprel(sym + add), data4 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL64MSB: i32 = 0xb6; /* @dtprel(sym + add), data8 MSB */
#[warn(non_camel_case_types)]
pub const R_IA64_DTPREL64LSB: i32 = 0xb7; /* @dtprel(sym + add), data8 LSB */
#[warn(non_camel_case_types)]
pub const R_IA64_LTOFF_DTPREL22: i32 = 0xba; /* @ltoff(@dtprel(s+a)), imm22 */


// SH specific declarations




// Processor specific flags for the ELF header e_flags field.


#[warn(non_camel_case_types)]
pub const EF_SH_MACH_MASK: i32 = 0x1f;
#[warn(non_camel_case_types)]
pub const EF_SH_UNKNOWN: i32 = 0x0;
#[warn(non_camel_case_types)]
pub const EF_SH1: i32 = 0x1;
#[warn(non_camel_case_types)]
pub const EF_SH2: i32 = 0x2;
#[warn(non_camel_case_types)]
pub const EF_SH3: i32 = 0x3;
#[warn(non_camel_case_types)]
pub const EF_SH_DSP: i32 = 0x4;
#[warn(non_camel_case_types)]
pub const EF_SH3_DSP: i32 = 0x5;
#[warn(non_camel_case_types)]
pub const EF_SH4AL_DSP: i32 = 0x6;
#[warn(non_camel_case_types)]
pub const EF_SH3E: i32 = 0x8;
#[warn(non_camel_case_types)]
pub const EF_SH4: i32 = 0x9;
#[warn(non_camel_case_types)]
pub const EF_SH2E: i32 = 0xb;
#[warn(non_camel_case_types)]
pub const EF_SH4A: i32 = 0xc;
#[warn(non_camel_case_types)]
pub const EF_SH2A: i32 = 0xd;
#[warn(non_camel_case_types)]
pub const EF_SH4_NOFPU: i32 = 0x10;
#[warn(non_camel_case_types)]
pub const EF_SH4A_NOFPU: i32 = 0x11;
#[warn(non_camel_case_types)]
pub const EF_SH4_NOMMU_NOFPU: i32 = 0x12;
#[warn(non_camel_case_types)]
pub const EF_SH2A_NOFPU: i32 = 0x13;
#[warn(non_camel_case_types)]
pub const EF_SH3_NOMMU: i32 = 0x14;
#[warn(non_camel_case_types)]
pub const EF_SH2A_SH4_NOFPU: i32 = 0x15;
#[warn(non_camel_case_types)]
pub const EF_SH2A_SH3_NOFPU: i32 = 0x16;
#[warn(non_camel_case_types)]
pub const EF_SH2A_SH4: i32 = 0x17;
#[warn(non_camel_case_types)]
pub const EF_SH2A_SH3E: i32 = 0x18;


// SH relocs.


#[warn(non_camel_case_types)]
pub const R_SH_NONE: i32 = 0;
#[warn(non_camel_case_types)]
pub const R_SH_DIR32: i32 = 1;
#[warn(non_camel_case_types)]
pub const R_SH_REL32: i32 = 2;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8WPN: i32 = 3;
#[warn(non_camel_case_types)]
pub const R_SH_IND12W: i32 = 4;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8WPL: i32 = 5;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8WPZ: i32 = 6;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8BP: i32 = 7;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8W: i32 = 8;
#[warn(non_camel_case_types)]
pub const R_SH_DIR8L: i32 = 9;
#[warn(non_camel_case_types)]
pub const R_SH_SWITCH16: i32 = 25;
#[warn(non_camel_case_types)]
pub const R_SH_SWITCH32: i32 = 26;
#[warn(non_camel_case_types)]
pub const R_SH_USES: i32 = 27;
#[warn(non_camel_case_types)]
pub const R_SH_COUNT: i32 = 28;
#[warn(non_camel_case_types)]
pub const R_SH_ALIGN: i32 = 29;
#[warn(non_camel_case_types)]
pub const R_SH_CODE: i32 = 30;
#[warn(non_camel_case_types)]
pub const R_SH_DATA: i32 = 31;
#[warn(non_camel_case_types)]
pub const R_SH_LABEL: i32 = 32;
#[warn(non_camel_case_types)]
pub const R_SH_SWITCH8: i32 = 33;
#[warn(non_camel_case_types)]
pub const R_SH_GNU_VTINHERIT: i32 = 34;
#[warn(non_camel_case_types)]
pub const R_SH_GNU_VTENTRY: i32 = 35;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_GD_32: i32 = 144;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_LD_32: i32 = 145;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_LDO_32: i32 = 146;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_IE_32: i32 = 147;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_LE_32: i32 = 148;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_DTPMOD32: i32 = 149;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_DTPOFF32: i32 = 150;
#[warn(non_camel_case_types)]
pub const R_SH_TLS_TPOFF32: i32 = 151;
#[warn(non_camel_case_types)]
pub const R_SH_GOT32: i32 = 160;
#[warn(non_camel_case_types)]
pub const R_SH_PLT32: i32 = 161;
#[warn(non_camel_case_types)]
pub const R_SH_COPY: i32 = 162;
#[warn(non_camel_case_types)]
pub const R_SH_GLOB_DAT: i32 = 163;
#[warn(non_camel_case_types)]
pub const R_SH_JMP_SLOT: i32 = 164;
#[warn(non_camel_case_types)]
pub const R_SH_RELATIVE: i32 = 165;
#[warn(non_camel_case_types)]
pub const R_SH_GOTOFF: i32 = 166;
#[warn(non_camel_case_types)]
pub const R_SH_GOTPC: i32 = 167;
// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_SH_NUM: i32 = 256;


// S/390 specific definitions.




// Valid values for the e_flags field.




#[warn(non_camel_case_types)]
pub const EF_S390_HIGH_GPRS: i32 = 0x00000001; /* High GPRs kernel facility needed.  */


// Additional s390 relocs




#[warn(non_camel_case_types)]
pub const R_390_NONE: i32 = 0; /* No reloc.  */
#[warn(non_camel_case_types)]
pub const R_390_8: i32 = 1; /* Direct 8 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_12: i32 = 2; /* Direct 12 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_16: i32 = 3; /* Direct 16 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_32: i32 = 4; /* Direct 32 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_PC32: i32 = 5; /* PC relative 32 bit.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOT12: i32 = 6; /* 12 bit GOT offset.  */
#[warn(non_camel_case_types)]
pub const R_390_GOT32: i32 = 7; /* 32 bit GOT offset.  */
#[warn(non_camel_case_types)]
pub const R_390_PLT32: i32 = 8; /* 32 bit PC relative PLT address.  */
#[warn(non_camel_case_types)]
pub const R_390_COPY: i32 = 9; /* Copy symbol at runtime.  */
#[warn(non_camel_case_types)]
pub const R_390_GLOB_DAT: i32 = 10; /* Create GOT entry.  */
#[warn(non_camel_case_types)]
pub const R_390_JMP_SLOT: i32 = 11; /* Create PLT entry.  */
#[warn(non_camel_case_types)]
pub const R_390_RELATIVE: i32 = 12; /* Adjust by program base.  */
#[warn(non_camel_case_types)]
pub const R_390_GOTOFF32: i32 = 13; /* 32 bit offset to GOT.	 */
#[warn(non_camel_case_types)]
pub const R_390_GOTPC: i32 = 14; /* 32 bit PC relative offset to GOT.  */
#[warn(non_camel_case_types)]
pub const R_390_GOT16: i32 = 15; /* 16 bit GOT offset.  */
#[warn(non_camel_case_types)]
pub const R_390_PC16: i32 = 16; /* PC relative 16 bit.	*/
#[warn(non_camel_case_types)]
pub const R_390_PC16DBL: i32 = 17; /* PC relative 16 bit shifted by 1.  */
#[warn(non_camel_case_types)]
pub const R_390_PLT16DBL: i32 = 18; /* 16 bit PC rel. PLT shifted by 1.  */
#[warn(non_camel_case_types)]
pub const R_390_PC32DBL: i32 = 19; /* PC relative 32 bit shifted by 1.  */
#[warn(non_camel_case_types)]
pub const R_390_PLT32DBL: i32 = 20; /* 32 bit PC rel. PLT shifted by 1.  */
#[warn(non_camel_case_types)]
pub const R_390_GOTPCDBL: i32 = 21; /* 32 bit PC rel. GOT shifted by 1.  */
#[warn(non_camel_case_types)]
pub const R_390_64: i32 = 22; /* Direct 64 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_PC64: i32 = 23; /* PC relative 64 bit.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOT64: i32 = 24; /* 64 bit GOT offset.  */
#[warn(non_camel_case_types)]
pub const R_390_PLT64: i32 = 25; /* 64 bit PC relative PLT address.  */
#[warn(non_camel_case_types)]
pub const R_390_GOTENT: i32 = 26; /* 32 bit PC rel. to GOT entry >> 1. */
#[warn(non_camel_case_types)]
pub const R_390_GOTOFF16: i32 = 27; /* 16 bit offset to GOT. */
#[warn(non_camel_case_types)]
pub const R_390_GOTOFF64: i32 = 28; /* 64 bit offset to GOT. */
#[warn(non_camel_case_types)]
pub const R_390_GOTPLT12: i32 = 29; /* 12 bit offset to jump slot.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOTPLT16: i32 = 30; /* 16 bit offset to jump slot.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOTPLT32: i32 = 31; /* 32 bit offset to jump slot.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOTPLT64: i32 = 32; /* 64 bit offset to jump slot.	*/
#[warn(non_camel_case_types)]
pub const R_390_GOTPLTENT: i32 = 33; /* 32 bit rel. offset to jump slot.  */
#[warn(non_camel_case_types)]
pub const R_390_PLTOFF16: i32 = 34; /* 16 bit offset from GOT to PLT. */
#[warn(non_camel_case_types)]
pub const R_390_PLTOFF32: i32 = 35; /* 32 bit offset from GOT to PLT. */
#[warn(non_camel_case_types)]
pub const R_390_PLTOFF64: i32 = 36; /* 16 bit offset from GOT to PLT. */
#[warn(non_camel_case_types)]
pub const R_390_TLS_LOAD: i32 = 37; /* Tag for load insn in TLS code.  */
































































#[warn(non_camel_case_types)]
pub const R_390_TLS_DTPMOD: i32 = 54; /* ID of module containing symbol.  */
#[warn(non_camel_case_types)]
pub const R_390_TLS_DTPOFF: i32 = 55; /* Offset in TLS block.	 */




#[warn(non_camel_case_types)]
pub const R_390_20: i32 = 57; /* Direct 20 bit.  */
#[warn(non_camel_case_types)]
pub const R_390_GOT20: i32 = 58; /* 20 bit GOT offset.  */
#[warn(non_camel_case_types)]
pub const R_390_GOTPLT20: i32 = 59; /* 20 bit offset to jump slot.  */




// Keep this the last entry.


#[warn(non_camel_case_types)]
pub const R_390_NUM: i32 = 61;




// CRIS relocations.


#[warn(non_camel_case_types)]
pub const R_CRIS_NONE: i32 = 0;
#[warn(non_camel_case_types)]
pub const R_CRIS_8: i32 = 1;
#[warn(non_camel_case_types)]
pub const R_CRIS_16: i32 = 2;
#[warn(non_camel_case_types)]
pub const R_CRIS_32: i32 = 3;
#[warn(non_camel_case_types)]
pub const R_CRIS_8_PCREL: i32 = 4;
#[warn(non_camel_case_types)]
pub const R_CRIS_16_PCREL: i32 = 5;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_PCREL: i32 = 6;
#[warn(non_camel_case_types)]
pub const R_CRIS_GNU_VTINHERIT: i32 = 7;
#[warn(non_camel_case_types)]
pub const R_CRIS_GNU_VTENTRY: i32 = 8;
#[warn(non_camel_case_types)]
pub const R_CRIS_COPY: i32 = 9;
#[warn(non_camel_case_types)]
pub const R_CRIS_GLOB_DAT: i32 = 10;
#[warn(non_camel_case_types)]
pub const R_CRIS_JUMP_SLOT: i32 = 11;
#[warn(non_camel_case_types)]
pub const R_CRIS_RELATIVE: i32 = 12;
#[warn(non_camel_case_types)]
pub const R_CRIS_16_GOT: i32 = 13;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_GOT: i32 = 14;
#[warn(non_camel_case_types)]
pub const R_CRIS_16_GOTPLT: i32 = 15;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_GOTPLT: i32 = 16;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_GOTREL: i32 = 17;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_PLT_GOTREL: i32 = 18;
#[warn(non_camel_case_types)]
pub const R_CRIS_32_PLT_PCREL: i32 = 19;


#[warn(non_camel_case_types)]
pub const R_CRIS_NUM: i32 = 20;




// AMD x86-64 relocations.


#[warn(non_camel_case_types)]
pub const R_X86_64_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_X86_64_64: i32 = 1; /* Direct 64 bit  */
#[warn(non_camel_case_types)]
pub const R_X86_64_PC32: i32 = 2; /* PC relative 32 bit signed */
#[warn(non_camel_case_types)]
pub const R_X86_64_GOT32: i32 = 3; /* 32 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_X86_64_PLT32: i32 = 4; /* 32 bit PLT address */
#[warn(non_camel_case_types)]
pub const R_X86_64_COPY: i32 = 5; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_X86_64_GLOB_DAT: i32 = 6; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_X86_64_JUMP_SLOT: i32 = 7; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_X86_64_RELATIVE: i32 = 8; /* Adjust by program base */




#[warn(non_camel_case_types)]
pub const R_X86_64_32: i32 = 10; /* Direct 32 bit zero extended */
#[warn(non_camel_case_types)]
pub const R_X86_64_32S: i32 = 11; /* Direct 32 bit sign extended */
#[warn(non_camel_case_types)]
pub const R_X86_64_16: i32 = 12; /* Direct 16 bit zero extended */
#[warn(non_camel_case_types)]
pub const R_X86_64_PC16: i32 = 13; /* 16 bit sign extended pc relative */
#[warn(non_camel_case_types)]
pub const R_X86_64_8: i32 = 14; /* Direct 8 bit sign extended  */
#[warn(non_camel_case_types)]
pub const R_X86_64_PC8: i32 = 15; /* 8 bit sign extended pc relative */
#[warn(non_camel_case_types)]
pub const R_X86_64_DTPMOD64: i32 = 16; /* ID of module containing symbol */
#[warn(non_camel_case_types)]
pub const R_X86_64_DTPOFF64: i32 = 17; /* Offset in module's TLS block */
#[warn(non_camel_case_types)]
pub const R_X86_64_TPOFF64: i32 = 18; /* Offset in initial TLS block */








#[warn(non_camel_case_types)]
pub const R_X86_64_DTPOFF32: i32 = 21; /* Offset in TLS block */




#[warn(non_camel_case_types)]
pub const R_X86_64_TPOFF32: i32 = 23; /* Offset in initial TLS block */
#[warn(non_camel_case_types)]
pub const R_X86_64_PC64: i32 = 24; /* PC relative 64 bit */
#[warn(non_camel_case_types)]
pub const R_X86_64_GOTOFF64: i32 = 25; /* 64 bit offset to GOT */




#[warn(non_camel_case_types)]
pub const R_X86_64_GOT64: i32 = 27; /* 64-bit GOT entry offset */




#[warn(non_camel_case_types)]
pub const R_X86_64_GOTPC64: i32 = 29; /* 64-bit PC relative offset to GOT */
#[warn(non_camel_case_types)]
pub const R_X86_64_GOTPLT64: i32 = 30; /* like GOT64, says PLT entry needed */




#[warn(non_camel_case_types)]
pub const R_X86_64_SIZE32: i32 = 32; /* Size of symbol plus 32-bit addend */
#[warn(non_camel_case_types)]
pub const R_X86_64_SIZE64: i32 = 33; /* Size of symbol plus 64-bit addend */
#[warn(non_camel_case_types)]
pub const R_X86_64_GOTPC32_TLSDESC: i32 = 34; /* GOT offset for TLS descriptor.  */




#[warn(non_camel_case_types)]
pub const R_X86_64_TLSDESC: i32 = 36; /* TLS descriptor.  */
#[warn(non_camel_case_types)]
pub const R_X86_64_IRELATIVE: i32 = 37; /* Adjust indirectly by program base */
#[warn(non_camel_case_types)]
pub const R_X86_64_RELATIVE64: i32 = 38; /* 64-bit adjust by program base */


#[warn(non_camel_case_types)]
pub const R_X86_64_NUM: i32 = 39;




// AM33 relocations.


#[warn(non_camel_case_types)]
pub const R_MN10300_NONE: i32 = 0; /* No reloc.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_32: i32 = 1; /* Direct 32 bit.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_16: i32 = 2; /* Direct 16 bit.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_8: i32 = 3; /* Direct 8 bit.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_PCREL32: i32 = 4; /* PC-relative 32-bit.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_PCREL16: i32 = 5; /* PC-relative 16-bit signed.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_PCREL8: i32 = 6; /* PC-relative 8-bit signed.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GNU_VTINHERIT: i32 = 7; /* Ancient C++ vtable garbage... */
#[warn(non_camel_case_types)]
pub const R_MN10300_GNU_VTENTRY: i32 = 8; /* ... collection annotation.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_24: i32 = 9; /* Direct 24 bit.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOTPC32: i32 = 10; /* 32-bit PCrel offset to GOT.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOTPC16: i32 = 11; /* 16-bit PCrel offset to GOT.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOTOFF32: i32 = 12; /* 32-bit offset from GOT.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOTOFF24: i32 = 13; /* 24-bit offset from GOT.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOTOFF16: i32 = 14; /* 16-bit offset from GOT.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_PLT32: i32 = 15; /* 32-bit PCrel to PLT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_PLT16: i32 = 16; /* 16-bit PCrel to PLT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOT32: i32 = 17; /* 32-bit offset to GOT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOT24: i32 = 18; /* 24-bit offset to GOT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GOT16: i32 = 19; /* 16-bit offset to GOT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_COPY: i32 = 20; /* Copy symbol at runtime.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_GLOB_DAT: i32 = 21; /* Create GOT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_JMP_SLOT: i32 = 22; /* Create PLT entry.  */
#[warn(non_camel_case_types)]
pub const R_MN10300_RELATIVE: i32 = 23; /* Adjust by program base.  */


#[warn(non_camel_case_types)]
pub const R_MN10300_NUM: i32 = 24;




// M32R relocs.


#[warn(non_camel_case_types)]
pub const R_M32R_NONE: i32 = 0; /* No reloc. */
#[warn(non_camel_case_types)]
pub const R_M32R_16: i32 = 1; /* Direct 16 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_32: i32 = 2; /* Direct 32 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_24: i32 = 3; /* Direct 24 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_10_PCREL: i32 = 4; /* PC relative 10 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_18_PCREL: i32 = 5; /* PC relative 18 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_26_PCREL: i32 = 6; /* PC relative 26 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_HI16_ULO: i32 = 7; /* High 16 bit with unsigned low. */
#[warn(non_camel_case_types)]
pub const R_M32R_HI16_SLO: i32 = 8; /* High 16 bit with signed low. */
#[warn(non_camel_case_types)]
pub const R_M32R_LO16: i32 = 9; /* Low 16 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_SDA16: i32 = 10; /* 16 bit offset in SDA. */
#[warn(non_camel_case_types)]
pub const R_M32R_GNU_VTINHERIT: i32 = 11;
#[warn(non_camel_case_types)]
pub const R_M32R_GNU_VTENTRY: i32 = 12;
// M32R relocs use SHT_RELA.


#[warn(non_camel_case_types)]
pub const R_M32R_16_RELA: i32 = 33; /* Direct 16 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_32_RELA: i32 = 34; /* Direct 32 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_24_RELA: i32 = 35; /* Direct 24 bit. */
#[warn(non_camel_case_types)]
pub const R_M32R_10_PCREL_RELA: i32 = 36; /* PC relative 10 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_18_PCREL_RELA: i32 = 37; /* PC relative 18 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_26_PCREL_RELA: i32 = 38; /* PC relative 26 bit shifted. */
#[warn(non_camel_case_types)]
pub const R_M32R_HI16_ULO_RELA: i32 = 39; /* High 16 bit with unsigned low */
#[warn(non_camel_case_types)]
pub const R_M32R_HI16_SLO_RELA: i32 = 40; /* High 16 bit with signed low */
#[warn(non_camel_case_types)]
pub const R_M32R_LO16_RELA: i32 = 41; /* Low 16 bit */
#[warn(non_camel_case_types)]
pub const R_M32R_SDA16_RELA: i32 = 42; /* 16 bit offset in SDA */
#[warn(non_camel_case_types)]
pub const R_M32R_RELA_GNU_VTINHERIT: i32 = 43;
#[warn(non_camel_case_types)]
pub const R_M32R_RELA_GNU_VTENTRY: i32 = 44;
#[warn(non_camel_case_types)]
pub const R_M32R_REL32: i32 = 45; /* PC relative 32 bit.  */


#[warn(non_camel_case_types)]
pub const R_M32R_GOT24: i32 = 48; /* 24 bit GOT entry */
#[warn(non_camel_case_types)]
pub const R_M32R_26_PLTREL: i32 = 49; /* 26 bit PC relative to PLT shifted */
#[warn(non_camel_case_types)]
pub const R_M32R_COPY: i32 = 50; /* Copy symbol at runtime */
#[warn(non_camel_case_types)]
pub const R_M32R_GLOB_DAT: i32 = 51; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_M32R_JMP_SLOT: i32 = 52; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_M32R_RELATIVE: i32 = 53; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_M32R_GOTOFF: i32 = 54; /* 24 bit offset to GOT */
#[warn(non_camel_case_types)]
pub const R_M32R_GOTPC24: i32 = 55; /* 24 bit PC relative offset to GOT */








#[warn(non_camel_case_types)]
pub const R_M32R_GOT16_LO: i32 = 58; /* Low 16 bit GOT entry */




















#[warn(non_camel_case_types)]
pub const R_M32R_GOTOFF_LO: i32 = 64; /* Low 16 bit offset to GOT */
#[warn(non_camel_case_types)]
pub const R_M32R_NUM: i32 = 256; /* Keep this the last entry. */




// TILEPro relocations.


#[warn(non_camel_case_types)]
pub const R_TILEPRO_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_32: i32 = 1; /* Direct 32 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_16: i32 = 2; /* Direct 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_8: i32 = 3; /* Direct 8 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_32_PCREL: i32 = 4; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_16_PCREL: i32 = 5; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_8_PCREL: i32 = 6; /* PC relative 8 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_LO16: i32 = 7; /* Low 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_HI16: i32 = 8; /* High 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_HA16: i32 = 9; /* High 16 bit, adjusted */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_COPY: i32 = 10; /* Copy relocation */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_GLOB_DAT: i32 = 11; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_JMP_SLOT: i32 = 12; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_RELATIVE: i32 = 13; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_BROFF_X1: i32 = 14; /* X1 pipe branch offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_JOFFLONG_X1: i32 = 15; /* X1 pipe jump offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_JOFFLONG_X1_PLT: i32 = 16; /* X1 pipe jump offset to PLT */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_X0: i32 = 17; /* X0 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_Y0: i32 = 18; /* Y0 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_X1: i32 = 19; /* X1 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_Y1: i32 = 20; /* Y1 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MT_IMM15_X1: i32 = 21; /* X1 pipe mtspr */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MF_IMM15_X1: i32 = 22; /* X1 pipe mfspr */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0: i32 = 23; /* X0 pipe 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1: i32 = 24; /* X1 pipe 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_LO: i32 = 25; /* X0 pipe low 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_LO: i32 = 26; /* X1 pipe low 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_HI: i32 = 27; /* X0 pipe high 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_HI: i32 = 28; /* X1 pipe high 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_HA: i32 = 29; /* X0 pipe high 16-bit, adjusted */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_HA: i32 = 30; /* X1 pipe high 16-bit, adjusted */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_PCREL: i32 = 31; /* X0 pipe PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_PCREL: i32 = 32; /* X1 pipe PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_LO_PCREL: i32 = 33; /* X0 pipe PC relative low 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_LO_PCREL: i32 = 34; /* X1 pipe PC relative low 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_HI_PCREL: i32 = 35; /* X0 pipe PC relative high 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_HI_PCREL: i32 = 36; /* X1 pipe PC relative high 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_HA_PCREL: i32 = 37; /* X0 pipe PC relative ha() 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_HA_PCREL: i32 = 38; /* X1 pipe PC relative ha() 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_GOT: i32 = 39; /* X0 pipe 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_GOT: i32 = 40; /* X1 pipe 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_GOT_LO: i32 = 41; /* X0 pipe low 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_GOT_LO: i32 = 42; /* X1 pipe low 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_GOT_HI: i32 = 43; /* X0 pipe high 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_GOT_HI: i32 = 44; /* X1 pipe high 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_GOT_HA: i32 = 45; /* X0 pipe ha() 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_GOT_HA: i32 = 46; /* X1 pipe ha() 16-bit GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MMSTART_X0: i32 = 47; /* X0 pipe mm "start" */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MMEND_X0: i32 = 48; /* X0 pipe mm "end" */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MMSTART_X1: i32 = 49; /* X1 pipe mm "start" */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_MMEND_X1: i32 = 50; /* X1 pipe mm "end" */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_SHAMT_X0: i32 = 51; /* X0 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_SHAMT_X1: i32 = 52; /* X1 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_SHAMT_Y0: i32 = 53; /* Y0 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_SHAMT_Y1: i32 = 54; /* Y1 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_DEST_IMM8_X1: i32 = 55; /* X1 pipe destination 8-bit */
// Relocs 56-59 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEPRO_TLS_GD_CALL: i32 = 60; /* "jal" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_X0_TLS_GD_ADD: i32 = 61; /* X0 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_X1_TLS_GD_ADD: i32 = 62; /* X1 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_Y0_TLS_GD_ADD: i32 = 63; /* Y0 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM8_Y1_TLS_GD_ADD: i32 = 64; /* Y1 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_TLS_IE_LOAD: i32 = 65; /* "lw_tls" for TLS IE */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_GD: i32 = 66; /* X0 pipe 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_GD: i32 = 67; /* X1 pipe 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_GD_LO: i32 = 68; /* X0 pipe low 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_GD_LO: i32 = 69; /* X1 pipe low 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_GD_HI: i32 = 70; /* X0 pipe high 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_GD_HI: i32 = 71; /* X1 pipe high 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_GD_HA: i32 = 72; /* X0 pipe ha() 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_GD_HA: i32 = 73; /* X1 pipe ha() 16-bit TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_IE: i32 = 74; /* X0 pipe 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_IE: i32 = 75; /* X1 pipe 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_IE_LO: i32 = 76; /* X0 pipe low 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_IE_LO: i32 = 77; /* X1 pipe low 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_IE_HI: i32 = 78; /* X0 pipe high 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_IE_HI: i32 = 79; /* X1 pipe high 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_IE_HA: i32 = 80; /* X0 pipe ha() 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_IE_HA: i32 = 81; /* X1 pipe ha() 16-bit TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_TLS_DTPMOD32: i32 = 82; /* ID of module containing symbol */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_TLS_DTPOFF32: i32 = 83; /* Offset in TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_TLS_TPOFF32: i32 = 84; /* Offset in static TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_LE: i32 = 85; /* X0 pipe 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_LE: i32 = 86; /* X1 pipe 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_LE_LO: i32 = 87; /* X0 pipe low 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_LE_LO: i32 = 88; /* X1 pipe low 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_LE_HI: i32 = 89; /* X0 pipe high 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_LE_HI: i32 = 90; /* X1 pipe high 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X0_TLS_LE_HA: i32 = 91; /* X0 pipe ha() 16-bit TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_IMM16_X1_TLS_LE_HA: i32 = 92; /* X1 pipe ha() 16-bit TLS LE offset */


#[warn(non_camel_case_types)]
pub const R_TILEPRO_GNU_VTINHERIT: i32 = 128; /* GNU C++ vtable hierarchy */
#[warn(non_camel_case_types)]
pub const R_TILEPRO_GNU_VTENTRY: i32 = 129; /* GNU C++ vtable member usage */


#[warn(non_camel_case_types)]
pub const R_TILEPRO_NUM: i32 = 130;




// TILE-Gx relocations.


#[warn(non_camel_case_types)]
pub const R_TILEGX_NONE: i32 = 0; /* No reloc */
#[warn(non_camel_case_types)]
pub const R_TILEGX_64: i32 = 1; /* Direct 64 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_32: i32 = 2; /* Direct 32 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_16: i32 = 3; /* Direct 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_8: i32 = 4; /* Direct 8 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_64_PCREL: i32 = 5; /* PC relative 64 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_32_PCREL: i32 = 6; /* PC relative 32 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_16_PCREL: i32 = 7; /* PC relative 16 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_8_PCREL: i32 = 8; /* PC relative 8 bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW0: i32 = 9; /* hword 0 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW1: i32 = 10; /* hword 1 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW2: i32 = 11; /* hword 2 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW3: i32 = 12; /* hword 3 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW0_LAST: i32 = 13; /* last hword 0 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW1_LAST: i32 = 14; /* last hword 1 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_HW2_LAST: i32 = 15; /* last hword 2 16-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_COPY: i32 = 16; /* Copy relocation */
#[warn(non_camel_case_types)]
pub const R_TILEGX_GLOB_DAT: i32 = 17; /* Create GOT entry */
#[warn(non_camel_case_types)]
pub const R_TILEGX_JMP_SLOT: i32 = 18; /* Create PLT entry */
#[warn(non_camel_case_types)]
pub const R_TILEGX_RELATIVE: i32 = 19; /* Adjust by program base */
#[warn(non_camel_case_types)]
pub const R_TILEGX_BROFF_X1: i32 = 20; /* X1 pipe branch offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_JUMPOFF_X1: i32 = 21; /* X1 pipe jump offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_JUMPOFF_X1_PLT: i32 = 22; /* X1 pipe jump offset to PLT */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X0: i32 = 23; /* X0 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y0: i32 = 24; /* Y0 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X1: i32 = 25; /* X1 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y1: i32 = 26; /* Y1 pipe 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_DEST_IMM8_X1: i32 = 27; /* X1 pipe destination 8-bit */
#[warn(non_camel_case_types)]
pub const R_TILEGX_MT_IMM14_X1: i32 = 28; /* X1 pipe mtspr */
#[warn(non_camel_case_types)]
pub const R_TILEGX_MF_IMM14_X1: i32 = 29; /* X1 pipe mfspr */
#[warn(non_camel_case_types)]
pub const R_TILEGX_MMSTART_X0: i32 = 30; /* X0 pipe mm "start" */
#[warn(non_camel_case_types)]
pub const R_TILEGX_MMEND_X0: i32 = 31; /* X0 pipe mm "end" */
#[warn(non_camel_case_types)]
pub const R_TILEGX_SHAMT_X0: i32 = 32; /* X0 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEGX_SHAMT_X1: i32 = 33; /* X1 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEGX_SHAMT_Y0: i32 = 34; /* Y0 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEGX_SHAMT_Y1: i32 = 35; /* Y1 pipe shift amount */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0: i32 = 36; /* X0 pipe hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0: i32 = 37; /* X1 pipe hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1: i32 = 38; /* X0 pipe hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1: i32 = 39; /* X1 pipe hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW2: i32 = 40; /* X0 pipe hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW2: i32 = 41; /* X1 pipe hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW3: i32 = 42; /* X0 pipe hword 3 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW3: i32 = 43; /* X1 pipe hword 3 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST: i32 = 44; /* X0 pipe last hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST: i32 = 45; /* X1 pipe last hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST: i32 = 46; /* X0 pipe last hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST: i32 = 47; /* X1 pipe last hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW2_LAST: i32 = 48; /* X0 pipe last hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW2_LAST: i32 = 49; /* X1 pipe last hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_PCREL: i32 = 50; /* X0 pipe PC relative hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_PCREL: i32 = 51; /* X1 pipe PC relative hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_PCREL: i32 = 52; /* X0 pipe PC relative hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_PCREL: i32 = 53; /* X1 pipe PC relative hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW2_PCREL: i32 = 54; /* X0 pipe PC relative hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW2_PCREL: i32 = 55; /* X1 pipe PC relative hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW3_PCREL: i32 = 56; /* X0 pipe PC relative hword 3 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW3_PCREL: i32 = 57; /* X1 pipe PC relative hword 3 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST_PCREL: i32 = 58; /* X0 pipe PC-rel last hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST_PCREL: i32 = 59; /* X1 pipe PC-rel last hword 0 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST_PCREL: i32 = 60; /* X0 pipe PC-rel last hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST_PCREL: i32 = 61; /* X1 pipe PC-rel last hword 1 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW2_LAST_PCREL: i32 = 62; /* X0 pipe PC-rel last hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW2_LAST_PCREL: i32 = 63; /* X1 pipe PC-rel last hword 2 */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_GOT: i32 = 64; /* X0 pipe hword 0 GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_GOT: i32 = 65; /* X1 pipe hword 0 GOT offset */
// Relocs 66-71 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST_GOT: i32 = 72; /* X0 pipe last hword 0 GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST_GOT: i32 = 73; /* X1 pipe last hword 0 GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST_GOT: i32 = 74; /* X0 pipe last hword 1 GOT offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST_GOT: i32 = 75; /* X1 pipe last hword 1 GOT offset */
// Relocs 76-77 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_TLS_GD: i32 = 78; /* X0 pipe hword 0 TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_TLS_GD: i32 = 79; /* X1 pipe hword 0 TLS GD offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_TLS_LE: i32 = 80; /* X0 pipe hword 0 TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_TLS_LE: i32 = 81; /* X1 pipe hword 0 TLS LE offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: i32 = 82; /* X0 pipe last hword 0 LE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: i32 = 83; /* X1 pipe last hword 0 LE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: i32 = 84; /* X0 pipe last hword 1 LE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: i32 = 85; /* X1 pipe last hword 1 LE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: i32 = 86; /* X0 pipe last hword 0 GD off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: i32 = 87; /* X1 pipe last hword 0 GD off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: i32 = 88; /* X0 pipe last hword 1 GD off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: i32 = 89; /* X1 pipe last hword 1 GD off */
// Relocs 90-91 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_TLS_IE: i32 = 92; /* X0 pipe hword 0 TLS IE offset */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_TLS_IE: i32 = 93; /* X1 pipe hword 0 TLS IE offset */
// Relocs 94-99 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: i32 = 100; /* X0 pipe last hword 0 IE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: i32 = 101; /* X1 pipe last hword 0 IE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: i32 = 102; /* X0 pipe last hword 1 IE off */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: i32 = 103; /* X1 pipe last hword 1 IE off */
// Relocs 104-105 are currently not defined.


#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_DTPMOD64: i32 = 106; /* 64-bit ID of symbol's module */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_DTPOFF64: i32 = 107; /* 64-bit offset in TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_TPOFF64: i32 = 108; /* 64-bit offset in static TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_DTPMOD32: i32 = 109; /* 32-bit ID of symbol's module */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_DTPOFF32: i32 = 110; /* 32-bit offset in TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_TPOFF32: i32 = 111; /* 32-bit offset in static TLS block */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_GD_CALL: i32 = 112; /* "jal" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X0_TLS_GD_ADD: i32 = 113; /* X0 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X1_TLS_GD_ADD: i32 = 114; /* X1 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y0_TLS_GD_ADD: i32 = 115; /* Y0 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y1_TLS_GD_ADD: i32 = 116; /* Y1 pipe "addi" for TLS GD */
#[warn(non_camel_case_types)]
pub const R_TILEGX_TLS_IE_LOAD: i32 = 117; /* "ld_tls" for TLS IE */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X0_TLS_ADD: i32 = 118; /* X0 pipe "addi" for TLS GD/IE */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_X1_TLS_ADD: i32 = 119; /* X1 pipe "addi" for TLS GD/IE */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y0_TLS_ADD: i32 = 120; /* Y0 pipe "addi" for TLS GD/IE */
#[warn(non_camel_case_types)]
pub const R_TILEGX_IMM8_Y1_TLS_ADD: i32 = 121; /* Y1 pipe "addi" for TLS GD/IE */


#[warn(non_camel_case_types)]
pub const R_TILEGX_GNU_VTINHERIT: i32 = 128; /* GNU C++ vtable hierarchy */
#[warn(non_camel_case_types)]
pub const R_TILEGX_GNU_VTENTRY: i32 = 129; /* GNU C++ vtable member usage */


#[warn(non_camel_case_types)]
pub const R_TILEGX_NUM: i32 = 130;
