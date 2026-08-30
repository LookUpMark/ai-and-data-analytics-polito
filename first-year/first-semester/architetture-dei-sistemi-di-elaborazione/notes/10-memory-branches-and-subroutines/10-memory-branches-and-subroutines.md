---
title: Memory, Branches and Subroutines in ARM
aliases: [ARM Memory Access, ARM Subroutines, Stack and ABI]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The three final ARM topics: (1) memory access — the v7-M memory map with region attributes, the load/store instruction families with signed variants and double-word transfers, pre/post-indexed addressing with writeback, and look-up tables; (2) branches — B/BX/BL/BLX, the BX bit-0 rule, branch ranges, conditional branches, CBZ/CBNZ, loop implementation patterns (while, for, do-while) and IT blocks; (3) the stack and subroutines — the four stack types, LDM/STM addressing modes, PUSH/POP, nested calls, the three parameter-passing approaches with full code examples, and the ARM ABI / AAPCS calling convention.

## Accessing Memory

### Memory map (v7-M)

| Region | Range | Size |
|---|---|---|
| Code | 0x00000000 – 0x1FFFFFFF | 0.5 GB |
| SRAM | 0x20000000 – 0x3FFFFFFF | 0.5 GB |
| Peripherals | 0x40000000 – 0x5FFFFFFF | 0.5 GB |
| External RAM | 0x60000000 – 0x9FFFFFFF | 1 GB |
| External device | 0xA0000000 – 0xDFFFFFFF | 1 GB |
| Private peripheral bus: internal | 0xE0000000 – 0xE003FFFF | 256 kB |
| Private peripheral bus: debug/external | 0xE0040000 – 0xE0100000 | ≈ 0.5 GB / 1 MB |
| Vendor specific / System | up to 0xFFFFFFFF | — |

Memory access **attributes**:

| Region | Bufferable | Cacheable | Executable |
|---|---|---|---|
| Code | no | yes (write through) | yes |
| SRAM | yes | yes (write back) | yes |
| Peripherals | no | no | no |
| External RAM | yes | yes (write back) | no |
| External device | no | no | no |
| System | no | no | no |

### Load and store instructions

`load/store <Rd>, <addressing_mode>`:

| Load | Store | Size and type |
|---|---|---|
| `LDR` | `STR` | word (32 bits) |
| `LDRB` | `STRB` | byte (8 bits) |
| `LDRH` | `STRH` | halfword (16 bits) |
| `LDRSB` | — | signed byte |
| `LDRSH` | — | signed halfword |
| `LDRD` | `STRD` | two words |
| `LDM` | `STM` | multiple words |

**Exercise** (r0 = 0x00008004, memory little-endian: bytes `8D 62 6C 79` from 0x8004):

```asm
LDR   r1, [r0]     ; r1 = 0x796C628D   (word)
LDRB  r2, [r0]     ; r2 = 0x0000008D   (byte, zero-extended)
LDRH  r3, [r0]     ; r3 = 0x0000628D   (halfword, zero-extended)
LDRSB r4, [r0]     ; r4 = 0xFFFFFF8D   (byte, sign-extended)
LDRSH r5, [r0]     ; r5 = 0x0000628D   (halfword: MSB = 0 -> sign-extension adds nothing)
```

**Stores** write only the relevant low part of the register: `STRB r1, [r0]` copies the LSB to one memory location; `STRH` the lower 16 bits to two consecutive locations; `STR r1, [r0]` the whole register to four consecutive locations.

**Double word**: `LDRD r1, r2, [r0]` loads two registers (r1 = word at r0, r2 = word at r0+4); `STRD r1, r2, [r0]` stores two registers into eight consecutive locations.

### Addressing modes

- **Addressing**: pre-indexed (with or without writeback) or post-indexed.
- **Offset**: a fixed value, or a (shifted) register.

> [!definition] **Pre-indexed addressing**
> `load/store <Rd>, [<Rn>, <offset>]{!}` — the address is computed by **summing the offset to the base register Rn**; the offset is either a **12-bit constant** or a **register** (which can be shifted left up to 3 positions). The optional **`!`** updates Rn at the end of the instruction (auto-indexing).

> [!definition] **Post-indexed addressing**
> `load/store <Rd>, [<Rn>], <offset>` — the address is simply **Rn**; **then** Rn is updated by adding the offset. `!` is not written because Rn is **always** updated.

