# TypeORM

*   **Context:** This section provides detailed information about **TypeORM**, which is a key technology required for implementing the project, specifically for the second assignment task related to data persistence.
*   **Topic Sequence:** This lesson focuses entirely on **TypeORM**. It follows the previous lesson on TypeScript and will be followed by an overview of the project structure itself.

---

## What is TypeORM?

### TypeORM Defined

*   **Core Identity:** **TypeORM** is a software library that serves as an **Object-Relational Mapping (ORM)** tool.
*   **Language Focus:** It is specifically designed to be used within applications written using **TypeScript** and standard **JavaScript**.
*   **Primary Goal:** Its main purpose is to make it significantly easier for developers to interact with **relational databases** (databases structured with tables, rows, and columns, like MySQL, PostgreSQL, SQLite, SQL Server, Oracle, etc.) from within an object-oriented programming language.
*   **How it Achieves This:** TypeORM allows you to manage your database data using an **object-oriented (OOP) style**. This means you define data structures using classes and objects in your code and interact with them using class methods and property access, rather than writing raw database commands in SQL.
*   **Reducing SQL:** A major benefit and explicit goal of TypeORM is to **reduce or eliminate the need to write traditional SQL queries** for many common database operations (such as selecting data, inserting new records, updating existing ones, or deleting records).

TypeORM is built with several key characteristics to support this:

*   **Broad Database Compatibility:** It is engineered to support a **wide variety of relational database systems**, allowing you flexibility in choosing the database technology for your project.
*   **Decorator-Based Mapping:** It makes extensive use of **decorators** (a language feature available in TypeScript and increasingly in JavaScript) within your code. These decorators are placed on classes and properties to explicitly define how your application's object structures (called **entities**) should be mapped to the tables and columns in your database.
*   **Built-in Functionalities:** It includes integrated features for common database development tasks, such as:
    *   Managing database schema changes over time (referred to as **migration**).
    *   Building database queries programmatically using method calls instead of string concatenation (known as **query building**).
    *   Defining and correctly handling **relationships** (like one-to-one, one-to-many, many-to-many) between different data entities.

In essence, TypeORM serves as a powerful bridge between your object-oriented TypeScript/JavaScript code and your relational database, aiming to make data access more structured, maintainable, and integrated with your application's language.

---

### Key Features Summary

TypeORM provides a set of important capabilities:

*   **Full TypeScript Support:** It is developed using TypeScript and offers strong type checking and type safety when used in TypeScript projects. It is also compatible with plain JavaScript.
*   **Decorator-Based Definition:** It relies heavily on decorators (like `@Entity()`, `@Column()`, `@ManyToOne()`) placed directly on your classes and properties to define the database mapping and relationships.
*   **Wide Database Engine Support:** It connects to a broad range of relational database systems (including SQLite, MySQL, PostgreSQL, SQL Server, Oracle, and others).
*   **Automatic Migrations & Schema Synchronization:** It includes tools to manage database schema changes. The `synchronize` option allows for automatic schema updates during development, while its migration system is used for controlled changes in production.
*   **Flexible Loading Strategies:** It supports different methods for fetching related data (relationships), including **Lazy Loading** (fetch related data only when accessed) and **Eager Loading** (fetch related data automatically with the main entity). These help optimize query performance.
*   **Framework Compatibility:** It integrates well with common Node.js web application frameworks like **NestJS** and **Express**, simplifying its setup within these environments.

---

## DataSource

### The Central DataSource Object

*   **Core Role:** In TypeORM, the **DataSource** object is the most fundamental and essential piece. It acts as the central point of control and management for **connecting your application to the database**.
*   **Manager of Connections:** It oversees and manages the actual database connection or connections that your application uses to interact with the database system.
*   **Configuration Holder:** It stores all the vital configuration settings required for the database connection. This includes:
    *   Which type of database you are using.
    *   The connection details (hostname, port, database name/file path, credentials).
    *   Where your ORM **Entity classes** are located in your project files.
    *   Settings related to database schema management (like `synchronize`).
    *   Logging configurations.
*   **Single Instance:** It's a common and recommended practice for an application to use only a **single instance of the DataSource** throughout its lifecycle to manage all database interactions.
*   **Lifecycle - Initialization:** Before your application can perform *any* database operation using TypeORM (such as saving entities, querying data, or managing repositories), the DataSource **must be successfully initialized**. The initialization process involves TypeORM reading its configuration and establishing the actual connection to the database.
*   **Lifecycle - Closing:** When your application is shutting down, the DataSource **should be properly closed or destroyed**. This is important to release the database connections and any other system resources held by TypeORM, preventing resource leaks.
*   **Organization:** For clarity and maintainability, it is highly recommended to **create and configure the DataSource object in its own dedicated file** within your project structure (for example, located at `src/dataSource.ts` or `src/database/connection.ts`).

---

### DataSource Connection Configuration

Setting up a DataSource instance requires providing a configuration object that tells TypeORM how to connect to your database and where to find your ORM entities.

Here are the common parameters you need to specify in the configuration object:

1.  **Database Type (`type`):** You must specify which database engine you are connecting to using a string literal. Examples: `"sqlite"`, `"mysql"`, `"postgres"`, `"sqlserver"`, `"oracle"`.
2.  **Connection Details:** Provide parameters specific to connecting to your database:
    *   For most server-based databases (MySQL, Postgres, etc.): `host`, `port`, `username`, `password`, `database` (name of the database).
    *   For SQLite: `database` (file path or `:memory:`).
3.  **Entities Location (`entities`):** An array of file paths or globs (patterns) specifying where TypeORM can find your Entity classes. TypeORM needs to load these classes to understand your data structure and schema.
4.  **Schema Synchronization Options (`synchronize`, `migrations`, etc.):** Configuration related to how TypeORM handles the database schema.
5.  **Logging (`logging`):** Configure logging to see the actual SQL queries executed by TypeORM and other debugging information (e.g., `logging: ['query', 'error']` or `logging: true`).

#### The `synchronize` Option (Detailed)

The `synchronize` option is a key setting for managing the database schema based on your entity definitions:

