import requests

import re
from pprint import pprint

def lines():
    h = requests.get("https://raw.githubusercontent.com/lattera/glibc/a2f34833b1042d5d8eeb263b4cf4caaea138c4ad/elf/elf.h").text
    for line in h.splitlines():
        yield line


def parseTypeDef(s):
    print s

print ("// AUTO-GENERATED FILE, DO NOT EDIT [elftools_const.rs]")
print ("\n")
print ("extern crate libc;")
# print ("use libc::void_t;")
print ("\n")

test = ["^typedef", parseTypeDef]
type_mapping = {
        "uint16_t": "u16",
        "uint32_t": "u32",
        "uint64_t": "u64",
        "int16_t": "i16",
        "int32_t": "i32",
        "int64_t": "i64",
        "unsigned char": "u8",
}

name_type_mapping = {
        "EI_NIDENT": "usize",
}


# todo
# types for names (regex)
# grouping

#from pyparsing import Word, alphas?

h = requests.get("https://raw.githubusercontent.com/lattera/glibc/a2f34833b1042d5d8eeb263b4cf4caaea138c4ad/elf/elf.h").text
i = 0
while i < len(h):
    end = h.find("\n", i) + 1

    p = re.compile("^typedef\s+struct\s+$", re.MULTILINE).match(h, i, end)
    if p:
        p = re.compile("^typedef\s+struct\s+{(((?:\s+.+?)\n)+)}\s+(\w+)\;", re.MULTILINE).match(h, i)
        #print ("struct {0}".format(h[p.start():p.end()]))
        print ("#[repr(C)]")
        print ("#[derive(Clone, Debug)]")
        print ("pub struct {0} {{".format(p.group(3)))
        for l in p.group(1).split('\n'):
            j = re.compile("^\s+((?:unsigned?\s+)?[A-Za-z0-9_]+)\s+([A-Za-z0-9_]+)\s*\;\s*(\/\*.+?\*\/)?\s*$", re.MULTILINE).match(l)
            if j:
                t = j.group(1)
                try:
                    t = type_mapping[j.group(1)]
                except:
                    pass

                try:
                    t = name_type_mapping[j.group(2)]
                except Exception, exc:
                    pass
                    pass

                print ("pub {2}: {0}, {3}".format(t, j.group(1), j.group(2), j.group(3)))
            j = re.compile("^\s+((?:unsigned?\s+)?[A-Za-z0-9_]+)\s+([A-Za-z0-9_]+)\[([A-Za-z0-9_]+)\]\s*\;\s*(\/\*.+?\*\/)?\s*$", re.MULTILINE).match(l)
            if j:
                t = j.group(1)
                try:
                    t = type_mapping[j.group(1)]
                except Exception, exc:
                    pass

                try:
                    t = name_type_mapping[j.group(2)]
                except Exception, exc:
                    pass

                print ("pub {2}: [{0}; {4}], {3}".format(t, j.group(2), j.group(2), j.group(4), j.group(3)))
        print ("}")
        #pprint(p)
        i = p.end()
        continue

    p = re.compile("^typedef\s+([A-Za-z0-9_]+)\s+([A-Za-z0-9_]+);$", re.MULTILINE).match(h, i, end)
    if p:
        #print ("typedef2 {0} {1} {2}".format(h[p.start():p.end()], p.group(1), p.group(2)))
        t = p.group(1)
        try:
            t = type_mapping[p.group(1)]
        except Exception, exc:
            pass

        try:
            t = name_type_mapping[p.group(2)]
        except Exception, exc:
            pass

        print "#[allow(non_camel_case_types)]\npub type {1} = {0};".format(t, p.group(2))
        # pprint(p)
        i = p.end()
        continue

    p = re.compile("^\/\/", re.MULTILINE).match(h, i, end)
    if p:
        # search for end */
        print ("{0}".format(h[p.start():p.end()]))
        i = p.end()
        continue

    p = re.compile("^\/\*", re.MULTILINE).match(h, i, end)
    if p:
        # search for end */
        n = re.compile("\*\/").search(h, p.end())
        print ("{0}".format(h[p.start():n.end()]))
        i = n.end()
        continue

    p = re.compile("^#define\s+([A-Za-z0-9_]+)\s+((?:\(.+?\))|(?:[A-Za-z0-9_]+))\s*(\/\*.+?\*\/)?\s*$", re.MULTILINE).match(h, i, end)
    #define STT_HP_OPAQUE		(STT_LOOS + 0x1)
    if p:
        # todo(nl5887): group and combine as bitflags!
        """
        print ("bitflags! {")
        print ("pub flags MODE: u32 {")
        print ("const MODE: u32 {")
        print ("}")
        group on left(("_"), add to array 
        """

        """
        bitflags! {
            pub flags Mode : u32 {
                    const MODE_LITTLE_ENDIAN = 0,
                    const MODE_BIG_ENDIAN = 1073741824,
                    const MODE_ARM = 1,
                    const MODE_THUMB = 16,
                    const MODE_V8 = 64,
                    const MODE_MICRO = 16,
                    const MODE_MIPS3 = 32,
                    const MODE_MIPS32R6 = 64,
                    const MODE_MIPS32 = 4,
                    const MODE_MIPS64 = 8,
                    const MODE_16 = 2,
                    const MODE_32 = 4,
                    const MODE_64 = 8,
                    const MODE_PPC32 = 4,
                    const MODE_PPC64 = 8,
                    const MODE_QPX = 16,
                    const MODE_SPARC32 = 4,
                    const MODE_SPARC64 = 8,
                    const MODE_V9 = 16,
            }
        }
        """
        t = "i32"

        try:
            t = name_type_mapping[p.group(1)]
        except Exception, exc:
            pass

        print ("#[warn(non_camel_case_types)]")
        print ("pub const {0}: {3} = {1}; {2}".format(p.group(1), p.group(2), p.group(3) or "", t))
        #pprint(p)
        i = p.end()
        continue

    # find next line
    i = end
    print ("\n")
    continue

