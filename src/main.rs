use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use algin::{Vector, Zero};

pub type DWord = u8;
pub type IWord = u16;

pub trait Word: Clone + Copy + Default {}

impl Word for DWord {}
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
    acc: Reg8,
    bacc: Reg8,
    carr: Reg8,
    datt: Reg8,
    idx: Reg8,
    hi: Reg8,
    lo: Reg8,
    ret: Reg8,
}
pub type RegBank = RegisterBank;

impl RegisterBank {
    pub fn init() -> Self {
        Self {
            acc: 0,
            bacc: 0,
            carr: 0,
            datt: 0,
            idx: 0,
            hi: 0,
            lo: 0,
            ret: 0,
        }
    }
}

impl Index<RegAddress> for RegisterBank {
    type Output = Reg8;

    fn index(&self, idx: RegAddress) -> &Reg8 {
        match idx {
            Acc => &self.acc,
            Bacc => &self.bacc,
            Carr => &self.carr,
            Datt => &self.datt,
            Idx => &self.idx,
            Hi => &self.hi,
            Lo => &self.lo,
            Ret => &self.ret,
        }
    }
}

impl IndexMut<RegAddress> for RegisterBank {
    fn index_mut(&mut self, idx: RegAddress) -> &mut Reg8 {
        match idx {
            Acc => &mut self.acc,
            Bacc => &mut self.bacc,
            Carr => &mut self.carr,
            Datt => &mut self.datt,
            Idx => &mut self.idx,
            Hi => &mut self.hi,
            Lo => &mut self.lo,
            Ret => &mut self.ret,
        }
    }
}

pub type Reg8 = u8;
pub type Reg16 = u16;

pub struct Memory<T: Word> {
    cells: Vector<T>,
}
pub type Mem<T> = Memory<T>;
pub type Mem8 = Memory<u8>;
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
pub enum Instruction {
    #[default]
    Nop,
    Halt,
    Init(RegAddress, Imed),
    And(RegAddress, RegAddress),
}
pub type Ins = Instruction;

impl Word for Instruction {}

impl Zero for Instruction {
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
    Datt,
    Idx,
    Hi,
    Lo,
    Ret,
}
use RegAddress::{Acc, Bacc, Carr, Datt, Hi, Idx, Lo, Ret};

pub struct Harvard<I, D>
where
    I: Word,
    D: Word,
{
    pub cpu: Processor,
    pub ins_mem: Memory<I>,
    pub data_mem: Memory<D>,
}

impl<D: Word> Harvard<Instruction, D> {
    pub fn execute(&mut self) {
        let regs = &mut self.cpu.bank;

        for ins in self.ins_mem.iter() {
            match ins {
                Ins::Nop => continue,
                Ins::Halt => break,
                Ins::Init(rd, im) => {
                    regs[*rd] = *im; 
                },
                Ins::And(rd, rs) => {
                    regs[*rd] = regs[*rd] & regs[*rs];
                },
                x => println!("not impl: {x:#?}"),
            }
        }
        println!("{:#?}", self.cpu.bank);    
    }    
}

fn main() {
    let root = Processor::new();
    let mut ins_mem: Mem<Ins> = Mem::new(256);
    let data_mem: Mem8 = Mem::new(256);

    ins_mem[0] = Ins::Init(Acc, 13);
    ins_mem[1] = Ins::Init(Bacc, 35);
    ins_mem[2] = Ins::And(Acc, Bacc);
    ins_mem[3] = Ins::Halt;

    let mut computer = Harvard {
        cpu: root,
        ins_mem,
        data_mem,
    };

    computer.execute();
}
