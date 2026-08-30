---
title: Wireless and Cellular Networks
aliases: [CNTS Lecture 03, GSM, LTE, 5G, IEEE 802.11, WiFi]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Two intertwined topics: (1) wireless fundamentals — network elements (hosts, base stations, links; infrastructure vs. ad-hoc), link impairments (path loss, interference, multipath), SNR/BER tradeoffs, hidden terminal problem; and (2) cellular networks — cells and frequency reuse (cluster size G, capacity/quality tradeoffs, splitting, sectoring, power control), mobility procedures (roaming, location updating, paging, handover and its classifications), the 1G→5G evolution, GSM architecture and FDMA/TDMA physical channels, LTE/EPC architecture with bearers, 5G (SBA, MEC, NR, massive MIMO, beamforming), and IEEE 802.11 WLANs (association, CSMA/CA, RTS/CTS, frame addressing, rate adaptation, power management).

## Wireless vs. Mobility

Two different challenges:

- **Wireless**: communication over a wireless link.
- **Mobility**: handling the mobile user who changes point of attachment to the network.

Context numbers (2019): 10× more wireless phone subscribers than fixed; 5× more mobile-broadband devices than fixed-broadband; 4G/5G networks now embrace the Internet protocol stack, including SDN.

## Elements of a Wireless Network

- **Wireless hosts**: laptop, smartphone, IoT; run applications; may be stationary — *wireless does not always mean mobility*.
- **Base station**: typically connected to the wired network; relays packets between the wired network and wireless hosts in its area (cell towers, 802.11 access points).
- **Wireless link**: connects mobiles to base station, also used as backbone link; a **multiple access protocol** coordinates link access; various rates/distances/frequency bands.
- **Infrastructure mode**: base station connects mobiles into the wired network; **handoff** = mobile changes base station.
- **Ad hoc mode**: no base stations; nodes only reach nodes within link coverage and organize themselves into a network, routing among themselves.

## Wireless Link Characteristics

1. **Decreased signal strength** — radio attenuates as it propagates (path loss).
2. **Interference** from other sources — shared frequencies (e.g., 2.4 GHz used by WiFi, cellular, motors).
3. **Multipath propagation** — signal reflects off objects/ground, arriving at slightly different times.

**SNR and BER:**

- Larger SNR → easier to extract signal from noise.
- Given a physical layer: increase power → SNR up → BER down.
- Given SNR: choose the physical layer that meets the BER requirement with the highest throughput (e.g., BPSK 1 Mbps vs QAM16 4 Mbps vs QAM256 8 Mbps curves).
- SNR changes with mobility → **dynamically adapt** the physical layer (modulation, rate).

**Hidden terminal problem**: A and C cannot hear each other but both reach B — they are unaware of interfering at B. Signal attenuation causes the same effect (A, C mutually out of range).

## Cellular Networks

> [!definition] **Cellular network**
> A network where a geographical area is covered by tessellation through adjacent or overlapping areas called **cells**. User terminals can move from one cell to another without communication disruption (**handover**).

### Coverage

- In theory: base station at the center with isotropic antenna, regular hexagonal cells; or 3 directional antennas with 120° opening at cell corners (sectorization).
- In practice: cells are not regular hexagons nor same size; shape/size depend on: emitted power, antenna height, antenna gain, area morphology (building height/shape), fading, propagation conditions. **Macrocells vs microcells** for different needs (macrocell BS ≈ 20–60 W, microcell BS ≈ 3 W).

### Channel access techniques

Sharing the wireless channel: **FDMA** (frequency), **TDMA** (time), **CDMA** (code), **SDMA** (space).

### Frequency reuse

> [!definition] **Cluster**
> The set of G adjacent cells that together use **all** the frequencies available to the system. Cells reusing the same set (same color in the plan) are **co-channel cells**.

Trade-offs:

| Fixed parameter | Variable | Effect |
|---|---|---|
| G (cluster size) | cell radius R ↓ | capacity ↑ (more channels per area), but more antennas/cost |
| R (cell size) | G ↑ | fewer channels per cell → capacity ↓; larger co-channel distance → less interference → quality ↑ |

