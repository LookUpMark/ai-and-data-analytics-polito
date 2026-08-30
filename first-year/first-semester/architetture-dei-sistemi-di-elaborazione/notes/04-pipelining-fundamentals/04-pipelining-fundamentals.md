---
title: Pipelining Fundamentals
aliases: [Pipeline Hazards, Basic Pipelining]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Pipelining as the implementation technique that overlaps the execution of multiple instructions. Starts from the classic 5-cycle unpipelined MIPS implementation (IF, ID, EX, MEM, WB) and its datapath, then builds the pipelined version with pipeline registers and split instruction/data memories. Quantifies the ideal speedup with a worked example, and covers the three hazard classes — structural, data (with forwarding and load-use stalls) and control (with freeze, predict-untaken/taken and delayed branch solutions) — including the load-interlock detection conditions and the cost/benefit trade-off of removing structural hazards.

## What Pipelining Is

> [!definition] **Pipelining**
> An implementation technique whereby **multiple instructions are overlapped in execution**. In a pipeline, different units (called **pipe stages** or **segments**) complete different parts of different instructions in parallel.

Example (4-stage pipeline F-D-E-W): instruction I1 finishes at cycle 4, then one instruction completes per cycle:

```
Clock cycle  1  2  3  4  5  6  7  8
I1           F  D  E  W
I2              F  D  E  W
I3                 F  D  E  W
I4                    F  D  E  W
```

> [!definition] **Throughput and machine cycle**
> The **throughput** of a pipelined processor is the number of instructions that exit the pipeline per time unit. All stages are synchronized (they all advance together); the time to execute one step is the **machine cycle**, normally one clock cycle. The machine-cycle length is determined by the **slowest stage**.

### Ideal pipeline

In an **ideal pipeline** all stages are perfectly balanced (same execution time), and:

```
throughput_pipelined = throughput_unpipelined × n
```

with n = number of pipeline stages.

## The Example Processor: Unpipelined Implementation

Execution of each instruction takes at most **five clock cycles**:

1. **IF** — Instruction fetch: `IR ← Mem[PC]`; `NPC ← PC + 4`
2. **ID** — Instruction decode / register fetch: `A ← Regs[IR6..10]`; `B ← Regs[IR11..15]`; `Imm ← ((IR16)16 ## IR16..31)` (sign extension)
3. **EX** — Execution / effective address:
   - memory reference: `ALUOutput ← A + Imm`
   - register-register ALU: `ALUOutput ← A op B`
   - register-immediate ALU: `ALUOutput ← A op Imm`
   - branch: `ALUOutput ← NPC + Imm`; `Cond ← (A op 0)`
4. **MEM** — Memory access / branch completion:
   - memory reference: `LMD ← Mem[ALUOutput]` or `Mem[ALUOutput] ← B`
   - branch: `if (cond) PC ← ALUOutput else PC ← NPC`
5. **WB** — Write-back:
   - register-register ALU: `Regs[IR16..20] ← ALUOutput`
   - register-immediate ALU / load: `Regs[IR11..15] ← ALUOutput` or `← LMD`

**Fixed-field decoding** allows decoding to be performed while registers are read (ID).

Behavior notes: all instructions require 5 clock cycles except branches, which require 4. Optimizations can reduce the average CPI (e.g., ALU instructions could complete during the MEM cycle); hardware can be shared to avoid duplicating ALUs and memory; a single-clock architecture (CPI = 1) is the alternative considered next.

## Basic Pipelined Version

Key rules of the pipelined datapath:

- a **new instruction is started at each clock cycle**; different resources work on different instructions at the same time;
- each resource can be used for **one purpose only** per cycle, therefore:
  - **separate instruction and data memories (caches)** are required;
  - the **register file** is used in two stages — read in the second half of the clock cycle in ID, write in the first half in WB — so it must support a read and a write in the same cycle;
  - the **PC must be changed in the IF stage** (which raises the branch problem);
  - **pipeline registers** (IF/ID, ID/EX, EX/MEM, MEM/WB) are added between stages.

### Pipeline performance

- Pipelining **increases throughput without making single instructions faster**; single-instruction processing is actually slower due to pipeline control overhead.
- The **depth** of a pipeline is limited by the need for balanced stages and by the **pipelining overhead** (pipeline register delay and clock skew).

> [!definition] **Speedup example**
> Unpipelined machine: clock = 1 ns; ALU and branches take 4 cycles, memory ops 5 cycles; frequencies 40%, 20%, 40%.
> Average instruction time = 1 ns × ((0.4+0.2)×4 + 0.4×5) = **4.4 ns**.
> If pipelining slows the clock by 20% (1.2 ns per stage/instruction):
> `speedup = 4.4 ns / 1.2 ns ≈ 3.7 times`

## Pipeline Hazards

> [!definition] **Hazard**
> A situation that prevents an instruction from executing during its designated clock cycle. Three classes:
> - **structural hazards** — resource conflicts;
> - **data hazards** — an instruction depends on the result of a previous instruction;
> - **control hazards** — arise from pipelining branches and other instructions that change the PC.

### Stalls and bubbles

One way to deal with hazards is to **stall** the pipeline. When an instruction is stalled:

- the instructions **following** it are also stalled;
- the instructions **preceding** it continue.

