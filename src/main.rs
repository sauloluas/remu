use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};

use algin::{Vector, Zero};

pub type Byte = u8;
pub type IWord = u16;

pub trait Word: Clone + Copy + Zero {}

impl Word for Byte {}
impl Word for IWord {}

pub struct Processor {
    bank: RegisterBank,
}
pub type Cpu = Processor;

impl Processor {
    pub fn new() -> Self {
        let bank = RegisterBank::init();

        Self { bank }
    }
}

#[derive(Debug)]
pub struct RegisterBank {
    acc: Byte,
    bacc: Byte,
    carr: Byte,
    idx: Byte,
    hi: Byte,
    lo: Byte,
    ret: Byte,
    tmp: Byte,
}
pub type RegBank = RegisterBank;

impl RegisterBank {
    pub fn init() -> Self {
        Self {
            acc: 0,
            bacc: 0,
            carr: 0,
            idx: 0,
            hi: 0,
            lo: 0,
            ret: 0,
            tmp: 0,
        }
    }
}

impl Index<RegAddress> for RegisterBank {
    type Output = Byte;

    fn index(&self, idx: RegAddress) -> &Byte {
        match idx {
            Acc => &self.acc,
            Bacc => &self.bacc,
            Carr => &self.carr,
            Idx => &self.idx,
            Hi => &self.hi,
            Lo => &self.lo,
            Ret => &self.ret,
            Tmp => &self.tmp
        }
    }
}

impl IndexMut<RegAddress> for RegisterBank {
    fn index_mut(&mut self, idx: RegAddress) -> &mut Byte {
        match idx {
            Acc => &mut self.acc,
            Bacc => &mut self.bacc,
            Carr => &mut self.carr,
            Idx => &mut self.idx,
            Hi => &mut self.hi,
            Lo => &mut self.lo,
            Ret => &mut self.ret,
            Tmp => &mut self.tmp
        }
    }
}

pub struct Memory<T: Word> {
    cells: Vector<T>,
}
pub type Mem<T> = Memory<T>;
pub type Mem8 = Memory<Byte>;
pub type Mem16 = Memory<u16>;

impl<T: Word + Zero> Memory<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cells: Vector::zeros(size),
        }
    }
}

impl<T: Word> Memory<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }
}

impl<T: Word> Index<usize> for Memory<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.cells[idx]
    }
}

impl<T: Word> IndexMut<usize> for Memory<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.cells[idx]
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub enum RootInsSet {
    #[default]
    Nop,
    Halt,
    Init(RegAddress, Imed),
    And(RegAddress, RegAddress, RegAddress),
    Or(RegAddress, RegAddress, RegAddress),
    Xor(RegAddress, RegAddress, RegAddress),
    Xnor(RegAddress, RegAddress, RegAddress),
    Add(RegAddress, RegAddress, RegAddress),
    Sub(RegAddress, RegAddress, RegAddress),
    Adi(RegAddress, Imed),
    Not(RegAddress, RegAddress),
    Shl(RegAddress, Imed),
    Shr(RegAddress, Imed),
    Ppct(RegAddress, RegAddress),
    Copy(RegAddress, RegAddress),
    Go(Label),
    Wz(RegAddress, Label),
    Wnz(RegAddress, Label),
    Wneg(RegAddress, Label),
    When(RegAddress, Label),
}
pub type Ris = RootInsSet;

impl Word for Ris {}

impl Zero for Ris {
    fn zero() -> Self {
        Self::Nop
    }
}

pub type Imed = u8;

#[derive(Copy, Clone, Debug)]
pub enum RegAddress {
    Acc,
    Bacc,
    Carr,
    Idx,
    Hi,
    Lo,
    Ret,
    Tmp
}
use RegAddress::{Acc, Bacc, Carr, Hi, Idx, Lo, Ret, Tmp};

impl fmt::Display for RegAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

