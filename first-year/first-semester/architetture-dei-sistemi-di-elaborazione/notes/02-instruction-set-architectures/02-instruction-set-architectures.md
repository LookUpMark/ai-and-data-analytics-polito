---
title: Instruction Set Architectures
aliases: [ISA Principles, Instruction Set Principles]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The Instruction Set Architecture (ISA) as the machine view seen by the programmer/compiler. Covers the CPU taxonomy by internal storage (stack, accumulator, register classes), memory addressing (endianness, alignment), the catalogue of addressing modes with experimental statistics on displacement/immediate sizes, the operations of an instruction set (with focus on control flow and procedures), operand types and sizes, and the three instruction-encoding styles (variable, fixed, hybrid). Closes with hardware–compiler interaction and design recommendations.

## The Instruction Set Architecture

> [!definition] **Instruction Set Architecture (ISA)**
> The ISA is how the computer is seen by the programmer or the compiler.

There are many alternatives for the ISA designer; any choice modifies the way instructions are encoded. The alternatives are evaluated in terms of:

- processor performance,
- processor complexity,
- compiler complexity,
- code size,
- power consumption, …

Different product areas may assign different weights to these parameters.

## Taxonomy by Internal Storage

CPUs are classified according to the type of their internal storage:

- **stack**
- **accumulator**
- **registers**:
  - register-memory
  - register-register (**load-store**)
  - memory-memory (no real cases)

For the code sequence `C = A + B`, the slide compares the code produced by each class (stack: push/pop sequence; accumulator: load/add/store; register-memory and load-store: explicit register operands).

### GPR machines

> [!definition] **General-Purpose Register (GPR) machine**
> Currently, all processors are GPR machines because registers are **faster than memory** and **easier for a compiler to use**.

CPUs can be further classified by:

- typical number of **operands per ALU instruction** (2 or 3);
- typical number of **memory operands per ALU instruction** (from 0 to 3).

Load-store machines (e.g., MIPS, RISC-V, ARM) have 0 memory operands per ALU instruction.

## Memory Addressing

Alternatives:

- **Little Endian vs. Big Endian**
- **Aligned vs. misaligned accesses**

> [!definition] **Little Endian**
> Puts the byte with the lower address (X...X000) at the *least significant* position; the address of the data is that of the least significant byte.

> [!definition] **Big Endian**
> Puts the byte with the lower address at the *most significant* position; the address of the data is that of the most significant byte.

Allowing only aligned accesses to memory is a limitation; allowing misaligned accesses requires **hardware overhead** and **performance overhead**.

## Addressing Modes

> [!definition] **Addressing mode**
> In GPR machines, an addressing mode specifies a constant, a register, or a memory location (through its *effective address*).

| Mode | Meaning (example) |
|---|---|
| Register | operand is in a register |
| Immediate | operand is a constant in the instruction |
| Displacement | effective address = register + constant |
| Register deferred (indirect) | effective address is in a register |
| Indexed | effective address = register + register (index) |
| Direct (absolute) | effective address is a constant in the instruction |
| Memory indirect | effective address is in a memory location |
| (Post) autoincrement | address = register, then register incremented |
| (Pre) autodecrement | register decremented, then used as address |
| Scaled | effective address = register + index × size |

Carefully choosing the addressing modes has consequences:

- it can **reduce the number of instructions**,
- but it **increases CPU architecture complexity**,
- and can **increase the average Cycles Per Instruction (CPI)**.

### Open issue: how many bits for a displacement?

When the selected addressing mode requires a displacement, how many bits should be devoted to it? How large can an embedded immediate value be? Experimental evaluation (VAX usage statistics, Alpha displacement/immediate distributions shown in the slides) gives the summary:

- Displacement, immediate and register **indirect modes represent from 75% to 99% of addressing modes**.
- The address size for displacement mode should be **12 to 16 bits** (75% to 99% of displacements).
- The size of the immediate field should be at least **8 or 16 bits** (50% and 80% of the cases, respectively).

## Operations in the Instruction Set

**Making the common case fast**: not all instructions are executed with the same frequency; when designing an instruction set, the most commonly executed instructions should be made faster (80x86 instruction frequency data shown in the slides).

### Control flow instructions

Four categories:

1. **conditional branches**
2. **jumps**
3. **procedure calls**
4. **procedure returns**

Conditional branches are by far the most frequently executed control-flow instructions.

**Destination address**: normally specified as a **displacement with respect to the current value of the Program Counter** (PC-relative). Benefits:

- saves bits, since the target instruction is often close to the source one;
- the code is **position-independent**.

**Branch distances**: measurements show that PC-relative branch displacements of at least 8 bits are the best choice (most branches are short).

**Register indirect jumps** and procedure calls allow:

- code with jump targets not known at compile time;
- implementation of **case/switch statements**;
- support for **dynamically shared libraries** (loaded only when called);
- support for **virtual functions** (calling different functions depending on the data type).

### Evaluating branch conditions

Approaches shown in the slides (RISC-V example): either compare-and-branch instructions or separate compare + conditional branch (see also the ARM condition flags in [[08-arm-instruction-set]] and MIPS branches in [[03-mips64-instruction-set]]).

### Procedures

Some information needs to be saved:

- the **return address**;
- the accessed registers — two strategies:
  - **caller saving**
  - **callee saving**

Register-indirect and PC-relative addressing can also be used in procedure call and return.

## Type and Size of Operands

Most frequently supported data types:

| Type | Size |
|---|---|
| char | 1 byte |
| half word | 2 bytes |
| word | 4 bytes |
| double word | 8 bytes |
| single-precision floating-point | 4 bytes |
| double-precision floating-point | 8 bytes |

(Distribution of data accesses by size shown in the slides: word accesses dominate.)

## Instruction Set Encoding

Instruction encoding depends on which instructions compose the set and which addressing modes are supported. With many addressing modes, an **address specifier field** specifies the mode and registers involved; with few modes, modes can be encoded together with the **opcode**.

| Style | Properties |
|---|---|
| Variable length | supports any number of operands; minimum code size; lower performance |
| Fixed length | fixed number of operands, address specifier in the opcode; fixed instruction length; maximum performance; larger code size |
| Hybrid (multiple formats) | formats specified by the opcode; allows trading off code size vs. performance |

### Conflicting issues

The designer must balance:

- the **code size**;
- the size of the instruction set, the number of addressing modes, and the number of registers;
- the complexity of the fetch and decoding hardware.

## Hardware–Compiler Interaction

- Assembly-level programs are now produced by **compilers only**.
- The CPU designer and the compiler writer must **interact and cooperate**.

> [!definition] **Register allocation**
> Choosing which variables go in which registers and when — one of the crucial optimization phases of a compiler. The problem is based on **graph coloring** and can be solved better if the number of registers is high (**>16**).

Optimizing variable access time by allocating variables to registers is possible only for variables stored in the **stack** or for **global variables** in memory. It is impossible for variables belonging to the **heap**, due to the **aliasing problem** (the variable is accessed through pointers).

### How the architect can help the compiler writer

*Make the frequent case fast and the rare case correct:*

- **Regularity** — make the orthogonal combination of operations and addressing modes consistent;
- **provide primitives, not solutions** — primitives the compiler can combine, not specialized "solutions" for one language;
- **simplify trade-offs** among alternatives;
- **provide instructions that bind quantities known at compile time as constants**.

### Recommendations

- At least **16 registers**;
- **Orthogonality**;
- **Simplicity**.
