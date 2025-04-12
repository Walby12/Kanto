use core::panic;
use std::usize;

const STACK_CAP: usize = 1024;

type Word = usize;

#[derive(Clone)]
struct Kanto {
    stack: Vec<Word>,
    stack_size: usize
}

#[derive(Debug)]
enum InstType {
    InstPush,
    InstPlus,
    InstMinus,
    InstMul,
    InstDiv,
    InstDump,
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
        _ => panic!("Unknown instruction: {}", inst),
    };

    Inst {
        itype: x,
        operand: op,
    }
}

fn exec_inst(vm: &mut Kanto, insts: Vec<Inst>) {
    for inst in insts.iter() {
        match inst.itype {
            InstType::InstPush => kanto_push(vm, inst),
            InstType::InstPlus => kanto_plus(vm),
            InstType::InstMinus => kanto_minus(vm),
            InstType::InstMul => kanto_mul(vm),
            InstType::InstDiv => kanto_div(vm),
            InstType::InstDump => kanto_dump(vm),
        }
    }
}

fn main() {
    let mut kanto = Kanto {
        stack: Vec::new(),
        stack_size: 0,
    };

    let program = vec![
        create_inst("push", 10),
        create_inst("push", 10),
        create_inst("minus", 0),
        create_inst("dump", 0),
    ];

    exec_inst(&mut kanto, program);
}
