use core::panic;
use std::usize;

const STACK_CAP: usize = 1024;

type Word = usize;

#[derive(Clone)]
struct Kanto {
    stack: Vec<Word>,
    stack_size: usize,
    ip: Word,
    halt: bool,
}

#[derive(Debug)]
enum InstType {
    InstPush,
    InstPlus,
    InstMinus,
    InstMul,
    InstDiv,
    InstDump,
    InstJmp,
    InstHalt,
    InstJmpIf0,
}

struct Inst {
    itype: InstType,
    operand: Word,
}


fn kanto_push(vm: &mut Kanto, inst: &Inst){
    if vm.stack_size > STACK_CAP {
        panic!("Stack overflow");
    }
    vm.stack.push(inst.operand);
    vm.stack_size += 1;
}

fn kanto_plus(vm: &mut Kanto){
    if vm.stack_size < 2 {
        panic!("Stack size must me at least two to perform the plus op");
    }

    let a = vm.stack.pop().unwrap();
    let b = vm.stack.pop().unwrap();
    vm.stack.push(a + b);
    vm.stack_size -= 1;
}

fn kanto_minus(vm: &mut Kanto) {
    if vm.stack_size < 2 {
        panic!("Stack size must me at least two to perform the minus op");
    }

    let a = vm.stack.pop().unwrap();
    let b = vm.stack.pop().unwrap();
    vm.stack.push(b - a);
    vm.stack_size -= 1;
}

fn kanto_mul(vm: &mut Kanto) {
    if vm.stack_size < 2 {
        panic!("Stack size must me at least two to perform the mul op");
    }

    let a = vm.stack.pop().unwrap();
    let b = vm.stack.pop().unwrap();
    vm.stack.push(a * b);
    vm.stack_size -= 1;
}

fn kanto_div(vm: &mut Kanto) {
    if vm.stack_size < 2 {
        panic!("Stack size must me at least two to perform the div op");
    }

    let a = vm.stack.pop().unwrap();
    if a == 0 {
        panic!("Division by zero is not possible!");
    }
    let b = vm.stack.pop().unwrap();
    vm.stack.push(b / a);
    vm.stack_size -= 1;
}

fn kanto_dump(vm: &mut Kanto) {
    println!("Stack: ");
    if vm.stack_size > 0 {
        for i in 0..vm.stack_size {
            println!("{}", vm.stack[i]);
        }
    } else {
        println!("[empty]");
    }
}

fn create_inst(inst: &str, op: usize) -> Inst {
    let x = match inst {
        "push" => InstType::InstPush,
        "plus" => InstType::InstPlus,
        "minus" => InstType::InstMinus,
        "mul" => InstType::InstMul,
        "div" => InstType::InstDiv,
        "dump" => InstType::InstDump,
        "jmp" => InstType::InstJmp,
        "halt" => InstType::InstHalt,
        "jmp_if_0" => InstType::InstJmpIf0,
        _ => panic!("Unknown instruction: {}", inst),
    };

    Inst {
        itype: x,
        operand: op,
    }
}

fn exec_inst(vm: &mut Kanto, insts: &[Inst]) {
    while !vm.halt {
        let inst = &insts[vm.ip];

        match inst.itype {
            InstType::InstPush => {
                kanto_push(vm, inst);
                vm.ip += 1;
            }
            InstType::InstPlus => {
                kanto_plus(vm);
                vm.ip += 1;
            }
            InstType::InstMinus => {
                kanto_minus(vm);
                vm.ip += 1;
            }
            InstType::InstDiv => {
                kanto_div(vm);
                vm.ip += 1;
            }
            InstType::InstMul => {
                kanto_mul(vm);
                vm.ip += 1;
            }
            InstType::InstDump => {
                kanto_dump(vm);
                vm.ip += 1;
            }
            InstType::InstJmp => {
                if inst.operand >= insts.len() {
                    println!("Jump out of bounds! Exiting.");
                    return;
                }
                vm.ip = inst.operand;
            }
            InstType::InstHalt => {
                println!("Execution halted!");
                break;
            }
            InstType::InstJmpIf0 => {
                if vm.stack_size == 0 {
                    println!("Stack underflow during jmp_if_0!");
                    return; 
                }

                let top = vm.stack[vm.stack_size - 1];

                if top == 0 {
                    if inst.operand >= insts.len() {
                        println!("Jump out of bounds! Exiting.");
                        return;
                    }
                    vm.ip = inst.operand;
                } else {
                    vm.ip += 1;
                }
            }

        }
    }
}

fn main() {
    let mut kanto = Kanto {
        stack: Vec::new(),
        stack_size: 0,
        ip: 0,
        halt: false,
    };

    let program = vec![
        create_inst("push", 0),
        create_inst("jmp_if_0", 4),
        create_inst("push", 99),
        create_inst("dump", 0),
        create_inst("push", 42),
        create_inst("dump", 0),
        create_inst("halt", 0),   
    ];

    exec_inst(&mut kanto, &program);
}
