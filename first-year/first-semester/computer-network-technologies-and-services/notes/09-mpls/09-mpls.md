---
title: MPLS — Multi-Protocol Label Switching
aliases: [CNTS Lecture 07, MPLS, LSP, LDP, RSVP-TE, Traffic Engineering]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> MPLS architecture: forwarding by label instead of longest-prefix matching (faster TCAM-free lookup, traffic engineering, connection-oriented paradigm without its drawbacks), the shim header (20-bit label, Exp, S, TTL), LSR/LER and Label Switched Paths. The control plane: FECs, the three key actions (label binding, mapping, distribution), static vs. dynamic (data- vs. control-driven) binding, the three label-distribution protocols (LDP, RSVP-TE, BGP), routing modes (hop-by-hop vs. explicit/constraint-based). Applications: traffic engineering (why pure IP routing oscillates), CoS/QoS, fast fault recovery (<50 ms), label-stack hierarchy, Penultimate Hop Popping, and the MPLS VPN enabler. History from Tag Switching to GMPLS.

## The Idea

> [!definition] **MPLS**
> Instead of the IP destination address (longest prefix matching), packets are forwarded according to a **label** prepended to them. The label is used as an index → **faster lookup**; labels also enable **traffic engineering**.

Context (from the slides): MPLS is "the enabling technology for the New Broadband (IP) Public Network". The pre-MPLS transport "onion" (IP over ATM, Frame Relay, SONET/SDH, DWDM, circuit emulation...) collapses into a single **WDM + IP + MPLS** stack.

MPLS introduces a **connection-oriented paradigm in IP networks**, while avoiding the classic drawbacks:

1. a connection must exist before transmission;
2. no immediate way to recover from a failure.

## Network Architecture

- **LSR** — Label Switch Router: core device that switches labeled packets.
- **LER** — Label Edge Router (ingress/egress LSR): classifies IP packets into FECs, imposes labels at ingress, removes them at egress. MPLS runs only in the backbone — end devices are unaware of it.
- **LSP** — Label Switched Path (a.k.a. **LSP tunnel**): the path followed by packets of one FEC through the MPLS cloud.

**Label switching**: each LSR keeps a forwarding table mapping `input label → (output label, output port)`. The label changes at every hop (swap), so labels have only **local significance** — no global label coordination is needed, only agreement between neighbors.

## The Shim Header

| Layer 2 header | MPLS header | IP packet |
|---|---|---|

MPLS "header" (shim, 32 bits): **Label (20 bit) | Exp (3 bit, experimental/CoS) | S (1 bit, bottom-of-stack) | TTL (8 bit)**.

On connection-oriented layer-2 technologies the label can live in the L2 header itself: **VCI/VPI** (ATM), **DLCI** (Frame Relay).

## Control Plane: FEC, Binding, Mapping, Distribution

> [!definition] **FEC — Forwarding Equivalence Class**
> A class of packets that are **treated the same way by each LSR**, **follow the same path** through the MPLS network, and therefore **receive the same label**.

Three key actions taken by LSRs:

1. **Label binding** — an LSR determines the label to prepend to packets of a given FEC. With **downstream binding**, the LSR at the *receiving end* of a link chooses the label with which it wants to receive that FEC's packets and notifies the upstream node (unsolicited or on-demand).
2. **Label mapping** — association between **input label, output label, and next hop** (input label chosen by the local LSR, output label chosen by the downstream LSR, next hop from routing). This is the actual creation of an LSP.
3. **Label distribution** — notification of the chosen label, following the binding; to neighboring LSRs (or at least the upstream one).

### Static vs. dynamic binding

- **Static** (through management): non-scalable, equivalent to ATM PVCs, no interoperability among management systems, LSPs cannot cross network boundaries.
- **Dynamic**:
  - *data/traffic driven* — triggered by data packets;
  - *control driven* — triggered by control messages (signaling or routing). **Topology based**: LSP creation tied to route discovery toward destinations; explicit signaling, initiated by label edge routers; on-demand or unsolicited.

### Label distribution protocols (three incompatible alternatives)

| Protocol | Nature |
|---|---|
| **LDP** (Label Distribution Protocol) | designed for the purpose (distance-vector flavored) |
| **RSVP** (Resource reSerVation Protocol) | designed for allocation in integrated-service networks |
| **BGP** | routing protocol; only topology based |

### Routing protocols in the MPLS context

Existing protocols (IS-IS, OSPF, BGP-4) carry topology and **determine LSP routing**, impacting the label-mapping phase. In MPLS they are **enhanced** to also carry **constraint data**: link capacity, link utilization, dependencies among links — used for fault recovery and constraint-based routing:

