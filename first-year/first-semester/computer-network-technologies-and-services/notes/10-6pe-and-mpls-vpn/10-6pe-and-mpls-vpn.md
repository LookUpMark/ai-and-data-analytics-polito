---
title: 6PE and MPLS VPN
aliases: [CNTS Lecture 08, 6PE, MPLS VPN, VRF, Route Distinguisher, RFC 2547bis, RFC 4798]
tags: [computer-science/networking, note/technology, level/intermediate]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> How MPLS carries VPN traffic: the peer model versus access/overlay VPNs, pseudo-wire emulation with two-level label stacking, and provider-provisioned Layer 3 VPNs — the RFC 2547bis BGP solution (VRF tables, VPN-IPv4 addresses, MP-BGP, PHP) versus the Virtual Router alternative. Then 6PE (RFC 4798): IPv6 islands over an **unchanged IPv4 MPLS core**, with dual-stack PEs, MP-BGP over IPv4 with IPv4-mapped next hops, and the same two-label scheme as MPLS VPNs.

## VPN Models: Access, Overlay, Peer

> [!definition] **VPN (Virtual Private Network)**
> Services similar to those of a private network, but provided on a public IP infrastructure: privacy and security, **overlapping private address spaces** (non-unique addresses), CoS/QoS support. MPLS labels "hide" user IP addresses on the public network and allow overlapping address spaces (see note 09).

The lecture classifies VPN approaches (slide "Multi-Protocol Support"):

| Model | Technology | Key idea |
|---|---|---|
| **Access VPN** | L2TP, PPTP | transparent remote access |
| **Overlay** (IPsec based) | IPsec; **GRE** (Generic Routing Encapsulation, RFC 1702) | tunnels that can transport any layer 3 protocol within IP |
| **Peer** (MPLS based) | built into MPLS | customer routing is exchanged *with the provider* ("peers"), not tunneled across it |

Why peer wins at scale (recap from lecture 07): overlay approaches require **explicit, manual setup of tunnels between each pair of sites** — the number of tunnels grows with the number of VPNs × number of VPN members. MPLS-based VPNs are the **scalable** alternative.

## Layer 2 First: Pseudo Wire Emulation

Before L3 VPNs, the slides present **Pseudo Wire Emulation End-to-End**: several services on the **same** network — IP, but also leased lines, Frame Relay, ATM, Ethernet.

- The **Customer Edge (CE)** device exposes the **native service interface** (e.g., an Ethernet or ATM port).
- Traffic is carried through an **LSP between CEs**.
- **Two labels**:
  - **External** — for routing within the provider network; identifies the **access point** to the network;
  - **Internal** — **multiplexing of several users/services at the same access point**.
- Aggregation devices may exist inside the network (e.g., an ATM switch switching traffic between users); the LSP ends on that device. Setup here is **mainly manual**; proposals existed for deploying LDP and BGP.

This two-label pattern (outer = transport through the core, inner = who exactly inside the edge device) reappears in both L3 VPNs and 6PE.

## MPLS-Based Layer 3 VPNs

**Provider-provisioned** solutions:

- VPN policies are **implemented by the Service Provider** — no experience needed on the customer side;
- **Scalability**: large-scale deployments.

Two alternative solutions:

| Solution | Initially supported by | Note |
|---|---|---|
| **RFC 2547bis** (BGP) | Cisco Systems | currently the **most widely deployed** approach |
| **Virtual Router** | Nortel, Lucent | PE runs one router instance per VPN |

### Architecture and roles

| Role | What it does |
|---|---|
| **CE** (Customer Edge) | creates a **routing adjacency with its PE**: advertises its destinations and receives advertisements of other VPN destinations. Uses static routing or an IGP (e.g., OSPF, RIP) |
| **PE** (Provider Edge) | keeps **VRFs**, exchanges VPN routing with the other PEs, sets up LSPs; **keeps routes only for the VPNs connected to it** |
| **P** (Provider core) | **have routes to PE routers only** — no VPN routes, no customer state |

- PE-to-PE transport: **PEs set up LSPs among themselves** using LDP and/or RSVP (and/or I-BGP) — e.g., **topology-based label binding**.
- PE-to-PE routing exchange: **I-BGP** in the BGP-based solution, **IGP** in the Virtual Router solution.

> [!definition] **VRF — VPN Routing and Forwarding table**
> A forwarding table **associated to one or more (non-MPLS) ports** of a PE router; it holds the forwarding information to be used for traffic **received through those ports**. In the lecture example, PE2 looks up the destination in the "**cyan**" VRF of the ingress port — a different VPN's packets would hit a different VRF.

### BGP/MPLS VPNs (RFC 2547bis)

- Routing exchange at the edges is based on **MP-BGP** (Multi-Protocol BGP): support for addresses of **different families**.
- **Route filtering**: PE routers **determine which routes to install in each VRF**.
- **Overlapping address spaces** are supported through the **VPN-IPv4 address family**.

