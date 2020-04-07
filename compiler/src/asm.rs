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
    Label(String),
    Movq(u64, Register),
    Addq(u64, Register),
    Subq(u64, Register),
    Cmpq(u64, Register),
    Jmp(String),
    Jne(String),
    Ret
}

impl Asm {
    pub fn to_string(&self) -> String {
        match &*self {
            Asm::Label(l)         => format!("{}:", l),
            Asm::Movq(val, ref r) => format!("\tmovq ${}, {}", val, r.as_str()),
            Asm::Addq(val, ref r) => format!("\taddq ${}, {}", val, r.as_str()),
            Asm::Subq(val, ref r) => format!("\tsubq ${}, {}", val, r.as_str()),
            Asm::Cmpq(val, ref r) => format!("\tcmpq ${}, {}", val, r.as_str()),
            Asm::Jmp(label)       => format!("\tjmp {}", label),
            Asm::Jne(label)       => format!("\tjne {}", label),
            Asm::Ret              => "ret".to_string()
        }
    }
}

pub fn asm_to_string(asm: Vec<Asm>) -> String {
    let instructions = asm.into_iter()
        .map(|instr| instr.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    format!(".text
.globl _entry
_entry:
{}
", instructions)
}