*   **If `synchronize: true`:** When `AppDataSource.initialize()` is called, TypeORM will automatically compare your defined entity structure with the current structure of the database. If there are differences, it will attempt to automatically create missing tables, add missing columns, update column types, and establish/update constraints (including foreign keys and primary keys) in the database to match your entity definitions.
*   **Use Case (Development):** Setting `synchronize` to `true` is extremely **convenient and useful during the development phase**. It allows you to quickly change your entity definitions and have those changes automatically reflected in your database schema every time your application starts, without needing to write manual schema update scripts.
*   **Major Risk (Production):** Using `synchronize: true` is **highly dangerous and is strongly discouraged in production environments**. The automatic schema changes can be unpredictable. TypeORM might drop tables, delete columns, or alter schemas in ways that lead to **irreversible data loss or corrupt your production database**.
*   **Recommendation (Production):** In production, the recommended approach is to use TypeORM's **migration system**. Migrations involve creating specific migration files that contain the explicit, controlled steps (SQL commands or TypeORM's query builder commands) for changing your database schema from one version to the next. You then manually apply these migrations in a controlled deployment process. When using migrations, you typically set `synchronize: false` and specify your migration files in the DataSource config.

#### Example Snippet: DataSource Configuration

```typescript
// File: src/dataSource.ts
import { DataSource } from 'typeorm';
// Import your entity classes here
import { User } from './models/entities/User';
import { Network } from './models/entities/Network';
import { Gateway } from './models/entities/Gateway';
import { Sensor } from './models/entities/Sensor';
import { Measurement } from './models/entities/Measurement';
// Import other entities...

// Load configuration (e.g., from a config file or environment variables)
import { DB_CONFIG } from './config'; // Assuming config is in src/config.ts

// Create and configure the DataSource instance
export const AppDataSource = new DataSource({
  type: DB_CONFIG.TYPE, // e.g., "sqlite", "mysql"
  database: DB_CONFIG.DATABASE, // e.g., "./data/app.sqlite" or ":memory:" or "mydb"
  host: DB_CONFIG.HOST, // e.g., "localhost" (for server DBs)
  port: DB_CONFIG.PORT, // e.g., 3306 (for server DBs)
  username: DB_CONFIG.USERNAME, // (for server DBs)
  password: DB_CONFIG.PASSWORD, // (for server DBs)

  entities: [User, Network, Gateway, Sensor, Measurement /* list all your entities */],

  // --- Schema Management Options ---
  // IMPORTANT: Use synchronize: true ONLY FOR DEVELOPMENT OR TESTING!
  synchronize: DB_CONFIG.SYNCHRONIZE, // e.g., true in dev, false in prod
  // For production, use migrations:
  // migrations: ["./src/migration/**/*.ts"],
  // migrationsRun: false, // Don't run migrations automatically on startup in prod, manage manually

  // --- Other Options ---
  logging: DB_CONFIG.LOGGING, // e.g., true, false, or ['query', 'error']
});

// Call initialize elsewhere when your app starts (e.g., in server.ts)
// AppDataSource.initialize()
//     .then(() => { console.log("Data Source has been initialized!"); })
//     .catch((err) => { console.error("Error during Data Source initialization:", err); });
```

---

### SQLite Database

**SQLite** is a specific type of database system that's often a good choice for certain use cases, including this project's initial assignments.

*   **Nature:** It is a **lightweight, file-based, self-contained, relational database system**.
*   **No Server:** Unlike most other relational databases (MySQL, PostgreSQL, etc.), SQLite **does not run as a separate server process**. The entire database engine is included directly within your application when you use a library like `sqlite3` (which TypeORM uses internally for SQLite connections).
*   **Storage:** The entire database structure and all its data are typically stored in a **single file** on the file system (e.g., `mydata.sqlite`).
*   **Use Cases:** SQLite is commonly used for:
    *   Embedded databases in mobile apps, desktop software, or smaller devices.
    *   Simple websites or prototypes.
    *   As a convenient and fast database for **testing environments** during development.
*   **TypeORM Support:** TypeORM provides full support for connecting to and interacting with SQLite databases.

#### SQLite Operating Modes

SQLite can operate in two main modes regarding where it stores its data, configured via the `database` parameter in the TypeORM DataSource config:

1.  **File-based Mode:**
    *   **Configuration:** You provide a **file path** as the `database` value (e.g., `./data/mydatabase.sqlite`).
    *   **Behavior:** Data is read from and written to this file on disk.
    *   **Persistence:** Data stored in a file-based SQLite database is **persistent**. It remains available even after the application closes and restarts.
2.  **In-memory Mode:**
    *   **Configuration:** You provide the special string literal `":memory:"` as the `database` value.
    *   **Behavior:** The entire database is created and runs exclusively in the computer's **RAM (Random Access Memory)**.
    *   **Persistence:** Data stored in an in-memory SQLite database is **volatile**. It is automatically and completely **lost** as soon as the application process stops or the DataSource connection is closed.
    *   **Use Case:** This mode is particularly **useful for automated testing**. Each test run can initialize a completely fresh, empty, and fast in-memory database instance, ensuring tests are isolated and deterministic. **This mode will likely be used for the Assignment 1 tests.**

---

## Entities Definition

In TypeORM, you define your persistent data structures (your database tables) using **Entity classes**. These are standard TypeScript/JavaScript classes that are annotated with special **decorators** to map them to the database.

You use the following decorators on your classes and their properties to define the database schema mapping:

1.  **`@Entity(options)`:**
    *   **Placement:** Applied directly above a class definition.
    *   **Purpose:** Marks the class as a TypeORM **Entity**. This tells TypeORM that this class represents a table in your database.
    *   **Options:** Can optionally take an options object. The most common option is providing a string `name` to specify the desired database table name (e.g., `@Entity("users_table")`). If no name is provided, TypeORM typically uses a converted version of the class name (often snake\_case).
2.  **`@Column(options)`:**
    *   **Placement:** Applied directly above a property within an entity class.
    *   **Purpose:** Maps the property to a **column** in the corresponding database table.
    *   **Configuration:** Takes an optional options object to configure the database column's behavior. Common options include:
        *   `type`: Explicitly defines the database column type (e.g., `"varchar"`, `"int"`, `"boolean"`, `"text"`, `"timestamp"`). TypeORM can often infer the type from the TypeScript property type, but explicit definition is sometimes needed or clearer.
        *   `nullable`: `true` or `false`. If `true`, the database column will allow `NULL` values. Defaults to `false`.
        *   `unique`: `true` or `false`. If `true`, the database will enforce a unique constraint on this column across all rows. Defaults to `false`.
        *   `default`: Specifies a default value for the column if not provided on insert. Can be a literal value, a function (`() => 'CURRENT_TIMESTAMP'`), or a database function string.
        *   `length`: Specifies the maximum length for string/varchar types.