Example — load 4 words into r2–r5 with r0 = 0x00008000:

```asm
; constant offsets: r0 unchanged
LDR r2, [r0]
LDR r3, [r0, #4]
LDR r4, [r0, #8]
LDR r5, [r0, #12]           ; at the end r0 = 0x00008000

; constant offsets + writeback (auto-indexing)
LDR r2, [r0]
LDR r3, [r0, #4]!
LDR r4, [r0, #4]!
LDR r5, [r0, #4]!           ; at the end r0 = 0x0000800C

; register offset
MOV r1, #4
LDR r3, [r0, r1]            ; r1 holds the offset

; shifted register offset
LDR r4, [r0, r1, LSL #1]    ; offset = r1 * 2
```

### Look-up tables

> [!definition] **Look-up table**
> An array of pre-calculated constants. *Pro*: frequently used values are not computed at run time (or computed only the first time). *Con*: additional memory space is required. Easily accessed with indexed addressing.

Byte table — compute `x² + 2x + 1` for 0 ≤ x ≤ 10 (r2 holds x):

```asm
        AREA    |.text|, CODE, READONLY
Reset_Handler PROC
        EXPORT  Reset_Handler [WEAK]
        MOV r2, #8
        LDR r0, =lookup
        LDRB r4, [r0, r2]        ; r4 = table[x] (byte table)
stop    B stop
lookup  DCB 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
        ENDP
```

Word table — factorial of x: the index is scaled with `LSL #2`:

```asm
        MOV r2, #8
        LDR r0, =lookup
        LDR r4, [r0, r2, LSL #2] ; r4 = table[x] (word table)
lookup  DCD 1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800
```

## Branches

### Unconditional branch instructions

| Instruction | Syntax | Effect |
|---|---|---|
| branch | `B <label>` | PC ← label |
| branch indirect | `BX <Rn>` | PC ← Rn |
| branch and link | `BL <label>` | LR ← return address; PC ← label (call subroutine) |
| branch indirect with link | `BLX <Rn>` | LR ← return address; PC ← Rn |

**Infinite loops**: a stand-alone program without an OS cannot continue beyond its end (unpredictable behavior), so the last instruction is:

```asm
stop  B stop          ; or:
      LDR r1, =stop
stop  BX r1
```

### LDR vs ADR with BX

Instructions are 16 or 32 bits long, so their address is always **halfword aligned**.

> [!definition] **BX bit-0 rule**
> `BX` requires the **last bit of the register to be 1**, otherwise a **usage fault exception** is raised. BX jumps to the address created by changing the last bit of the register to 0. `LDR rX, =label` sets the last bit to **1 if the label is in a code area** (0 if in a data area); `ADR`/`ADRL` do **not** change the last bit.

Examples:

```asm
0x000000CC  LDR r1, =stop        ; Ok: r1 = 0x000000CF (bit 0 set)
0x000000CE  stop BX r1

0x000000CC  ADRL r1, stop        ; NO: r1 = 0x000000D4 (bit 0 = 0) -> usage fault
0x000000D4  stop BX r1

0x000000CC  ADRL r1, stop        ; Ok: force bit 0 manually
0x000000D4  ORR r1, r1, #1       ; r1 = 0x000000D9
0x000000D8  stop BX r1
```

### Branch range and MOV-based jumps

- In `B`, the opcode is 8 bits and the immediate is **24 bits**; since addresses are halfword-aligned, the immediate specifies bits 24–1 of the relative address, and the 25th bit is the sign → relative range **±2²⁴ bytes = ±16 MB**.
- `BX` can jump to any 32-bit value (**4 GB**).
- A jump can also be implemented by writing the PC: `MOV PC, Rd` or `LDR PC, =label` — these **force the last bit of PC to 0**; MOV instead of BX is **discouraged** (the assembler generates a warning).

### Conditional branches

| `??` | Flags | Meaning | | `??` | Flags | Meaning |
|---|---|---|---|---|---|---|
| EQ | Z = 1 | equal | | VS | V = 1 | overflow |
| NE | Z = 0 | not equal | | VC | V = 0 | no overflow |
| CS/HS | C = 1 | unsigned ≥ | | HI | C=1, Z=0 | unsigned > |
| CC/LO | C = 0 | unsigned < | | LS | C=0 or Z=1 | unsigned ≤ |
| MI | N = 1 | negative | | GE | N = V | signed ≥ |
| PL | N = 0 | positive or 0 | | LT | N ≠ V | signed < |
| | | | | GT | Z=0, N=V | signed > |
| | | | | LE | Z=1 or N≠V | signed ≤ |

