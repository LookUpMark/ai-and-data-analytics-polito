---
title: IPv4 Review and Multicast
aliases: [IPv4 Addressing and Routing, CNTS Lecture 01, IP Multicast]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Review of IPv4 addressing and routing as used throughout the course: address structure (network/host parts), special addresses, classful ranges, CIDR prefix notation and valid netmasks, longest-prefix-match routing, and the 7-step addressing-plan methodology from the exercise deck. The second half covers IPv4 multicast: Class D group addresses, IGMP membership management, the IP-to-MAC multicast mapping (01-00-5E-...), IGMP snooping, multicast distribution trees, and the limited real-world deployment.

## IP Addressing: Terminology

> [!definition] **IP address**
> A 32-bit identifier assigned to host and router interfaces. It is split into a **network part** (high-order bits) and a **host part** (low-order bits).

> [!definition] **IP network**
> The set of IP devices whose interfaces (1) have the same network part of the IP address and (2) are connected to the same physical (link-layer) network.

## Special Addresses

| Network ID | Host part | Meaning |
|---|---|---|
| Some value | All 0s | The (sub)network ID itself |
| Some value | All 1s | Directed broadcast for that network |
| All 0s | All 0s | "This host" (all 0s) |
| All 1s | All 1s | Limited broadcast (local network) |
| 127 | Anything (often 1) | Loopback |

## Addressing Classes (legacy)

| Class | 1st byte range | # networks | Size |
|---|---|---|---|
| A | 0–127 | 128 | 24-bit host part |
| B | 128–191 | 16K | 16-bit host part |
| C | 192–223 | 2M | 8-bit host part |
| D | 224–239 | — | Multicast |
| E | 240–255 | — | Reserved/experimental |

## CIDR — Classless InterDomain Routing

> [!definition] **CIDR**
> Network portion of the address of **arbitrary length**: the address format is `network ID + prefix length` (e.g., `200.23.16.0/23`) or `network ID + netmask` (e.g., `200.23.16.0 255.255.254.0`). The netmask has all `1`s in the network part and all `0`s in the host part.

### Valid netmask byte values

| Byte value | Binary | Addresses "covered" |
|---|---|---|
| 0 | 0000 0000 | 256 |
| 128 | 1000 0000 | 128 |
| 192 | 1100 0000 | 64 |
| 224 | 1110 0000 | 32 |
| 240 | 1111 0000 | 16 |
| 248 | 1111 1000 | 8 |
| 252 | 1111 1100 | 4 (smallest usable netmask) |
| 254 | 1111 1110 | 2 — **not valid in the 4th byte** |
| 255 | 1111 1111 | 1 (single device) |

Each IP network must contain at least the **network ID** and the **broadcast address** (so a /n network has 2^(32−n) − 2 usable host addresses).

### Valid vs invalid network IDs

A pair `IP/prefix` is a valid network only if all host-part bits are zero:

- Valid: `130.192.1.4/30`, `130.192.1.16/30`, `130.192.1.16/29`
- Invalid: `130.192.1.1/30`, `130.192.1.4/29`, `130.192.1.24/28`

From the exercise deck: `192.168.2.36/30` is valid (36 = 0b100100, last 2 bits zero), `192.168.2.36/29` is not; `192.168.2.0/31` is **not** a valid network (no usable host addresses).

## IP Routing

General rules:

- Given a destination IP address, the device searches its **routing table** for a match.
- With multiple matches, it selects the **most specific one (longest prefix matching)**.

Example from the slides:

| Destination | Output link |
|---|---|
| 200.23.16.0/20 | 1 |
| 200.23.18.0/23 | 2 |
| 199.31.0.0/16 | 2 |

A packet for `200.23.18.5` matches both /20 and /23 and exits on link 2 (the /23 wins); a packet for `200.23.17.9` exits on link 1.

## Addressing-Plan Methodology (7 steps)

From the exercise deck, the method to design an IP addressing plan:

1. **Location of IP networks** (how many, where).
2. **Amount of required addresses** per network (hosts + router interfaces).
3. **Amount of allocated addresses** (power of two ≥ required, e.g., 43 hosts → 64).
4. **Address range validity** (check the total fits the assigned block).
5. **Netmask / prefix length** per network.
6. **Address range** assignment (largest first, keep ranges contiguous for aggregation).
7. **Host addresses** (router interfaces typically take .1, .2, ...).

Worked example (LAN 1: 40 end systems, LAN 2: 100, router link: 2 → 43/103/4 required → 64/128/4 allocated, out of 10.0.0.0/24):

