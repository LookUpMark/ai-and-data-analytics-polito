---
title: IPv4 to IPv6 Transition
aliases: [IPv4 IPv6 Transition, CNTS Lecture 02a, Dual Stack, 6to4, DS-Lite, A+P, NAT64, DNS64]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Transition mechanisms for the long IPv4/IPv6 coexistence: the four-step migration path (isolated IPv6 islands → native IPv6 → IPv4-in-IPv6 tunnels), dual-stack limitations, tunneling (GRE, IPv6-in-IPv4 proto 41, 6to4 with the 2002::/16 prefix and its pitfalls), and the carrier-grade solutions — DS-Lite and A+P with the AFTR over an IPv6-only backbone, and NAT64+DNS64 with the 64:FF9B::/96 well-known prefix. Includes the NAT444/CGN baseline these solutions evolve from and the limitations of each mechanism.

## The Road to IPv6

The transition must ideally be **incremental, seamless, smooth** — achieved through three building blocks:

- **Dual-stack approach** — IPv6 as a new layer-3 protocol; hosts generate/receive v4 or v6 packets as needed;
- **Address mapping**;
- **Tunneling** and **translation mechanisms**.

### The four steps

| Step | Network state | Mechanism |
|---|---|---|
| 1 | Isolated IPv6 networks (≈0.1% of hosts) | IPv6-in-IPv4 tunnels between dual-stack hosts/routers |
| 2 | IPv6 islands grow | Dual-stack + translating devices at the edges |
| 3 | Native IPv6 connectivity | Large portions of the Internet become IPv6-native |
| 4 | IPv6 takes over | Residual IPv4 islands carried in **IPv4-in-IPv6 tunnels** |

**Are we ready?** All protocols specified since 1996; IPv6 on routers (also hardware L3 switches) and end systems (Windows since 2000, Unix, macOS). Adoption (per Google, Oct 2023): ~45% globally — 48% US, **15% Italy**, ~75% France/Germany. The only real motivation is **address-space depletion**; there is **no official switch-off date** — migration is gradual. Working assumption: **IPv4 and IPv6 will coexist for a long time.**

## Dual Stack

> [!definition] **Dual stack**
> Both IPv4 and IPv6 capabilities on hosts and routers; hosts communicate natively with both. Requires **complete duplication of all protocol stack components** — routing protocols, routing tables, access lists.

Limitations:

- Does **not reduce the need for IPv4 addresses** (each host still needs an IPv4 address to speak IPv4).
- Applications decide whether to use IPv4 or IPv6.

Translation is unavoidable because:

- IPv6 hosts must reach IPv4 hosts through IPv4 networks (and vice versa) — tunnels;
- IPv6 hosts must talk to IPv4 hosts — translation;
- The reverse (IPv4 host initiating toward IPv6) is *not targeted*: hard to map the large IPv6 space onto the smaller IPv4 space.

## Tunneling

> [!definition] **Tunneling**
> Encapsulation of IPv6 packets into IPv4 packets (or vice versa) to traverse a network of the other family. It emulates a "direct" link among IPv6 devices; dual-stack routers at the ends add/strip the outer IPv4 header.

- **End points**: hosts and routers.
- **Protocols**: GRE (Generic Routing Encapsulation); IPv6-in-IPv4 (**protocol type 41**).
- **Setup**: manual or automatic — IPv4-compatible addresses, 6over4 (RFC 2529), **6to4**, Tunnel Broker (RFC 3053), ISATAP, Teredo.

### 6to4

> [!definition] **6to4**
> Each 6to4 router gets an IPv6 prefix `2002::/16` followed by its **32-bit public IPv4 address** (→ a /48 per site). IPv6 packets are encapsulated in IPv4 by the ingress 6to4 router, carried across the IPv4 Internet, and decapsulated by the egress 6to4 router (or relay).

Example from the slides: router with IPv4 `192.1.2.3` → `2002:c001:0203::/48`; the peer `9.254.2.252` → `2002:09fe:02fc::/48`. 6to4 is *not* meant for IPv4-host ↔ IPv6-host communication.

**Why 6to4 was phased out (from 2010):**

- complexity in network configuration; address conflicts and misconfigurations (large IPv6 pool mapped onto a smaller IPv4 pool);
- not all networks had globally routable IPv4 addresses;
- NAT and firewalls interfered;
- security vulnerabilities (routing/address spoofing).

## Carrier-grade / Scalable Solutions

Goal: support IPv4 servers talking to IPv6 hosts and IPv4 clients, **scalably**. Options listed: **DS-Lite, A+P, NAT64, MAP-T and MAP-E, 6PE (MPLS-based)**.

### The NAT baseline (today)