Techniques to reduce interference and increase capacity: **splitting** (large cells replaced by groups of small cells — microcells in cities, macrocells in rural areas), **sectoring/shaping** (directional antennas, ad-hoc cell shapes, umbrella coverage, microcells following users, highway/railway coverage), **tilting**, **femtocells**.

### Power control

Goal: reduce interference and energy consumption; open (distance-based) vs. closed (feedback) control. Strategies: uplink closed/open/outer loop; downlink power control. **Open loop (uplink)**: terminal measures downlink quality from the BS and sets its transmit power so the estimated SINR exceeds a threshold — no feedback; inaccurate because UL/DL use different channels; closed loop is more accurate but more complex.

### Channel allocation

- **FCA** (Fixed): frequencies statically assigned per the cluster plan; changed rarely.
- **DCA** (Dynamic): central controller assigns resources to cells as needed; the frequency plan adapts over time.
- **HCS** (Hybrid): part static (FCA), part dynamic (DCA).

### Network architecture and procedures

Architecture: BS (base station) ↔ S/R (switch/router) ↔ core network with databases, toward other networks; MT = mobile terminal.

- **Registration**: MT connects to the network and identifies/authenticates itself — at switch-on, at service access, periodically.
- **Mobility management**: roaming, location updating, paging, handover.
  - The area is divided into **Location Areas (LAs)**, each with an **LAI**; the user position is stored in databases.
  - **Location updating**: a control channel broadcasts the LAI in each cell; if the MT hears a new LAI, it updates its position in the database.
  - **Paging**: the system broadcasts a paging message within the LA where the user is, to notify an incoming call/data delivery.
  - **Handover**: transfers an active connection between cells while the terminal moves.

> [!definition] **Handover classifications**
> - **Intra vs. inter-cell**: between frequencies in the same cell or between cells.
> - **Soft vs. hard**: both radio channels active during handover (soft) or only one at a time (hard).
> - **MT- vs. BS-initiated**: which entity sends the first control message / performs the measurements.
> - **Backward vs. forward**: signaling via the origin BS or the destination BS.

### Generations

| Generation | Era | Technology | Key traits |
|---|---|---|---|
| 1G | ~1980s | TACS | Analog, FDMA, voice only, low quality; 900 MHz, 25 kHz channels (TIM shut down 31/12/05) |
| 2G | 1990s | GSM (EU), IS-95 (US), PDC (JP) | Digital; SMS, cryptography, advanced voice coding; FDMA/TDMA in EU, CDMA in US; bands 850/900/1800/1900 MHz |
| 2.5G | — | GPRS/EDGE, IS-95B | Packet-switched data (170 kb/s GPRS, 384 kb/s EDGE), traffic-based billing |
| 3G | 2000s | UMTS (EU/JP), CDMA2000 (US) | CDMA, spatial diversity, up to 2 Mb/s, vertical handover 2G–3G |
| 3.5G | — | HSPA/HSPA+ | physical-layer evolution of UMTS; up to 56 Mb/s DL, 22 Mb/s UL |
| 4G | 2010s | LTE / LTE-Advanced | up to ~250–300 Mb/s DL, 50 UL (64QAM); MIMO; all-IP; VoLTE. Strictly "4G" = IMT-Advanced (1 Gb/s fixed, 100 Mb/s mobile) — LTE was "3.9G"; LTE-Advanced (2011) meets it |
| 5G | 2020s | NR (New Radio) | integrates LTE-A evolution, WiFi, mmWave; SDN control; NFV service implementation |

## GSM — the 2nd Generation

Still viable for voice; running GSM calls frees frequencies for 4G/5G data. Services: voice full rate 13 kbit/s / half rate 6.5 kbit/s, SMS, supplementary services.

### Architecture