3.  **`@PrimaryColumn(options)` or `@PrimaryGeneratedColumn(strategy, options)`:**
    *   **Placement:** Applied directly above one or more properties to designate them as the **primary key field(s)** for this entity.
    *   **Purpose:** Maps the property(s) to the primary key column(s) in the database table. Primary keys uniquely identify each row.
    *   **`@PrimaryGeneratedColumn`:** A convenience decorator for primary keys whose values are automatically generated by the database or TypeORM. It takes a `strategy` argument (e.g., `"increment"`, `"uuid"`, `"rowid"`, `"identity"`) to specify how the value is generated.
    *   **Implicit Constraints:** Properties marked as primary keys are always implicitly **unique** and **not nullable** at the database level.

#### Example Snippet: Entity Definition

```typescript
// File: src/models/entities/User.ts
import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm';

// Use the @Entity() decorator to map this class to a database table (defaults to 'user')
@Entity()
export class User {
  // @PrimaryGeneratedColumn() automatically creates an integer primary key that auto-increments
  // Use 'uuid' for a string UUID primary key: @PrimaryGeneratedColumn('uuid')
  // Use 'rowid' for a SQLite-specific auto-incrementing primary key: @PrimaryGeneratedColumn('rowid')
  @PrimaryGeneratedColumn('uuid') // Using UUIDs is common
  id!: string; // Non-nullable primary key field

  // @Column() maps this property to a database column (defaults to 'username')
  // Configure options: unique=true, length=100
  @Column({ unique: true, length: 100 })
  username!: string; // Non-nullable, must be unique

  @Column({ length: 255 })
  password!: string; // Non-nullable password hash

  // Enum type, maps to text in DB by default
  // Can specify { type: "enum", enum: UserType } if needed for stricter DB type
  @Column()
  type!: string; // e.g., 'admin', 'operator', 'viewer'

  @Column({ type: 'timestamp', default: () => 'CURRENT_TIMESTAMP' })
  createdAt!: Date; // Automatically set to current timestamp on creation

  @Column({ type: 'timestamp', default: () => 'CURRENT_TIMESTAMP', onUpdate: 'CURRENT_TIMESTAMP' })
  updatedAt!: Date; // Automatically set on creation and update
}
```
*Note the use of the definite assignment assertion `!` for properties that are non-nullable in the database and managed by TypeORM.*

---

## Entities Relationships

### Entities Relationships - Decorator Structure

TypeORM uses a consistent decorator structure to define relationships between entities. You apply these decorators to the properties in your entity classes that link to instances of other entities.

The general syntax for a relationship decorator is:

```typescript
@RelationshipDecorator(() => TargetEntity, (target) => target.property, options)
```

Let's break down the parts:

*   `@RelationshipDecorator`: This is the specific decorator for the type of relationship you're defining:
    *   `@OneToOne`
    *   `@OneToMany`
    *   `@ManyToOne`
    *   `@ManyToMany`
*   `() => TargetEntity`: The **first argument** is a function (specifically, an arrow function `() => ...`) that returns the **class of the related entity**. This is crucial for TypeScript/JavaScript to avoid circular dependency issues when entities reference each other.
*   `(target) => target.property`: The **second argument** is an optional function (an arrow function) that defines the **inverse side of the relationship**. It takes an instance of the `TargetEntity` (`target`) as an argument and should return the property *on that `TargetEntity`* that points back to the *current* entity. This second argument is **only required when defining a bidirectional relationship** and is placed on *one* side, while the inverse property definition goes on the other.
*   `options`: The **third argument** is an optional object used to configure various aspects of the relationship, such as cascading operations, delete behavior, and loading strategies.

#### Example Snippet: Decorator Structure in Bidirectional Relationship

```typescript
// File: src/models/entities/Post.ts
import { Entity, Column, PrimaryGeneratedColumn, ManyToOne } from 'typeorm';
import { User } from './User'; // Import the related entity

@Entity()
export class Post {
  @PrimaryGeneratedColumn()
  id!: number;

  @Column()
  title!: string;

  // Many posts belong to one user (Many-to-One)
  // This is the 'many' side, where the foreign key lives
  // (@ManyToOne(() => User, (user) => user.posts, options))
  @ManyToOne(() => User, (user) => user.posts) // Link to User entity, inverse property is 'posts' on User
  user!: User; // Property to access the related User object

  // ... other post properties
}
```

```typescript
// File: src/models/entities/User.ts
import { Entity, Column, PrimaryGeneratedColumn, OneToMany } from 'typeorm';
import { Post } from './Post'; // Import the related entity

@Entity()
export class User {
  @PrimaryGeneratedColumn()
  id!: number;

  @Column()
  username!: string;

  // User has many posts (One-to-Many)
  // This is the 'one' side, the inverse of the Many-to-One on Post
  // (@OneToMany(() => Post, (post) => post.user, options))
  @OneToMany(() => Post, (post) => post.user) // Link to Post entity, inverse property is 'user' on Post
  posts!: Post[]; // Property to access the collection of related Post objects

  // ... other user properties
}
```
*In this bidirectional Many-to-One / One-to-Many relationship between `User` and `Post`, the `@ManyToOne` on `Post` and the `@OneToMany` on `User` each specify the *inverse* property (`(user) => user.posts` on `Post`'s decorator, `(post) => post.user` on `User`'s decorator) to link the two sides.*

---

### Relationships - Common Options (Cascade & OnDelete)

Options are provided in the third argument object `{ ...options }` of the relationship decorator.

#### Cascade Options (`cascade`)

The `cascade` option controls whether database **persistence operations** (saving, updating, removing) performed on the **entity instance where the relationship is defined** (the "owner" of the relationship property) should automatically propagate to the related entities linked by that property.