Example:

```asm
; r0 contains the exam score
CMP r0, #18
BEQ refuse        ; == 18: study more
BLO reject        ; < 18 unsigned: study much more
BHI accept        ; > 18: go on holiday
```

### Compare and branch

```asm
CBZ  <Rn>, <label>    ; jump if Rn = 0
CBNZ <Rn>, <label>    ; jump if Rn ≠ 0
```

Constraints: Rn must be among **r0–r7**; only **forward** branches (range 4–130 bytes). vs. `CMP`+`BEQ`: CMP sets the flags while CBZ/CBNZ **do not**; CBZ/CBNZ jump only forward with a shorter range and **cannot be used within an IT block**.

### Loop patterns

While `while (r0 != N) { … }`:

```asm
        B   test
loop    …               ; do something
test    CMP r0, #N
        BNE loop
; variant 2: test at the top with BE exit / B test
```

While with CBZ (only if N = 0):

```asm
loop    …               ; do something
        CBZ r0, exit
        B   loop
exit    …
```

For `for (i = 0; i < N; i++) { … }` — countdown optimization:

```asm
        MOV r0, N       ; (naive version counts up with CMP/BHS)
loop    …               ; do something
        SUBS r0, r0, #1
        BNE  loop       ; CBNZ cannot be used: the branch is backward
```

Do-while `do { … } while (r0 != N);`:

```asm
loop    …               ; do something
        CMP r0, #N
        BNE loop
```

Absolute value of N − M (branchy version):

```asm
        MOV r0, #N
        MOV r1, #M
        CMP r0, r1
        BLT neg
        SUB r0, r0, r1
        B   exit
neg     SUB r1, r1, r0
exit    …
```

### IT blocks (If-Then)

> [!definition] **IT block**
> `ITxyz <cond>` followed by up to 4 instructions suffixed with `<cond>` or its inverse. Avoids the **branch penalty** because there is no change to program flow (each instruction is simply executed or skipped based on the flags, costing at most its 1 cycle).

```asm
        MOV r0, #N
        MOV r1, #M
        CMP r0, r1
        ITE GE             ; If-Then-Else on GE
        SUBGE r0, r0, r1   ; true case:  r0 = N - M
        SUBLT r1, r1, r0   ; false case: r1 = M - N
```

IT syntax rules: the **first statement after IT must be the true case**; up to 4 instructions (true or false); the number of instructions must match the number of T and E letters; the false condition is the inverse of the true one; branches **to** IT instructions are not allowed; an instruction inside an IT block can be a branch **only if it is the last one**.

## Stack and Subroutines

### The stack

> [!definition] **Stack**
> A Last-In-First-Out (LIFO) queue. Data is **pushed** (written) to and **popped** (read) from the top; the stack pointer holds the address of the top.

Four stack types, from two independent choices:

- **descending** vs. **ascending**: the top address **decreases** or **increases** after a push;
- **empty** vs. **full**: SP points to the entry where **new data will be pushed**, or to the **last pushed entry**.

### LDM and STM

```asm
LDM{xx}/STM{xx} <Rn>{!}, <regList>
```

- `Rn` = base register; `xx` = addressing mode (how/when Rn is updated during the instruction);
- with `!`, Rn is set to the updated value at the end; without `!`, Rn keeps its initial value;
- `regList` example: `{r0-r4, r10, LR}` = r0, r1, r2, r3, r4, r10, r14.

**Register list rules**: the written order does not matter — registers are automatically sorted in increasing order, the **lowest register at the lowest memory address**, the highest at the highest (`{r8, r1, r3-r5, r14}` → r1, r3, r4, r5, r8, r14). **SP cannot appear** in the list; **PC can appear only with LDM and only if LR is missing** from the list.

**Addressing modes**:

- **IA — increment after** (default): access at Rn, then increment Rn by 1 word (4 bytes), repeat;
- **DB — decrement before**: decrement Rn by 1 word first, then access at Rn, repeat.