- **OSPF-TE / IS-IS-TE**: constraint-based routing is fundamental to support **traffic engineering**.
- Routing modes:
  - **Hop-by-hop routing**: each LSR decides the next LSR (e.g., from the IP routing table) — same route as traditional IP routing;
  - **Explicit routing**: a **single switch** (e.g., the ingress LSR) chooses the **whole LSP path**; the choice is **constraint based**. Constraint-based routing cannot be distributed: no unique route-selection criteria, conflicting constraints, and constraint info changes more frequently than topology — hard to keep synchronized.

Label distribution protocols for explicit routing: **CR-LDP** (Constraint-based Routing LDP) and **RSVP-TE** (RSVP for Traffic Engineering), to be used with OSPF-TE/IS-IS-TE.

## Traffic Engineering

Traditional IP routing sends all traffic for destination D along the **(single) optimal path** — with heavy traffic this overloads some links (congestion) while leaving others underutilized. All traffic would have to move together to the unused links — impossible to split with plain routing tables.

Could IP routing choose paths according to load? Yes, but then: routing tables change → load shifts → tables change again → **instability** (oscillation).

> [!definition] **Traffic engineering with MPLS**
> LSPs let the operator **distribute traffic toward the same destination over multiple paths** (some LSPs on the non-optimal route), eliminating congestion and using links uniformly. Even if the routing table changes, **the forwarding table does not** — established LSPs keep carrying traffic stably. MPLS enables per-class traffic engineering, guaranteed QoS, and fast fault recovery.

Traffic engineering without MPLS historically used **ATM** under IP: two control planes, ATM-unaware routers, a great number of adjacencies, limited scalability. MPLS gives **one control plane operating on the physical topology** — simpler, more scalable, IP-aware.

## CoS and QoS

Explicit support is required in the LSR data plane and control plane; resources and service modes may be associated to a **FEC at LSP setup**.

- **CoS (Class of Service)**: relative priority among FECs (no absolute guarantees); supports the **DiffServ** model with per-hop behaviors (**EF** expedited forwarding, **AF** assured forwarding); per-class ("DS-aware") traffic engineering.
- **QoS (Quality of Service)**: specific guarantees on **bandwidth, delay, burst size**.

Marketing message: a **unified network supporting all types of services** — QoS/real-time services over IP were not ready, and multi-service networks ran "ships in the night" (ATM control plane for ATM services, MPLS control plane for IP services).

## Fast Fault Recovery

Recovery in **less than 50 ms** (not yet supported in early deployments):

- **Link protection / link re-routing**: a re-routing node locally detours the LSP around a failed link.
- **Edge-to-edge re-routing**: a **backup/protection LSP** is pre-established end-to-end; on failure the edge switches onto it.

## Label Stack, Hierarchy, Scalability

A packet can carry a **stack of labels** (S bit marks the bottom): crossing nested MPLS domains, an outer label per domain is pushed/popped. Benefits: **routing table and forwarding table reduction** in transit routers — hierarchy like tunneling LSPs between edge routers; also **simpler and faster exact label matching** vs. longest prefix matching.

## Penultimate Hop Popping (PHP)

- The **last-but-one node** on the LSP pops the label, so the egress **LER** receives a plain packet and routes it on the IP address (or the next label in the stack) — saves the egress one lookup.
- Distribution of **label 3** indicates (implicit) PHP; **explicit PHP** swaps label 0 (used when the shim header is still needed, e.g., for the Exp bits).

## MPLS and VPNs

MPLS is the enabler for scalable provider-provisioned VPNs (services similar to a private network on a public IP infrastructure, with privacy, overlapping private address spaces, CoS/QoS):

- Labels **hide user IP addresses** on the public network and allow **overlapping address spaces** (the label/FEC identifies the VPN);
- Other approaches need **manually configured tunnels between each pair of sites** (number of VPNs × number of members); MPLS scales instead (details in note 10).

## History and Extensions

- **Tag Switching** (Cisco) → merged with IBM ARIS ideas into MPLS: motivations included IP over ATM without address-resolution problems, simpler signaling, one control plane, reuse of ATM switching hardware.
- **MPλS** (Multi-Protocol Lambda Switching): MPLS control plane in **optical** networks.
- **GMPLS** (Generalized MPLS): one MPLS control plane for any switching technology — packet, cell, circuit (SONET/SDH), lambda, "anything switching".

## Standardization and References

- IETF MPLS Working Group; FR/MPLS Alliance (vendor consortium: deployment, aspects omitted by IETF such as VoMPLS, ADSL).
- Key RFCs (from the slide bibliography): RFC 3031 (MPLS Architecture), RFC 3032 (Label Stack Encoding), RFC 3270 (MPLS Support of Differentiated Services), RFC 3035 (MPLS using LDP and ATM VC Switching), RFC 3036 (LDP Specification).