*   **Configuration:** Set `cascade` to `true` or an array of specific operations: `["insert", "update", "remove", "soft-remove", "recover"]`.
*   **Behavior:** When you call `repository.save(mainEntity)` or `repository.remove(mainEntity)` in your application code:
    *   `"insert"`: If you add a *new* related entity to the relationship property of `mainEntity` (e.g., `user.posts.push(newPost)`) and then save `mainEntity`, TypeORM will also insert `newPost` into the database.
    *   `"update"`: If you make changes to an *existing* related entity linked by the relationship property and then save `mainEntity`, TypeORM will also save the changes to that related entity.
    *   `"remove"`: If you remove a related entity from the relationship property (e.g., `user.posts = user.posts.filter(...)`) or remove `mainEntity` itself, related entities specified with `"remove"` cascade are also marked for deletion.
*   **How it's handled:** TypeORM manages these cascades primarily in your application's memory/session state before generating the necessary SQL commands.

#### Delete Behavior (`onDelete`)

The `onDelete` option defines what happens to **related database records** when the **database record of the entity where the foreign key constraint is defined** is deleted directly in the database. This is a database-level foreign key constraint.

*   **Configuration:** Set `onDelete` to a string: `"CASCADE"`, `"SET NULL"`, `"RESTRICT"`, `"NO ACTION"`, `"SET DEFAULT"`. These are standard SQL foreign key behaviors.
*   **Placement:** This option is configured on the relationship decorator where the **foreign key constraint is *created* in the database table**. This is typically the `@ManyToOne` side (where `@JoinColumn` is used) or the `@ManyToMany` side (where `@JoinTable` is used).
*   **Behavior (Database Enforced):**
    *   `"CASCADE"`: If the referenced parent record is deleted, the database automatically deletes the related records in the current table that point to it.
    *   `"SET NULL"`: If the referenced parent record is deleted, the database sets the foreign key column(s) in the current table's records to `NULL`. Requires the foreign key column to be nullable.
    *   `"RESTRICT"`: The database prevents deletion of the parent record if there are any related records in the current table referencing it.
*   **How it's handled:** TypeORM includes the specified `ON DELETE` clause in the SQL command it generates when creating the foreign key constraint in the database schema (during synchronization or migrations). The database system enforces this rule directly when receiving a `DELETE` command.

**Comparison Table: TypeORM `cascade` vs. `onDelete`**

| Feature          | `cascade` Option                       | `onDelete` Option                      |
| :--------------- | :------------------------------------- | :------------------------------------- |
| **Purpose**      | Defines **application-level** operations on related entities when main entity is *saved* or *removed* via TypeORM. | Defines **database-level** behavior on related records when a main record is *deleted* directly in the database. |
| **Managed By**   | TypeORM (analyzes object graph in memory) | The Database Management System (DBMS)  |
| **When Applied** | During TypeORM `save()` or `remove()` calls in your code. | When a `DELETE` SQL command affects a record pointed to by a foreign key with this constraint. |
| **Applies To**   | Persistence operations (`insert`, `update`, `remove`, etc.) | Database `DELETE` operation enforced via foreign key constraints. |
| **Placement**    | On the relationship decorator (`@OneToMany`, `@ManyToOne`, etc.) | On the relationship decorator where the foreign key constraint is *defined* (`@ManyToOne`, `@OneToOne`) or on `@JoinTable` for `@ManyToMany`. |

---

### Relationships - Common Options (Loading Strategy)

The **Loading Strategy** determines *when* TypeORM fetches related entity data from the database.

**Comparison Table: Eager Loading (`eager: true`) vs. Lazy Loading (`lazy: true` or default)**

| Feature            | Eager Loading (`eager: true`)                                  | Lazy Loading (`lazy: true` or default)                           |
| :----------------- | :------------------------------------------------------------- | :--------------------------------------------------------------- |
| **Loading Time**   | Related data is loaded **immediately** when the main entity is loaded. | Related data is **not loaded initially**. It's fetched only when the property is accessed. |
| **Mechanism**      | TypeORM often uses database **JOIN** operations to fetch data in a single query. | TypeORM executes a **separate query** upon first access to the property. |
| **Property Type**  | Property type is the **actual Target Entity class** (e.g., `author: User;`). | Property type is a **Promise** wrapping the target entity or entity array (e.g., `author: Promise<User>;`, `posts: Promise<Post[]>;`). |
| **Access**         | Access the property directly (e.g., `post.author.name`).       | Must `await` or resolve the Promise to get the data (e.g., `let author = await post.author; author.name`). |
| **Performance**    | Can reduce N+1 queries for frequently accessed relations. Can increase initial query size/time if loading large/unneeded data. | Fast initial load. Can lead to N+1 queries if many related items are accessed individually after loading. |
| **Bidirectional**  | Can only be set on **one side** of a bidirectional relationship. | Can be set on **both sides** of a bidirectional relationship.      |
| **Configuration**  | Set `eager: true` in the relationship options.                     | Set `lazy: true` or omit `eager` option (default behavior).               |

---

### Relationships - One-to-One (`@OneToOne`)

*   **Definition:** One instance of Entity A relates to one instance of Entity B.
*   **Decorator:** `@OneToOne(() => TargetEntity, (target) => target.property, options)`
*   **Foreign Key:** One of the two tables will contain a foreign key column referencing the primary key of the other table.
*   **Owning Side:** You **must apply the `@JoinColumn()` decorator** to the property on the side that **owns the foreign key column** in the database.
*   **Example Snippet (Conceptual):**

    ```typescript
    // User has one Profile, Profile belongs to one User (bidirectional)
    // File: src/models/entities/UserProfile.ts (Owning side)
    import { Entity, PrimaryGeneratedColumn, Column, OneToOne, JoinColumn } from 'typeorm';
    import { User } from './User'; // Import User

    @Entity()
    export class UserProfile {
        @PrimaryGeneratedColumn('uuid')
        id!: string;

        @Column()
        bio?: string;

        // One-to-One relationship with User
        // This side owns the foreign key column linking to User
        @OneToOne(() => User, user => user.profile) // Link to User, inverse property is 'profile' on User
        @JoinColumn({ name: 'userId' }) // Creates the 'userId' foreign key column in the 'user_profile' table
        user!: User; // Property to access the related User object

        // ... other profile fields
    }
    ```

    ```typescript
    // File: src/models/entities/User.ts (Inverse side)
    import { Entity, PrimaryGeneratedColumn, Column, OneToOne } from 'typeorm';
    import { UserProfile } from './UserProfile'; // Import Profile

    @Entity()
    export class User {
        @PrimaryGeneratedColumn('uuid')
        id!: string;

        @Column({ unique: true })
        username!: string;

        // One-to-One relationship with UserProfile
        // This side does NOT own the foreign key
        // (@OneToOne(() => UserProfile, profile => profile.user, options))
        @OneToOne(() => UserProfile, profile => profile.user) // Link to Profile, inverse property is 'user' on Profile
        profile!: UserProfile; // Property to access the related Profile object
        // Configure eager/lazy loading here if needed (e.g., { eager: true })
        // Cannot be eager: true on both sides if relationship is bidirectional.
    }
    ```

