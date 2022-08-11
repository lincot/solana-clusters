use anchor_lang::prelude::*;

#[account]
#[derive(Copy, Debug)]
pub struct MyAcc {
    pub a: u64,
    pub b: u64,
}
impl MyAcc {
    pub const LEN: usize = 8 + 8;
}
