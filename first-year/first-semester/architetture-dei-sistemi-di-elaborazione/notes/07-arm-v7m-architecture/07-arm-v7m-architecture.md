---
title: ARM v7-M Architecture
aliases: [ARM Cortex-M3, ARM Based Systems]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The ARM world and the v7-M architecture used in the course labs. Covers ARM history and business model (cores integrated in SoCs), Cortex families (A/R/M), the development tool chain, and the Landtiger/LPC1768 case study. Then the v7-M architecture in detail: the ARM datapath with barrel shifter and instruction execution flows, the Cortex-M3 3-stage pipeline, the 16+2 register set, the Program Status Register, Thumb and Thumb-2, operating modes and privilege levels, the AMBA bus system, clocking, power management, the linear 4 GB memory map, and exception/interrupt handling with the NVIC and the interrupt vector table.

## ARM: History and Business Model

- The ARM processor was first developed (**1983–1985**) by **Acorn Computers, Ltd.** (Cambridge, UK); the designers were heavily influenced by **Berkeley RISC I**.
- In **1990** ARM Ltd. was founded by Acorn, Apple and VLSI.
- ARM cores are popular among SoC designers for their **good trade-off between performance and power consumption**.
- **ARM does not manufacture silicon**: processors are mainly sold as **cores** for integration in Systems on Chip:
  - **hard cores** — ARM provides a physical layout in a given technology;
  - **soft cores** — ARM provides a high-level description that the designer synthesizes to any technology.
  In a few cases ARM processors have been delivered as stand-alone devices.

Market presence cited in the slides: ~70% of world population uses Arm technology, 280+ bn Arm-based chips shipped, 95% of smartphones, 50% of automotive MCUs, 90% of wearables. Iconic products: Nokia 3310 (ARM7TDMI), Playstation Vita (Cortex-A9), Galaxy Watch 4, Samsung Galaxy S22 (Cortex-X2/A710/A510), AWS Graviton3 (Neoverse-V1).

### Arm processor families

| Family | Class | Typical use |
|---|---|---|
| **Cortex-A** | Application-Class CPU | smartphones, datacenters/cloud/HPC |
| **Cortex-R** | Real-Time Class CPU | brake systems, avionics/automotive, medical devices |
| **Cortex-M** | Embedded, energy efficient | microcontrollers, IoT, embedded in other chips/ASICs |

### System-on-Chip (SoC)

> [!definition] **SoC**
> Entire systems integrated in a single piece of silicon, composed of modules called **Embedded Cores** (processor core, memory banks, ADC/DAC, pin pads, …). Commercial ARM-based SoCs come from Samsung, NXP, STMicroelectronics, and many others.

ARM-compliant OSes (Windows CE, Linux/Debian, U-Boot) all require a **bootloader** to be launched. The **tool chain** comprises: C/C++ and ASM code → **cross compiler** (with processor ISA information) → executable code → **loader** (SW debug/emulation, HW debug via **JTAG** IEEE 1149.1 access port and IEEE 1500-based structures such as Embedded ICE). Tools: Keil (Windows), CodeSourcery GNU toolchains (Linux).

### Case of study: Landtiger board

- Based on the **NXP LPC1768** SoC, including a 32-bit **ARM Cortex-M3** microcontroller with a full set of on-chip peripherals.
- **KEIL µVision** IDE: trial version with **32 KB code limitation**, full debugging features, accurate timing calculation; HW debug via ULINK2 (JTAG) through the RealView component.

What the course covers on this platform: ARM assembly principles and ISA, C+ASM programming following **ABI** standards, SoC-level programming (peripherals, clock and power modes), interrupt/exception management (SW interrupts → system calls; HW interrupts from timers, buttons; interrupt controller).

## The Generic ARM Datapath

The ARM generic architecture (dataflow shown in the slides) consists of: a **register bank**, a **barrel shifter**, the **ALU**, an **address register** and **incrementer**, the **multiply** unit, data-in/data-out registers, and the instruction **decode & control**, connected by A/B/ALU buses.

