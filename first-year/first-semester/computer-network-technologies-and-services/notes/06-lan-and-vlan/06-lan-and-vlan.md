---
title: LAN Design, Switching, and VLANs
aliases: [CNTS Lecture 04, Ethernet Switching, Spanning Tree, IEEE 802.1Q]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Modern LAN design: from history (WANs first, then LANs, IP as the winner) through device taxonomy — repeaters/hubs (L1), bridges/switches (L2), routers (L3) — collision vs. broadcast domains, full-duplex switched Ethernet. Core switching machinery: the filtering database, backward learning, the forwarding process and transient flooding, host mobility, the MAC-flooding attack, the loop problem (broadcast storm, unstable learning) and the Spanning Tree Protocol. Then Virtual LANs: motivation, per-VLAN forwarding tables, 802.1Q tagging (access vs. trunk ports), assignment methods, per-VLAN STP, isolation limits, VLAN-aware/unaware switches, campus design, and Cisco IOS configuration.

## A View from History

- WANs came first (1960s): few mainframes, remote access to partition their cost.
- LANs appeared later (late 1970s–early 1980s): minicomputers and PCs were cheap enough that small workgroups shared resources locally; mainframes remained for other purposes (e.g., scientific simulation).
- WAN and LAN protocols evolved independently (DECnet, SNA, IP vs. Novell, Banyan Vines, NetBEUI) until interconnection made **IP the winner**.
- Key standards: **IEEE 802** (protocols and physical layers), **EIA/TIA 568** and **ISO/IEC 11801** (structured cabling).

## LAN Devices in Brief

| Layer | Device | Separates |
|---|---|---|
| L1 | Repeater, Hub | separate **physical** domains, same **collision domain** |
| L2 | Bridge, Switch | separate **collision domains**, same **broadcast domain** |
| L3 | Router, L3 switch | separate **broadcast domains** (not LAN-specific) |

- **Repeater**: relays a bit stream; joins networks with the same MAC but different physical layers (e.g., coax ↔ fiber), recovering signal degradation over long distances.
- **Hub**: multiport repeater, needed for twisted-pair/fiber hub-and-spoke cabling.
- **Bridge**: introduced by DEC in 1983 (LANBridge100, software, 2 ports); interconnects at the data-link layer, also across different MACs (Ethernet ↔ Fast Ethernet, Ethernet ↔ WiFi). Works **store-and-forward**: store the frame, (optionally) modify it, send it out — this *decouples collision domains from the broadcast domain*.

> [!definition] **Collision domain**
> The area where a single instance of the medium access control algorithm (e.g., CSMA/CD) operates — the area covered by a single physical link; also called *network segment*.

> [!definition] **Broadcast domain**
> The area where frames can be propagated — the area on which the LAN operates. It can include several collision domains (bridges store and later forward frames across them).

## Half and Full Duplex

- Half duplex is the classic NIC mode: TX and RX cannot overlap (simultaneous activity = collision, as in 802.11).
- **Full duplex** (introduced with Fast Ethernet, 802.3x): possible whenever the other party can store the frame instead of repeating bits immediately. Simultaneous TX+RX — between host↔host, host↔bridge, bridge↔bridge.
- Advantages: bandwidth (in theory ×2; in practice most useful on backbone links — clients saturate the downlink, servers the uplink); **the real advantage: CSMA/CD is no longer needed** (no collisions possible) → no minimum Ethernet frame size requirement and no network-size limits from the collision domain.

## Switched Ethernet

- Modern wired LANs = **full-duplex switched Ethernet**: star topology, point-to-point host–switch links, no collision domain, Gigabit and above (10GE, 100GE); **CSMA/CD no longer used** (not even defined beyond 1GE).
- A **switch** is a multiport bridge — same functions, different internal architecture. Wireless LANs are different (CSMA/CA); "hubs" survive as e.g. WiFi extenders.

### Transparent bridges/switches (802.1D)

Transparency: plug-and-play, **no change to end systems** — same frames, same src/dst MAC addresses; only *which* frames are received may differ. Each switch port has a MAC address, but it is **never used for forwarding** — only for frames generated/received by the switch itself (e.g., management).

### Smart forwarding

- **Unicast**: forward only on the port toward the destination (destination-MAC-based forwarding).
- **Multicast/broadcast**: **flooding** — all ports except the incoming one (not necessarily simultaneously: delayed forwarding, unlike hubs that repeat bits immediately).

