---
title: MapReduce Programming
aliases: [Hadoop MapReduce patterns, word count Java, driver mapper reducer, TopK pattern, inverted index, reduce side join, distributed cache, MultipleInputs, MultipleOutputs, two-job chaining]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The Java MapReduce model as used in Labs 1–4 plus the professor's design-pattern catalogs (slides 05–07): the word count reference implementation, Driver/Mapper/Reducer anatomy, Writable types, the summarization patterns (numerical, inverted index, counters), the filtering patterns (filter, Top-K with in-mapper lists, distinct), the organization/join patterns (shuffling, job chaining, reduce-side and map-side natural join), the advanced mechanisms (MultipleInputs, MultipleOutputs, DistributedCache), and worked examples from the professor's exercise deck. Lab-specific solutions (two-job chains, TopKVector, user-bias normalization) are kept verbatim from the lab projects.

## The Programming Model

A MapReduce application is three classes (Lab 2; slide 04): **Driver** (configures and launches the job), **Mapper** (processes input key-value pairs and emits intermediate pairs), **Reducer** (receives, for each key, all values emitted for that key and emits the final pairs). Lab 2's first remark: *"Analyze the problem specification and decide if you really need all classes"* — some problems need no reducer (filter-only/map-only), some need no mapper logic.

Between map and reduce, the framework performs the **shuffle**: all pairs with the same key are routed to the same reducer. This is why the key you choose determines what your reducer sees — and why counting emitted pairs matters (Lab 2's complexity questions). Formal signatures and the framework anatomy (Writable types, InputFormats, job/task/split definitions) are in [02 Hadoop and HDFS](../02-hadoop-and-hdfs/02-hadoop-and-hdfs.md).

## Reference Implementation: Word Count (Lab 1 + slide 04)

From `labs/lab-1/mapreducebasicprojectwindows/src/main/java/it/polito/bigdata/hadoop/` — identical in structure to the slide-04 word count.

### Mapper

```java
class MapperBigData extends Mapper<
                    LongWritable, // Input key type (byte offset)
                    Text,         // Input value type (one line)
                    Text,         // Output key type
                    IntWritable> {// Output value type

    protected void map(LongWritable key, Text value, Context context)
            throws IOException, InterruptedException {

        // Split each sentence in words. Use whitespace(s) as delimiter
        String[] words = value.toString().split("\\s+");

        for (String word : words) {
            String cleanedWord = word.toLowerCase();
            // emit the pair (word, 1)
            context.write(new Text(cleanedWord), new IntWritable(1));
        }
    }
}
```

### Reducer

```java
class ReducerBigData extends Reducer<
                Text,           // Input key type
                IntWritable,    // Input value type
                Text,           // Output key type
                IntWritable> {  // Output value type

    protected void reduce(Text key, Iterable<IntWritable> values, Context context)
            throws IOException, InterruptedException {

        int occurrences = 0;
        // Iterate over the set of values and sum them
        for (IntWritable value : values) {
            occurrences = occurrences + value.get();
        }
        context.write(key, new IntWritable(occurrences));
    }
}
```

### Driver

```java
public class DriverBigData extends Configured implements Tool {

    @Override
    public int run(String[] args) throws Exception {
        Path inputPath;
        Path outputDir;
        int numberOfReducers;

        // Parse the parameters
        numberOfReducers = Integer.parseInt(args[0]);
        inputPath = new Path(args[1]);
        outputDir = new Path(args[2]);

        Configuration conf = this.getConf();
        Job job = Job.getInstance(conf);
        job.setJobName("PM10 Pollution Analysis per City Zone");

        FileInputFormat.addInputPath(job, inputPath);
        FileOutputFormat.setOutputPath(job, outputDir);
        job.setJarByClass(DriverBigData.class);
        job.setInputFormatClass(TextInputFormat.class);
        job.setOutputFormatClass(TextOutputFormat.class);

        job.setMapperClass(MapperBigData.class);
        job.setMapOutputKeyClass(Text.class);
        job.setMapOutputValueClass(IntWritable.class);

        job.setReducerClass(ReducerBigData.class);
        job.setOutputKeyClass(Text.class);
        job.setOutputValueClass(IntWritable.class);

        job.setNumReduceTasks(numberOfReducers);

        if (job.waitForCompletion(true)) return 0; else return 1;
    }

    public static void main(String args[]) throws Exception {
        int res = ToolRunner.run(new Configuration(), new DriverBigData(), args);
        System.exit(res);
    }
}
```

Driver checklist (every job needs all of these): input/output paths, `setJarByClass`, input/output format classes, mapper class **with its own output key/value classes** (`setMapOutputKeyClass`/`setMapOutputValueClass`), reducer class with final output classes, `setNumReduceTasks`, `waitForCompletion`.

## Design Patterns — Summarizations (slides 05, Part I)

Used to produce top-level/summarized views of the data.

### Numerical summarization

- **Goal**: group records by a key field and compute a numerical aggregate (average, max, min, standard deviation) per group.
- **Mappers** emit `(key fields, summary fields)`; **reducers** compute the final statistic per group; **combiners** help only if the statistic is commutative and associative.
- Known uses: word count, record count, min/max/count, average/median/std-dev per group.

Worked example (professor's exercise deck): **average PM10 per sensor**, input `sensorId,date,PM10 value` → mapper emits `(s1, 20.5)…`, reducer sums and divides ⇒ `(s1, 45.4)`, `(s2, 34.3)`. The **max-and-min variant** uses a complex value (`max=…_min=…`) — exactly the custom `Writable` case of note 02. The **count-above-threshold variant** emits `(sensor, value)` only when `value > 50` and the reducer counts non-null values per sensor ⇒ `(s1, 2), (s2, 1)`.

### Inverted index

- **Goal**: build an index from data to support faster searches — map terms to a list of identifiers.
- **Mappers** emit `(keyword, record identifier)`; **reducers** simply **concatenate** the identifiers: `(keyword X, list of identifiers)`.
- **Combiners usually not useful** (nothing to aggregate; sometimes deduplication).
- Most famous use: web search engine — word → list of URLs.

Worked example (exercises): sentences `sentenceId\tsentence`, ignore "and", "or", "not" ⇒ `(hadoop, [Sentence#1, Sentence#2, Sentence#3])`, `(spark, [Sentence#1, Sentence#2])`, …

### Counting with counters

- **Goal**: count summarizations of datasets without emitting pairs.
- **Map-only job**: mappers increment counters; **no reducers, no combiners**; results are stored/printed by the **Driver** at the end of the job (enum counters — see note 02).
- Known uses: count number of records (exercise: "Total count" over the PM10 file ⇒ `6`), count a small number of unique instances, summarizations.

## Design Patterns — Filtering (slides 05)

Used to select the subset of records of interest.

### Filtering

- **Mappers** emit one `(primary key, record)` pair for each record satisfying the rule; **the reducer is useless** — run a **map-only job** (`setNumReduceTasks(0)`).
- Known uses: record filtering, tracking events, distributed grep, data cleaning. Exercise: *select outliers* = print records with `PM10 < threshold` (threshold from `args`); *filter readings* = keep lines with temperature > 30.0.

### Top K

- **Goal**: select the top-K records by a ranking function (outliers, most profitable items).
- **Mappers**: initialize an **in-mapper local top-k list** in `setup` (k is small, e.g. 10); `map` updates it; `cleanup` emits the k pairs with a **"null key"** and the record as value.
- **Reducer**: **one single reducer** — it receives all pairs under the same key, so `reduce` is called once and merges the local lists into the final top-k.
- Assumption: ranked records are unique (otherwise aggregate first in a preceding job).
- Exercise: *Top 1 / Top 2 most profitable dates* on `date\tincome` with tie-break "first date" — local top-k in each mapper, merge in the single reducer. The Spark-side equivalent is `top(n, key)` (note 05).

### Distinct

- **Mappers** emit `(record, null)` for each input record; **reducers** emit one pair per distinct key (each key group collapses to one).
- Known uses: duplicate removal, distinct value selection. Exercise: *Dictionary* = list of distinct words (mapper emits `(word, NullWritable)`, reducer emits each key once); the *word→integer mapping* variant assigns increasing integers in the single reducer.

## Design Patterns — Organization and Workflow (slides 07, Part II)

### Shuffling (randomization)

- **Goal**: randomize the order of records (anonymization, selecting a random subset).
- **Mappers** emit `(random key, record)`; **reducers** emit each value of `(key, [values])` unchanged — the random keys redistribute the records.

### Job chaining

- **Goal**: execute a sequence of jobs, synchronizing them. Each phase = one MapReduce job; the output of a phase is the input of the next; **one single Driver contains the workflow** and executes the jobs in order. More complex parallel workflows are possible but synchronization becomes harder.
- Implementation (as in Labs 3–4):

```java
Job job = Job.getInstance(conf);
// ... configure job 1 (TextInputFormat) ...
if (job.waitForCompletion(true) == true) {
    Job job2 = Job.getInstance(conf);
    // input of job2 = output of job1
    FileInputFormat.addInputPath(job2, outputDir);
    FileOutputFormat.setOutputPath(job2, outputDir2);
    job2.setInputFormatClass(KeyValueTextInputFormat.class);
    // ... mapper2, reducer2 ...
    job2.setNumReduceTasks(1);   // single reducer: one output part file
    if (job2.waitForCompletion(true) == true) exitCode = 0; else exitCode = 1;
} else exitCode = 1;
```

Both labs use exactly **1 reducer** in job 2 — the final output is a single sorted/part file.

## Advanced Mechanisms (slides 06)

### Multiple inputs (one mapper per dataset)

When data are read from two or more datasets with different formats: **one different mapper per input dataset**, all emitting key-value pairs **consistent in data types**. Typical use case: same measurement, different vendor formats.

```java
MultipleInputs.addInputPath(job, new Path(args[1]), TextInputFormat.class, Mapper1.class);
MultipleInputs.addInputPath(job, new Path(args[2]), TextInputFormat.class, Mapper2.class);
```

Exercise: *maximum temperature per date from two files* with column orders `sensorID,date,hour,temperature` and `date,hour,temperature,sensorID` — two mappers parse the two formats and both emit `(date, temperature)`; the reducer computes the max ⇒ `2016-01-01 30.2`.

### Multiple outputs (splitting by prefix)

Store output pairs in **different files inside one single output directory**, one prefix per "type" of output (useful for split/filter operations):

```java
// in the Driver, once per output file type
MultipleOutputs.addNamedOutput(job, "hightemp", TextOutputFormat.class, Text.class, NullWritable.class);
MultipleOutputs.addNamedOutput(job, "normaltemp", TextOutputFormat.class, Text.class, NullWritable.class);

// in the mapper (map-only job) or reducer
private MultipleOutputs<Text, NullWritable> mos = null;
protected void setup(Context context) { mos = new MultipleOutputs<Text, NullWritable>(context); }
// in map/reduce:
mos.write("hightemp", key, value);      // -> files with prefix "hightemp-"
mos.write("normaltemp", key, value);
protected void cleanup(Context context) { mos.close(); }
```

There will be one output file of each type per reducer (per mapper for map-only jobs). Exercise: split sensor readings into `high-temp-m-00001` (> 30.0) and `normal-temp-m-00001` (≤ 30.0).

### DistributedCache (shared read-only files)

Some applications need small read-only files available **locally on every node** that runs tasks:

- In the Driver: `job.addCacheFile(new Path("hdfs path/filename").toUri());`
- At job initialization Hadoop creates a **local copy** of the cached files **only in the nodes used to run the application**; all mappers/reducers of a node share one local copy (efficiency depends on how many tasks run per node — without the cache each task would re-read the file from HDFS, which is slower).
- Read it in `setup`:

```java
protected void setup(Context context) throws IOException, InterruptedException {
    URI[] urisCachedFiles = context.getCacheFiles();
    // the local copy is in the root of a local temporary folder: use only the file name
    BufferedReader file = new BufferedReader(new FileReader(
            new File(new Path(urisCachedFiles[0].getPath()).getName())));
    String line;
    while ((line = file.readLine()) != null) { /* process the line */ }
    file.close();
}
```

Exercises using the cache: *stopword elimination* (large sentences file + small stopword file, map-only filtering), *word→integer conversion* with `dictionary.txt` (`word\tinteger`), *categorization rules* (small file of `Gender=<v> and YearOfBirth=<v> -> Category` rules, mutually exclusive; users matching no rule get `Unknown`).

## Pattern: Joins (slides 07)

Used to implement the relational join operators; the slides focus on the **natural join** (the pattern is analogous for theta-, semi-, outer-joins — only the "local join" logic changes).

### Reduce-side natural join (both tables large)

- **Two mapper classes**, one per table. Each mapper emits one pair per record: **key = value of the common attribute(s)**, **value = table name + record content**.
  Example: `(u1, "Users:name=Paolo,surname=Garza")` from Users, `(u1, "Likes:movieGenre=horror")` from Likes.
- **Reducers** iterate over the values of each key and compute the *local natural join*: one output pair for each combination of one record from each table. From `(u1, ["User:name=Paolo,surname=Garza", "Likes:movieGenre=horror", "Likes:movieGenre=adventure"])` ⇒
  `(u1, "name=Paolo,surname=Garza, genre=horror")`, `(u1, "name=Paolo,surname=Garza, genre=adventure")`.
- Exercise: *Mapping question–answers* — questions `QuestionId,Timestamp,Text` and answers `AnswerId,QuestionId,Timestamp,Text` joined on QuestionId ⇒ `Q1,What is ..?,A1,It is ..` etc.

### Map-side natural join (one table small)

- One table is large, the other **fits in main memory**: **map-only job**; the small table is provided to every mapper through the **DistributedCache** and loaded (e.g., into a `HashMap`) in `setup`; each mapper joins its records of the large table locally. No shuffle at all.

## Writable Types Seen in the Labs

`LongWritable` (line offset, mapper input key), `Text` (strings), `IntWritable`, `FloatWritable` (Lab 4 averages), `NullWritable` (Lab 3 imports, distinct pattern). Custom data types must implement the `Writable`/`Comparable` interfaces to travel between mappers and reducers — the Lab 3 template provides `WordCountWritable` (a `word,count` pair, usable as the *value* of emitted pairs).

## Input Formats and Header Filtering

- `TextInputFormat` (Labs 1–4): each line is a record; key = byte offset, value = line content.
- `KeyValueTextInputFormat` (Labs 2 ex.1, 3, 4 second job): the input is already `key\tvalue` per line, so key and value are split automatically — used when the input is the output of a previous word-count job.
- **Headers**: `Reviews.csv` starts with `Id,ProductId,…` and "the header of the file must be filtered" (Lab 4). The lab-4 solution skips it with a counter in the mapper:

```java
private int counter;

protected void setup(Context context) { counter = 0; }

protected void map(LongWritable key, Text value, Context context) {
    String[] fields = value.toString().split(",");
    if (counter == 0) {
        counter++;                       // first line = header, skip it
    } else {
        context.write(new Text(fields[2]),            // UserId
                      new Text(fields[1] + "," + fields[6])); // ProductId,Score
    }
}
```

## Passing Parameters to Mappers/Reducers

Job parameters reach distributed classes through the `Configuration` (see note 02). Lab 2 ex.1 ("accept the prefix as a command-line parameter") reads it in the **driver** with a `Scanner` and stores it:

```java
Scanner scanner = new Scanner(System.in);
System.out.print("Insert the prefix: ");
conf.set("prefix", scanner.nextLine());
```

and every **mapper** instance retrieves it:

```java
Configuration conf = context.getConfiguration();
String prefix = conf.get("prefix");

if (key.toString().toLowerCase().startsWith(prefix)) {
    context.write(key, new IntWritable(Integer.parseInt(value.toString())));
}
```

## Pattern: Filtering in the Labs (Lab 2 ex.1)

Lab 2 ex.1 keeps only lines `word\tfreq` whose word starts with a prefix. With `KeyValueTextInputFormat` the mapper already receives `(word, freq)` pairs, so the mapper applies the predicate and re-emits the pair unchanged. The reducer is only a pass-through (the lab keeps it; a true map-only version would set 0 reducers).

## Pattern: Frequency Distribution / Bucketing

Lab 2 ex.2 (redone in Spark in Lab 5 task 3): given `word\tfreq`, count how many words fall in each frequency group — Group 0: [0,100), Group 1: [100,200), … Group 5: [500,+inf). The bucket id is computed from the value, emitted as key, and summed by the reducer (the expected output for the toy input is `Group0 2`, `Group3 1`, `Group5 2`). In Java this is a mapper that maps each line to `(group(freq), 1)` and a reducer summing the ones — the same reducer shown above for word count.

## Pattern: Pairs from Lists ("People also like…", Lab 3)

Input: one line per reviewer, `reviewerId,product1,product2,…`. Goal: top-100 product pairs reviewed together.

**Job 1 — generate and count pairs.** The filled skeleton (`labs/lab-3/lab3windows-skeleton/.../mapperbigdata1.java`) emits each unordered pair once, sorted inside the pair so that `(p1,p2)` and `(p2,p1)` collapse to the same key:

```java
String[] fields = value.toString().split(",");
if (fields.length == 2) {
    context.write(new Text(fields[1] + "," + fields[1]), new IntWritable(1));
} else {
    for (int i = 1; i < fields.length - 1; i++) {
        for (int j = i + 1; j < fields.length; j++) {
            if (fields[i].compareTo(fields[j]) > 0)
                context.write(new Text(fields[i] + "," + fields[j]), new IntWritable(1));
            else
                context.write(new Text(fields[j] + "," + fields[i]), new IntWritable(1));
        }
    }
}
```

Its reducer sums the counts per pair (word-count reducer again).

**Job 2 — local TopK in the reducer.** Lab 3's `reducerbigdata2.java` keeps a `TopKVector<WordCountWritable>` as a *local, in-memory* structure and emits results only in `cleanup` (the Top-K pattern above, applied to the pair counts):

```java
private TopKVector<WordCountWritable> topKVector;

protected void setup(Context context) {
    topKVector = new TopKVector<>(100);
}

protected void reduce(Text key, Iterable<IntWritable> values, Context context) {
    int sum = 0;
    for (IntWritable value : values) sum += value.get();
    topKVector.updateWithNewElement(new WordCountWritable(key.toString(), new Integer(sum)));
}

protected void cleanup(Context context) {
    for (WordCountWritable wc : topKVector.getLocalTopK()) {
        context.write(new Text(wc.getWord()), new IntWritable(wc.getCount()));
    }
}
```

The lab 3 handout documents the two provided utilities: `WordCountWritable(word, count)` with `getWord()/getCount()` and `compareTo`, and `TopKVector<T>(k)` with `updateWithNewElement(t)` and `getLocalTopK()`.

## Pattern: Normalization per Group (Lab 4)

Goal: normalize Amazon ratings by user bias, then average per product. The matrix is sparse: "given an arbitrary user his/her list of ratings can be stored in a local Java variable" — use this "to avoid sending unnecessary (key, value) pairs on the network".

- **Mapper1** skips the header and emits `(UserId, "ProductId,Score")` (code above).
- **Reducer1** (key = user) builds a local `HashMap<product, score>`, computes the user's **mean**, subtracts it from each rating, and emits `(ProductId, normalizedScore)`:

```java
HashMap<String, Float> hashmap = new HashMap<>();
float mean = 0;

for (Text value : values) {
    String[] fields = value.toString().split(",");
    hashmap.put(fields[0], Float.parseFloat(fields[1]));
}
for (String k : hashmap.keySet()) mean += hashmap.get(k);
mean /= hashmap.size();
for (String k : hashmap.keySet()) {
    context.write(new Text(k), new FloatWritable(hashmap.get(k) - mean));
}
```

- **Reducer2** (key = product, job 2) averages the normalized scores and writes `(product, normalized average rating)`.

The handout's toy example shows why: product B5 has a bad raw average but, after removing that user A5 rates harshly, its normalized average improves relative to others.

## Pattern: Combiner (Lab solutions "withcombiner")

The Hadoop exercise set ships "withcombiner" variants (e.g. `exercises/hadoop/solutionsexmapreduce/exercise5withcombiner/`): a combiner is set in the driver with one extra line, between mapper and reducer:

```java
job.setMapperClass(MapperBigData.class);
// Set combiner class
job.setCombinerClass(CombinerBigData.class);
job.setReducerClass(ReducerBigData.class);
```

A combiner runs on map output *locally on each mapper*, pre-aggregating values to reduce shuffle traffic — safe when the reduce operation is associative and commutative (like the sum of ones in word count). Full caveats (not guaranteed to run, same class as the reducer in ~99% of cases) in note 02.

## More Worked Exercises (professor's deck, patterns recap)

| Exercise | Pattern | Sketch |
|---|---|---|
| Word count on a **folder** of files | summarization | same code; `FileInputFormat.addInputPath` on a folder reads all files inside |
| PM10: number of days above threshold per sensor | filter + count | mapper emits `(sensor, 1)` only for `value > 50`; reducer counts |
| PM10: list of dates above threshold per zone | summarization | reducer concatenates dates: `(zone1, [2016-01-03, 2016-01-02])` |
| Average / max+min per sensor | numerical summarization | float average or complex `max=…_min=…` Writable value |
| Inverted index on sentences (excluding and/or/not) | inverted index | `(word, sentenceId)`, reducer concatenates |
| Total number of records | counters | map-only, `context.getCounter(...).increment(1)` |
| Top-1 / Top-2 most profitable date (tie: first date) | Top-K | in-mapper top-k + single reducer; compare (income, date) |
| Dictionary of distinct words (+ word→integer) | distinct | `(word, NullWritable)`; integer variant counts keys in the single reducer |
| Max temperature per date, two file formats | multiple inputs | one mapper per format, common `(date, temperature)` pairs |
| Filter readings > / ≤ 30.0; split into high/normal temp files | filtering / multiple outputs | map-only; MultipleOutputs with `high-temp`/`normal-temp` prefixes |
| Stopword elimination | distributed cache + map-only filter | load stopwords in `setup`, emit cleaned sentences |
| Friends of a specific user; potential friends (≥1 common friend); remove direct friends | filter + set logic | mapper emits both `(u1,u2)` and `(u2,u1)`; per-user reducer computes intersections/differences |
| List of friends for each user | inverted index | emit both directions, reducer concatenates: `User1: User2 User3 User4` |
| Word→integer substitution via dictionary | distributed cache | load `word\tinteger` in `setup`, substitute in `map` |
| Categorization rules (mutually exclusive, default `Unknown`) | distributed cache + map-only | apply rules per record in the mapper |
| Question–answer mapping | reduce-side join | join on QuestionId, combine texts |
| Users liking both Commedia and Adventure (keep duplicates) | filter + per-user set | emit `(UserId, genre)` pairs, reducer checks both genres, output `Gender,YearOfBirth` via second job or in-memory user table |

## Key Takeaways

- Template: Driver (job config) + Mapper (emit `(key, value)`) + Reducer (aggregate per key); the chosen key defines the shuffle and thus the whole computation.
- Pattern vocabulary: summarization (group-by + aggregate), inverted index, counters (map-only), filtering (map-only), Top-K (in-mapper lists + single reducer), distinct, shuffling, job chaining, reduce-side/map-side join.
- Map-only jobs (`setNumReduceTasks(0)`) whenever the reducer is useless; MultipleInputs for heterogeneous sources; MultipleOutputs to split results by prefix; DistributedCache for small shared read-only files.
- Headers are filtered in code (counter on first line, or string check on the first field).
- Non-command-line parameters travel via `Configuration` (`conf.set` in driver, `context.getConfiguration().get` in mapper/reducer).
- TopK is computed **in-memory** (`TopKVector` + `setup`/`cleanup`, or the slide's local top-k list), never by sorting all data.
- Keep sparse-group data local (HashMap in the reducer) instead of shuffling it.