- **Mobile Station (MS)** = MT + **SIM** (smart card with processor/memory storing encrypted phone number, services, security parameters). Transmit power: ≤ 2 W phones, ≤ 8 W mobile devices, ≤ 20 W vehicle antennas.
- **Base Station Subsystem (BSS)**:
  - **BTS** (Base Transceiver Station): physical radio interface, access point for MTs; transmits only toward active users (unlike radio/TV); up to 32 FDM channels per BTS (half UL, half DL).
  - **BSC** (Base Station Controller): controls resources on the radio interface; controls tens-to-hundreds of BTSs (collocated with an MSC). Functions: transcoding 13↔64 kb/s, paging, dynamic frequency assignment to BTSs, signal quality measurement, handover between BTSs of the same BSC.
- **Network and Switching Subsystem (NSS)** — main functions: call handling, service support, mobility support, authentication:
  - **MSC** (Mobile Switching Center): call routing between MTs; **GMSC** interfaces other networks.
  - **HLR** (Home Location Register): database of permanent user data (id, services, security parameters) + dynamic mobility data (e.g., current VLR identifier).
  - **VLR** (Visitor Location Register): data on MTs currently in the MSC area — IDs, on/off status, LAI, routing info.
  - **AuC** (Authentication Center): challenge-response authentication; generation of over-the-air encryption keys.
  - **EIR** (Equipment Identity Register): database of stolen devices.

### Physical channels

- Access: **FDMA/TDMA**. Spectrum divided into **200 kHz FDM carriers**; each carrier divided into **TDM frames of 8 slots**; **frequency + time slot = physical channel**.
- Slot = **0.577 ms** carrying 156.25 bits; frame = 8 slots = **4.615 ms**; transmission rate **271 kbit/s**. Data transmitted in **bursts** (blocks on a physical channel) — "similar to packets, but still circuit switching": a full-rate (13 kb/s) call gets 1 slot per frame UL + 1 DL.
- UL and DL frames are slot-synchronized and **shifted by 3 slots**; FDD duplexing (e.g., GSM-900: UL 890–915 MHz, DL 935–960 MHz; DCS-1800 bands too). Separating UL/DL in frequency *and* time allows **one transceiver** in the MT.
- **Timing advance**: non-zero propagation could make bursts arrive when the slot is over (collisions); the MT starts transmitting *before* the nominal slot start, as instructed by the BTS.
- **Regular burst** (148 bits + 8.25 guard): tail 3 | **training sequence 26** (sync/equalization) | data 57 | stealing flag | data 57 | stealing flag | tail 3. Coded data: 114 bits per burst → 13 kb/s voice, ≤9.6 kb/s data.
- **Logical channels** carry *what* is transmitted (control vs. traffic) and are mapped onto the 8 physical channels per carrier (control: network signaling, cell parameters, synchronization, receiver tuning, call control, measurement delivery).

## LTE — the 4th Generation

3GPP Release 8 (LTE-Advanced from Rel. 10): up to 300 Mb/s DL – 50 Mb/s UL with up to 64 QAM, 10 ms data latency, **OFDMA** (DL) and **SC-FDM** (UL) replacing WCDMA, high-order **MIMO**, channels up to 20 MHz (scalable 1.4–20 MHz), **all-IP** core.

Terminology: downlink/uplink; **user plane** (user data transport) vs. **control plane** (set-up, control, maintenance); Access Stratum / Non-Access Stratum.

### Architecture

- **RAN = E-UTRAN**: base stations (**eNodeB**) interconnected by the **X2** interface (control + user); functions: radio resource management (bearer control, mobility, scheduling), header compression, security, connectivity to EPC.
- **Core = EPC**, clean-slate, packet-switched for all QoS classes. Components:
  - **MME** (Mobility Management Entity) — control plane: UE context, identity, authentication, authorization; NAS procedures (bearer management, connection/mobility management).
  - **S-GW** (Serving Gateway) — user plane: packets between eNodeB and core; routing/forwarding in EPC; anchor for intra-LTE mobility; lawful intercept.
  - **P-GW** (PDN Gateway) — user plane: connects EPC to external networks/Internet; UE IP address assignment, per-user packet filtering, NAT; anchor for non-3GPP mobility; lawful intercept.
  - **HSS** (Home Subscriber Server): user/subscriber database for authentication (with MME) and authorization — similar to GSM HLR.

