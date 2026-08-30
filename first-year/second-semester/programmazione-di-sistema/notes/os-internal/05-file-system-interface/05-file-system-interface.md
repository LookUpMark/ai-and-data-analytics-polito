# File System Interface

The file system is a crucial operating system component. It simplifies information storage by abstracting complex physical storage details. Fundamentally, it comprises two core elements: a collection of files (for data) and a directory structure (for file organization and metadata). This chapter will cover file concepts, directory organization, file sharing, and file protection.

## File Concept

The OS abstracts diverse storage media (NVM, HDD, magnetic tapes) into a uniform logical view. Within this view, a file is defined as a named, logical unit of storage, containing related information stored on secondary storage. Files can encompass programs (source, object, executable code) or data (numeric, alphabetic, binary). The creator primarily determines a file's internal structure and meaning, which is then interpreted by specific applications.

### File Attributes (Metadata)

Files possess descriptive properties, or metadata, which are distinct from their content. These attributes, stored as part of the directory structure on secondary storage, commonly include:

| Attribute | Description |
| :-------- | :------------------------------------------------------------------------------------------------------ |
| **Name** | Human-readable label. |
| **Identifier** | Unique internal tag assigned by the file system. |
| **Type** | Helps systems distinguish file categories (often via extensions). |
| **Location** | Pointers to physical storage on device. |
| **Size** | Current size in bytes or blocks. |
| **Protection** | Access control details (who can read, write, execute). |
| **Timestamps** | Creation, last modification, last access dates/times. |
| **Owner/Group** | User ID (UID) and Group ID (GID) for access permissions. |

### File Operations

A file, as an abstract data type (ADT), is defined by operations typically performed via OS system calls:

| Operation | Description | Common System Calls (Examples) |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| **Create** | Allocates space; adds directory entry. | `open()` (with creation flag), `creat()` |
| **Write** | Transfers data from memory to file at current pointer, then updates pointer. | `write()` |
| **Read** | Transfers data from file to memory from current pointer, then updates pointer. | `read()` |
| **Reposition (Seek)** | Moves file pointer to specific location for non-sequential access. | `lseek()` (Unix/Linux), `SetFilePointer()` (Windows) |
| **Delete** | Removes directory entry; releases file's storage space. | `unlink()` (Unix/Linux), `remove()`, `rm` (command) |
| **Truncate** | Erases file contents (length zero); keeps attributes. | `ftruncate()`, `truncate()` |
| **Open** | Finds directory entry, loads metadata to memory; returns handle/descriptor. | `open()` |
| **Close** | Releases system resources; flushes cached data; removes entry from open file tables. | `close()` |

### Open Files

To enhance performance, the OS caches information about currently open files directly in memory, thereby avoiding repeated and time-consuming directory searches on disk.

*   The **System-Wide Open File Table** is maintained by the kernel. This central table holds general information for all open files, including their location, size, global access dates, a disk location cache, and an open count. In parallel, each process possesses its own **Per-Process Open File Table**. This table contains pointers to entries in the system-wide table, the process's current position within the file (its file pointer), and its granted access rights (which are checked upon file opening). Consequently, processes use an index (a file descriptor in Unix/Linux, or a handle in Windows) to their per-process table to quickly access the system-wide information.

