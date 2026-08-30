---
title: Assembler Directives and Literal Pools
aliases: [ARM Directives, Literal Pool]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> ARM assembler source format and the main directives (AREA, RN, EQU, DCB/DCW/DCD, ALIGN, SPACE, LTORG, END), with byte-accurate memory layout examples of aligned vs. unaligned constant allocation. Then the problem of loading constants and addresses into registers: the immediate restrictions of MOV, the MVN/MOVW/MOVT variants, the `LDR Rd, =constant` pseudo-instruction and how the assembler builds a **literal pool** with a PC-relative load, including offset computation, the 4096-byte range rule and the use of LTORG. Ends with LDR =label vs. ADR/ADRL for loading addresses.

## Source Line Format

A general source line is:

```
{label} {operation} {;comment}
```

- `operation` may be an **instruction**, a **directive**, or a **pseudo-instruction**;
- **labels must start at the beginning of the line**;
- instructions, directives and pseudo-instructions must be **preceded by white space** (a tab or any number of spaces).

## Common Directives

| Directive | Purpose |
|---|---|
| `AREA` | defines a block of code or data |
| `RN` | associates a register with a name |
| `EQU` | equates a symbol to a numeric constant |
| `ENTRY` | declares an entry point to your program |
| `DCB, DCW, DCD` | allocate memory and specify initial runtime contents |
| `ALIGN` | align data or code to a particular memory boundary |
| `SPACE` | reserves a zeroed block of memory of a particular size |
| `LTORG` | assigns the starting point of a literal pool |
| `END` | designates the end of a source file |

### Sections of data and code: AREA

IDE tools must be told how to treat the parts of a program (data sections, program sections, blocks of coefficients…). These **indivisible, named sections** are manipulated by the linker and end up in the correct type of memory: read-write data in **RAM**, program code in **Flash**.

```asm
AREA sectionName {,attr} {,attr}…
```

- If `sectionName` starts with a number it must be enclosed in bars, e.g. `|1_DataArea|`.
- `|.text|` is used by the C compiler.
- **At least one AREA directive is mandatory**; it usually appears in the first few lines.
- Example: `AREA Example, CODE, READONLY`.

Section attributes:

| Attribute | Meaning |
|---|---|
| `CODE` | section contains machine code |
| `DATA` | section contains data |
| `READONLY` | section can be placed in read-only memory |
| `READWRITE` | section can be placed in read-write memory |
| `ALIGN = expr` | section aligned on a 2^expr-byte boundary |

### Register names

- Predefined: `r0`–`r15` / `R0`–`R15`; aliases `a1`–`a4` (r0–r3), `sp`/`SP` (r13), `lr`/`LR` (r14), `pc`/`PC` (r15).
- Custom names via RN: `coeff1 RN 8`.

### Constants: EQU and number bases

`name EQU expression` gives a symbolic name to a numeric constant (readability; single point of update). Numbers can be expressed in any base: decimal (`123`), hexadecimal (`0x3F`), or `n_xxx` where n is the base and xxx the value in that base (e.g. `8_163` octal, `2_1010` binary).

## Constant Allocation in Code Memory

`{label} DCxx expr{,expr}…` — the family:

| Directive | Size | Alignment |
|---|---|---|
| `DCB` | byte (8 bit) | none; also accepts strings |
| `DCW` / `DCWU` | half-word (16 bit) | DCW aligns to 2 bytes; DCWU is unaligned |
| `DCD` / `DCDU` | word (32 bit) | DCD aligns to 4 bytes; DCDU is unaligned |

`expr` is a numeric expression in the proper range, or a string (with DCB only).

### Memory layout examples

`DCB 65, 0x73, 8_163` followed by `DCB "embly"` produces, starting at 0x000000D2:

```
D2: 41 'A'   D3: 73 's'   D4: 73 's'   D5: 65 'e'
D6: 6D 'm'   D7: 62 'b'   D8: 6C 'l'   D9: 79 'y'
```

The same string continued with **DCW 0x626D, 0x796C**: DCW aligns to the next half-word, inserting a **NUL** padding byte (at D5 in the example), then stores `6D 62` and `6C 79`.

- With **DCWU** no padding byte is inserted (storage continues unaligned).
- With **DCD 0x796C626D** the word is aligned to the next 4-byte boundary (three NUL padding bytes in the example), then stored little-endian as `6D 62 6C 79`.
- With **DCDU** the word is stored immediately, unaligned.

### ALIGN

```asm
ALIGN {expr{, offset}}
```

Aligns the current location (padding with zeros) to the next address of the form `n × expr + offset`. Without `expr`, ALIGN sets the location to the **next word boundary**. Use case: the ADR Thumb pseudo-instruction can only load word-aligned addresses, but a label within Thumb code might not be word aligned — `ALIGN 4` ensures four-byte alignment.

Example: `DCB 65` then `ALIGN 2` inserts one zero byte before the next byte; `ALIGN 4` inserts three.

### SPACE and END

- `{label} SPACE expr` reserves a zeroed block of `expr` bytes — e.g. `long_var SPACE 8`.
- `END` tells the assembler the current location is the end of the source file.

## Loading Constants into Registers

### MOV and its limits

`MOV` assigns to a register the content of a register or a constant — **it cannot assign an instruction or data address**:

```asm
myCode …
        MOV r0, myData     ; error
        MOV r1, myCode     ; error
stop    B stop
myData  DCD 0xC90147D2
```