> [!definition] **LTE bearer**
> A **pipe (tunnel)** carrying data from the UE to LTE elements (e.g., the P-GW). A **default bearer** to the P-GW is established whenever the UE activates; **dedicated bearers** (possibly with Guaranteed Bit Rate) are added per QoS need (e.g., video streaming).

The bearer is a **concatenated tunnel** of three portions, established in this order: **S5/S8 bearer** (S-GW ↔ P-GW; S8 when extending toward the Internet), **S1 bearer** (eNodeB ↔ S-GW; handover sets a new S1 bearer), **radio bearer** (UE ↔ eNodeB; follows the user under MME direction during handovers).

### Data plane details

- **GTP tunneling**: mobile datagrams are encapsulated with **GTP (GPRS Tunneling Protocol) over UDP** from the base station to the S-GW, re-tunneled to the P-GW. Supporting mobility: **only the tunnel endpoints change** when the user moves.
- First-hop protocol stack: PDCP (header compression, encryption) / RLC (fragmentation-reassembly, reliable transfer) / MAC (requesting/using radio slots) / Physical. Downstream uses OFDM (FDM+TDM, orthogonal subchannels); each active mobile gets two or more 0.5 ms slots over 12 frequencies; scheduling not standardized (operator choice); 100s Mbps per device possible.
- **Physical channels**: DL — PBCH (MIB, RACH parameters), PCFICH (PDCCH format), PHICH (HARQ ACK/NACK), PDCCH (DL scheduling, UL grants, power control, paging), PDSCH (user data, SIBs). UL control — PRACH (random access preamble, timing sync), PUCCH (HARQ ACK/NACK, CQI, MIMO feedback, scheduling requests). UL data — PUSCH (1 ms TTI; TTI bundling of 4 at cell edge).
- **Association**: BS broadcasts primary sync signal every 5 ms on all frequencies → mobile finds it, reads secondary sync and configuration/carrier info → selects BS (e.g., home carrier) → then authentication, state set-up, data plane.
- **Sleep modes**: light sleep after 100s ms of inactivity (wake periodically for downstream); deep sleep after 5–10 s (may change cells → re-association needed).

## 5G

- Objectives: highly mobile fully connected society — IoT proliferation, vertical-market automation (energy, e-health, smart city, connected cars, manufacturing), human-centric apps (VR/AR, 4K streaming). Coexistence of human-centric and machine-type applications with very diverse KPIs.
- Unlike previous generations: requires integration of **massive computing and storage infrastructures**; operators deploy **orchestrators** allocating computing/logical network resources to services — logical networks called **network slices**, with cross-domain orchestration over multiple administrative domains.
- **Use cases**: eMBB (enhanced Mobile Broadband), mMTC (massive Machine Type Communication), URLLC (Ultra-Reliable Low-Latency Communication).
- **Radio access (NR)**: flexible slot-based framework (variable slots/subframe, transmissions can start mid-slot), different subcarrier spacings ("numerology"), slot aggregation; alternatives to pure OFDM (RBF-OFDM, FBMC, GFDM, UFMC); co-located **massive MIMO** (antennas outnumber users); **mmWave** (20–80 GHz, up to 2 GHz contiguous bandwidth); **beamforming** — LTE applies it to data only, 5G also to control; high frequencies use it for range, mid/low bands as part of MIMO; distributed-MIMO planned.
- **Core (5GC)**: SDN (centralized control decoupled from distributed data plane) + NFV (network services as software building blocks, NFV chaining). Differences vs 4G: E-UTRA→**NR**, E-UTRAN→5G-RAN (base stations = **gNodeB**), EPC→5GC; **full CP/UP split** (LTE had mixed functions); network slicing; **flow-based QoS** instead of end-to-end bearers; **Service Based Architecture**:
  - **AMF** — Access and Mobility Management Function (5G evolution of MME);
  - **SMF** — Session Management Function (evolution of S-GW/P-GW control plane);
  - **UPF** — User Plane Function (evolution of the data plane; performs traffic steering).