```mermaid
graph TD
    subgraph "Process A"
        direction LR
        PA_Tbl["Per-Process Table A"] --> FD0_A["fd: 0<br>pos: 0"]
        PA_Tbl --> FD1_A["fd: 1<br>pos: 0"]
        PA_Tbl --> FD3_A["fd: 3<br>pos: 1200"]
    end

    subgraph "Process B"
        direction LR
        PB_Tbl["Per-Process Table B"] --> FD0_B["fd: 0<br>pos: 0"]
        PB_Tbl --> FD3_B["fd: 3<br>pos: 50"]
        PB_Tbl --> FD4_B["fd: 4<br>pos: 300"]
    end

    subgraph "Kernel: System-Wide Data"
        Sys_Tbl["System-Wide Open File Table"]
        Entry1["Entry 1: File X Info<br>Location, Size, Open Count=2"]
        Entry2["Entry 2: File Y Info<br>Location, Size, Open Count=1"]
        Entry3["Entry 3: File Z Info<br>Location, Size, Open Count=1"]
        Entry4["Entry 4: Terminal Info<br>(stdin/stdout)"]

        Sys_Tbl --- Entry1 & Entry2 & Entry3 & Entry4
    end

    FD0_A -- "Points to" --> Entry4
    FD1_A -- "Points to" --> Entry4
    FD3_A -- "Points to" --> Entry1

    FD0_B -- "Points to" --> Entry4
    FD3_B -- "Points to" --> Entry2
    FD4_B -- "Points to" --> Entry1

    Entry1 --> DiskX((Disk Location of File X))
    Entry2 --> DiskY((Disk Location of File Y))
    Entry3 --> DiskZ((Disk Location of File Z))
```

### Open-File Locking

File locking coordinates shared file access, ensuring data integrity by preventing concurrent modifications that could lead to corruption.

*   File locking mechanisms typically come in two **Types**: A **Shared Lock** permits multiple processes (readers) to acquire it simultaneously while blocking exclusive writers. Conversely, an **Exclusive Lock** allows only one process (a writer) to hold it, blocking all other readers and writers. For **Mechanisms** of enforcement: **Mandatory Locking** means the OS actively prevents conflicting access (e.g., Windows). In contrast, **Advisory Locking (Cooperative)** expects processes to voluntarily respect locks; the OS does not strictly enforce them (e.g., Unix/Linux).

---

## Access Methods

Operating systems provide various methods for accessing file information.

### 1. Sequential Access

Information is processed strictly in order. `read_next()` reads data and advances an internal file pointer; `write_next()` appends data and advances the pointer. The pointer can usually `reset` to the beginning. This method is common for applications like text editors, compilers, and log file processing.

```mermaid
graph LR
    start((Start)) --> A[Record 1]
    A --> B[Record 2]
    B --> C[Record 3]
    C --> D[Record 4]
    D -- "..." --> E[End of File]
    style A fill:#fff,stroke:#333,stroke-width:2px,rx:5px,ry:5px
    style B fill:#fff,stroke:#333,stroke-width:2px,rx:5px,ry:5px
    style C fill:#fff,stroke:#333,stroke-width:2px,rx:5px,ry:5px
    style D fill:#fff,stroke:#333,stroke-width:2px,rx:5px,ry:5px
    style E fill:#fff,stroke:#333,stroke:#333,stroke-width:2px,rx:5px,ry:5px
```

### 2. Direct Access (Relative Access)

This method allows arbitrary read/write access to any location within the file, aligning with the disk's random access capabilities. Files are viewed as numbered, fixed-length logical blocks. Operations like `read(n)` and `write(n)` specify a relative block number `n`, which the OS then translates to a physical address. A `seek(n)` operation can explicitly set the file pointer to a desired block. This method is commonly used by databases.

### Simulating Sequential Access on Direct Access Files

Sequential access can be emulated using direct access by tracking a "current position" (comprising a current block number `cp_block` and an offset `cp_offset` within that block) and updating it after each `read_next()` or `write_next()` operation.

```mermaid
graph LR
    subgraph "File (Logical View)"
        A[Block 0] --- B[Block 1] --- C[Block 2] --- D[Block 3] --- E[Block 4]
    end

    subgraph Simulation
        F["Current Position (CP)"]
        G["read_next() / write_next()"]
    end

    F --> G
    G --> A
    G --> B
    G --> C
    G --> D
    G --> E
```

### 3. Other Access Methods (Indexed Access)

Built upon direct access, these methods employ an index (typically consisting of key-pointer pairs) to quickly locate specific records within large files. The index can reside in-memory (offering fast access) or on-disk (potentially requiring a multi-level structure for very large files). The Indexed Sequential Access Method (ISAM) serves as a classic example, which maintains sorted data along with primary and secondary indexes.