---

### Relationships - One-to-Many (`@OneToMany`)

*   **Definition:** One instance of Entity A relates to *many* instances of Entity B. Defined on the "one" side.
*   **Decorator:** `@OneToMany(() => TargetEntity, (target) => target.property, options)`
*   **Foreign Key:** The foreign key column lives in the table of the "many" side (the `TargetEntity`).
*   **Join Column:** You **do not** use `@JoinColumn()` on the `@OneToMany` side. The foreign key definition and `@JoinColumn()` belong on the `@ManyToOne` side.
*   **Example Snippet (Conceptual):**

    ```typescript
    // User has many Orders (One-to-Many)
    // File: src/models/entities/User.ts (The 'one' side)
    import { Entity, PrimaryGeneratedColumn, Column, OneToMany } from 'typeorm';
    import { Order } from './Order'; // Import the related entity

    @Entity()
    export class User {
      @PrimaryGeneratedColumn('uuid')
      id!: string;

      @Column()
      username!: string;

      // One-to-Many relationship with Order
      // This property holds the collection of related Orders
      // (@OneToMany(() => Order, (order) => order.user, options))
      @OneToMany(() => Order, order => order.user, { cascade: ['insert', 'update', 'remove'], eager: true }) // Optional: eager load orders
      orders!: Order[]; // Property to access the collection of related Order objects

      // ... other user properties
    }
    ```

---

### Relationships - Many-to-One (`@ManyToOne`)

*   **Definition:** *Many* instances of Entity B relate to *one* instance of Entity A. Defined on the "many" side. This is the inverse of `@OneToMany`.
*   **Decorator:** `@ManyToOne(() => TargetEntity, (target) => target.property, options)`
*   **Foreign Key:** The foreign key column lives in the table of the "many" side (the entity where `@ManyToOne` is used).
*   **Join Column:** You **must apply the `@JoinColumn()` decorator** to the property on the side that **owns the foreign key column** in the database, which is the `@ManyToOne` side. This defines the FK column(s).
*   **Example Snippet (Conceptual):**

    ```typescript
    // Order belongs to one User (Many-to-One)
    // File: src/models/entities/Order.ts (The 'many' side)
    import { Entity, PrimaryGeneratedColumn, Column, ManyToOne, JoinColumn } from 'typeorm';
    import { User } from './User'; // Import the related entity

    @Entity()
    export class Order {
      @PrimaryGeneratedColumn('uuid')
      id!: string;

      @Column('decimal')
      amount!: number;

      // Many-to-One relationship with User
      // This property holds the single related User object
      // (@ManyToOne(() => User, (user) => user.orders, options))
      @ManyToOne(() => User, user => user.orders, { onDelete: 'SET NULL', nullable: true, eager: false }) // Optional: lazy load user
      @JoinColumn({ name: 'userId' }) // Creates the 'userId' foreign key column in the 'order' table
      user?: User; // Use '?' if nullable: true

      // ... other order properties
    }
    ```

---

### Relationships - Many-to-Many (`@ManyToMany`)

*   **Definition:** *Many* instances of Entity A relate to *many* instances of Entity B.
*   **Decorator:** `@ManyToMany(() => TargetEntity, (target) => target.property, options)`
*   **Junction Table:** Relational databases use an intermediate **junction table** (or linking table) to implement this. TypeORM automatically creates and manages this table based on your definitions.
*   **Join Table:** On the **owning side** of the `@ManyToMany` relationship, you **must apply the `@JoinTable()` decorator** to the property. This decorator configures the junction table's creation and column names.
*   **Example Snippet (Conceptual):**

    ```typescript
    // Student has many Courses, Course has many Students (bidirectional Many-to-Many)
    // File: src/models/entities/Student.ts (Owning side)
    import { Entity, PrimaryGeneratedColumn, Column, ManyToMany, JoinTable } from 'typeorm';
    import { Course } from './Course'; // Import related entity

    @Entity()
    export class Student {
      @PrimaryGeneratedColumn('uuid')
      id!: string;

      @Column()
      name!: string;

      // Many-to-Many relationship with Course
      // This property holds the collection of related Courses
      // This side is the 'owning' side (arbitrarily chosen for ManyToMany)
      @ManyToMany(() => Course, course => course.students) // Link to Course, inverse property is 'students' on Course
      @JoinTable({ // Apply @JoinTable on the owning side
         name: 'students_courses', // Custom name for the junction table (optional)
         // TypeORM creates columns like 'studentId', 'courseId' automatically by default
      })
      courses!: Course[]; // Collection of related Courses
      // Configure eager/lazy loading here if needed (e.g., { eager: true })

      // ... other student properties
    }
    ```

    ```typescript
    // File: src/models/entities/Course.ts (Inverse side)
    import { Entity, PrimaryGeneratedColumn, Column, ManyToMany } from 'typeorm';
    import { Student } from './Student'; // Import related entity

    @Entity()
    export class Course {
      @PrimaryGeneratedColumn('uuid')
      id!: string;

      @Column()
      title!: string;

      // Many-to-Many relationship with Student
      // This property holds the collection of related Students
      // This side is the 'inverse' side (defined by not having @JoinTable)
      // (@ManyToMany(() => Student, (student) => student.courses, options))
      @ManyToMany(() => Student, student => student.courses) // Link to Student, inverse property is 'courses' on Student
      students!: Student[]; // Collection of related Students
      // Configure eager/lazy loading here (e.g., { lazy: true } -- CANNOT be eager if Student.courses is eager)

      // ... other course properties
    }
    ```

---

### Bidirectional Relationships - Eager/Lazy Constraint