pub struct Harvard<I, D>
where
    I: Word,
    D: Word,
{
    pub cpu: Processor,
    pub ins_mem: Memory<I>,
    pub data_mem: Memory<D>,
}

impl<D: Word> Harvard<Ris, D> {
    pub fn execute(&mut self) {
        let regs = &mut self.cpu.bank;
        let ins = &mut self.ins_mem;

        let mut cycles = 0;
        let mut i = 0;

        while i < ins.size() {
            cycles += 1;
            print!("{i}. ");
            match ins[i] {
                Ris::Nop => {
                    i += 1;
                    continue;
                }
                Ris::Halt => {
                    println!("Halt!");
                    break;
                }
                Ris::Init(rd, im) => {
                    regs[rd] = im;
                    println!("{rd} <- {im}");
                }
                Ris::And(rd, rs, rt) => {
                    regs[rd] = regs[rs] & regs[rt];
                    println!("{rd} <- {rs} & {rt}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Or(rd, rs, rt) => {
                    regs[rd] = regs[rs] | regs[rt];
                    println!("{rd} <- {rs} | {rt}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Xor(rd, rs, rt) => {
                    regs[rd] = regs[rs] ^ regs[rt];
                    println!("{rd} <- {rs} ^ {rt}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Xnor(rd, rs, rt) => {
                    regs[rd] = !(regs[rs] ^ regs[rt]);
                    println!("{rd} <- !({rs} ^ {rt})");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Add(rd, rs, rt) => {
                    regs[rd] = regs[rs].wrapping_add(regs[rt]);
                    println!("{rd} <- {rs} + {rt}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Sub(rd, rs, rt) => {
                    regs[rd] = regs[rs].wrapping_sub(regs[rt]);
                    println!("{rd} <- {rs} - {rt}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Adi(rd, im) => {
                    regs[rd] = regs[rd].wrapping_add(im);
                    println!("{rd} <- {rd} + {im}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Not(rd, rs) => {
                    regs[rd] = !regs[rs];
                    println!("{rd} <- !{rs}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Shl(rd, im) => {
                    regs[rd] = regs[rd] << im;
                    println!("{rd} <- {rd} << {im}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Shr(rd, im) => {
                    regs[rd] = regs[rd] >> im;
                    println!("{rd} <- {rd} >> {im}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Ppct(rd, rs) => {
                    regs[rd] = regs[rs].count_ones() as u8;
                    println!("{rd} <- popcount({rs})");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Copy(rd, rs) => {
                    regs[rd] = regs[rs];
                    println!("{rd} <- {rs}");
                    println!("   {rd} = {}", regs[rd]);
                }
                Ris::Go(lbl) => {
                    println!("PC <- {lbl}");
                    println!("");
                    i = lbl as usize;
                    continue;
                }
                Ris::Wz(rd, lbl) => {
                    println!("? {rd} == 0");
                    if regs[rd] == 0 {
                        println!("   True");
                        println!("   PC <- {lbl}");
                        println!("");
                        i = lbl as usize;
                        continue;
                    } else {
                        println!("   False");
                        i += 1;
                        println!("");
                        continue;
                    }
                }
                Ris::Wnz(rd, lbl) => {
                    println!("? {rd} != 0");
                    if regs[rd] != 0 {
                        println!("   True");
                        println!("   PC <- {lbl}");
                        println!("");
                        i = lbl as usize;
                        continue;
                    } else {
                        println!("   False");
                        i += 1;
                        println!("");
                        continue;
                    }
                }
                Ris::Wneg(rd, lbl) => {
                    println!("? {rd} < 0");
                    if regs[rd] > 127 {
                        println!("   True");
                        println!("   PC <- {lbl}");
                        println!("");
                        i = lbl as usize;
                        continue;
                    } else {
                        println!("   False");
                        i += 1;
                        println!("");
                        continue;
                    }
                }
                Ris::When(rd, lbl) => {
                    println!("? {rd}");
                    if regs[rd] == 0xFF {
                        println!("   True");
                        println!("   PC <- {lbl}");
                        println!("");
                        i = lbl as usize;
                        continue;
                    } else {
                        println!("   False");
                        i += 1;
                        println!("");
                        continue;
                    }
                }
            }
            println!("");
            i += 1;
        }
        // println!("{:#?}", self.cpu.bank);
        println!("\n\ncycles taken: {cycles}");
    }
}

type Label = usize;

fn main() {
    let root = Processor::new();
    let mut ins_mem: Mem<Ris> = Mem::new(256);
    let data_mem: Mem8 = Mem::new(256);

    ins_mem[0] = Ris::Init(Acc, 11);
    ins_mem[1] = Ris::Init(Bacc, 17);
    ins_mem[2] = Ris::Xor(Carr, Acc, Bacc);
    ins_mem[3] = Ris::And(Tmp, Acc, Bacc);
    ins_mem[4] = Ris::Shl(Tmp, 1);
    ins_mem[5] = Ris::And(Lo, Tmp, Carr);
    ins_mem[6] = Ris::Copy(Acc, Carr);
    ins_mem[7] = Ris::Copy(Bacc, Tmp);
    ins_mem[8] = Ris::Wnz(Lo, 2);
    ins_mem[9] = Ris::Or(Carr, Carr, Tmp);
    ins_mem[10] = Ris::Halt;

    let mut computer = Harvard {
        cpu: root,
        ins_mem,
        data_mem,
    };

    computer.execute();

    println!("\n\nCarr: {}", computer.cpu.bank[Carr]);
}

    // Or: And + Not
    // ins_mem[2] = Ins::Not(Acc, Acc);
    // ins_mem[3] = Ins::Not(Tmp, Bacc);
    // ins_mem[4] = Ins::And(Acc, Tmp);
    // ins_mem[5] = Ins::Not(Acc, Acc);

    // Mul: Add + Go
    // let _loop: Label = 2;
    // let _end: Label = 6;
    // ins_mem[0] = Ins::Init(Acc, 7);
    // ins_mem[1] = Ins::Init(Bacc, 5);
    // ins_mem[_loop] = Ins::Adi(Bacc, 255);
    // ins_mem[3] = Ins::Add(Tmp, Acc);
    // ins_mem[4] = Ins::Wz(Bacc, _end);
    // ins_mem[5] = Ins::Go(_loop);
    // ins_mem[_end] = Ins::Copy(Acc, Tmp);
    // ins_mem[7] = Ins::Halt;

    // Xor: Or
    // ins_mem[0] = Ins::Init(Acc, 3);
    // ins_mem[1] = Ins::Init(Bacc, 5);
    // ins_mem[2] = Ins::Copy(Tmp, Bacc);
    // ins_mem[3] = Ins::And(Tmp, Acc);
    // ins_mem[4] = Ins::Not(Tmp, Tmp);
    // ins_mem[5] = Ins::Or(Acc, Bacc);
    // ins_mem[6] = Ins::And(Acc, Tmp);
    // ins_mem[7] = Ins::Halt;

    // Xor: Not + And    
    // ins_mem[0] = Ins::Init(Acc, 3);
    // ins_mem[1] = Ins::Init(Bacc, 5);
    // ins_mem[2] = Ins::Copy(Tmp, Bacc);
    // ins_mem[3] = Ins::And(Tmp, Acc);
    // ins_mem[4] = Ins::Not(Tmp, Tmp);
    // ins_mem[5] = Ins::Not(Acc, Acc);
    // ins_mem[6] = Ins::Not(Carr, Bacc);
    // ins_mem[7] = Ins::And(Acc, Carr);
    // ins_mem[8] = Ins::Not(Acc, Acc);
    // ins_mem[9] = Ins::And(Acc, Tmp);
    // ins_mem[10] = Ins::Halt;