A "smart" switch needs three components: **(1) a local forwarding table (filtering database), (2) backward learning, (3) loop detection (spanning tree)** — the goal being zero explicit configuration.

## The Filtering Database

Table of `MAC address | destination port | ageing time`:

- **Dynamic entries**: populated/updated by backward learning; capacity 2 ÷ 64 K entries.
- **Static entries**: not touched by learning; usually < 1 K.
- Old dynamic entries are purged after the **ageing time — default 300 s** ("zombies" discarded).

(The name "filtering" comes from seeing smart forwarding as a way to *filter out* unwanted traffic from each link. Real implementations use one TCAM.)

### Forwarding process

For a frame received on port X:

1. Errors (collision, CRC)? → **discard**.
2. Destination MAC in the database?
   - **Yes** → is the destination port == X? Yes → discard (frame stays local); No → forward on the selected port.
   - **No** → **flood** on all ports except X (the switch behaves like a hub in port selection).

> [!definition] **Transient**
> The common situation where the destination MAC is not yet in the filtering database: the frame is duplicated on all ports except the incoming one. MAC flooding is the only option until learning kicks in.

## Backward Learning

> [!definition] **Backward learning**
> If a switch receives a frame with **source MAC H1 on port P1**, then H1 is reachable through P1. The topology is learned by inspecting **received** frames — only the **source** MAC is analyzed; the destination is ignored by the learning process.

- Works across multiple switches: remote switches learn the station's position anyway when the flooded frame transits.
- Database updates refresh **Age** (keeps the entry alive) and **Port** (relocates the host). Learning flowchart: source in DB? update port+age : add new entry; a background process discards zombies.

### Consequences and mobility

- A station **not** in the DB is always reachable (flooded) — even a *non-existing* station: frames to it traverse the whole network (ARP gets no reply, so this is rare in practice).
- A station **in** the DB may be unreachable for up to the ageing time after moving.
- Host mobility cases: if the moved host immediately sends **broadcast**, all switches relearn → no problem; if it sends **unicast** only, parts of the network keep the stale entry → frames may be lost (H3→H2 lost while H4→H2 still delivered). In practice this copes with manual moves (office → lab); fault-tolerant NICs need faster reaction — the NIC driver may generate an extra broadcast frame.

### MAC flooding attack

Generating frames with **random source MACs** fills the filtering database → the switch starts flooding most frames (those with unknown destinations). Objectives: force switch behavior like a hub to **intercept other stations' traffic**; slow the network. Mitigation: some vendors allow limiting the number of MAC addresses learned per port.

## Switches and Meshes: the Loop Problem

With meshed switches (physical redundancy), two problems:

1. **Frames loop**: broadcast frames (very common — ARP) circulate forever.
   - **Broadcast storm**: massive load from broadcast/multicast traffic on a LAN; "one of the most dangerous problems at the data-link layer"; no solution except **physically disabling loops**; due to the **lack of a TTL field in L2 frames** (L3 tolerates transient loops thanks to IP TTL). Can be abused as a cheap line-rate traffic generator.
2. **Learning breaks**: entries keep flipping between ports indefinitely (an unstable/inconsistent filtering database); transient loops between back-to-back switches.

> [!definition] **Spanning Tree (IEEE 802.1D)**
> Algorithm (original idea by Radia Perlman at DEC) that **detects and temporarily disables loops** in the meshed physical topology, turning the network into a **tree** with a **unique path between any source and destination**. Loops remain physically connected but logically disabled for robustness.

## Routers vs. L2

- Routers are **not transparent**: they rewrite L2 headers (new src/dst MAC), separate **broadcast domains**; different IP networks live on the two sides.
- **L2 vs. L3**: L2 works wherever you are and with any network protocol; L3 addresses depend on position and many parameters (firewalls, access lists) are bound to the address. → keep L2 "as long as the network can operate as a single L2 entity" (remember L2 scalability limits), but a **single gigantic LAN is undesirable**:
  - *performance*: too much broadcast traffic (not filtered by switches), flooded traffic (frequent STP reconfiguration);
  - *privacy/security*: stations should not leak information (MAC flooding attacks);
  - *management*: smaller networks allow simple, uniform policies.
- Full separation with N physical networks wastes resources (N links, N devices, unused switch ports, multiple backbone fibers).

## Virtual LANs (VLANs)