Typical deployment: RFC 1918 private addressing in the home (CPE does **NAT44**), then Carrier-Grade NAT / Large Scale NAT (**CGN/LSN**) at the provider (**NAT444**). Properties of NAT:

- problematic with **inbound sessions** (servers); NAT + STUN/TURN may be OK for peer-to-peer;
- **bottleneck and single point of failure**;
- several cascaded NAT instances are now common (even starting from virtual machines);
- hard to do without, due to scarce addresses.

### AFTR

> [!definition] **AFTR — Address Family Transition Router**
> Allows IPv4 hosts to communicate with IPv4 hosts over an **IPv6 carrier infrastructure**. It combines an **IPv6 tunnel concentrator** and (in DS-Lite) a large-scale NAT. Used by DS-Lite and A+P.

### DS-Lite (Dual-Stack Lite)

- **Dual-stack at the edge, IPv6-only provider backbone.**
- The DS-Lite CPE tunnels IPv4 traffic **inside IPv6** toward the AFTR; IPv6 traffic goes natively to the IPv6 Internet.
- The AFTR decapsulates and performs **NAT44** into a **shared pool of public IPv4 addresses** toward the IPv4 Internet.

Properties:

- reduces IPv4 address needs vs. dual stack (which needs a public IPv4 address per host);
- extended NAT enables customer-assigned, even **overlapping** private addressing (the IPv6 address of the CPE identifies the customer in the NAT table).

Limitations:

- **NAT is not under customer control** (same problem as CGN);
- problematic with servers: **static mapping / port forwarding cannot be configured** by the customer.

### A+P (Address plus Port)

- Evolves DS-Lite: **NAT is under control of the customer** — ranges of TCP/UDP **ports are assigned to each customer**; only those ports are used on the outside.
- The CPE translates private IPv4 → one public IPv4 address with the pre-assigned port set, then tunnels packets (IPv4-in-IPv6, proto-41) toward the AFTR.
- **The same public IPv4 address is reused across many users** (different port ranges).

Features:

- no problems with **overlapping private address spaces** at customers';
- ports can be negotiated dynamically by the CPE with the **Port Control Protocol (PCP)**;
- the AFTR becomes just a tunnel terminator — **no NAT44 in the AFTR**.

### NAT64 + DNS64

Deployment: an IPv6-only network with a **NAT64 router** toward the IPv4 Internet and a **DNS64** server (dual-stack) serving the clients.

> [!definition] **NAT64 prefix**
> An IPv6 prefix dedicated to mapped IPv4 addresses (RFC 6052) — either **well-known** (`64:FF9B::/96`, as in the slide example `64:FF9B::20.2.2.2`) or network-specific. The NAT64 router advertises it into the IPv6 network to attract traffic toward IPv4 hosts.

**Name resolution (DNS64):** the client asks for `AAAA www.example.com` → the authoritative DNS returns *name error* (IPv4-only server) → DNS64 asks for the **A record** (`www.example.com is 20.2.2.2`) → DNS64 **synthesizes an AAAA record** `64:FF9B::20.2.2.2` and serves both record types to the client.

**Packet forwarding (NAT64, outbound):** client sends TCP SYN to `64:FF9B::20.2.2.2`; the NAT64 translates the IPv6 packet to IPv4, **picks a free IPv4 address/port from its public pool** (e.g., 30.3.3.3), and builds a NAT session entry; return traffic is translated back.

**Limitations:**

- works **only when DNS is involved** — fails if the user directly specifies an IPv4 literal (e.g., `ping 1.2.3.4`);
- **no DNSSEC**: DNSSEC authoritative servers sign records, but DNS64 **modifies** them (signature breaks).

## Mechanism Comparison

| Mechanism | Where the tunnel/NAT lives | Customer address plan | Key limitation |
|---|---|---|---|
| Dual stack | none — duplicated stacks | public IPv4 **and** IPv6 per host | does not save IPv4 addresses |
| 6to4 | automatic 6-in-4 tunnels, prefix 2002::/16 | global /48 derived from IPv4 | phased out: misconfiguration, NAT/firewall interference, spoofing |
| DS-Lite | CPE tunnels 4-in-6 to AFTR; NAT44 in AFTR | overlapping private IPv4 allowed | NAT at provider, no port forwarding |
| A+P | CPE NATs to assigned port range; AFTR only terminates tunnel | private IPv4 + port ranges (PCP) | port exhaustion per customer |
| NAT64+DNS64 | no tunnel; stateful NAT64 at v6/v4 boundary | IPv6-only clients | DNS-dependent; breaks DNSSEC; v4-literal addresses fail |
| 6PE | MPLS core carries IPv6 over IPv4 LSPs | dual-stack PEs | requires MPLS core (see note 10) |
