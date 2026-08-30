---
title: Routing Essentials
aliases: [CNTS Lecture 06, Distance Vector, Link State, OSPF, BGP]
tags: [computer-science/networking, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Principles of dynamic routing: routing (control plane) vs. forwarding (data plane), the algorithm taxonomy (static, centralized, isolated, distributed), Distance Vector / Bellman-Ford with its pathologies (count to infinity, bouncing effect) and remedies (split horizon, path hold-down, poisoned reverse), Path Vector, and Link State with Dijkstra. Then the Internet routing architecture: Autonomous Systems, intra-AS protocols (RIP, OSPF and its hierarchical variant, IS-IS) vs. inter-AS routing with BGP — sessions (eBGP/iBGP), attributes (AS-PATH, NEXT-HOP, local preference), messages, route selection, hot-potato routing, transit vs. peering, and policy enforcement through advertisements.

## Routing vs. Forwarding

> [!definition] **Routing**
> Determination of the (optimal) path from source to destination, based on routing algorithms and protocols. It is a **control plane** operation.

> [!definition] **Forwarding**
> Transmission of packets from one device to the next hop in their route: no real-time decision-making, just a **routing-table look-up** (the next hop was predetermined by the routing algorithm). It is a **data plane** operation.

Forwarding procedures: by **network address** (most common — longest prefix match), **label swapping** (e.g., MPLS), **source routing** (rarely used). Forwarding phases: next-hop/output-port selection (routing tables) → switching (transfer to output port) → transmission.

## Algorithm Taxonomy

- **Non-adaptive (static)**:
  - *Fixed directory routing* (static, manual configuration) — admin has full control, but error-prone and non-adapting;
  - *(selective) flooding* — forward on any port except the incoming one (careful: broadcast storm);
  - *random* — pick a random output port ("flipper ball"; eventually arrives);
  - *deflection / hot potato* — used on regular topologies; route on the best port if available, else on any available (shortest-queue) port.
- **Adaptive (dynamic)**:
  - **Centralized**: a Routing Control Center (RCC) collects info from all nodes, computes paths, distributes tables. PROs: optimizes performance, simplifies troubleshooting. CONs: single point of failure, bottleneck, heavy traffic near the RCC, unsuitable for highly dynamic networks.
  - **Isolated**: each node decides independently, no information exchange (e.g., **backward learning** in IEEE 802.1D switches).
  - **Distributed**: routers cooperate by exchanging connectivity information; each decides independently but coherently. Two families:
    - *with partial information* → **Distance Vector**;
    - *with global information* → **Link State**.

## Distance Vector (DV)

> [!definition] **Distance Vector algorithm (Bellman-Ford)**
> Every node **periodically** exchanges with **adjacent** nodes a Distance Vector: reachable destinations and the current distance to each (e.g., hop count). Each node keeps a **Distance Table (DT)** (cost to each destination **through each neighbor**) and a **Routing Table (RT)** (selected cost and next hop per destination). The DV contains the same info as the RT **minus the next hop**.

DT update rule for node X, destination Y via neighbor Z:

`D_X(Y,Z) = c(X,Z) + min_w { D_Z(Y,w) }`

- Nodes compare received vectors with DT and RT, adding new destinations, switching to shorter paths, updating costs.
- From DT to RT: per destination, select the minimum-cost column; the chosen column is the next hop.
- **Cold start**: nodes boot with only themselves in their tables; DVs propagate and tables converge in rounds.
- A topology change (cost increase) can make a DV "void" and force re-convergence — slowly (see below).

### Pathologies

Common root cause: **lack of information on the global topology**.

> [!definition] **Count to infinity**
> After a link failure, nodes exchange **stale/inconsistent information** and ratchet each other's cost to an unreachable destination up step by step (2 → 3 → 4 → 5 ...) — the distance "counts to infinity".

> [!definition] **Bouncing effect**
> A **temporary loop** caused by inconsistent routing tables after a failure: B forwards to C while C forwards to B; packets bounce between them (each believes the other reaches A).

- **Black hole**: a malicious node advertises impossibly short non-existent routes to attract traffic (DoS), which is then discarded — possible unless routes are authenticated.

### (Partial) remedies

| Mechanism | Rule | Notes |
|---|---|---|
| **Split horizon** | "If C reaches A through B, it is useless for B to try to reach A through C" — the DV sent by C to B omits destinations reached *through B* | prevents 2-node loops, speeds convergence; "personalized" DVs per neighbor |
| **Path hold-down** | after link L fails, destinations reachable through L are considered unreachable for a period (quarantine) | the failing router stays out of loops until the timer expires; **count to infinity may still occur** elsewhere; slower convergence |
| Ignore cost increases | cost-increasing routes in DVs are not used (two back-to-back cost increases) | may block legitimate cost increases |
| **Split horizon with poisoned reverse** | instead of omitting, advertise the invalid route at **infinite** distance | more aggressive, faster convergence (no route-expiry wait); can substitute or complement hold-down |

### DV bottom line

- PROs: simple to implement and deploy, very little configuration.
- CONs: exponential worst-case complexity and convergence time O(n²); routers do **not know the topology** (cannot distinguish ambiguous scenarios); convergence paced by slowest links/routers; complex tuning and troubleshooting; large routing traffic and storage → **not suitable for large complex networks**.

## Path Vector

Nodes exchange **dynamically-updated path information** (destination, cost, **path vector** of traversed nodes). Loops are **easy to detect** in the vector (a node seeing itself in the path rejects it) → no count-to-infinity. Cost: increased overhead. This is the family of **BGP**.

## Link State (LS)

> [!definition] **Link State algorithm**
> Every node broadcasts the **cost (state) of each of its links** to **all other nodes** (flooding/multicast among routers). Every node builds the full **network topology** in a **Link State Database (LSDB)** — all databases are identical — and independently computes minimum-cost paths with **Dijkstra's algorithm**, filling the routing table.

- LSAs are generated **on topology change** in principle; actual protocols generate them **periodically** too (reliability).
- **Dijkstra**: iterative; after k iterations the minimum-cost paths toward k destinations are known; after each step the "nearest" new node is finalized and its next hop enters the routing table. Complexity ≈ L·log(N) (L links, N nodes); requires **positive costs**.
- PROs: rapid convergence (LSAs spread quickly, no intermediate processing); limited routing traffic and storage (link states are small; efficient neighbor greeting); rarely generates loops; easy to troubleshoot (identical databases everywhere).
- CONs: high implementation complexity (selective flooding; first implementations took years); protocols with complex configuration.

## Internet Routing Architecture

**Routing protocols** let routers exchange network information and determine the best routes: they use routing algorithms and define metrics, encoding, timing. Problem: what is the *operational domain* of the protocol? Making routing scalable requires answering two needs:

- **Scale**: billions of destinations cannot all live in one flat table; table exchange would swamp links;
- **Administrative autonomy**: the Internet is a network of networks; each admin wants control over routing in its own network.

> [!definition] **Autonomous System (AS)**
> A set of subnets grouped by topology, organizational criteria (e.g., all subnets of a large ISP) and administration, with autonomous internal routing choices and negotiated external choices. Identified by a **2-byte AS number assigned by IANA**.

- **Intra-AS routing** (interior gateway protocols, IGP) among hosts/routers of the same AS; all routers in the AS run the same intra-domain protocol; different ASes may run different ones. **Gateway routers** sit at the AS edge with links to other ASes.
- **Inter-AS routing** among ASes; gateways perform inter-domain routing *and* intra-domain routing. The forwarding table is configured by both: intra-AS entries for internal destinations; inter-AS + intra-AS cooperation for external ones (e.g., 1a learns via iBGP that destination X is behind 1c, then intra-domain OSPF says which local interface reaches 1c).
- Inter-AS tasks: learn which destinations are reachable through which neighboring AS; propagate this reachability to all routers in the own AS.

### Why different intra- vs. inter-AS routing?

- **Policy**: inter-AS admin wants control over how its traffic is routed and who routes through its net; intra-AS has a single admin → no policy needed.
- **Scale**: hierarchical routing saves table size and update traffic.
- **Performance**: intra-AS can focus on performance; inter-AS **policy may dominate over performance**.

## Intra-AS Routing Protocols

| Protocol | Algorithm | Status |
|---|---|---|
| **RIP** | distance vector | obsolete |
| **OSPF** | link state | dominant; IS-IS essentially the same |
| **IGRP** | Cisco proprietary | for decades, until 2016 |

**OSPF (Open Shortest Path First)**: "open" = publicly available. Uses the link-state algorithm: LSA flooding to all routers in the AS, topology map at each node, Dijkstra route computation. OSPF messages ride **directly over IP** (not TCP/UDP). Advanced features:

- all OSPF messages **authenticated** (against malicious intrusion);
- **multiple same-cost paths** allowed (RIP: only one);
- multiple cost metrics per link for different **ToS** (e.g., satellite link cheap for best-effort, expensive for real-time);
- integrated uni+multicast (MOSPF shares the topology database);
- **hierarchical OSPF** in large domains: two-level hierarchy of **areas** + **backbone**; LSAs only within an area; each node knows its area in detail but only the direction (shortest path) to other areas' nets; **area border routers** summarize distances to their area's nets and advertise them to other border routers; **backbone routers** run OSPF limited to the backbone; **boundary routers** connect to other ASes.

## Inter-AS Routing: BGP

> [!definition] **BGP — Border Gateway Protocol**
> The de-facto inter-domain routing protocol — "the glue that holds the Internet together". It lets each AS **obtain subnet reachability information from neighboring ASes (eBGP)**, **propagate it to all internal routers (iBGP)**, and determine "good" routes based on reachability **and policy** (not necessarily shorter paths; choices reflect agreements among ASes). It also lets a subnet advertise its existence ("I am here") and enables **aggregation** (195.1.2.0/24 + 195.1.3.0/24 announced as 195.1.2.0/23).

- **BGP session**: two BGP routers ("peers") exchange messages over a **semi-permanent TCP connection**, advertising paths to destination prefixes. BGP is a **path vector** protocol: when AS3 gateway 3a advertises path `AS3, X` to AS2's 2c, AS3 **promises** AS2 to forward datagrams toward X.
- Gateway routers run **both eBGP and iBGP**.

### Paths, attributes, policy

- Advertised prefix + attributes = **route**. Key attributes:
  - **AS-PATH**: list of ASes the advertisement traversed;
  - **NEXT-HOP**: the specific internal-AS router toward the next-hop AS.
- **Policy-based routing**: a gateway receiving an advertisement applies **import policy** to accept/decline the path (e.g., never route through AS Y); AS policy also decides whether to **re-advertise** the path to other neighbors.

### Route selection (in order)

1. **local preference** attribute (policy decision);
2. **shortest AS-PATH**;
3. **closest NEXT-HOP router** — *hot potato routing*: pick the local gateway with the least **intra-domain** cost (2d chooses 2a even if it means more AS hops — don't worry about inter-domain cost; plain BGP criteria would have selected (AS3, X) via 2c);
4. additional criteria.

### Messages

| Message | Purpose |
|---|---|
| **OPEN** | opens the TCP connection to the remote peer, authenticates |
| **UPDATE** | advertises a new path (or withdraws an old one) |
| **KEEPALIVE** | keeps the connection alive without UPDATEs; ACKs OPEN |
| **NOTIFICATION** | reports errors; also used to close the connection |

Advertisement flow example: 2c receives `AS3,X` via eBGP → accepts per AS2 policy → propagates via iBGP to all AS2 routers → 2a advertises `AS2,AS3,X` via eBGP to AS1's 1c. A gateway learning **multiple** paths to X picks one per policy and advertises it within the AS via iBGP.

### Connection types and business models

- **Transit**: an ISP provides reachability to the **entire Internet** for another endpoint (enterprise, content/application provider, residential broadband provider); the endpoint **pays** the ISP to carry traffic to/from the Internet.
- **Peering**: networks interconnect to exchange **only traffic that originates or terminates within their networks** (customers included, for carriers/Tier-1):
  - *public peering* through an **IXP** (Internet Exchange Point) — peering with multiple networks;
  - *private peering*: bilateral direct interconnection.

### Achieving policy via advertisements

- **Private peering case**: X (dual-homed customer of providers A and B) does **not** want to carry B↔C traffic → X simply **does not advertise** to B a route to C.
- **Peering agreements case**: A advertises path `A,w` to B and C; B chooses **not** to advertise `B,A,w` to C — B gets no revenue for routing `C,B,A,w` since none of C, A, w is B's customer; C never learns the path via B and routes `C,A,w` directly.
- **Rule of thumb**: traffic crossing an ISP's backbone must have source or destination (or both) in a network that is a **customer** of that ISP.