| Network | Prefix | Addresses |
|---|---|---|
| LAN 2 (100 end systems) | 10.0.0.0/25 (255.255.255.128) | router .1, hosts .2–.101 |
| LAN 1 (40 end systems) | 10.0.0.128/26 (255.255.255.192) | router .129, hosts .130–.169 |
| Router link | 10.0.0.192/30 (255.255.255.252) | .193, .194 |

### Hosts → netmask (Exercise 1 solution)

| Hosts | Netmask | Prefix | Available addresses |
|---|---|---|---|
| 2 | 255.255.255.252 | /30 | 4 (−2) |
| 5 | 255.255.255.248 | /29 | 8 (−2) |
| 10 | 255.255.255.240 | /28 | 16 (−2) |
| 27 | 255.255.255.224 | /27 | 32 (−2) |
| 55 | 255.255.255.192 | /26 | 64 (−2) |
| 100 | 255.255.255.128 | /25 | 128 (−2) |
| 167 | 255.255.255.0 | /24 | 256 (−2) |
| 300 | 255.255.254.0 | /23 | 512 (−2) |
| 1010 | 255.255.252.0 | /22 | 1024 (−2) |
| 1540 | 255.255.248.0 | /21 | 2048 (−2) |

### Configuration-error and aggregation exercises

- **Exercise 3**: a router with `192.168.1.1/28` cannot reach DNS 2 configured as `192.168.1.23/28`, because .23 falls in the `192.168.1.16/28` network, not in `.0/28`. The netmask on that segment is wrong; DNS 2 becomes unreachable from outside. Using an *external* DNS server is fine per se — the error is the subnet mismatch.
- **Exercise 5 (route aggregation on R2)**: aggregate `/26 + /28 + /25` behind R1 into `130.192.2.0/24`-style supernets where bit boundaries allow: routing table = default route `0.0.0.0/0` via eth2, `130.192.2.128/25` direct on eth1, `130.192.2.0/25` via R3 on eth0.
- **Exercise 8 (aggregation-maximizing plan)**: group networks into *areas* so each area gets one aggregate prefix (Area 1: 256 addresses → /24; Area 2: 128 → /25), then subnet inside each area. This minimizes routing-table lines on R1 (two static routes instead of one per network).
- **Exercise 9 (address-minimizing plan)**: subnet tightly (no area aggregation); R1's table grows to one route per network — the trade-off between deployed addresses and table size.
- **Exercises 10–13 (sniffer exercises)**: count frames captured on Host A's cable with empty caches. `ping` of a same-subnet host: 10 frames on a Windows host (ARP request/reply broadcast pair + echo request/reply + their repetitions); `ping www.google.com` across a router with DNS in another subnet: 17 frames on Windows (ARP for gateway, DNS query/reply routed, ARP by the router for the destination, echo request/reply — Windows repeats ARP for its own gateway maintenance). Exact counts depend on the host OS and ARP cache behavior.

## IPv4 Multicast

> [!definition] **IP multicast**
> Packets are routed from one source to **multiple destinations** (group communication, e.g., videoconferencing, video broadcasting). A **multicast address identifies a group of hosts**; a packet sent to that address is delivered to all group members anywhere in the network.

### Multicast addressing

- **Class D** addresses: begin with `1110` → `224.0.0.0` – `239.255.255.255`.
- The address identifies a **host group**, not an individual host.

### Host group membership

- Hosts **join and leave dynamically**.
- The **IGMP** (Internet Group Management Protocol), encapsulated in IPv4 datagrams, handles membership; it is used by both hosts and routers.
- Recipients (not the source) determine who receives a packet; in unicast it is the source that picks the destination. Controlling traffic reach is therefore harder.

### Within an IEEE 802 network

Group delivery is delegated to the MAC layer:

- The IP multicast address is **mapped** to a MAC multicast address `01-00-5E-xx-xx-xx`:
  - fixed prefix `01-00-5E-` + 1 bit set to 0 + the **23 least significant bits** of the IP address (4 bits of the class D prefix + 5 bits of the OUI field are fixed).
- The interface card is configured to receive that MAC multicast.
- **Switch support: IGMP snooping** — the switch listens to IGMP and delivers multicast only on ports connected to group members; this is a cross-layer approach.

### Beyond a single network

- Routers discover host groups on each LAN using **IGMP**.
- Routers announce host groups to other routers via **multicast routing protocols**.
- Routers build a **distribution tree** per host group, reaching all LANs with at least one member.

### Deployment status

- **Not widely supported** on the public Internet; unsuitable for common traffic control/engineering practice at large.
- Mostly limited to **controlled environments** (e.g., video broadcasting within an ISP network).
