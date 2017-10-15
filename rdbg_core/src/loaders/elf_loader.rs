use goblin;

use formats::elf_program::ElfProgram;
use loaders::ProgramLoader;
use util::error::RdbgResult;

pub struct ElfLoader;

impl ProgramLoader for ElfLoader {
    fn load(buffer: &[u8]) -> RdbgResult<ElfProgram> {
        let mut program = ElfProgram::new();
        let binary = goblin::elf::Elf::parse(&buffer)?;

        program.entry = binary.entry;
        program.is_64 = binary.is_64;
        program.is_lib = binary.is_lib;


        debug!("elf: {:#?}", &binary);
        Ok(program)
    }
}