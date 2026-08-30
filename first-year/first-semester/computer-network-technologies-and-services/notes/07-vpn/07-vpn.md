---
title: Virtual Private Networks
aliases: [CNTS Lecture 05, VPN, GRE, L2TP, PPTP, IPsec]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> VPN concepts and protocols: the definition (private-network policies over a shared infrastructure), motivations (cost, selective access), security objectives (authentication, integrity, confidentiality, separation) and tunneling. Taxonomy in three dimensions — flavors (site-to-site, end-to-end, remote access), scenarios (intranet/extranet, centralized vs. distributed Internet access, hub-and-spoke vs. mesh topologies), and models (overlay vs. peer, customer- vs. provider-provisioned, layer 2/3/4 VPNs). Protocol details: GRE and enhanced GRE, L2TP (LAC/LNS, tunnels and sessions, header, security issues), PPTP, and IPsec (AH, ESP, transport and tunnel modes).

## Definition and Motivations

> [!definition] **Virtual Private Network**
> Connectivity realized on a **shared infrastructure** — private/public network (ISP network, IP, Frame Relay, ATM, the Internet) — such that **policies** (security, QoS, reliability, addressing, ...) can be enforced **as in a private network**.

Why VPNs:

- **Cost**: private networks rely on leased lines and long-distance dial-up; VPNs are cheaper.
- **Selective and flexible access**: external users get limited, high-security access (few services through the firewall); corporate users get **all intranet functionalities from the Internet**, as if directly connected.

Key elements:

- **Tunnel** — (secure) encapsulation of corporate traffic while in transit on the shared network (not present in some solutions);
- **VPN Gateway** — termination device on the corporate network; may be a tunnel endpoint.

## Security Objectives

| Objective | Meaning |
|---|---|
| End-point authentication | ensure source/destination is what/who it declares |
| Data integrity | data (including its origin) is not changed in transit |
| Data confidentiality | only the intended destination can read the data |
| Data separation | achieved through **tunneling** |

> [!definition] **Tunneling**
> A packet (or frame) between private sites is carried through the public network **inside a packet handled by public nodes**: outer header from tunnel end-point X to tunnel end-point Y encapsulates the original `header | payload`. Encapsulation at X, decapsulation at Y.

## Flavors (who terminates the tunnel)

| Flavor | Connects | Virtualizes | Tunnel endpoints |
|---|---|---|---|
| **Site-to-site (s2s)** | remote networks | leased line | gateways |
| **End-to-end (e2e)** | remote hosts | leased line | the end systems |
| **Access / remote VPN** ("virtual dial-in") | terminal to remote network | dial-up access | end system + VPN gateway |

## Scenarios

- **Intranet VPN**: interconnection of headquarters, remote/branch offices, telecommuters, traveling employees.
- **Extranet VPN**: interconnection of customers, suppliers, partners to a corporate intranet with **controlled access**. Specific issues: restricted resource access (firewall at the VPN), **overlapping address spaces** (solved with NAT), open standard-based solutions for interoperability, traffic control (partner traffic must not compromise corporate performance).
- **Internet access** for VPN members:
  - **Centralized**: remote branches/users use the public network only to reach headquarters; Internet access only from HQ; the VPN carries Internet traffic too; a single firewall/access-control point (no security breaches) but every corporate packet crosses the HQ link — **not scalable**.
  - **Distributed**: branches/users access the Internet directly through their local connection; the VPN carries only corporate traffic; one firewall per site (possible breaches) but scalable.
- **VPN topologies**: *hub-and-spoke* — every branch talks to HQ directly; fits data-center/mainframe-centered corporations; routing sub-optimal, few tunnels, hub may become a bottleneck. *Mesh* — more tunnels, harder manual configuration, optimized routing.

## Deployment, Provisioning, and Layer Models

