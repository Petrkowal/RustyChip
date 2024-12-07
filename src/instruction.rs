use crate::datatypes::datatypes::*;
use crate::registers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    SYS(Address),                                  // 0nnn - SYS addr
    CLS,                                           // 00E0 - CLS
    RET,                                           // 00EE - RET
    JP(Address),                                   // 1nnn - JP addr
    CALL(Address),                                 // 2nnn - CALL addr
    SE(VRegisterNumber, Byte),                     // 3xkk - SE Vx, byte
    SNE(VRegisterNumber, Byte),                    // 4xkk - SNE Vx, byte
    SEV(VRegisterNumber, VRegisterNumber),         // 5xy0 - SE Vx, Vy
    LD(VRegisterNumber, Byte),                     // 6xkk - LD Vx, byte
    ADD(VRegisterNumber, Byte),                    // 7xkk - ADD Vx, byte
    LDV(VRegisterNumber, VRegisterNumber),         // 8xy0 - LD Vx, Vy
    OR(VRegisterNumber, VRegisterNumber),          // 8xy1 - OR Vx, Vy
    AND(VRegisterNumber, VRegisterNumber),         // 8xy2 - AND Vx, Vy
    XOR(VRegisterNumber, VRegisterNumber),         // 8xy3 - XOR Vx, Vy
    ADDV(VRegisterNumber, VRegisterNumber),        // 8xy4 - ADD Vx, Vy
    SUB(VRegisterNumber, VRegisterNumber),         // 8xy5 - SUB Vx, Vy
    SHR(VRegisterNumber, VRegisterNumber),         // 8xy6 - SHR Vx {, Vy}
    SUBN(VRegisterNumber, VRegisterNumber),        // 8xy7 - SUBN Vx, Vy
    SHL(VRegisterNumber, VRegisterNumber),         // 8xyE - SHL Vx {, Vy}
    SNEV(VRegisterNumber, VRegisterNumber),        // 9xy0 - SNE Vx, Vy
    LDI(Address),                                  // Annn - LD I, addr
    JPVX(VRegisterNumber, Address),                // Bnnn - JP V0, addr
    RND(VRegisterNumber, Byte),                    // Cxkk - RND Vx, byte
    DRW(VRegisterNumber, VRegisterNumber, Nibble), // Dxyn - DRW Vx, Vy, nibble
    SKP(VRegisterNumber),                          // Ex9E - SKP Vx
    SKNP(VRegisterNumber),                         // ExA1 - SKNP Vx
    LDDT(VRegisterNumber),                         // Fx07 - LD Vx, DT
    LDK(VRegisterNumber),                          // Fx0A - LD Vx, K
    LDDTV(VRegisterNumber),                        // Fx15 - LD DT, Vx
    LDST(VRegisterNumber),                         // Fx18 - LD ST, Vx
    ADDI(VRegisterNumber),                         // Fx1E - ADD I, Vx
    LDF(VRegisterNumber),                          // Fx29 - LD F, Vx
    LDB(VRegisterNumber),                          // Fx33 - LD B, Vx
    LDIV(VRegisterNumber),                         // Fx55 - LD [I], Vx
    LDVI(VRegisterNumber),                         // Fx65 - LD Vx, [I]

    // super chip-48 instructions
    SCU(Nibble),                            // 00CN - SCU N
    SCR,                                    // 00FB - SCR
    SCL,                                    // 00FC - SCL
    EXIT,                                   // 00FD - EXIT
    LOW,                                    // 00FE - LOW
    HIGH,                                   // 00FF - HIGH
    DRW0(VRegisterNumber, VRegisterNumber), // Dxy0 - DRW Vx, Vy, 0
    LDHF(VRegisterNumber),                  // Fx30 - LD HF, Vx
    LDR(VRegisterNumber),                   // Fx75 - LD R, Vx
    LDRV(VRegisterNumber),                  // Fx85 - LD Vx, R
}