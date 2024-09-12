use std::fmt::format;

use crate::{ir::IrOperand, types, Type};

#[allow(unused)]
pub enum Registers {
    Rip,
    Rsp,
    Rax,
    Rdi,
    Rsi,
    Rdx,
    Rcx,
    R8,
    R9,
    R10,
    R11,
    Rbx,
    Rbp,
    R12,
    R13,
    R14,
    R15,
}
//RDI, RSI, RDX, RCX, R8, R9
//XMM0 - XMM7
//stack
const INT_ARG_NAMES: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
const FLOAT_ARG_NAMES: &[&'static str] = &[
    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
];
pub struct ArgCPU {
    int_registers: [u8; 6],
    float_registers: [u8; 8],
}
impl ArgCPU {
    pub fn new() -> Self {
        return Self {
            int_registers: [0, 0, 0, 0, 0, 0],
            float_registers: [0, 0, 0, 0, 0, 0, 0, 0],
        };
    }
    pub fn get_next_location(&mut self) -> Option<String> {
        for i in 0..6 {
            if self.int_registers[i] == 0 {
                self.int_registers[i] = 8;
                return Some(String::from(INT_ARG_NAMES[i]));
            }
        }
        unreachable!();
        None
    }
    pub fn get_next_fp_location(&mut self) -> Option<String> {
        for i in 0..6 {
            if self.float_registers[i] == 0 {
                self.float_registers[i] = 8;
                return Some(String::from(FLOAT_ARG_NAMES[i]));
            }
        }
        None
    }
    fn generate_basic_arg(
        &mut self,
        op_name: &str,
        size: usize,
        offset: usize,
        to_pop_stack: &mut usize,
        is_address_of:bool
    ) -> String {
        static SIZES: &[&'static str] = &["BYTE", "WORD", "", "DWORD", "", "", "", "QWORD"];
        println!("{op_name}");
        if let Some(rname) = self.get_next_location() {
            if !is_address_of{
                return format!("    mov {}, {}\n", rname, op_name);
            }
            else{
                if offset != 0{
                    return format!("    mov {}, {} [{}-{offset}]\n", rname, SIZES[size-1], op_name)
                } else{
                    return format!("    mov {}, {} [{}]\n", rname, SIZES[size-1], op_name);
                }

            }
        }
        *to_pop_stack += 1;
        if op_name.as_bytes()[0] != b'r' {
            return format!("    push {}\n", op_name);
        }
        return format!("    push {}\n", op_name);
    }
    pub fn generate_arg(&mut self, arg_v: &str, arg_t: &Type, to_pop_stack: &mut usize) -> String {
        let mut out = String::new();
        match arg_t {
            types::Type::ArrayT {
                size: _,
                array_type: _,
            } => {
                unreachable!();
            }
            types::Type::BoolT => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, false);
            }
            types::Type::CharT => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,false);
            }
            types::Type::FloatT => {
                todo!();
            }
            types::Type::IntegerT => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,false);
            }
            types::Type::PointerT { ptr_type: _ } => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, false);
            }
            types::Type::SliceT { ptr_type: _ } => {
                out += &self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,true);
                out += &self.generate_basic_arg(arg_v, 8, 8, to_pop_stack, true);
                return out;
            }
            types::Type::StringT => {
                out += &self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, true);
                out += &self.generate_basic_arg(arg_v, 8, 8, to_pop_stack, true);
                return out;
            }
            types::Type::StructT {
                name: _,
                components,
            } => {
                let mut offset = 0;
                for i in components {
                    let op = format!("{arg_v}-{offset}");
                    out += &self.generate_arg(&op, arg_t, to_pop_stack);
                    offset += i.1.get_size_bytes();
                }
                return out;
            }
            _ => {
                unreachable!();
            }
        }
        return out;
    }
}