- The register bank has **two read ports and one write port**; one additional read and one additional write port are reserved for **r15** (the PC).
- The **barrel shifter** processes one operand *before* the ALU (combined ALU+shifter operations, see [[08-arm-instruction-set]]).

### Instruction execution flows

For a **data processing reg-reg** instruction: two operands are read from Rn and Rm → one operand is possibly rotated → the ALU generates the result → the result is written to Rd → the next instruction is fetched → the PC is updated. The **reg-imm** variant is identical except the second operand is an immediate (`[7:0]` rotated).

**Data transfer instructions** require **two clock cycles** for the Execute stage:
1. compute the address using one register and one immediate (`= A / A + B / A − B`);
2. access memory; for `STR`, the source register is sent to memory (auto-indexing updates the base register).

**Branch instructions**: first compute the target address by adding an immediate (shifted left by 2) to the PC; then the **pipeline is flushed and refilled**. **Branch and link** requires a further clock cycle (while the pipeline refills) to save the return address in **r14**.

## ARM Cortex-M3

The Cortex-M3 is the course case study. Its datapath and pipeline:

- **3-stage fetch–decode–execute pipeline**;
- **branch**: it takes **3 cycles** to complete a branch; worst case = indirect branch taken; branches **always flush and refill** the pipeline; **no delayed-branch mechanism** is supported;
- **LDR**: the read cycle must complete on the bus before LDR can complete, since there is only **one write-back port** in the register file.

### Programmer's view

- **16+2 32-bit registers**;
- efficient interrupt handling; power management (idle mode); debug support (breakpoints, watchpoints, instruction trace); strong OS support (user/supervisor model);
- designed to be **fully programmed in C/C++**, even reset, interrupts and exceptions.

### Register set

| Register | Role |
|---|---|
| R0–R12 | general purpose |
| **R13 (SP)** | stack pointer — a replica (**PSP**, process stack pointer) is available to ease interrupt management (plus MSP, main stack pointer: `SP_proc` / `SP_main`) |
| **R14 (LR)** | link register |
| **R15 (PC)** | program counter |

Supported data types: **byte (8), halfword (16), word (32)**.

### Program Status Register (PSR)

The PSR can be accessed all at once or as 3 registers:

- **APSR** (Application PSR): **N** (negative), **Z** (zero), **C** (carry), **V** (overflow), **Q** (sticky), **GE** (greater-than-or-equal) flags;
- **EPSR** (Execution PSR): **T** (Thumb) bit, **ICI** (interrupt-continuable instruction) bits, **IT** (if-then) status bits;
- **IPSR** (Interrupt PSR): **exception number** (ISRNUM) used in exception handling.

The **T bit** selects the instruction set: T=1 → fetched code interpreted as **Thumb** instructions; T=0 → usual ARM instructions. T can be changed via software.

### Thumb and Thumb-2

- **Thumb** (processors with a T in the acronym, e.g. ARM7TDMI): instructions encoded on **16 bits**; less powerful, more of them.
- **Thumb-2** (introduced 2003, supported by latest cores building on the ARM7/v7 architecture): a **superset of Thumb** (backward compatible) that mixes **new 16-bit** and some **32-bit** instructions. Faster than Thumb while still producing very compact code.

### Operating modes and privilege levels

| Concept | Values |
|---|---|
| Operating mode | **thread mode** (on reset or after an exception return) / **handler mode** (when an exception occurs) |
| Access level | **user level** (limited access) / **privileged level** (all resources) |

Handler mode is **always privileged**.

## AMBA Bus System, Clocks, Power

The **AMBA** specification includes 3 buses:

- **AHB** (Advanced High-performance Bus): connects high-performance modules; supports **burst transfers** and **split transactions**; all timing referenced to a single clock edge.
- **ASB** (Advanced System Bus): old specification, substituted by AHB (legacy).
- **APB** (Advanced Peripheral Bus): simpler interface for low-performance peripherals; generally a local secondary bus appearing as a **slave module on the AHB**.

### Clock distribution

Two clocks are needed: **high frequency** for the CPU and high-speed system components, **low frequency** for peripherals requiring less performance or limited speed (I/O). The CPU clock (**CCLK**) and peripheral clock (**PCLK**) get their input from a **PLL** (Phase Lock Loop), a **VPB divider**, or an external source (oscillator/crystal). After RESET, configuring the PLL and VPB divider is the **first thing to do**.

### Power management

Multiple sleep modes:

- **Sleep Now** — Wait for Interrupt/Event instructions;
- **Sleep On Exit** — sleep immediately on return from the last ISR;
- **Deep Sleep** — long-duration sleep so the PLL can be stopped.

The Cortex-M3 system is **clock-gated** in all sleep modes; the sleep signal is exported so external logic can also be gated; the **NVIC** interrupt interface stays awake; a **Wake-Up Interrupt Controller (WIC)** plus an external wake-up detector allows the core to be fully powered down (effective with State-Retention/Power-Gating, SRPG).

## Memory Map

- Very simple **linear 4 GB memory map** (2³² addresses, 32-bit address bus).
- The **Bus Matrix** partitions memory access via the AHB and PPB buses.
- Regions (detailed in [[10-memory-branches-and-subroutines]]): Code, SRAM, Peripherals, External RAM, External device, and the Private Peripheral Bus (internal at 0xE0040000–0xE00FFFFF, external/debug at 0xE0100000–0xFFFFFFFF).
- In the NXP LPC176x/5x not all 4 GB are used: there are "holes" in the memory map.
- The **Interrupt Vector Table** is placed at the bottom of the code region.

## Exception and Interrupt Handling

Exception classes in v7-M: **Reset, NMI, Faults** (Hard Fault, Memory Manage, Bus Fault, Usage Fault), **SVCall, Debug Monitor, PendSV, SysTick**, and **External Interrupts**.

> [!definition] **Interrupt (ARM usage)**
> In the ARM scenario, "interrupt" identifies an exception caused by an **external event**.

- One **Non-Maskable Interrupt (NMI)** is supported.
- A **Nested Vectored Interrupt Controller (NVIC)** is tightly coupled with the core; **1–240 prioritizable interrupts** are supported.

### Interrupt Vector Table (IVT)

> [!definition] **Interrupt Vector Table**
> A data structure associating a list of interrupt handlers with a list of interrupt requests. Two possibilities: the table content is composed of **branch instructions** to the handlers, or the table stores **addresses** of the handlers, loaded into the PC as soon as the exception arises (this is the v7-M case).

| Exception type | Index | Vector address |
|---|---|---|
| (Top of Stack) | 0 | 0x00000000 |
| Reset | 1 | 0x00000004 |
| NMI | 2 | 0x00000008 |
| Hard fault | 3 | 0x0000000C |
| Memory management fault | 4 | 0x00000010 |
| Bus fault | 5 | 0x00000014 |
| Usage fault | 6 | 0x00000018 |
| SVCall | 11 | 0x0000002C |
| Debug monitor | 12 | 0x00000030 |
| PendSV | 14 | 0x00000038 |
| SysTick | 15 | 0x0000003C |
| Interrupts | ≥16 | ≥0x00000040 |

Each line contains an address copied into the PC when the specific exception occurs. The table access is **hardware-based and transparent** to the programmer, but **setting up the IVT at boot time is the programmer's duty**.

## Features of the ARM Instruction Sets

- Instructions are **32 (or 16) bits** long.
- **Every instruction can be conditionally executed**.
- **Load/store architecture**: data processing acts only on registers.
- **Three-operand format**.
- **Combined ALU and shifter**.
- Memory access instructions with **auto-indexing**.
