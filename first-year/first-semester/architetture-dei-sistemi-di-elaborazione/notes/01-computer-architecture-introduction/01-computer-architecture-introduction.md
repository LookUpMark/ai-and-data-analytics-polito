---
title: Computer Architecture Introduction
aliases: [Computer Design Introduction, ASE Lecture 01]
tags: [computer-science/computer-architecture, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> First lecture of the course: evolution of computers and microprocessors, the classes of the computer market (from PMDs to embedded systems), the parallelism levels exploited by modern architectures, and the three aspects of computer design (ISA, organization, hardware). Introduces the quantitative foundations of the discipline: Moore's law, cost and yield, power and energy, dependability metrics (MTTF/MTBF/MTTR), performance measurement (response time vs. throughput), benchmarks, Amdahl's law, and the CPU performance equation.

## Computer Evolution and Performance Growth

The first general-purpose computer was created in the late 1940s. A PC bought today for about $500 is practically equivalent, in performance and memory, to what cost about $1M in 1985. This evolution was enabled by three factors:

- advances in semiconductor technology;
- innovations in computer design;
- advances in software.

### Chip-design history

| Year | ~1970 | 1980 | 1990 | 2000 | 2015 | Today |
|---|---|---|---|---|---|---|
| # transistors | 2,000 | 30,000 | 1,000,000 | 40,000,000 | >10,000,000,000 | HUGE |
| Design method | Design entry | Design Rule Checking | Logic Synthesis | P&R, Timing closure | IP-Based Design | Design platform / AI-Based Design |

The annual size of the global *Datasphere* (source: IDC Datasphere whitepaper) is measured in zettabytes: 1 ZB = 10²¹ bytes = 2⁷⁰.

### Microprocessor performance growth

- **1970s**: annual performance increase of about **25–30%**, when mainframes and minicomputers dominated.
- **1980s**: the annual increase rose to **more than 50%** with RISC architectures.
- **From 2002**: the yearly increase dropped to about **20%** due to power issues, lower instruction-level parallelism, and unchanged memory latency.
- **Since 2004**: major industries cancelled the race for higher-frequency single processors and embraced **multicore CPUs**.
- Afterwards: **12%** improvement per year (doubling every 8 years), mainly due to limits on instruction-level parallelism; then **3.5%** per year (doubling every 20 years) — "is this the end of Moore's law?"

This growth comes from improvements in technology, microprocessor architecture, and software development. Historical examples shown in the slides: the Intel 8086 (Bus Interface Unit + Instruction Queue + Execution Unit on a system bus) and the Core 2 generation. The major players (Intel, AMD, IBM, ARM) no longer invest in faster single processors but in **multiprocessor single-chip systems (multicore devices)**.

## The Computer Market

The market is currently split in 5 areas:

| Area | Emphasis | System price | Microprocessor price |
|---|---|---|---|
| Personal Mobile Device (PMD) | energy efficiency, real-time applications | $100 – $1,000 | $10 – $100 |
| Desktop computing | price-performance ratio | $300 – $2,500 | $50 – $500 |
| Servers | availability, scalability, throughput | $5,000 – $10,000,000 | $200 – $2,000 |
| Clusters / Warehouse-Scale Computers (WSC) | availability, price-performance, power consumption | $100,000 – $200,000,000 | $50 – $250 |
| Embedded computers | real-time, memory/power minimization, reliability | $10 – $100,000 | $0.01 – $100 |

- **Servers** provide larger-scale and more reliable computing services.
- **WSC** target "Software/Platform as a Service" (SaaS/PaaS); supercomputers emphasize floating-point performance and fast internal networks.
- **Embedded computers** are the fastest-growing portion of the market: they cover all special-purpose computer-based applications (from microwaves to automotive), use processors from cheap 8-bit to high-end ones, and cannot run third-party software. Typical solutions: standard processor + custom logic + custom SW, standard processor + custom SW, standard DSP + custom SW; FPGAs play a growing role.

## Classes of Parallelism and Parallel Architectures

> [!definition] **Data-level Parallelism (DLP)**
> Many data items can be operated on at the same time.

> [!definition] **Task-level Parallelism (TaskLP)**
> Different tasks of a work can operate independently.

Parallel architectures exploit them as follows:

| Architecture | Parallelism exploited |
|---|---|
| Instruction-level Parallelism (ILP) | modestly exploits DLP |
| Vector architectures and GPUs | exploits DLP |
| Thread-level Parallelism (TLP) | exploits DLP and TaskLP |
| Request-level Parallelism (RLP) | parallelism among decoupled tasks |

## Computer Architecture, Cost, and Power

### What "designing a computer" means

Designing a computer means determining which attributes are important for the new machine and designing a machine that **maximizes performance** while matching **cost and power constraints**. Computer architecture includes three aspects of computer design:

1. **Instruction set architecture**
2. **Organization**
3. **Hardware**

The computer architect must meet: functional requirements, price, power, performance, dependability. In 2001 the difference between the highest-performance microprocessors and what technology alone would have delivered was estimated at **more than a factor of 15** (architectural innovation + software).

> [!definition] **Moore's Law**
> The number of devices (transistors) that can be integrated into a single chip **doubles every 18/24 months**.

### IC manufacturing cost

When evaluating cost, the impact of **yield** is crucial: the percentage of products that pass the test phase. The production process undergoes a *learning curve*: when yield increases, cost decreases. **More than 50% of manufacturing cost is due to validation and testing procedures.**

### Power and energy

- **Dynamic power** (dominant until recently), consumed by each transistor when switching:
  `Power_dynamic = ½ × capacitive load × voltage² × frequency`
- **Static power**: `Power_static = V × I`, accounting for 25–50% of total power consumption.
- For this reason, voltage has continuously dropped over the years.
- **Energy** (mainly of interest for portable/mobile devices): `Energy_dynamic = capacitive load × voltage²`

## Dependability

> [!definition] **Dependability**
> The quality of a system to deliver a correct service. It can be lowered by bugs in the hardware design, bugs in the software, defects introduced by the manufacturing process, and faults happening during product operation.

Historically critical in space, avionics and nuclear plant control, dependability now matters in an expanded set of safety-critical areas: railroad traffic control, automotive, biomedical, telecommunications.

### Dependability metrics

- **MTTF** — Mean Time To Failure; its reciprocal is **FIT** (Failures In Time): 1 FIT = 1 failure in one billion hours.
- **MTBF** — Mean Time Between Failures.
- **MTTR** — Mean Time To Repair.

They are related by:

```
MTBF = MTTF + MTTR
```

**Availability** is the probability that a system works correctly at a generic time instant.

## Computer Performance

> [!definition] **Performance**
> - **User point of view**: performance = *response time* (time between start and completion of an operation).
> - **System manager point of view**: performance = *throughput* (total amount of work done per time unit).

### Which time to measure

- **Elapsed time** (includes everything: CPU, I/O, OS, idle time)
- **CPU time**, split into **user CPU time** and **system CPU time**.

UNIX provides all of them through the `time` command, e.g. `90.7s 12.9s 2:39 65%` (user time, system time, elapsed time, CPU time / elapsed time).

### Benchmarks

Performance evaluation is done by executing applications and observing behavior, but the choice of applications severely affects results. Possible benchmarks:

- **Real programs** (C compilers, text processors, special-purpose tools), possibly modified;
- **Kernels** (e.g., Livermore Loops, Linpack);
- **Toy benchmarks** (e.g., Quicksort, Sieve of Eratosthenes);
- **Synthetic benchmarks** (e.g., Whetstone, Dhrystone).

**Benchmark suites** (e.g., SPEC — Standard Performance Evaluation Corporation; MiBench for embedded) mix kernels, program fragments and applications so that the weakness of any component is lessened by the presence of the others. Reported results should allow **reproducibility**: detailed information about hardware (system configuration), software (OS, compiler, program), and program input.

### Summarizing performance

- **Total execution time** (arithmetic mean): `Σ Time_i / n` over all benchmarks.
- **Normalized execution time**: a reference machine is adopted (e.g., the VAX-11/780) and times are normalized to it.
- **Weighted arithmetic mean**: `Σ (Weight_i × Time_i)` — the suggested solution is to measure a real workload and weight programs by their frequency of execution, with carefully specified program inputs.

## Guidelines and Principles for Computer Design

### Amdahl's Law

> [!definition] **Amdahl's Law**
> The speedup resulting from an enhancement depends on:
> - `fraction_enhanced` — the fraction of the computation time that takes advantage of the enhancement;
> - `speedup_enhanced` — the improvement on the parts it affects.
>
> `Execution_time_new = Execution_time_old × [(1 − fraction_enhanced) + fraction_enhanced / speedup_enhanced]`
>
> `speedup_overall = 1 / [(1 − fraction_enhanced) + fraction_enhanced / speedup_enhanced]`

**Example**: an enhancement makes a machine 10× faster for 40% of the programs:

```
speedup_overall = 1 / (0.6 + 0.4/10) = 1 / 0.64 ≈ 1.56
```

**Choosing between two solutions** (floating-point performance):

- Solution 1: make square root 10× faster (square root = 20% of execution time): `speedup = 1/(0.8 + 0.2/10) = 1.22`
- Solution 2: make all FP operations 2× faster (FP = 50% of execution time): `speedup = 1/(0.5 + 0.5/2) = 1.33`

Solution 2 wins.

### The CPU Performance Equation

> [!definition] **CPU Performance Equation**
> `CPU_time = ( Σ_i CPI_i × IC_i ) × Clock_cycle_time`
> where `CPI_i` = clock cycles required by instruction *i*, `IC_i` = number of times instruction *i* is executed, and clock cycle time is the inverse of clock frequency.

What each factor depends on:

| Factor | Depends on |
|---|---|
| `CPI_i` | technology and hardware organization |
| `IC_i` | instruction set architecture and compiler |
| clock cycle time | technology and organization |

**Limitation**: in pipelined processors, `CPI_i` may vary for a given instruction depending on the instructions executed before and after it, and on memory system behavior (e.g., cache hit or miss). Evaluating execution time analytically therefore becomes much harder.

Execution time can be measured by observing the real system, by simulation, or by applying the CPU performance equation.