Three classification dimensions (the slides' cube): **deployment model**, **provisioning model**, **protocol layer**.

### Overlay vs. Peer

- **Overlay model**: the public network does **not** participate in the VPN — it does not know where VPN destinations are; it merely provides connectivity among VPN gateways. Each gateway must be "in touch" with every other (highly meshed tunnels); **routing is performed by the VPN gateways**.
- **Peer model**: each VPN gateway interacts with a public router (its peer), **exchanging routing information**; the provider network disseminates it and routes traffic between gateways of the same VPN — more scalable.

### Customer- vs. Provider-provisioned

- **Customer provisioned**: the customer owns/configures/manages the VPN devices; the provider is unaware the traffic is VPN; **CE (Customer Equipment) terminates tunnels**.
- **Provider provisioned**: the provider implements the VPN; VPN state is kept and traffic of different VPNs separated **by provider devices**; the CE "behaves as if connected to a private network"; **PE (Provider Equipment) terminates tunnels**.

Comparison for **access VPNs**:

| | Customer provisioned | Provider provisioned |
|---|---|---|
| Remote host addresses | 2: ISP-assigned + corporate (VPN GW assigns a corporate address) | 1: corporate only |
| Tunnel terminated by | the remote host itself | the NAS |
| Always on VPN? | no — if the tunnel is not activated, the client operates without VPN | yes |
| Internet access | from any ISP, also distributed | only centralized |
| Constraint | works from any Internet connection | requires access to a specific ISP |

### Layer N VPN

Packet transport (tunneling) provided **by** a Layer-N protocol **and/or as** a Layer-N service:

- **Layer 2 VPNs**:
  - *Virtual Private LAN Service* (VPLS): emulates LAN functionality, connects LAN segments into a single LAN through the public network; the VPN emulates learning bridges, routing on MAC addresses;
  - *Virtual Private Wire Service*: emulates a leased line, carries any protocol;
  - *IP-Only LAN-like Service*: CEs are IP routers/hosts (not Ethernet switches); only IP (+ ICMP, ARP) travels through the VPN.
- **Layer 3 VPNs**: L3 packets are forwarded through the public network, routing on layer-3 addresses — *peer model*: VPN/corporate/customer addresses; *overlay*: backbone addresses. CEs are IP routers or hosts. L3 tunneling over IP: **IP-in-IP** (GRE, IPsec) or **layer-2 frames inside IP** (L2TP, PPTP/GRE).
- **Layer 4 VPNs**: tunnels realized by **TCP connections**, security via **SSL/TLS**; s2s (terminated on gateways) or e2e (terminated on end systems).

Tunneling by itself does **not** ensure security (IP-in-IP merely enables communication between corporate addresses A and B, possibly private ones).

## GRE — Generic Routing Encapsulation

Not designed for VPN, but for **tunneling**: encapsulation of **any protocol** (including IP) into IP — **IP Protocol 47**. Header (version 0): flags C, R, K, S (presence of Checksum, Key, Sequence number), strict source routing bit *s* (drop the packet if the source route ends before the destination), Recur (max additional encapsulations, must be 0), Protocol type (e.g., 0x0800 IP, 0x6558 transparent Ethernet bridging, IPX, ...), optional Checksum/Offset/Key/Sequence/Routing (source-route list of router IPs or ASs).

**Enhanced GRE (version 1)** — deployed by PPTP: adds Acknowledgment number (cumulative ack; A flag); Key high word = payload length, Key low word = **Call ID** (session ID); flow control by sliding window; out-of-order packets **discarded** (PPP cannot handle them); timeouts recomputed on ack reception; timeouts do **not** trigger retransmission (only move the window — packets will be lost).

## Access VPN Protocols

| | **L2TP** | **PPTP** |
|---|---|---|
| Provisioning | initially only provider provisioned | customer provisioned (PAC functionality embedded in the host) |
| Origin | IETF standard | Microsoft/Apple proprietary (RFC 2637), in end-user OSs |
| Security | through IPsec (strong but complicated) | weak encryption/authentication (MPPE, MS-CHAP), proprietary key management |
| Layer-2 dependence | independent of the host's L2 protocol | PPP over GRE |

### L2TP — Layer 2 Tunneling Protocol

Components:

- **LAC** (L2TP Access Concentrator): network access device (NAS) supporting L2TP;
- **LNS** (L2TP Network Server): the corporate (VPN) gateway.

Reference scenario (provider provisioned): the host speaks **PPP** to the LAC; every PPP frame arriving at the LAC is encapsulated (L2TP over UDP port 1701 over IP) toward the LNS, which extracts the PPP frame and handles it as if the host were attached to itself, forwarding it into the corporate network. Also supports **wholesale dial-up** between access provider and ISP. *Customer provisioned mode* by including LAC functionality in the host.

**Tunnels and sessions**: multiple **sessions** may exist within the same tunnel; multiple tunnels may be established between the same LAC and LNS or toward multiple LNSs.

**Operation**:

1. Establish a **control connection** (tunnel) between LAC and LNS — before any call request.
2. Establish one or more **sessions** triggered by a call request — before tunnelling PPP frames.

Tunnel establishment authenticates the peer with a **CHAP-like mechanism**: a challenge is proposed; the correct answer requires a **shared secret** (a cryptographic hash of challenge+secret is exchanged, never the secret itself). Endpoints exchange local tunnel IDs.

**L2TP header**: T (0 data / 1 control), L/S/O flags (Length, Ns/Nr, Offset present; for control messages L=S=1, O=0), P priority, Ver=2, **Tunnel ID**, **Session ID** (locally meaningful), Ns/Nr sequence numbers, Offset. The **control channel is reliable** (acks and retransmission, selective repeat, 32K windows); the **data channel is unreliable** — sequence numbers only detect out-of-order packets (no retransmission; L2 protocols may handle it).

**Security issues**: authentication only at tunnel establishment → someone snooping traffic can **inject packets into a session** (choose tunnel/session IDs unpredictably, not sequentially); packet-level security (encryption, authentication, integrity) must come from the transport — e.g., **IPsec**; end-to-end authentication also delegated to IPsec.

### PPTP — Point-to-Point Tunneling Protocol

- IETF RFC 2637; tunnels **PPP frames** over packet-switched networks; **PNS** = PPTP Network Server (corporate gateway), **PAC** = PPTP Access Concentrator (for provider-provisioned mode).
- **Data tunneling**: PPP frames over **GRE** over IP. **Control connection**: over **TCP, PNS port 1723** (messages: Start/Stop-Control-Connection Request/Reply, Echo Request/Reply, Outgoing/Incoming-Call Request/Reply/Connected, Call-Clear-Request, Call-Disconnect-Notify, WAN-Error-Notify, Set-Link-Info).
- Microsoft stack: encryption with **MPPE**, authentication with **MS-CHAP** (challenge-response like CHAP). In customer-provisioned mode the LAC/PAC functionality runs in the host's network stack; the host authenticates to the PNS and gets a corporate address, then sends/receives packets directly.

## IPsec VPNs

IPsec is used (especially in **site-to-site** VPNs) to make IP packets private, integral, and endpoint-authenticated; the VPN tunnel between gateways X and Y is an **IPsec tunnel** carrying the corporate A→B packets.

Two protocols:

> [!definition] **AH — Authentication Header (IP Protocol 51)**
> Provides **source authentication + data integrity, no confidentiality**. The AH header sits between the IP header and the payload. Routers process datagrams as usual — **but not NAT**, which rewrites the IP header and breaks the signature/digest.

AH fields: **SPI** (Security Parameter Index — session ID referencing algorithm and key), authentication data (crypto signature/digest), Next Header.

> [!definition] **ESP — Encapsulating Security Payload (IP Protocol 50)**
> Provides **data confidentiality** (payload and ESP trailer encrypted; next-header field in the trailer), plus **host authentication and data integrity** (authentication field similar to AH).

Two modes:

| Mode | What is protected | Structure |
|---|---|---|
| **Transport** | payload only; IP header not fully protected (only authenticated with AH) | `IP header | IPsec header | payload | IPsec trailer` |
| **Tunnel** | **both IP header and payload** | `new IP header | IPsec header | old IP header | payload | IPsec trailer` — the whole original packet is encapsulated |

## SSL/TLS VPNs (Layer 4)

- **s2s**: tunnels are TCP connections between VPN gateways; security via SSL/TLS: `IP hdr (X→Y) | TCP/SSL | [hdr A→B] | payload`.
- **e2e**: the tunnel can terminate on the end systems themselves (`IP hdr (A→B) | TCP/SSL | payload`).