> [!definition] **VLAN**
> A logical partition of a single physical infrastructure into **different LANs = different broadcast domains**: Ethernet frames cannot cross VLANs; no broadcast, no MAC-flooding attacks, no ARP spoofing across VLANs. Created through a logical separation on switches (intra-switch or inter-switch).

- **Switch architecture**: each VLAN gets its **own filtering database**, own backward learning, and own Spanning Tree instance, feeding a shared forwarding process.
- **VLANs and IP**: broadcast (ARP) cannot cross VLAN boundaries → hosts in different VLANs **must belong to different IP networks** (e.g., 10.0.1.0/24 and 10.0.2.0/24, one router interface — or router sub-interface — each).
- **Interconnecting VLANs** requires a **router** (L3 lookup; the original L2 header is discarded and a new one is created):
  - *one-arm router*: a single router interface attached to the switch trunk, one sub-interface per VLAN.

### Associating frames to VLANs: 802.1Q tagging

- On a single switch: simplest is **port-based** (the frame belongs to the VLAN configured on the receiving port).
- Across switches / for shared devices (servers, routers): one link carries multiple VLANs → frames must be **tagged**.

> [!definition] **IEEE 802.1Q tag**
> A 4-byte header inserted after the MAC source address: **EtherType 0x8100 | User priority | CFI | VLAN ID (12 bits, values 1–4094)**. Tagging is required **only on links that transport traffic of different VLANs**. The maximum Ethernet frame grows from 1518 to **1522 bytes**; the minimum stays 64 bytes. (An 802.3/LLC-SNAP encapsulation variant exists.)

### Port types

| Port type | Frames on the link | Configuration | Use |
|---|---|---|---|
| **Access** | untagged only | default | end stations, hosts unaware of VLANs; incoming traffic is associated with the port's VLAN |
| **Trunk** | tagged | explicit | switch-to-switch, servers/routers with multi-VLAN membership |

- Access-port VLAN values are **not propagated outside the switch**: two switches whose access ports are all RED on SW-1 and all GREEN on SW-2 still interconnect their hosts as one LAN.
- A station can belong to **multiple VLANs** (servers, routers) — requiring trunk ports on the device (cooperative assignment: frames tagged by the host itself).

### Assignment methods

- **Port-based** (most common);
- **Transparent**: e.g., based on MAC addresses;
- **Per-user** (802.1x);
- **Cooperative**: hosts tag their own frames.

### VLANs and Spanning Tree

In theory independent: compute ST first (disable loops), then lay VLANs on the tree — a **unique forwarding tree for all VLANs**. Almost all vendors offer **Per-VLAN Spanning Tree**; most can revert to a single STP via configuration; **Cisco devices cannot** (per-VLAN STP is the only option).

### Isolation limits and caveats

- Isolation is **not complete**: links are shared — a broadcast storm on one VLAN **saturates trunk links**, congesting other VLANs that share the trunk. Per-VLAN **QoS** may be needed (e.g., round-robin per VLAN ID guaranteeing each VLAN a minimum bandwidth).
- **VLAN-aware** switches handle tagged frames; **VLAN-unaware** switches do not (may discard oversized frames; low-end devices). Mixing them: unaware switches work only at the access side, with all clients in the **same VLAN**; STP interactions can block ports and partition traffic unpredictably. Network managers should account for cheap unaware switches that end users install on their own.
- VLANs are **no longer plug-and-play** (unlike STP) — typical users cannot configure them, hence domestic switches do not support VLANs.

### LAN design take-aways

- Modern wired LANs: switched Ethernet, star topology, full-duplex links, no CSMA/CD; fault tolerance from redundancy + STP.
- VLANs widely adopted for traffic isolation and broadcast-domain reduction; routers/L3 switches required for inter-VLAN and external communication — "L3 closer to the users" (L3 switches in the distribution/core, VLAN-unaware switches tolerated at the very edge; trunk links in the backbone; data center/CED and Internet gateways at the top).

## Annex: Cisco IOS Configuration (from the slides)

```text
! VLAN creation (syntax varies across IOS versions/devices)
Switch# vlan database
Switch(vlan)# vlan 2 name Administration
Switch(vlan)# exit

! Port-based association (default: access, default VLAN)
Switch# configure terminal
Switch(config)# interface FastEthernet0/1
Switch(config-if)# switchport access vlan 2
Switch# show vlan brief

! Trunk port
Switch(config)# interface FastEthernet0/2
Switch(config-if)# switchport mode trunk
Switch(config-if)# switchport trunk allowed vlan add 1,2
```