A stall introduces a **bubble** in the pipeline. (Example: with a single memory port, the fetch of instruction i+3 conflicts with a data access and must be delayed; as a consequence no instruction completes at cycle #8.)

## Structural Hazards

They happen when some pipeline unit cannot execute all the operations scheduled for a cycle. Examples:

- a unit cannot complete its task in one clock cycle;
- the pipeline has **one register-file write port** but a cycle needs two register writes;
- a **single-port memory** when different instructions want to access memory in the same cycle.

**Removing** structural hazards requires adding or improving hardware; the designer trades performance against cost based on the frequency of occurrence.

> [!definition] **Structural hazard trade-off example**
> 40% of instructions access memory; the machine with the structural hazard has a clock 1.05× faster.
> With hazard: `Avg time = (1 + 0.4×1) × Clock_ideal/1.05 = 1.33 × Clock_ideal`.
> Without hazard: `Avg time = 1 × Clock_ideal`. → The machine **without** the structural hazard is faster (1.33 > 1.05).

## Data Hazards

Overlapping execution changes the **order of read/write accesses to operands**, which can produce wrong results or undeterministic behavior (an interrupt during a critical fragment may "fix" correctness only sometimes).

Classic example (5-stage pipeline, register file written in WB, read in ID):

```asm
ADD R1, R2, R3     ; writes R1 in WB
SUB R4, R1, R5     ; reads R1 in ID 2 cycles too early  -> wrong
AND R6, R1, R7     ; reads R1 1 cycle too early         -> wrong
OR  R8, R1, R9     ; reads R1 in the write cycle        -> correct only if writes happen before reads in the same cycle
XOR R10, R1, R11   ; reads R1 one cycle after the write -> correct
```

### Overcoming data hazards

- **stall** the instructions needing the data until it is available;
- implement **forwarding (bypassing)**.

> [!definition] **Forwarding**
> Special hardware detects when a previous ALU operation will write the register that is a source of the current ALU operation, and selects the ALU result as ALU input rather than the value read from the register file. The hardware must be able to forward from any previously started instruction (that has not yet written its final location) to any input, and must not forward anything if a following instruction is stalled or an interrupt occurred.

Generalizing: to always avoid stalling, forwarding should be possible **between any pipeline register and any input of any functional unit** (ALU inputs, data-memory inputs — e.g., `ADD R1…; LD R4, 0(R1); SD R4, 12(R1)` — and the zero-detection unit for branches).

### Data hazards requiring stalls (load-use)

Not all hazards can be solved by forwarding:

```asm
LD  R1, 0(R2)     ; data available at end of MEM
SUB R4, R1, R5    ; would need R1 at the start of EX — forwarding would go "back in time": impossible
```

Solution: the **pipeline interlock** circuitry forces a **bubble** (one stall cycle) to preserve correctness; after the stall, forwarding completes the path.

**Implementation of the control** — at each clock cycle:

- all data-hazard tests are performed when the instruction is in the **ID stage**;
- on a hazard, either the appropriate **forwarding** is activated, or the instruction is **stalled before issue**.

**Load interlock detection** (Load in EX, consumer in ID): if the destination register of the load matches a source register of the instruction in ID, stall.

**Introducing a stall** in the EX stage is done by:

- forcing all 0s in the ID/EX pipeline register (equivalent to a `nop`);
- forcing the IF/ID pipeline register to hold its current value;
- freezing the PC (IF status unaltered).

**Forwarding logic** compares the destination fields of the IRs in the **EX/MEM** and **MEM/WB** registers with the source fields of the IRs in the **IF/ID, ID/EX and EX/MEM** registers; forwarding paths go from the ALU or data-memory outputs to the ALU inputs, data-memory inputs, or the zero-detection unit.

### Causes of data hazards

A hazard exists whenever there is a **dependence** between instructions close enough that pipelining would change the order of operand access:

- **register operands**;
- **memory operands** — possible if load/store accesses are not made in the same stage, or if execution proceeds while an instruction waits for a cache miss to be solved.

## Control Hazards

Control hazards are due to branches (conditional and unconditional), which may change the PC **after** the following instruction has already been fetched. For conditional branches the taken/untaken decision comes even later. In the MIPS implementation considered here, the PC is written with the target address at the **end of the ID stage**.

**Basic solution**: stall the pipeline as soon as a branch is detected (ID), and reduce the penalty by deciding earlier and computing the target earlier:

- the **comparison unit is moved ahead one stage**, so the branch decision can be taken earlier;
- a **further adder** computes the target address one clock cycle earlier.

In the resulting schedule, the IF of the cycle right after the branch fetches the following instruction "as if the branch is not taken" — that slot is **always useless** when the branch is taken.

### Techniques for reducing the branch penalty

| Technique | Description |
|---|---|
| **Freezing the pipeline** | stall (flush) as soon as a branch is detected, until the outcome is known. Simplest to implement. |
| **Predict untaken** | assume the branch is not taken; do not change the pipeline status until the decision; **undo** (turn fetched instructions into `nop`) if the branch turns out taken. Cost differs taken vs. untaken. |
| **Predict taken** | if the target address is known before the outcome, assume the branch is taken and fetch from the target. |
| **Delayed branch** | fill the slot after the branch (**branch-delay slot**) with instructions that must execute regardless of the outcome; the CPU does nothing special when decoding the branch. |

**Compiler role**: with predict taken/untaken hardware, the compiler can generate code that maximizes prediction accuracy — e.g., `for` loops suit the predict-untaken scheme, `do-while` loops the predict-taken scheme.

**Delayed-branch effectiveness**: depends on the compiler's ability to find useful instructions for the delay slots; with this technique only about **30% of branches** still produce a penalty.

**Trend**: with deeply pipelined processors, delay slots become longer and their advantage smaller; several current RISC architectures no longer support delayed branches (the ARM Cortex-M3 of [[07-arm-v7m-architecture]] is one such case: branches always flush and refill the pipeline).
