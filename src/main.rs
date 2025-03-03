mod parser;
mod riscv;
mod binary;

use tracing::Level;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use crate::parser::token::get_tokens;
use binary::{elf::Elf, Binary, Section};
use object::write::Object;
use object::{Architecture, Endianness};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Cannot set default subscriber");

    let input = std::fs::read_to_string("test.tir").unwrap();
    let tokens = get_tokens(input);
    println!("{:?}", tokens); 

    let mut elf = Elf::new(Architecture::Riscv64, Endianness::Little);

    let mut f = std::fs::File::create("output.elf").unwrap();

    elf.create_section(Section::Text);
    elf.write_section(Section::Text, vec![]);
    
    elf.save(&mut f).unwrap();
}
