---
title: ARM Instruction Set
aliases: [ARM ALU Instructions, ARM Condition Codes]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The ARM instruction set from the programmer's viewpoint: the special registers (PC, LR, SP), system registers (PRIMASK, FAULTMASK, BASEPRI, CONTROL), the condition flags N/Z/C/V and their exact semantics, conditional execution with the condition field and the S suffix. Then the ALU instruction families in detail — comparisons (CMP, CMN, TST, TEQ), PSR access (MRS/MSR), arithmetic (ADD/ADC/SUB/SBC/RSB…), multiplication and division, logic, shift and rotate — with the operand2 rules imposed by the barrel shifter and worked examples including 64-bit arithmetic.

## Register Set and System Registers

The Cortex-M3 has **16+2 registers**, all 32-bit (see [[07-arm-v7m-architecture]] for the architecture overview). Three registers have an architectural role:

- **R15 = PC** (program counter): in ARM state all instructions are 32 bits and word aligned, so the PC value is stored in bits [31:2] with bits [1:0] equal to zero. Differently from processors like the 80x86 family, ARM permits directly **writing the PC**, which is mapped over R15.
- **R14 = LR** (link register): stores the return address of Branch-with-Link operations. To return from a linked branch: `MOV r15, r14` (or `MOV pc, lr`).
- **R13 = SP** (stack pointer): autonomously updated; its **initial value at boot time is hardware-loaded from the vector table** (IVT entry 0), and it is updated during execution by stack-oriented instructions.

Data types: byte (8 bits), halfword (16), word (32).

### System registers

| Register | Function |
|---|---|
| **BASEPRI** | Base Priority Mask register |
| **PRIMASK** | 1-bit interrupt mask: when set, blocks all interrupts apart from NMI and hard fault; prevents activation of all exceptions with configurable priority |
| **FAULTMASK** | prevents activation of all exceptions except NMI |
| **CONTROL** | controls the stack used and the privilege level in thread mode; if implemented, indicates whether the FPU state is active |

**CONTROL register bits**:

- `CONTROL[2]` (only Cortex-M4/M7): 0 = FPU not active, 1 = FPU active;
- `CONTROL[1]`: 0 in handler mode → MSP selected (no alternate stack for handler mode); 0 in thread mode → default stack MSP; 1 in thread mode → alternate stack **PSP**;
- `CONTROL[0]` (not Cortex-M0): 0 = thread mode **privileged**, 1 = thread mode **user** state.

## Condition Flags

The uppermost nibble of the PSR contains the flags N, Z, C, V (plus Q and GE). Flags are set/cleared by:

- **comparison instructions** (CMP, TST, …);
- **ALU instructions with the `S` suffix**;
- a **direct write** to the program status register.

| Flag | Logical instruction | Arithmetic instruction |
|---|---|---|
| **N** = 1 | no meaning | bit 31 of the result set → negative number (signed ops) |
| **Z** = 1 | result is all zeroes | result was zero |
| **C** = 1 | '1' left in carry flag after a shift operation | result was greater than 32 bits |
| **V** = 1 | no meaning | result greater than 31 bits → possible corruption of the sign bit |

**Carry flag C details**:

- After a **sum**, C = 1 if the result size is 33 bits (carry out of the MSBs). E.g. `1111 + 0001 = 0000` → C = 1; `0111 + 0001 = 1000` → C = 0.
- After a **subtraction**, C is *inverted* w.r.t. the intuitive borrow: `0000 − 0001 = 1111` → C = 1; `1000 − 0001 = 0111` → C = 0. (The carry/borrow flag is set if the subtraction requires a borrow into the MSBs.)
- After a **move or logical instruction**, C is the last bit shifted out by the inline **barrel shifter** operation.
- In unsigned arithmetic, watch C to detect errors; in signed arithmetic, C is used for comparisons together with the other flags.

**Negative flag N**: corresponds to bit 31 of the result; if N = 1 a 2's-complement number is negative (e.g. −4 + −3 = −7 → N = 1; adding two positives that overflows into bit 31 also sets N).

**Overflow flag V**: set if, in a sum of values **with the same sign**, there is a change in the MSB (carry into the sign bit without carry out); V = 0 when both the carry into bit 31 and the carry out agree — the result is right.

**Zero flag Z**: set if the result is zero.