> [!definition] **Route Distinguisher (RD)**
> Field combined with an IPv4 address to form a **VPN-IPv4 address** (`VPN-IPv4 = Route Distinguisher + IPv4 address`). Two customers may both use `10.1.3.0/24`; their VPN-IPv4 addresses differ, so BGP treats them as distinct routes.

The slide set describes the route-installation decision as *route filtering performed by the PE*; it does not name the Route Target attribute that implements this import/export policy in RFC 2547bis.

Example I-BGP advertisement between PEs (from the slides): *"Cyan `10.1.3.0/24` is reachable through PE1 (with label L1)"* — i.e., MP-BGP carries **next hop + VPN label**, not just reachability.

### Packet forwarding walkthrough (two labels + PHP)

Example from the slides: packet from cyan `10.2.3.4` to `10.1.3.8`, customer prefix `10.2/16`:

| Step | Node | Action |
|---|---|---|
| 1 | CE1 | destination outside `10.2/16` → send to **default gateway = PE2** |
| 2 | PE2 | looks up `10.1.3.8` in the **cyan VRF** → next hop **PE1**, label **L1** (distributed by PE1 for cyan `10.1.3.0/24`) |
| 3 | PE2 | looks up **PE1 in the main table** → next hop P1, label **L2** (LSP from PE2 to PE1) |
| 4 | PE2 | **pushes L1 and L2** on the label stack → `\| L2 \| L1 \| IP payload \|` |
| 5 | P routers | forward the packet to PE1 **using L2 only** (no VPN knowledge) |
| 6 | Penultimate hop | **pops L2 (PHP)** |
| 7 | PE1 | receives the packet with **L1**, uses it to route to the proper output interface; pops it → **plain IP packet** to the CE |

The two labels, unified with the pseudo-wire terminology:

| Label | Distributed by | Purpose |
|---|---|---|
| **External** (L2) | LDP/RSVP in the core | routing **within the provider network** — identifies the LSP / access point |
| **Internal** (L1) | MP-BGP among PEs | **multiplexing of users/services** at the same access point — identifies the VPN recipient within the PE |

### Virtual Router VPNs

- PEs execute a **(virtual) router instance for each VPN**, each with **separate data structures**.
- VRs of the same VPN **communicate through LSPs**.
- PE-to-PE routing exchange uses **IGP** (per-VR) instead of I-BGP.

### Benefits

- **No constraints on the addressing plan**: address uniqueness is only required **within the VPN**; CE routers do **not** exchange information with each other.
- The customer does not manage the backbone; providers do **not** run one virtual backbone per customer.
- A VPN **can span multiple providers**.
- **Security equivalent to Frame Relay or ATM**: traffic isolation — but **no cryptography** (no confidentiality).
- **QoS** supported through the **Exp bits** in the MPLS header.

## 6PE — IPv6 Islands over an IPv4 MPLS Core

> [!definition] **6PE**
> **IPv6 Provider Edge routers** connect IPv6 islands over an IPv4 MPLS network (RFC 4798). The idea: **keep the core unchanged**, add IPv6 support **at the edge**, and distribute IPv6 routing information in MPLS/BGP **"in the same way as we currently do with VPNs"**.

### The three options for IPv6 over MPLS

| Option | How it works | Drawbacks |
|---|---|---|
| **1. Native IPv6 over MPLS** | IPv6 and IPv4 traffic treated **identically** by core routers | core needs a **full control-plane upgrade to IPv6**: IPv6 routing in the core, IPv6 LDP in the core, dual control-plane management |
| **2. IPv6 over circuit transport over MPLS** | L2 frames (e.g., Ethernet frames, ATM cells) encapsulated into MPLS frames | **no changes to P routers**, but **scalability problems in heavily (L2 tunnel) meshed topologies** |
| **3. 6PE** | IPv6 routing distributed over BGP; IPv6 packets carried in LSPs | similar to MPLS VPNs in implementation and complexity — no core changes |

### Applicability

6PE is a good choice when:

- the NSP already has an **MPLS core** supporting MPLS VPN (or other) services;
- IPv6 services are requested by a **limited number of customers**;
- the ISP wants to avoid **either** fully upgrading the core **or** deploying IPv6-over-IPv4 tunnels.

If the number of IPv6 customers grows so much that **most access routers** would become 6PE, the NSP should consider **upgrading the whole network** instead.

### Requirements

- **PE routers**: upgraded to **dual stack IPv4/IPv6 with MP-BGP** support.
- **P routers**: **no change** in configuration or software — they remain IPv4-only.

### Control plane: announcing IPv6 networks (4 steps)

