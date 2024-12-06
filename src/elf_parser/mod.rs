use crate::vm::WORD_SIZE;

pub(crate) struct ElfHeader {
    pub(crate) e_ident: u32,  // file identifier
    pub(crate) e_os_abi: u32, // os abi
    pub(crate) e_type: u32,
    pub(crate) e_machine: u32, // machine type  - riscv
    pub(crate) e_version: u32,
    pub(crate) e_entry: u32, // program entry point
    pub(crate) e_phoff: u32, // program header offset
    pub(crate) e_shoff: u32, // segment header offset
    pub(crate) e_flags: u32,
    pub(crate) e_ehsize: u32,
    pub(crate) e_phentsize: u32, // program header entry size
    pub(crate) e_phnum: u32,     // number of program headers
    pub(crate) e_shentsize: u32,
    pub(crate) e_shnum: u32,
    pub(crate) e_shstrndx: u32,
    pub(crate) e_ph: Vec<ProgramHeader>, // vec of program headers
    pub(crate) e_sh: Vec<SectionHeader>, // vec of program headers
}

impl Default for ElfHeader {
    fn default() -> Self {
        Self {
            e_ident: Default::default(),
            e_os_abi: Default::default(),
            e_type: Default::default(),
            e_machine: Default::default(),
            e_version: Default::default(),
            e_entry: Default::default(),
            e_phoff: Default::default(),
            e_shoff: Default::default(),
            e_flags: Default::default(),
            e_ehsize: Default::default(),
            e_phentsize: Default::default(),
            e_phnum: Default::default(),
            e_shentsize: Default::default(),
            e_shnum: Default::default(),
            e_shstrndx: Default::default(),
            e_ph: Default::default(),
            e_sh: Default::default(),
        }
    }
}

// Segment
#[derive(Debug)]
pub(crate) struct ProgramHeader {
    pub(crate) ph_type: u32,
    pub(crate) offset: u32,
    pub(crate) virtual_address: u32,
    pub(crate) physical_address: u32,
    pub(crate) file_size: u32,
    pub(crate) memory_size: u32,
    pub(crate) flags: u32,
    pub(crate) align: u32,
}

impl Default for ProgramHeader {
    fn default() -> Self {
        Self {
            ph_type: Default::default(),
            offset: Default::default(),
            virtual_address: Default::default(),
            physical_address: Default::default(),
            file_size: Default::default(),
            memory_size: Default::default(),
            flags: Default::default(),
            align: Default::default(),
        }
    }
}

// Sections
#[derive(Debug)]
pub(crate) struct SectionHeader {
    name: u32,
    sh_type: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    addr_align: u32,
    ent_size: u32, // entry size
}

impl Default for SectionHeader {
    fn default() -> Self {
        Self {
            name: Default::default(),
            sh_type: Default::default(),
            flags: Default::default(),
            addr: Default::default(),
            offset: Default::default(),
            size: Default::default(),
            link: Default::default(),
            info: Default::default(),
            addr_align: Default::default(),
            ent_size: Default::default(),
        }
    }
}

impl ElfHeader {
    pub(crate) fn decode_elf(file_in_byte: &Vec<u8>) -> ElfHeader {
        assert_eq!(
            get_data_at_index(file_in_byte, 0x00, WORD_SIZE),
            [0x7f, 0x45, 0x4c, 0x46],
            "Magic number: Not elf format"
        );
        assert_eq!(
            get_data_at_index(file_in_byte, 0x04, 1)[0],
            1,
            "File should be 32 bits"
        );
        assert_eq!(
            get_data_at_index(file_in_byte, 0x05, 1)[0],
            1,
            "File should be LSB encoded"
        );
        assert_eq!(
            get_data_at_index(file_in_byte, 0x12, 1)[0],
            0xF3,
            "E_Machine: machine type should be riscv"
        );

        let mut res = ElfHeader::default();

        res.e_entry = u32::from_le_bytes(
            get_data_at_index(file_in_byte, 0x18, WORD_SIZE)
                .try_into()
                .unwrap(),
        ); // entry point to the program
        res.e_phoff = u32::from_le_bytes(
            get_data_at_index(file_in_byte, 0x1c, WORD_SIZE)
                .try_into()
                .unwrap(),
        ); // program header offset
        res.e_shoff = u32::from_le_bytes(
            get_data_at_index(file_in_byte, 0x20, WORD_SIZE)
                .try_into()
                .unwrap(),
        ); // section header offset

        let mut os_abi = [0; 4];
        os_abi[0..1].copy_from_slice(get_data_at_index(file_in_byte, 0x07, 1).try_into().unwrap());

        res.e_os_abi = u32::from_le_bytes(os_abi);

        let mut phentsize = [0; 4];
        phentsize[0..2]
            .copy_from_slice(get_data_at_index(file_in_byte, 0x2A, 2).try_into().unwrap());
        res.e_phentsize = u32::from_le_bytes(phentsize);

        let mut phnum = [0; 4];
        phnum[0..2].copy_from_slice(get_data_at_index(file_in_byte, 0x2C, 2).try_into().unwrap());
        res.e_phnum = u32::from_le_bytes(phnum);

        let mut shentsize = [0; 4];
        shentsize[0..2]
            .copy_from_slice(get_data_at_index(file_in_byte, 0x2E, 2).try_into().unwrap());
        res.e_shentsize = u32::from_le_bytes(shentsize);

        let mut shnum = [0; 4];
        shnum[0..2].copy_from_slice(get_data_at_index(file_in_byte, 0x30, 2).try_into().unwrap());
        res.e_shnum = u32::from_le_bytes(shnum);

        let mut shstrndx = [0; 4];
        shstrndx[0..2]
            .copy_from_slice(get_data_at_index(file_in_byte, 0x32, 2).try_into().unwrap());
        res.e_shstrndx = u32::from_le_bytes(shstrndx);

        for i in 0..res.e_phnum {
            let start = (res.e_phoff + (i * res.e_phentsize)) as usize;
            let ph_data: &[u8] = get_data_at_index(file_in_byte, start, res.e_phentsize as usize)
                .try_into()
                .unwrap();

            let mut ph = ProgramHeader::default();
            ph.ph_type = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x00, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            ph.offset = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x04, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            ph.virtual_address = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x08, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            ph.file_size = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x10, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            ph.memory_size = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x14, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            ph.flags = u32::from_le_bytes(
                get_data_at_index(&ph_data, 0x18, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );

            res.e_ph.push(ph);
        }

        for i in 0..res.e_shnum {
            let start = (res.e_shoff + (i * res.e_shentsize)) as usize;

            let sh_data: &[u8] = get_data_at_index(file_in_byte, start, res.e_shentsize as usize)
                .try_into()
                .unwrap();

            let mut sh = SectionHeader::default();

            sh.name = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x00, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            sh.sh_type = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x04, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            sh.flags = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x08, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            sh.addr = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x0C, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            sh.offset = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x10, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );
            sh.size = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x14, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );

            sh.ent_size = u32::from_le_bytes(
                get_data_at_index(&sh_data, 0x24, WORD_SIZE)
                    .try_into()
                    .unwrap(),
            );

            res.e_sh.push(sh);
        }

        res
    }
}

pub(crate) fn get_data_at_index(data: &[u8], start: usize, size: usize) -> &[u8] {
    &data[start..start + size]
}
