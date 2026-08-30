---
title: MIPS64 Instruction Set
aliases: [MIPS64, WinMIPS64]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Introduction to the MIPS64 architecture used throughout the pipelining lectures: the programmer's model (32 GPRs with R0 hardwired to zero, 32 FPRs, HI/LO, PC), data types, addressing modes, and the three instruction formats (I, R, J). Catalogues load/store, ALU, branch/jump, floating-point and miscellaneous instructions with syntax examples, the technique to build 32/64-bit constants, assembler directives, and the structure of a WinMIPS64 assembly program (data section + code section), with worked examples (C = A + B, sum of an array).

## MIPS64: Generalities

> [!definition] **MIPS**
> *Microprocessor without Interlocked Pipeline Stages*: a family of RISC processors, very successful for embedded applications. The first processor of the family was introduced in **1985**; several versions followed.

Key characteristics:

- simple **load-store** instruction set;
- designed for **pipeline efficiency**;
- **fixed instruction length**;
- suited for **low-power** applications.

The version described in the course is a simplified version of MIPS64. Reference: *MIPS64 Architecture For Programmers*, Vol. I–III, MIPS Technologies, Inc. The simulator used in the course is **WinMIPS64** (Mike Scott), which models a 64-bit MIPS with forwarding, delay slot, and branch prediction.

## Programmer's Model

| Register group | Size | Notes |
|---|---|---|
| General-purpose integer registers R0–R31 | 64-bit | **R0 is always 0** |
| Floating-point registers F0–F31 | 64-bit | used by FP instructions |
| Special-purpose registers: HI, LO, PC | 64-bit | PC is the program counter |

Supported **data types**: byte (8 bits), half word (16), word (32), double word (64), 32-bit single-precision and 64-bit double-precision floating-point.

## Addressing Modes

MIPS64 uses a 16-bit immediate field and displacement addressing; the other common modes are obtained as special cases:

```asm
DADDUI R1, R2, #32      ; immediate:  R1 <- R2 + 32
DADDUI R1, R0, #32      ; constant:   R1 <- 32
LD     R1, 30(R2)       ; displacement: R1 <- MEM[R2 + 30]
LD     R1, 0(R2)        ; register indirect (displacement = 0)
LD     R1, 64(R0)       ; absolute addressing (base = R0)
```

## Instruction Formats

A CPU instruction is a single **32-bit aligned word** with a 6-bit primary opcode. Three formats exist:

> [!definition] **I-type (Immediate)**
> `| opcode (6) | Rs (5) | Rt (5) | Immediate (16) |` — the 16-bit signed immediate is used for logical/arithmetic operands, load/store address byte offsets, and PC-relative branch displacements. Rs = source register, Rt = target (source/destination) register.

> [!definition] **R-type (Register)**
> `| opcode (6) | Rs (5) | Rt (5) | Rd (5) | Sa (5) | Function (6) |` — Rd = destination register, Sa = shift amount, Function selects functions within the primary opcode SPECIAL.

> [!definition] **J-type (Jump)**
> `| opcode (6) | Offset (26) |` — the 26-bit index is shifted left two bits to supply the low-order 28 bits of the jump target address (offset added to PC).

Each instruction is 32 bits long. The instruction set groups by function: **load and store, ALU operations, branches and jumps, floating point, miscellaneous**.

## Load and Store

MIPS is a load/store architecture: main memory is accessed **only** through load and store instructions; all other operations work on registers.

| Instruction | Meaning | Example |
|---|---|---|
| `LD` | load double word | `LD R1, 28(R8)` → `R1 <- MEM[R8+28]` |
| `LB` | load byte (sign-extend) | `LB R1, 28(R8)` → `R1 <- (MEM[...])⁷ ## MEM[R8+28]` |
| `LBU` | load byte unsigned | `LBU R1, 28(R8)` → `R1 <- 0⁵⁶ ## MEM[R8+28]` |
| `L.S` | load FP single | `L.S F4, 46(R5)` → `F4 <- MEM[R5+46] ## 0³²` |
| `L.D` | load FP double | `L.D F4, 46(R5)` |
| `SD` | store double word | `SD R1, 28(R8)` |
| `SW` | store word (32 LSBs) | `SW R1, 28(R8)` |
| `SH` | store half word (16 LSBs) | `SH R1, 28(R8)` |
| `SB` | store byte (8 LSBs) | `SB R1, 28(R8)` |
| `S.S` / `S.D` | store FP single/double | `S.S F4, 28(R8)` stores `F4⁶³..³²` |

WinMIPS64 also provides the full family: `lb, lbu, sb, lh, lhu, sh, lw, lwu, sw, ld, sd, l.d, s.d`.

## ALU Operations

All operations are performed on operands held in processor registers. Instruction types: immediate and three-operand, two-operand, shift, multiply/divide; arithmetic uses 2's complement (add, subtract, multiply, divide).

**R0 usage** — since R0 is always 0:

```asm
DADDUI R1, R0, 25      ; load a constant: R1 <- 25
DADD   R1, R0, R2      ; register copy:   R1 <- R2
```

Examples:

```asm
DADDU  R1, R2, R3      ; double add unsigned:  R1 <- R2 + R3
DADDUI R1, R2, 74      ; add unsigned imm.:    R1 <- R2 + 74
DSLL   R1, R2, 3       ; double shift left:    R1 <- R2 << 3
SLT    R1, R2, R3      ; set less than: R1 <- 1 if (R2 < R3), else 0
```

> [!definition] **LUI — Load Upper Immediate**
> `LUI R1, 0x47` loads the immediate into the upper part of the register: `R1 <- 0⁶³..³² ## 0x47 ## 0¹⁵..⁰`.

### HowTo: build 32-bit constants

```asm
; +2,147,483,647 -> 0x7FFF_FFFF
lui r7, 0x7FFF          ; r7 = 0x0000_0000_7FFF_0000
ori r7, r7, 0xFFFF      ; r7 = 0x0000_0000_7FFF_FFFF
```

Note: **all logical instructions extend the immediate with 0** (zero-extension), which is why `LUI` + `ORI` composes the constant bit-by-bit. A shifted-constant example: to obtain `r7 = 0x0000_0000_C1A0_FEDE` one can use LUI+ORI, or load `0xC1A0_FEDE` and then `DSLL`/`DSRL` by 32 bits.

## Branch and Jump

- **PC-relative conditional branches**
- **Absolute (register) unconditional jumps**
- procedure calls that record a **return link address in a general register**

```asm
J     name             ; unconditional jump: PC <- name
JAL   name             ; jump and link:       R31 <- PC+4; PC <- name
JALR  R4               ; jump and link register: R31 <- PC+4; PC <- R4
JR    R3               ; jump register:       PC <- R3
BEQZ  R4, name         ; branch if R4 == 0
BNE   R3, R4, name     ; branch if R3 != R4
```

WinMIPS64 branch set: `j, jr, jal, jalr, beq, bne, beqz, bnez`.

## Floating Point

The FPU includes almost the same instruction types as the integer unit: data transfer (`L.D`, `S.D`), arithmetic (`add.d`, `sub.d`, `mul.d`, `div.d`, `mov.d`), conditional branch, and miscellaneous instructions.

## Miscellaneous

```asm
MOVZ R1, R2, R3        ; conditional move if zero: if (R3 = 0) then R1 <- R2
NOP                    ; no operation; it means: SLL R0, R0, 0
```

WinMIPS64 also has `movn` (move if register not zero).

## Assembler Programs

An assembly program has two sections:

- **Data section** — variables and constants;
- **Code section** — program, routines, subroutines.

```asm
;******** MIPS64 INITIAL PROGRAM ********
        .data
Prompt: .ascii  "An integer value >1:\0"   ; string constant
Vector: .word   1, 2, 3, 4, 5              ; variables (words)
Result: .space  4                          ; reserve 4 bytes

        .code
        .global main
main:   ...
```

A source line contains: labels (`main:`, `Loop:`), the opcode, operands, and `;`-comments.

### WinMIPS64 assembler directives

| Directive | Meaning |
|---|---|
| `.data` | start of data segment |
| `.text` / `.code` | start of code segment |
| `.org <n>` | start address |
| `.space <n>` | leave n empty bytes |
| `.ascii <s>` / `.asciiz <s>` | enter (zero-terminated) ASCII string |
| `.align <n>` | align to n-byte boundary |
| `.word <n1>,<n2>…` | enter 64-bit word(s) |
| `.word32` / `.word16` / `.byte` | enter 32-bit / 16-bit numbers / bytes |
| `.double <n1>,…` | enter floating-point number(s) |

## Worked Examples

### C = A + B (MIPS vs. 8086)

```asm
        .data
Val_A:  .word 10
Val_B:  .word 20
Val_C:  .word 0

        .text
Main:   ld   R1, Val_A(R0)
        ld   R2, Val_B(R0)
        dadd R3, R2, R1
        sd   R3, Val_C(R0)
```

Code analysis from the slides:

| Program | # instructions | Code size [bytes] | Execution time [clock cycles] |
|---|---|---|---|
| MIPS64 | 4 | 16 | 4 |
| 8086 (`mov AX, Val_A` … `add AX, Val_B` … `mov Val_C, AX`) | 3 | 8 | 33 |

The 8086 version is more compact (variable-length, memory-oriented instructions) but far slower.

### Sum of 10 integer values

```asm
        .data
values: .word 1, 2, 3, 4, 5, 6, 7, 8, 9, 10  ; 64-bit integers
result: .space 8

        .text
MAIN:   daddui R1, R0, 10       ; R1 <- 10 (counter)
        dadd   R2, R0, R0       ; R2 <- 0 (pointer)
        dadd   R3, R0, R0       ; R3 <- 0 (result)
LOOP:   ld     R4, values(R2)   ; get value in R4
        dadd   R3, R3, R4       ; R3 <- R3 + R4
        daddi  R2, R2, 8        ; pointer += 8 (double word)
        daddi  R1, R1, -1       ; counter--
        bnez   R1, LOOP
        sd     R3, result(R0)   ; store result
HALT:                            ; the end
```

This loop pattern (pointer register + counter register + backward branch) is reused in the pipelining examples of [[04-pipelining-fundamentals]].
