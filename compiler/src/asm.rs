pub enum Register {
  Rax
}

impl Register {
    pub fn as_str(&self) -> &str {
        match *self {
            Register::Rax => "%rax"
        }
    }
}

pub enum Asm {
    Movq(u64, Register),
    Ret
}

impl Asm {
    pub fn to_string(&self) -> String {
        match *self {
            Asm::Movq(val, ref r) => format!("movq ${}, {}", val, r.as_str()),
            Asm::Ret             => "ret".to_string()
        }
    }
}

pub fn asm_to_string(asm: Vec<Asm>) -> String {
    let instructions = asm.into_iter()
        .map(|instr| format!("\t{}", instr.to_string()))
        .collect::<Vec<String>>()
        .join("\n");
    format!(".text
.globl _entry
_entry:
{}
", instructions)
}