## Conditional Execution

- Most instruction sets only allow **branches** to be conditional; ARM reuses the condition-evaluation hardware so **every instruction can be conditional** — effectively increasing the number of instructions.
- All instructions contain a **condition field** (top 4 bits, cond[31:28]) determining whether the CPU will execute them.
- **Non-executed instructions soak up 1 cycle** (they still complete the cycle to allow fetching/decoding of the following instructions).
- Benefit: removes the need for many branches, which stall the pipeline (**3 cycles to refill**); enables very dense in-line code without branches; the penalty of skipping a few conditional instructions is frequently less than the branch/call overhead.

### The condition field

| Code | Suffix | Flags | Meaning |
|---|---|---|---|
| 0000 | EQ | Z set | equal |
| 0001 | NE | Z clear | not equal |
| 0010 | HS / CS | C set | unsigned higher or same (≥) |
| 0011 | LO / CC | C clear | unsigned lower (<) |
| 0100 | MI | N set | negative |
| 0101 | PL | N clear | positive or zero |
| 0110 | VS | V set | overflow |
| 0111 | VC | V clear | no overflow |
| 1000 | HI | C set and Z clear | unsigned higher (>) |
| 1001 | LS | C clear or Z set | unsigned lower or same (≤) |
| 1010 | GE | N == V | signed ≥ |
| 1011 | LT | N ≠ V | signed < |
| 1100 | GT | Z clear and N == V | signed > |
| 1101 | LE | Z set or N ≠ V | signed ≤ |
| 1110 | AL | — | always |
| 1111 | NV | — | reserved |

### Using and updating the condition field

- Postfix the instruction with the condition: `ADDEQ r0, r1, r2` executes `r0 = r1 + r2` only if Z is set. (Plain `ADD` is implicitly `ADDAL`.)
- **By default, data processing operations do NOT affect the condition flags** (apart from comparisons, where setting flags is the only effect).
- To update the flags, set the **S bit** by postfixing the instruction (and any condition code) with **`S`**: `ADDS r0, r1, r2` → `r0 = r1 + r2` **and** flags updated.

Example — `if R4−R3 == 0 then R0 = R1 else R0 = R2`:

```asm
SUBS   R4, R4, R3     ; sets the flags
MOVEQ  R0, R1         ; taken when Z = 1
MOVNE  R0, R2         ; taken when Z = 0
```

Example — `LDR r0, =0xFFFFFFF9` then `ADDS r1, r0, #7` gives flags N = 0, Z = 1, C = 1, V = 0.

## Comparison Instructions

`compare/test <Rd>, <operand2>` — they set the flags **without updating Rd**. The second operand can be:

- a register with an optional shift;
- a constant obtained by shifting left an 8-bit value;
- a constant of the form `0x00XY00XY`, `0xXY00XY00`, or `0xXYXYXYXY`.

| Instruction | Operation | Notes |
|---|---|---|
| **CMP** | subtracts operand2 from Rd, updates flags | operands not modified; e.g. with r0 = 12: `CMP r0, #10` → N=0,Z=0,C=1,V=0; `CMP r0, #12` → Z=1,C=1; `CMP r0, #14` → N=1,C=0 |
| **CMN** | adds operand2 to Rd, updates flags | the assembler may rewrite: `CMP r0, #-8` becomes `CMN r0, #8` |
| **TST** | logical AND of operand2 and Rd; updates all flags except V | `TST r1, r0, LSL #4` → "is the 4th bit of r1 set?" |
| **TEQ** | logical EOR of operand2 and Rd; updates all flags except V | `TEQ r2, r3` → "are r2 and r3 equal?" |

### Accessing the PSR

```asm
MRS r0, APSR      ; copy special register (flags) into r0 (uppermost nibble)
MSR APSR, r0      ; copy general-purpose register into special register
```

`Sreg` can be APSR, EPSR, IPSR, or PSR.

## Internal Data Path and Operand2

The ALU data path has **operand 1 = a register**, and **operand 2** which can be:

- a **shifted register**;
- a **shifted 8-bit constant**;
- `0x00XY00XY`, `0xXY00XY00`, or `0xXYXYXYXY`;
- a **12-bit constant (ADD and SUB only)**.

The barrel shifter sits on the operand2 path before the ALU.