- **Edge = MEC** (Multiaccess Edge Computing): IT/cloud environment at the network edge near subscribers (standardization since 2014, first specs 2017). Benefits: ultra-low latency, high bandwidth, real-time radio-network context, location awareness, flexible service framework. The **MEC host** contains the MEC platform + virtualization infrastructure (compute/storage/network) for **MEC applications** (VMs), which discover/advertise/consume/offer MEC services through the platform.

## Wireless LANs — IEEE 802.11

| Standard | Year | Max rate | Range | Band |
|---|---|---|---|---|
| 802.11b | 1999 | 11 Mbps | 30 m | 2.4 GHz |
| 802.11g | 2003 | 54 Mbps | 30 m | 2.4 GHz |
| 802.11n (WiFi 4) | 2009 | 600 Mbps | 70 m | 2.4/5 GHz |
| 802.11ac (WiFi 5) | 2013 | 3.47 Gbps | 70 m | 5 GHz |
| 802.11ax (WiFi 6) | 2021 | 14 Gbps | 70 m | 2.4/5 GHz |
| 802.11af | 2014 | 35–560 Mbps | 1 km | unused TV bands (54–790 MHz) |
| 802.11ah | 2017 | 347 Mbps | 1 km | 900 MHz |

All use **CSMA/CA**; all have base-station and ad-hoc variants.

- **Architecture**: host communicates with an **access point (AP)**; **Basic Service Set (BSS)** = cell (hosts + AP) in infrastructure mode; ad hoc mode: hosts only.
- **Channels/association**: spectrum divided into channels chosen by the AP admin (interference possible with neighbors); arriving host **scans** for **beacon frames** (SSID + MAC), selects an AP, may authenticate, then typically runs DHCP in the AP's subnet.
- **Scanning**: *passive* (listen for beacons, then Association Request/Response) or *active* (broadcast Probe Request, receive Probe Responses, then associate).
- **Mobility within the same subnet**: IP address stays; the switch re-learns which port reaches H1 (self-learning).

### CSMA/CA (802.11 MAC)

- Sense before transmitting; **no collision detection** — a wireless node cannot sense collisions (own transmitted signal overwhelms reception; hidden terminals, fading). Goal: **avoid** collisions.
- Sender: if idle for **DIFS**, transmit the entire frame (no CD); receiver returns **ACK after SIFS** (needed because of the hidden terminal problem). If busy: start a **random backoff timer**, count down while idle, transmit on expiry; if no ACK, increase the backoff interval and repeat.
- **RTS/CTS reservation**: sender transmits a small **RTS** (using CSMA; RTSs may collide but are short); the AP broadcasts **CTS** heard by **all** nodes; sender transmits data; other stations defer. Resolves hidden terminals (A and C both send RTS; collision only hits the short RTSs; C hears CTS(A) and defers).

### 802.11 frame addressing

Four address fields: **Address 1** = receiver (wireless host or AP); **Address 2** = transmitter (host or AP); **Address 3** = MAC of the router interface the AP is attached to (e.g., R1); **Address 4** = only in ad hoc mode. Frame control carries type (RTS/CTS/ACK/data), From/To AP, retry, power management, more data/frag flags; Duration field carries the reserved transmission time (RTS/CTS).

### Advanced capabilities

- **Rate adaptation**: as the mobile moves and SNR varies, base station and mobile change the physical-layer modulation — SNR decreases → BER grows → switch to a lower-rate, more robust modulation.
- **Power management**: node tells the AP "sleeping until next beacon"; AP buffers frames; node wakes before the beacon, which lists mobiles with waiting frames; node stays awake if frames are pending, else sleeps again.