### Stack-oriented suffixes

| Stack type | PUSH | POP |
|---|---|---|
| Full descending (FD) | `STMDB` / `STMFD` | `LDM` / `LDMIA` / `LDMFD` |
| Empty ascending (EA) | `STM` / `STMIA` / `STMEA` | `LDMDB` / `LDMEA` |

> [!definition] **PUSH and POP**
> `PUSH <regList>` ≡ `STMDB SP!, <regList>` and `POP <regList>` ≡ `LDMIA SP!, <regList>` — convenience instructions for the **full descending** stack.

### Subroutines

- Called with **BL** and **BLX**: they write the address of the next instruction to **LR** and the target (label or Rn) to PC.
- A reentrant procedure returns with a branch to the address in LR (`MOV PC, LR` — or better `POP {…, PC}`).
- The directives **PROC/FUNCTION** and **ENDP/ENDFUNC** optionally mark the beginning and end of a subroutine.

**Nested calls problem**: when sub1 calls sub2, **LR is overwritten** — sub1 can no longer return to main; sub2 may also clobber registers used by sub1. Every subroutine should therefore save LR and its used registers first and restore them at the end:

```asm
sub     PROC
        PUSH {regList, LR}     ; save return address + used registers
        …
        POP  {regList, PC}     ; restore and return
        ENDP
```

### Passing parameters and results

Three approaches: **in registers**, **by reference** (register with a memory address), **on the stack**. Example: main calls a subroutine computing the absolute difference of two unsigned numbers.

**1. In registers:**

```asm
        MOV r0, #0x34
        MOV r1, #0xA3
        BL  sub1               ; result in r2
sub1    PROC
        PUSH {LR}
        CMP  r0, r1
        SUBHS r2, r0, r1
        SUBLO r2, r1, r0
        POP  {PC}
        ENDP
```

**2. By reference** (r3 points to a memory cell holding both operands, then the result):

```asm
        MOV r0, #0x34
        MOV r1, #0xA3
        LDR r3, =mySpace
        STMIA r3, {r0, r1}     ; store parameters
        BL  sub2
        LDR r2, [r3]           ; result
sub2    PROC
        PUSH {r2, r4, r5, LR}
        LDMIA r3, {r4, r5}
        CMP r4, r5
        SUBHS r2, r4, r5
        SUBLO r2, r5, r4
        STR r2, [r3]           ; save result
        POP {r2, r4, r5, PC}
        ENDP
```

**3. On the stack** (arguments pushed, result written into the reserved slot):

```asm
        MOV r0, #0x34
        MOV r1, #0xA3
        PUSH {r0, r1, r2}      ; prepare arguments, leave space for result
        BL  sub3
        POP  {r0, r1, r2}      ; r2 = result
sub3    PROC
        PUSH {r6, r4, r5, LR}  ; save locals + LR
        LDR r4, [sp, #16]      ; first parameter (offset skips the 4-word saved frame)
        LDR r5, [sp, #20]      ; second parameter
        CMP r4, r5
        SUBHS r6, r4, r5
        SUBLO r6, r5, r4
        STR r6, [sp, #24]      ; save result into the reserved slot
        POP {r6, r4, r5, PC}
        ENDP
```

### ABI — Application Binary Interface

> [!definition] **ABI**
> An interface between two binary program modules — often one is a library or OS facility and the other a user program. A common aspect is the **calling convention**: how data is provided as input to / read as output from computational routines.

ABI components for ARM: the **Procedure Call Standard for the ARM architecture (AAPCS)** — use of the run-time stack and stack invariants — and the **Exception Handling ABI (EHABI)** — table-based stack unwinding separating language-independent unwinding from language-specific concerns.

**AAPCS essentials**:

- The first four registers **r0–r3 (a1–a4)** pass argument values into a subroutine and return a result; if there are **more than 4 formal arguments**, the rest go on the stack.
- A subroutine **must preserve** the contents of **r4–r8, r10, r11 and SP** (callee saving of caller status); r0–r3 can be freely used to hold local variables (caller saves when needed).
- The stack is **full-descending**, with the current extent held in SP (r13); the stack has a base and a limit, though an application may not be able to determine either.
- **Local variables (stack frame)**: created by simply **subtracting the required number of bytes from SP** — this sets memory aside without storing data.