## Arithmetic Instructions

```asm
ADD  <Rd>, <Rn>, <op2>     ; Rd = Rn + op2
ADC  <Rd>, <Rn>, <op2>     ; Rd = Rn + op2 + C   (add with carry)
ADDW                        ; like ADD but takes only a 12-bit value, cannot update flags
SUB  <Rd>, <Rn>, <op2>     ; Rd = Rn - op2
SBC  <Rd>, <Rn>, <op2>     ; Rd = Rn - op2 + C - 1
SUBW                        ; like SUB but takes only a 12-bit value, cannot update flags
RSB  <Rd>, <Rn>, <op2>     ; Rd = op2 - Rn  (reverse subtraction)
```

**64-bit arithmetic** using the carry chain:

```asm
ADDS r4, r0, r2        ; low words, sets C
ADC  r5, r1, r3        ; high words + carry: r5,r4 = r1,r0 + r3,r2

SUBS r4, r0, r2
SBC  r5, r1, r3        ; r5,r4 = r1,r0 - r3,r2
```

**RSB advantages**: either operand can be shifted before the subtraction, and a register can be subtracted from a constant:

```asm
SUB r0, r1, r2, LSL #2     ; r0 = r1 - r2*4
RSB r0, r2, r1, LSL #2     ; r0 = r1*4 - r2
```

## Multiplication and Division

| Instruction | Operation |
|---|---|
| `MUL Rd, Rn, Rm` | 32-bit result (no signed/unsigned distinction) |
| `UMULL Rd1, Rd2, Rn, Rm` | unsigned 64-bit result |
| `SMULL Rd1, Rd2, Rn, Rm` | signed 64-bit result |
| `MLA Rd, Rn, Rm, Ra` | Rd = Rn × Rm + Ra (accumulate) |
| `MLS Rd, Rn, Rm, Ra` | Rd = Rn × Rm − Ra |
| `UMLAL Rd1, Rd2, Rn, Rm` | Rd1,Rd2 = Rn × Rm + Rd1,Rd2 (accumulate long) |
| `SMLAL …` | same as UMLAL, signed |

All multiplication **operands must be registers**.

**Division**: `UDIV Rd, Rn, Rm` (unsigned) and `SDIV Rd, Rn, Rm` (signed). If Rn is not exactly divisible by Rm, the result is **rounded toward zero**. **UDIV and SDIV do not change the flags** (the `S` suffix cannot be added).

## Logic Instructions

```asm
AND Rd, Rn, op2     ; Rn AND op2
BIC Rd, Rn, op2     ; Rn AND NOT op2
ORR Rd, Rn, op2     ; Rn OR  op2
EOR Rd, Rn, op2     ; Rn XOR op2
ORN Rd, Rn, op2     ; Rn OR  NOT op2
MVN Rd, Rn          ; NOT Rn
```

## Shift and Rotate Instructions

| Instruction | Behaviour |
|---|---|
| `LSL Rd, Rn, op2` | logical shift left (zeros in, last bit out to C) |
| `LSR Rd, Rn, op2` | logical shift right (0 in, last bit out to C) |
| `ASR Rd, Rn, op2` | arithmetic shift right (sign replicated, last bit out to C) |
| `ROR Rd, Rn, op2` | rotate right (wrap-around, bit out to C) |
| `RRX Rd, Rn` | rotate right **1 bit through carry** (C participates in the rotation) |

**Exercise — 64-bit shifts/rotates** of the pair r1,r0 by one bit:

```asm
; logical shift left: result in r3,r2
LSLS r2, r0, #1      ; low word shifted, bit 31 -> C
RRX  r3, r1          ; rotates C into bit 0 of high word

; logical shift right: result in r5,r4
LSRS r4, r1, #1      ; high word shifted, bit 0 -> C  (high result goes to r5/r4 pair logic)
; ... carry chain through RRX/moves, mirroring the LSL case

; arithmetic shift right, plain rotate and rotate-through-carry follow the same
; pairing pattern using ASRS/ROR/RRX on the two words.
```

(The slide proposes the exercise for LSL, LSR, ASR, ROR and RRX on the register pair r1,r0 with results in r3,r2 / r5,r4 / r7,r6 / r9,r8 / r11,r10 respectively — the key mechanism is passing the displaced bit between words through the **C flag**.)