`MOV Rd, Rm {, shift}` accepts an optional shift on Rm (`ASR #n`, `LSL #n`, `LSR #n`, `ROR #n`, `RRX`), but the equivalent **shift instruction is preferred** (`LSL r0, r1, #3` instead of `MOV r0, r1, LSL #3`).

### Valid MOV immediates

`MOV Rd, #constant` accepts:

- a **16-bit value** (0–65535);
- a value obtained by **shifting left an 8-bit value** (even shift amounts 0…24), e.g. `0xFF`, `0x3FC` (≪2), `0xFF0` (≪4), `0xFF00` (≪8), … `0xFF000000` (≪24);
- values of the form `0x00XY00XY`, `0xXY00XY00`, `0xXYXYXYXY`.

| Left shift | Max decimal | Max hex |
|---|---|---|
| 0 | 255 | 0xFF |
| 2 | 1020 | 0x3FC |
| 4 | 4080 | 0xFF0 |
| 8 | 65280 | 0xFF00 |
| 24 | 0–255 × 2²⁴ | 0xFF000000 |

Exercise from the slides — valid for MOV: `0x00004B4B`, `0x004B4B00`, `0x004B004B`, `0x004B4B4B`, `0x4B4B4B4B` (and `0x4B000000`, `0x004B0000` via shifting); invalid: none among these is rejected except combinations that do not fit any pattern.

### MOVW, MVN, MOVT

- **MOVW** is like MOV but takes only a 16-bit value.
- **MVN** (move negative) moves the **one's complement** of the operand; same syntax as MOV except it does **not accept a 16-bit value**. `MVN r0, #0` → `r0 = 0xFFFFFFFF`. The assembler can change a MOV into a MVN: `MOV r0, #-2` becomes `MVN r0, #1` (because −2 = 0xFFFFFFFE is not in the range of MOV).
- **MOVT** (move top) moves a 16-bit value into the **high halfword** of a register. Any 32-bit constant can be built with MOV + MOVT:

```asm
MOV  r0, #0x47D2     ; low halfword
MOVT r0, #0xC901     ; high halfword -> r0 = 0xC90147D2
```

## The Literal Pool

> [!definition] **LDR pseudo-instruction for constants**
> `LDR <Rd>, =<constant>` loads a constant into a register:
> - if the constant is among the valid MOV values, the assembler replaces it with `MOV Rd, #constant`;
> - **otherwise a block of constants — the literal pool — is created**, and the instruction becomes `LDR <Rd>, [PC, #<offset>]`.

### Offset computation

The offset is the difference between the address of the literal pool and the PC, where PC = **address of the current instruction + 4, with the second bit cleared for word alignment**. The offset is expressed with **12 bits**.

Worked example:

```
0x00000118   LDR r0, =0xC90147D2      ; becomes LDR r0, [PC, #40]
             ...
0x00000144   0x47D2                   ; literal pool (little-endian words)
0x00000146   0xC901
```

1. 0x118 = 0b100011000
2. 0x118 + 4 = 0x11C
3. PC = 0x11C (bit 1 already clear)
4. offset = 0x144 − 0x11C = 0x28 = **40** → `LDR r0, [PC, #40]`

### Literal pool placement

- **By default** the literal pool is placed at the **END directive**, after the last instruction.
- Since the offset is 12 bits, the distance between the instruction and the pool must be **< 4096 bytes**; otherwise **LTORG** must be used to place the pool closer.

Error case (from the slides):

```asm
Reset_Handler PROC
        EXPORT  Reset_Handler [WEAK]
        LDR r0, =0xC90147D2
stop    B stop
myEmptySpace SPACE 4100      ; pool at END is now > 4096 bytes away!
        ENDP
        END                  ; literal pool saved here -> out of range
```

Correct version — bring the pool inside range with LTORG:

```asm
Reset_Handler PROC
        EXPORT  Reset_Handler [WEAK]
        LDR r0, =0xC90147D2
        B stop
        LTORG                ; literal pool saved here
stop    B stop
myEmptySpace SPACE 4100
        ENDP
        END
```

## Loading Addresses into Registers

Two pseudo-instructions:

```asm
LDR <Rd>, =<label>     ; creates a constant in a literal pool, PC-relative load
ADR <Rd>, <label>      ; adds/subtracts an offset to/from PC
```

**LDR =label**:

- can reference a label **outside the current section**;
- increases code size (pool entry + load).

```asm
Stack_Size EQU 0x00000200
        AREA STACK, NOINIT, READWRITE
Stack_Mem SPACE Stack_Size
        AREA |.text|, CODE, READONLY
        LDR r12, =Stack_Mem   ; r12 = address of the bottom of the stack = r13 - 0x200
        …
        END                   ; literal pool saved here
```

**ADR**:

- does **not increase code size**, but **cannot create all offsets**;
- addresses generated must be **multiple of 4** (word aligned);
- loads addresses in the **same section** only;
- is replaced with `LDR Rd, [PC, #offset]` with a 12-bit offset; if the offset is higher than 4095 bytes, **ADRL** must be used instead — ADRL generates **two operations** and reaches up to **1 MB**.

```asm
Reset_Handler PROC
        ADR r0, myData      ; same-section, aligned label
stop    B stop
myData  DCD 0xC90147D2
        ENDP
```

The interaction of LDR/ADR with the **BX** instruction (bit 0 semantics for Thumb state) is covered in [[10-memory-branches-and-subroutines]].