---

## Directory and Disk Structure

Files are stored on disks and are managed by directories (which act as symbol tables mapping names to file metadata and locations) as well as by specific disk allocation structures. Both components reside on secondary storage.

```mermaid
graph TD
    A[Directory] --> B{Entry for File_A};
    A --> C{Entry for File_B};
    A --> D{Entry for File_C};

    B -- "Maps Name to" --> E[File_A Metadata];
    C -- "Maps Name to" --> F[File_B Metadata];
    D -- "Maps Name to" --> G[File_C Metadata];

    E --> H[File_A Data Blocks];
    F --> I[File_B Data Blocks];
    G --> J[File_C Data Blocks];

    subgraph "Metadata Content"
        E1["Name: File_A<br/>Identifier: 123<br/>Type: Text<br/>Location: Disk Block X<br/>Size: 10KB<br/>Protection: rwx<br/>Timestamps: ...<br/>Owner: User1"]
    end
    
    E -.-> E1
```

### Disk Structure Considerations

When considering disk structure, several key factors are involved:
*   **Partitions:** Physical disks can be logically divided into partitions (sometimes called minidisks or slices), which can be used for file systems or swap space.
*   **RAID:** Redundant Array of Independent Disks combines multiple disks or partitions to enhance performance or reliability.
*   **Raw vs. Formatted:** Partitions can be accessed directly as raw storage or can have a file system installed on them (formatted).
*   **Volume:** A volume is a logical entity containing a file system, typically corresponding to a partition.
*   **Volume Metadata:** Each volume possesses its own control structures, such as a device directory or Volume Table of Contents (VTOC), which are essential for managing its files and directories.
*   **Specialized File Systems:** Operating systems may also employ specialized file systems for specific functions, such as exposing kernel information (e.g., Linux's `/proc`, `/sys`).

### Typical File System Organization on Disk

A formatted volume generally follows this layout:

```mermaid
graph LR
    A[Boot Control Block] --> B["Volume Control Block (Superblock)"]
    B --> C[Directory Structure]
    C --> D["File Metadata (e.g., Inodes)"]
    D --> E[Data Blocks]
    E --> F[Free Space Management]

    subgraph Formatted Volume Layout
        A
        B
        C
        D
        E
        F
    end

    style A fill:#D0F0C0,stroke:#3C763D,stroke-width:2px
    style B fill:#F0E68C,stroke:#B8860B,stroke-width:2px
    style C fill:#ADD8E6,stroke:#6A5ACD,stroke-width:2px
    style D fill:#FFDAB9,stroke:#FF8C00,stroke-width:2px
    style E fill:#CCEEFF,stroke:#6495ED,stroke-width:2px
    style F fill:#F0A0A0,stroke:#DC143C,stroke-width:2px
```

Specifically, a formatted volume typically includes the following components:
*   **Boot Control Block:** Contains information necessary for loading the OS if the volume is bootable.
*   **Volume Control Block (Superblock):** Stores file system-wide parameters, such as total blocks, block size, and counts/pointers for free blocks and inodes.
*   **Directory Structure:** Provides the hierarchical arrangement that links filenames to their corresponding metadata.
*   **File Metadata (e.g., Inodes):** These data structures hold file attributes and pointers to the actual data blocks.
*   **Data Blocks:** Store the actual file contents.
*   **Free Space Management:** Comprises structures that track unused blocks available for allocation.

### Directory Operations

OS system calls for directories include: Search, Create, Delete, List, Rename, and Traverse.

### Organizing Directories: Logical Structures

Effective directory organization aims for efficiency, convenience in naming (supporting unique names or multiple names for a single entity), and logical grouping of related files and subdirectories.

#### 1. Single-Level Directory

In a **Single-Level Directory** structure, all files reside in one central directory. While simple for single-user systems, this approach quickly suffers from naming collisions and manageability issues as the file count grows, leading to limited practical use.

```mermaid
graph TD
    Root["System Root Directory"] --> File1("file1.txt")
    Root --> File2("program.exe")
    Root --> File3("data.log")
    Root --> FileN("report.pdf")
```

#### 2. Two-Level Directory

A **Two-Level Directory** structure assigns each user a dedicated User File Directory (UFD), which is mapped by a Master File Directory (MFD). This design allows different users to have same-named files without conflict and provides basic isolation between user files. However, it notably lacks intra-user grouping capabilities and makes inter-user file sharing cumbersome. This structure was common in early time-sharing systems.

```mermaid
graph TD
    MFD["Master File Directory"] --> UFD_User1("User1's File Directory")
    MFD --> UFD_User2("User2's File Directory")
    MFD --> UFD_UserN("UserN's File Directory")

    UFD_User1 --> File1_User1("file1.txt (User1)")
    UFD_User1 --> ProgA_User1("programA.exe (User1)")

    UFD_User2 --> File1_User2("file1.txt (User2)")
    UFD_User2 --> DocB_User2("documentB.pdf (User2)")
```

#### 3. Tree-Structured Directory

The **Tree-Structured Directory** represents a general hierarchical structure where directories can contain both files and other subdirectories, all originating from a single root. Every file or directory within this structure possesses a unique absolute path. Users typically operate within a current working directory (CWD), which permits the use of relative paths. This model is the dominant directory structure in modern operating systems, offering flexible, deep, and logical file organization. Nevertheless, sharing files or directories between unrelated branches of the tree can be cumbersome.

```mermaid
graph TD
    Root("Root (/)") --> Home("home")
    Home --> User1("user1")
    Home --> User2("user2")

    User1 --> Docs1("docs")
    User1 --> Photos1("photos")

    User2 --> Docs2("docs")
    User2 --> Projects2("projects")

    Docs1 --> Report1("report.txt")
    Docs1 --> Memo1("memo.pdf")

    Photos1 --> Vacay("vacay.jpg")

    Docs2 --> Thesis("thesis.docx")

    Projects2 --> ProjectA("projectA")
    Projects2 --> ProjectB("projectB")

    ProjectA --> Source("source.c")
```

### Implementing Shared Files in Hierarchical Structures

Implementing shared files in hierarchical structures typically involves creating special directory entries called "links," which serve as pointers to existing files or subdirectories.

Specifically, **Hard Links** (common in Unix/Linux) are simply another directory entry pointing directly to the same underlying file metadata (inode). This means multiple names can refer to the exact same data, and the content and attributes persist as long as at least one link exists. Hard links are restricted to the same file system. In contrast, **Symbolic Links** (also known as Symlinks, Soft Links, or Shortcuts) are special files that contain the path to their target file or directory. They are more flexible, capable of crossing file systems, but come with the drawback of potentially becoming "dangling" if their target file or directory is moved or deleted.

### Acyclic Graph Directory Structure (DAG)

An **Acyclic Graph Directory Structure (DAG)** allows directories to share files or subdirectories through links, crucially preventing the formation of cycles. This structure offers more flexible sharing capabilities (e.g., for collaborative projects) compared to a strict tree, and is typically implemented using symbolic or hard links.

```mermaid
graph TD
    Root("Root (/)") --> Home("home")
    Home --> User1("user1")
    Home --> User2("user2")

    User1 --> Docs1("docs")
    User1 --> ProjectA("projectA")

    User2 --> Docs2("docs")
    User2 --> ProjectA_Shared("projectA")

    ProjectA --- SharedProjectDir("Shared Project Directory")
    ProjectA_Shared --- SharedProjectDir
    
    SharedProjectDir --> FileX("shared_file.txt")
    SharedProjectDir --> FolderY("subfolder/")
```

### General Graph Directory Structure (Potential for Cycles)

A **General Graph Directory Structure** allows unrestricted linking, especially to directories, which can inadvertently create cycles. Such cycles complicate file system traversal (e.g., leading to infinite loops during search or backup operations). Solutions to this problem include disallowing directory links (specifically for hard links), employing periodic garbage collection (to mark accessible nodes), or performing computationally expensive cycle detection at the time of link creation. Modern operating systems often manage symbolic link cycles by limiting traversal depth or by actively detecting loops.

```mermaid
graph TD
    Root("Root (/)") --> DirA("Directory A")
    Root --> DirB("Directory B")

    DirA --> File1("file1.txt")
    DirA --> LinkToB("Link to DirB")

    DirB --> File2("file2.txt")
    DirB --> LinkToA("Link to DirA")

    LinkToB --points to--> DirB
    LinkToA --points to--> DirA

    style LinkToB fill:#F0A0A0,stroke:#DC143C,stroke-width:2px
    style LinkToA fill:#F0A0A0,stroke:#DC143C,stroke-width:2px
```

---

## Protection

Robust protection mechanisms are essential in multi-user systems. Their purpose is to precisely control who can access files and what operations they can perform, thereby ensuring both data privacy and integrity. This protection fundamentally relies on User ID (UID) and Group ID (GID); consequently, files are always associated with an owner and a group.

### Protection Mechanisms

Access control is typically managed for a range of operations, including: Read (to view contents or list a directory), Write (to modify contents, create, delete, or rename items within a directory), Execute (to run a program or traverse a directory), Append (to add data to the end of a file), Delete, List (to view names or attributes only), and Attribute Change (to modify metadata).

### Implementation of Protection

Two common user identity-based approaches are employed for file protection:
1.  **Access Control List (ACL):** With an ACL, each file or directory possesses an explicit list detailing users and groups along with their specific access rights. This offers fine-grained control but can be complex to manage and store efficiently.
2.  **Simplified Access Control (Owner/Group/Other - Unix Model):** This model categorizes users into three distinct groups: the Owner, the Group, and Other users (representing all other users on the system). It defines basic permissions (Read, Write, Execute) for each category, typically stored as a compact 9-bit mask within the file metadata. Many modern operating systems combine this simplified model with more extensive ACLs for greater flexibility.

```mermaid
graph TD
    File("File: myfile.txt") --> Owner["Owner Permissions"]
    File --> Group["Group Permissions"]
    File --> Other["Other (World) Permissions"]

    Owner --> OR("Read (r)")
    Owner --> OW("Write (w)")
    Owner --> OX("Execute (x)")

    Group --> GR("Read (r)")
    Group --> GW("Write (w)")
    Group --> GX("Execute (x)")

    Other --> RR("Read (r)")
    Other --> RW("Write (w)")
    Other --> RX("Execute (x)")

    style Owner fill:#ADD8E6,stroke:#3182bd,stroke-width:2px
    style Group fill:#FFFACD,stroke:#FFD700,stroke-width:2px
    style Other fill:#D3D3D3,stroke:#808080,stroke-width:2px
    style OR fill:#90EE90,stroke:#228B22,stroke-width:1px
    style OW fill:#90EE90,stroke:#228B22,stroke-width:1px
    style OX fill:#90EE90,stroke:#228B22,stroke-width:1px
    style GR fill:#90EE90,stroke:#228B22,stroke-width:1px
    style GW fill:#90EE90,stroke:#228B22,stroke-width:1px
    style GX fill:#90EE90,stroke:#228B22,stroke-width:1px
    style RR fill:#90EE90,stroke:#228B22,stroke-width:1px
    style RW fill:#90EE90,stroke:#228B22,stroke-width:1px
    style RX fill:#90EE90,stroke:#228B22,stroke-width:1px
```

Illustrating a practical application, in the **Unix/Linux group permissions workflow**: An administrator first creates a group and assigns users to it. Subsequently, a file's group ownership is set to this newly created group. Permissions are then configured to grant appropriate access specifically to the `group` category, while simultaneously restricting `other` users. Members of the designated group can then access the file according to these defined permissions.