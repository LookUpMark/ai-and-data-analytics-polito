---
title: Multicycle Operations and Hazards
aliases: [FP Pipelining, MIPS R4000 Pipeline]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Extends the 5-stage integer pipeline with multicycle floating-point units. Defines latency and initiation interval (with the reference table of functional units), analyses the new structural hazards (unpipelined divide unit, single register-write port) and the worsened/new data hazards (longer RAW stalls, WAW hazards from out-of-order completion), and lists the three checks performed in the ID stage. Then presents the MIPS R4000 8-stage superpipeline, its load (2-cycle) and branch (3-cycle) delay slots, the structure of its FP pipeline, and its measured CPI.

## Floating-Point Operations

Floating-point units perform more complex operations than integer ones. Forcing them to complete in a single clock cycle would require either:

- a very slow clock, or
- very complex units.

The popular alternative: **FP units require more than one clock cycle**. The EX stage is composed of different functional units and is **repeated as many times as the instruction requires** — the integer pipeline (IF ID EX MEM WB) becomes something like:

```
Integer:   IF  ID  EX      MEM  WB
FP add:    IF  ID  A1 A2 A3 A4  MEM  WB
FP mult:   IF  ID  M1 ... M7    MEM  WB
```

## Latency and Initiation Interval

> [!definition] **Latency**
> The number of cycles that must elapse between an instruction that produces a result and an instruction that uses that result.

> [!definition] **Initiation interval**
> The number of cycles that must elapse between issuing two operations of the same type to the same unit.

Reference values used in the course:

| Functional unit | Latency | Initiation interval |
|---|---|---|
| Integer ALU | 0 | 1 |
| Data memory | 1 | 1 |
| FP add | 3 | 1 |
| FP/integer multiply | 6 | 1 |
| FP/integer divide | 24 | 25 |

FP units can be **pipelined** (A stages, initiation interval 1) or **not pipelined** (B clock cycles to complete, initiation interval equal to B — the divide unit above is the unpipelined case with interval 25).

### Stage notation for the FP pipeline

The multicycle EX of each FP class has its own stage names (used in the timing diagrams of the slides):

| Instruction | Pipeline stages |
|---|---|
| Integer ALU / load-store | `IF ID EX MEM WB` |
| FP add (`ADDD`) | `IF ID A1 A2 A3 A4 MEM WB` |
| FP multiply (`MULTD`) | `IF ID M1 M2 M3 M4 M5 M6 M7 MEM WB` |
| FP divide (`DIV.D`) | uses the unpipelined divider stage D for ~24 cycles |

Timing example (from the slides) showing where data is produced vs. needed:

```
MULTD F2,F4,F6   IF ID M1 M2 M3 M4 M5 M6 M7 MEM WB
ADDD  F8,F2,F4        IF ID  A1  A2  A3  A4 MEM WB   ; needs F2 in A1
LD    F2,0(R1)              IF ID EX MEM WB          ; makes F2 available earlier
SD    F8,0(R1)                   IF ID EX MEM WB
```

## Hazards with Multicycle Operations

Due to the different structure of the EX stage, hazards become more frequent.

### Structural hazards

- **Unpipelined divide unit**: several instructions could need it at the same time.
- **Register-write port**: since instructions have varying running times, the number of register writes required in one cycle can be larger than 1 (two instructions can reach WB simultaneously).

Solutions:

- adding more write ports (normally **too expensive**);
- forcing a structural hazard: stall instructions **in the ID stage**, or stall them **before entering MEM or WB**.

### More frequent data hazards (RAW)

Because of the longer latency of FP operations, stalls for data hazards may last longer. Example from the slides:

```
MULTD F2,F4,F6   IF ID M1 M2 M3 M4 M5 M6 M7 MEM WB
ADDD  F8,F2,F4        IF ID  A1 A2 A3 A4  MEM WB
LD    F2,0(R1)              IF ID EX MEM WB
SD    F8,0(R1)                   IF ID EX MEM WB
```

The `ADDD` needs F2 in A1 while `MULTD` makes it available only after M7: a **Read After Write (RAW)** hazard with a long stall.

### New data hazards (WAW)

Instructions no longer reach WB **in order**, so new hazard kinds appear:

```asm
DIV.D F2, F4, F6
ADD.D F2, F8, F10    ; shorter: completes before DIV.D
```

`ADD.D` could write F2 **before** `DIV.D` — a **Write After Write (WAW)** hazard: the final value of F2 would be wrong.

**Solution**: before issuing an instruction to the EX stage, check whether it will write the same register as an instruction still in the EX stage; if so, **stall the new instruction in ID**.

### Summary: the three checks in ID

If hazard detection is always performed in the ID stage, three checks are needed:

1. **structural hazards** — involving the divide unit and the register write port;
2. **RAW data hazards** — check whether some source register is listed among the destination registers of pending instructions and whether it will not be available at the right moment;
3. **WAW data hazards** — check whether the instruction currently in ID has the same destination register as any instruction in A1…A4, D, M1…M7.

## The MIPS R4000 Pipeline

> [!definition] **Superpipeline**
> The MIPS R4000 (64-bit microprocessor, **1991**, instruction set similar to MIPS64) uses a **deeper 8-stage pipeline** to account for slower cache access and higher clock frequency; memory accesses are decomposed into several stages. Long pipelines of this kind take the name of *superpipelines*.

### The 8 stages

| Stage | Function |
|---|---|
| IF | first part of instruction fetch |
| IS | second part of instruction fetch |
| RF | instruction decode, register fetch, hazard checking |
| EX | execution: effective address calculation, ALU operation, branch target computation, condition evaluation |
| DF | first part of data fetch |
| DS | second part of data fetch |
| TC | last part of data fetch |
| WB | write-back for loads and register-register operations |

Both instruction and data memory accesses are pipelined: **a new instruction can start on every clock cycle**.

Consequences of the deeper pipeline:

- **more forwarding** is required;
- increased **load delay slot: 2 cycles** (data available at the end of DS; with forwarding it is passed to the dependent instruction);
- increased **branch delay slot: 3 cycles** (condition evaluation is performed during EX).

### FP pipeline structure

The FP unit is composed of three functional units — **divider, multiplier, adder** — and can be thought of as 8 different stages:

| Stage | Functional unit | Description |
|---|---|---|
| A | adder | Mantissa ADD |
| D | divider | divide |
| E | multiplier | exception test |
| M | multiplier | multiplier I |
| N | multiplier | multiplier II |
| R | adder | rounding |
| S | adder | operand shift |
| U | adder | unpack numbers |

The **latency is reduced by 1** if the destination is a **store** instruction (the value can be forwarded directly to memory, skipping the register write).

### Performance

- For **integer programs**, **branch delays** are the most important contributors to total CPI.
- For **FP programs**, **FP result stalls** are the most important contributors.
- **Total CPI varies between 1.2 and 2.8**, depending on the program.