import sys
sys.exit(0)

#print (h)

#m = re.search('typedef\s+struct\s+{(.+?)}\s+\w+\;', h)
m = re.compile('typedef\s+struct\s+{(((?:\s+.+?)\n)+)}\s+(\w+)\;', re.MULTILINE)
result = m.finditer(h)
for n in result:
    #print n.group(0)
    print "#[derive(Debug)]\npub struct {0} {{".format(n.group(2))
    for l in n.group(1).split('\n'):
        print l
    #pprint (n.group(1))
    #print n.group(3)

    print "}"


m = re.compile('#define\s+(\w+)\s+(.+?)\s+(\/\*.+?\*\/)?\n', re.MULTILINE)
result = m.finditer(h)
for n in result:
    print "pub const {0} : i32 = {1}; {2}".format(n.group(1), n.group(2), n.group(3) or "")

#
#typedef struct
#{
#  unsigned char	e_ident[EI_NIDENT];	/* Magic number and other info */
#  Elf64_Half	e_type;			/* Object file type */
#  Elf64_Half	e_machine;		/* Architecture */
#  Elf64_Word	e_version;		/* Object file version */
#  Elf64_Addr	e_entry;		/* Entry point virtual address */
#  Elf64_Off	e_phoff;		/* Program header table file offset */
#  Elf64_Off	e_shoff;		/* Section header table file offset */
#  Elf64_Word	e_flags;		/* Processor-specific flags */
#  Elf64_Half	e_ehsize;		/* ELF header size in bytes */
#  Elf64_Half	e_phentsize;		/* Program header table entry size */
#  Elf64_Half	e_phnum;		/* Program header table entry count */
#  Elf64_Half	e_shentsize;		/* Section header table entry size */
#  Elf64_Half	e_shnum;		/* Section header table entry count */
#  Elf64_Half	e_shstrndx;		/* Section header string table index */
#} Elf64_Ehdr;
#
#