*   **Bidirectional Link:** A relationship is bidirectional if you define the mapping property on both participating entity classes (`user.posts` and `post.user`).
*   **Eager Loading Limitation:** As mentioned earlier, TypeORM **cannot support `eager: true` on *both* sides** of a **bidirectional relationship simultaneously**. This is true for `@OneToOne`, `@OneToMany`/`@ManyToOne` pairs, and `@ManyToMany` relationships.
*   **Reason:** If both sides were eager, when TypeORM tries to load one entity (e.g., `User`), it would eagerly try to load its related collection (`posts`), which in turn would eagerly try to load its related parent (`user`), which would load its collection (`posts`), and so on, creating an infinite loop during the database query process.
*   **Solution:** In a bidirectional relationship, you **must choose one side to be eager** (if needed for performance) **and configure the other side to be lazy** (or omit the relationship property entirely if navigation isn't needed from that side).

*   **Lazy Loading & Promises (Again):** When a relationship property is configured with `lazy` loading, accessing that property (e.g., `user.posts` or `post.author`) **does not directly return the related object(s)**. It returns a **Promise**.
*   **Fetching Trigger:** The database query to fetch the related data is triggered **only when you `await` or resolve the Promise** returned by the lazy property access.
    ```typescript
    // Example: Assuming 'user.posts' is lazy
    const user = await userRepository.findOne(userId); // Posts are NOT loaded yet
    // ... do something with user ...
    const userPosts = await user.posts; // <-- DATABASE QUERY IS EXECUTED HERE
    console.log(`User has ${userPosts.length} posts.`);
    ```

---

## Repositories

### Repositories: Data Access Objects

*   **Pattern Implementation:** TypeORM implements the widely used **Repository pattern**.
*   **Purpose:** Repositories provide a structured, object-oriented, and abstract way to interact with your database data for a specific entity type (like all operations related to the `User` entity).
*   **Abstraction:** They hide the underlying database access details (like SQL queries, connection specifics, TypeORM internal methods) from the rest of your application's logic (like service classes or controllers).
*   **Core Role:** A Repository is responsible for:
    *   Retrieving entities from the database.
    *   Persisting new entities (inserting).
    *   Updating existing entities.
    *   Deleting entities.
*   **TypeORM's Built-in Implementation:** TypeORM provides a default, pre-built `Repository` implementation for each of your entities.

### Accessing and Using TypeORM Repositories

*   You get an instance of the standard TypeORM repository for a specific entity type by calling `DataSource.getRepository(EntityClass)`. This is typically done after the DataSource has been initialized.

```typescript
// Example: Getting a repository instance
import { AppDataSource } from './dataSource'; // Assuming your DataSource is here
import { User } from './models/entities/User'; // Import the entity class

// Get the repository for the User entity
const userRepository = AppDataSource.getRepository(User);

// Now you can use the repository methods:
// const allUsers = await userRepository.find();
// const userById = await userRepository.findOne({ where: { id: userId } });
// const newUser = userRepository.create({ username: 'test', ... }); // Create entity instance
// await userRepository.save(newUser); // Save (insert) the new user
// await userRepository.delete(userId); // Delete by ID
```

*   **Standard Methods:** The base TypeORM repository provides many common database methods (like `find`, `findOne`, `save`, `remove`, `delete`, `count`, `createQueryBuilder`).
*   **Object-Oriented Queries:** When using these methods, you specify filtering, sorting, and other query parameters using **TypeScript/JavaScript objects and syntax** (e.g., `userRepository.find({ where: { isActive: true, type: 'admin' }, order: { username: 'ASC' } })`). TypeORM translates these object-oriented criteria into the correct SQL.

---

### Custom Repositories

While the built-in repository methods are sufficient for many tasks, you can create **custom repository classes** to encapsulate more complex or application-specific data access logic.

*   **Creating a Custom Repository:** You typically define a class that ** inherits from** TypeORM's base `Repository` class for your entity type.
*   **Purpose:** The main reason to create a custom repository is to add **domain-specific data access methods** that go beyond the basic CRUD operations (e.g., finding users by username, finding products with low stock, getting a user and all their related orders in one go).
*   **Implementation:** Inside your custom repository class, you define your own methods (e.g., `async findByUsername(username: string): Promise<User | null>`). These methods internally use the standard TypeORM repository functionalities (which are available via `this` when inheriting) or use TypeORM's Query Builder to build more complex queries.
*   **Centralization:** Placing all data access logic for a specific entity type within its custom repository **centralizes** this logic. This makes your code more organized, easier to understand, find, modify, and maintain.
*   **Separation:** It reinforces the separation between your Service/Business Logic layer (which *uses* the data) and your Data Access layer (which *gets* and *saves* the data).
*   **Example Snippet (Conceptual):**

    ```typescript
    // File: src/repositories/UserRepository.ts
    import { Repository } from 'typeorm'; // Import base Repository
    import { User } from '../models/entities/User'; // Import the entity class
    import { AppDataSource } from '../dataSource'; // Assuming your DataSource

    // Extend TypeORM's base Repository for the User entity
    export class UserRepository extends Repository<User> {
      constructor() {
        // Call the parent constructor, passing the target entity and the EntityManager
        // The EntityManager is typically obtained from the DataSource
        super(User, AppDataSource.manager);
      }

      // Add a custom method to find a user by username
      async findByUsername(username: string): Promise<User | null> {
        // Internally use TypeORM's findOne method with a 'where' clause
        // Access base methods via 'this' since we extended Repository
        return this.findOne({ where: { username: username } });
      }

      // Add a custom method using the Query Builder for a more complex query
      async findUsersWithRecentPosts(days: number): Promise<User[]> {
         const cutoffDate = new Date();
         cutoffDate.setDate(cutoffDate.getDate() - days);

         // Use TypeORM's createQueryBuilder to build a query programmatically
         return this.createQueryBuilder("user") // Start building query for 'user' entity (alias 'user')
             .leftJoinAndSelect("user.posts", "post") // Join with the 'posts' relation (alias 'post')
             .where("post.createdAt > :cutoffDate", { cutoffDate }) // Add a WHERE clause using a parameter
             .orderBy("user.username", "ASC") // Add sorting
             .getMany(); // Execute the query and return multiple results
      }
    }
    ```

---

## ORM Testing

### ORM Testing Strategies

Testing applications that interact with a database, especially when using an ORM like TypeORM, requires specific strategies to ensure tests are fast, reliable, isolated, and cover the interaction with the database correctly.

*   **Fundamental Rule:** The most critical rule is that your automated tests **must never, under any circumstances, interact with or modify the actual application database** used for development, staging, or production environments.
    *   **Why?** Using the real database in tests leads to flakiness (tests failing randomly due to external data changes), data pollution (test data mixed with real data), and the risk of data loss or corruption from test errors.

*   **Two Main Strategies for Testing Database Interactions:** Since you can't use the real application database, you use one of two main strategies for testing code that involves data persistence:

    1.  **Mocking the Persistence Layer:** The most common approach for **unit testing** logic that *calls* database operations. It involves replacing the ORM's functionality with mock objects.
    2.  **Using a Test DataSource:** This involves setting up and using a **separate, dedicated database instance solely for testing**. This test database should be completely isolated from any application database.

**Comparison Table: Mocking vs. Test DataSource for ORM Testing**

| Feature            | Mocking the Persistence Layer                                 | Using a Test DataSource                               |
| :----------------- | :------------------------------------------------------------ | :---------------------------------------------------- |
| **Purpose**        | **Unit testing** higher-level logic (e.g., service classes) that *depend on* ORM/repositories. | **Integration testing** (testing ORM+DB interaction) and **End-to-End testing** (testing the full stack including DB). |
| **Database Usage** | **No** actual database connection or interaction occurs.        | Uses a **real database instance** (the test database).|
| **Setup Complexity**| Requires setting up test mocks for ORM/repository methods.    | Requires configuring and managing the lifecycle of a test database (initialization, cleanup). |
| **Speed**          | **Very fast**, as it involves no database I/O.               | **Slower**, involves actual database operations.      |
| **Isolation**      | **Highly isolated**, tests focus purely on the component being tested. | Isolation requires careful setup (cleaning/recreating database per test). |
| **Realism**        | Less realistic regarding actual database behavior or ORM-to-DB translation. | More realistic, tests actual ORM-to-DB interaction.     |
| **What it Tests**  | Verifies that *your code* calls the correct ORM/repository methods with the right arguments and handles their simulated results/errors. | Verifies that *your code* uses the ORM correctly AND that the ORM translates operations into working database queries. |

---

#### Mocking the Persistence Layer

*   **Recommended For:** Unit testing classes that interact with repositories (e.g., **Service classes** that call repository methods, or **Controller methods** that call service methods which in turn call repositories).
*   **How it Works:** You replace the actual TypeORM Repository instances (or the `EntityManager` if working at a lower level) with **mock objects** provided by your testing framework (like Jest mocks).
*   **Controlling Behavior:** You configure these mock objects to simulate the expected behavior of the real repository methods. For example, you can tell a mock repository's `findOne` method to:
    *   Return a specific entity object.
    *   Return `null`.
    *   Throw a specific error (e.g., simulating a database error or a "not found" scenario).
*   **Verification:** In your test assertions, you verify that your code under test (e.g., the service method) calls the expected repository methods with the correct arguments.

**Example Snippet: Mocking a Repository for a Service Unit Test**

```typescript
// File: tests/unit/services/networkService.test.ts
import { NetworkService } from '../../../src/services/NetworkService'; // Service to test
import { NetworkRepository } from '../../../src/repositories/NetworkRepository'; // Dependency to mock
// Import custom errors if the service throws them
import { NotFoundError, ConflictError } from '../../../src/utils/errors';
import { Network } from '../../../src/models/entities/Network'; // Entity type

// --- 1. Create a mock for the dependency (NetworkRepository) ---
// Use Jest's jest.fn() for each method the service class is expected to call on the repository
const mockNetworkRepository: jest.Mocked<NetworkRepository> = {
  // Mock all methods the NetworkService might use on the repo
  find: jest.fn(),
  findOne: jest.fn(), // If service calls findOne
  findOneBy: jest.fn(), // If service calls findOneBy
  save: jest.fn(),
  create: jest.fn(), // Mock TypeORM's create method on the repo
  delete: jest.fn(), // Mock delete by criteria
  remove: jest.fn(), // Mock remove by entity instance
  // ... mock other necessary repository methods
  // If extending Repository, you might need to mock methods from the base class or AppDataSource.manager if accessed directly
  target: {} as any, // Mock properties required by Jest.Mocked
  manager: {} as any,
  metadata: {} as any,
} as any; // Use 'as any' or proper Jest typing for partial mocks

// --- 2. Create an instance of the service under test, injecting the mock dependency ---
// Assuming NetworkService constructor is: constructor(private networkRepo: NetworkRepository)
const networkService = new NetworkService(mockNetworkRepository); // Pass the mock here

// --- 3. Write test cases for the service methods ---
describe('NetworkService Unit Tests', () => {

  // Reset mocks before each test to ensure isolation
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should create a network if the code is unique', async () => {
    // Arrange: Define test data
    const newNetworkData: Partial<Network> = { code: 'NET001', name: 'TestNet' };
    const savedNetwork: Network = { ...newNetworkData, id: 'uuid-123', createdAt: new Date(), updatedAt: new Date() } as Network; // Simulate data returned by save

    // Configure the mock repository's behavior for this specific test case:
    // Simulate that findOneBy finds NO existing network
    mockNetworkRepository.findOneBy.mockResolvedValue(null);
    // Simulate that save returns the created network entity
    mockNetworkRepository.save.mockResolvedValue(savedNetwork);
    // Simulate TypeORM's create method returning an entity instance (often just returns the input data or simple object)
    mockNetworkRepository.create.mockReturnValue(newNetworkData as Network);


    // Act: Call the service method under test
    const result = await networkService.createNetwork(newNetworkData);

    // Assert: Check the outcome and verify interactions with the mock
    expect(result).toEqual(savedNetwork); // Check the return value
    expect(mockNetworkRepository.findOneBy).toHaveBeenCalledTimes(1); // Verify findOneBy was called
    expect(mockNetworkRepository.findOneBy).toHaveBeenCalledWith({ code: 'NET001' }); // Verify it was called with the correct argument
    expect(mockNetworkRepository.create).toHaveBeenCalledTimes(1); // Verify create method was called
    expect(mockNetworkRepository.create).toHaveBeenCalledWith(newNetworkData); // Verify it was called with the data
    expect(mockNetworkRepository.save).toHaveBeenCalledTimes(1); // Verify save was called
    // Expect save to be called with the entity instance returned by create
    expect(mockNetworkRepository.save).toHaveBeenCalledWith(expect.objectContaining(newNetworkData));
  });

  it('should throw ConflictError if network code already exists on creation', async () => {
    // Arrange: Define test data and simulate existing network
    const newNetworkData: Partial<Network> = { code: 'NET002', name: 'ExistingNet' };
    const existingNetwork: Network = { ...newNetworkData, id: 'existing-uuid' } as Network;

    // Configure mock to simulate finding an existing network with the same code
    mockNetworkRepository.findOneBy.mockResolvedValue(existingNetwork);

    // Act & Assert: Expect the service method call to reject with a specific error
    await expect(networkService.createNetwork(newNetworkData))
      .rejects // Expect the promise to be rejected
      .toThrow(ConflictError); // Expect the thrown error to be an instance of ConflictError

    // Verify interactions (should only call findOneBy)
    expect(mockNetworkRepository.findOneBy).toHaveBeenCalledTimes(1);
    expect(mockNetworkRepository.findOneBy).toHaveBeenCalledWith({ code: 'NET002' });
    expect(mockNetworkRepository.create).not.toHaveBeenCalled(); // Should not attempt to create/save
    expect(mockNetworkRepository.save).not.toHaveBeenCalled();
  });

  // Add more tests covering other methods (getNetworkByCode, deleteNetwork, updateNetwork, error cases, etc.)
});
```

---

#### Using a Test DataSource

*   **Recommended For:** **Integration tests** (testing the interaction between repositories and the actual database) and **End-to-End (E2E) tests** (testing the entire application stack including the database).
*   **How it Works:** These tests configure and use a **real TypeORM `DataSource` instance** that connects to a **separate database instance dedicated solely for testing**.
*   **Ideal Test Database:** For speed and isolation, an **in-memory database like SQLite (`:memory:`)** is often ideal for integration tests if your application logic and ORM features work the same way across different database types (which they usually do for common operations). If your application uses database-specific features or performance is critical, you might use a dedicated Docker container or test instance of your production database type.
*   **Setup and Teardown:** A critical part of these tests is the setup and teardown procedure for the test database. Before each test (or test suite), you typically:
    1.  Initialize the test `DataSource`.
    2.  Sync the schema (`synchronize: true` is acceptable *in test setup* because the data is throwaway, or use migrations if testing migrations).
    3.  Clear any data inserted by previous tests (e.g., `await dataSource.manager.clear(Entity)` or re-sync schema with `dropAndCreate`).
    After the tests, the `DataSource` should be closed.
*   **Verification:** These tests verify that your code correctly uses TypeORM methods and that TypeORM correctly translates those methods into working SQL queries against a real database.

**Example Snippet: Test DataSource Setup for Integration Tests**

```typescript
// File: tests/integration/repository/networkRepository.test.ts
import { DataSource } from 'typeorm'; // Import DataSource
import { NetworkRepository } from '../../../src/repositories/NetworkRepository'; // Repository to test
import { Network } from '../../../src/models/entities/Network'; // Entity
// Import other entities involved in tests...
import { User } from '../../../src/models/entities/User';

// --- 1. Configure a Test DataSource ---
// Use ':memory:' for an in-memory SQLite database for fast, isolated tests
const testDataSource = new DataSource({
  type: 'sqlite',
  database: ':memory:', // In-memory database
  entities: [Network, User /* list all entities involved in tests */], // List entities for this test suite
  synchronize: true, // Automatically create schema based on entities for each test run (safe in-memory)
  logging: false, // Turn off logging for cleaner test output
});

// --- 2. Get a repository instance from the test DataSource ---
let networkRepository: NetworkRepository; // Repository instance

// --- 3. Setup and Teardown the test DataSource ---
describe('NetworkRepository Integration Tests (SQLite In-memory)', () => {

  // Before ALL tests in this suite: initialize the DataSource
  beforeAll(async () => {
    await testDataSource.initialize();
    networkRepository = testDataSource.getCustomRepository(NetworkRepository); // Get your custom repository
    // If not using custom repositories, get the base one:
    // networkRepository = testDataSource.getRepository(Network);
  });

  // After ALL tests in this suite: destroy the DataSource
  afterAll(async () => {
    await testDataSource.destroy();
  });

  // Before EACH test: clear the data (synchronize: true helps, but clear adds certainty)
  beforeEach(async () => {
      // Optionally clear data from specific tables before each test
      // await testDataSource.manager.clear(Network);
      // await testDataSource.manager.clear(User);
      // Or if synchronize: true is used, dropping and recreating is another approach
  });


  // --- 4. Write test cases that use the repository and interact with the DB ---

  it('should find a network by its code', async () => {
    // Arrange: Create and save an entity using the repository
    const network = networkRepository.create({ code: 'TESTNET', name: 'Test Network' });
    await networkRepository.save(network); // Save to the in-memory DB

    // Act: Try to find the entity using the custom method
    const foundNetwork = await networkRepository.findByCode('TESTNET');

    // Assert: Verify the result
    expect(foundNetwork).not.toBeNull();
    expect(foundNetwork?.code).toBe('TESTNET');
    expect(foundNetwork?.name).toBe('Test Network');
    // Clean up is automatic with :memory: database per test run if beforeEach clears or schema syncs
  });

  it('should return null if a network code is not found', async () => {
    // Arrange: Database is clean from beforeEach

    // Act: Try to find a non-existent network
    const foundNetwork = await networkRepository.findByCode('NONEXISTENT');

    // Assert: Verify the result is null
    expect(foundNetwork).toBeNull();
  });

  it('should create and return a new network entity', async () => {
      // Arrange: Define data
      const networkData = { code: 'NEWNET', name: 'Newly Created' };

      // Act: Create and save using the repository
      const createdNetwork = networkRepository.create(networkData); // Create entity instance
      const savedNetwork = await networkRepository.save(createdNetwork); // Save to DB

      // Assert: Verify the saved entity has an ID and matches data
      expect(savedNetwork).toHaveProperty('id'); // Should get an auto-generated ID
      expect(savedNetwork.code).toBe(networkData.code);
      expect(savedNetwork.name).toBe(networkData.name);

      // Optionally verify by fetching it back
      const fetchedNetwork = await networkRepository.findOneBy({ code: 'NEWNET' });
      expect(fetchedNetwork).not.toBeNull();
      expect(fetchedNetwork?.id).toBe(savedNetwork.id); // Verify it was saved correctly
  });

  // Add more integration tests covering save, update, delete, relationship loading, etc.
});
```