1. **IPv4 reachability of 6PEs**: each 6PE advertises its IPv4 address into the **IGP of the IPv4/MPLS core**; every router in the MPLS domain ends up assigning a **label** to the route of each 6PE (in the example: OSPF propagates reachability of `20.2.2.2`; **LDPv4** binds a label to it).
2. **CE–PE exchange**: CE and 6PE are connected through (one or more) **logical or physical native IPv6 interfaces**; any common routing protocol (e.g., OSPF, eBGP, static or default routes) distributes IPv6 reachability — the IGP advertises `2001:3::/64`.
3. **PE–PE exchange**: customer IPv6 prefixes are exchanged among 6PE routers over an **MP-BGP session running over IPv4**; the 6PE conveys its **IPv4 address as the BGP Next-Hop** for the IPv6 prefixes — the Next Hop is the **IPv4-mapped IPv6 address** of the 6PE (`::FFFF:20.2.2.2`), **bound to a BGP label**. PE-2 advertises over MP-iBGP that `2001:3::/64` is reachable via Next Hop `::FFFF:20.2.2.2`, bound to the BGP label.
4. **Far side**: the IGP propagates the advertisement within the other customer's network (may not be needed, e.g., when a default route is used).

Label distribution is split by plane:

| Plane | IPv4 labels | IPv6 labels |
|---|---|---|
| Core (P routers) | **LDPv4** | — |
| Among PEs | — | **MP-iBGPv4** |

### Data plane: forwarding with two labels

The ingress 6PE tunnels the IPv6 packet over an **LSP toward the egress 6PE**, identified by the IPv4 address derived from the **IPv4-mapped IPv6 BGP Next Hop** of the prefix:

1. CE sends an IPv6 packet to 6PE-1 (e.g., to `2001:3::`).
2. 6PE-1 does a **lookup on the IPv6 prefix**; the result is two labels:
   - **Label 2** — bound by **MP-BGP** to `2001:3::` (inner);
   - **Label 1** — bound by **LDP/IGPv4** to the **IPv4 address of the BGP Next Hop** (6PE-2) (outer).
3. 6PE-1 pushes the stack `\| Label 1 \| Label 2 \| IPv6 packet \|` and forwards along the LSP.
4. P routers switch on the **outer** label only; **penultimate hop popping** removes it.
5. The egress 6PE uses the **inner label** to identify the actual recipient (the CE-facing context) and delivers the IPv6 packet.

| Label | Bound to | Identifies |
|---|---|---|
| **Outer** (LDP/IGPv4) | egress 6PE's **IPv4 address** | the IPv4 MPLS **"tunnel" terminator** |
| **Inner** ("aggregated IPv6 label", MP-BGP) | each advertised **destination IPv6 prefix** | the **actual recipient within the PE** |

The inner label is **in principle not required** for the operation — but it **keeps the solution the same as for VPNs**: without it, the penultimate-hop router would have to be able to forward a **plain IPv6 packet** to the egress 6PE, which an unchanged IPv4-only core cannot do.

### BGP/MPLS VPN vs. 6PE — side by side

| | BGP/MPLS VPN (RFC 2547bis) | 6PE (RFC 4798) |
|---|---|---|
| PE-PE protocol | I-BGP carrying **VPN-IPv4** + VPN labels | **MP-iBGP over IPv4** carrying IPv6 prefixes + labels |
| Address family | VPN-IPv4 = **RD + IPv4** | IPv6, Next Hop = **IPv4-mapped IPv6** address |
| Inner label | VPN label (per VPN destination) | "aggregated" label per IPv6 prefix |
| Outer label | LSP to egress PE (LDP/RSVP) | LSP to egress 6PE (LDPv4) |
| P routers | IPv4 only, VPN-unaware | IPv4 only, **IPv6-unaware, unchanged** |

### Conclusions (from the slides)

- PE routers must be dual stack with MP-BGP; **P routers need no modification**.
- **Native IPv6 services without changing the IPv4 MPLS core** → minimal operational cost and risk.
- The 6PE forwarding scenario is **similar to MPLS VPN packet forwarding**.
- IPv6 CEs have a **single routing peer** (their 6PE) and need **no change when remote IPv6 CEs are connected or removed**.
- 6PE fits the general MPLS philosophy, but **does not by itself justify deploying an MPLS core**: it is meant for scenarios where the MPLS core **is already available**.

## References (from the slide sets)

- J. De Clercq, D. Ooms, S. Prevost, F. Le Faucheur, "Connecting IPv6 Islands over IPv4 MPLS Using IPv6 Provider Edge Routers (6PE)," **RFC 4798**, February 2007.
- E. Rosen, Y. Rekhter, "BGP/MPLS VPNs," **RFC 2547**, March 1999; and *draft-rosen-rfc2547bis-02.txt*, July 2000.
- C. Semeria, "RFC 2547bis: BGP/MPLS VPN Fundamentals," Juniper Networks white paper 200012-001, March 2001.
- S. Hanks (ed.), "Generic Routing Encapsulation over IPv4," **RFC 1702**, October 1994.
- IETF Working Groups: MPLS, L2VPN (`ietf.org/html.charters/l2vpn-charter.html`), L3VPN (`ietf.org/html.charters/l3vpn-charter.html`).
