---
title: MapReduce Programming
aliases: [Hadoop MapReduce patterns, word count Java, driver mapper reducer, TopK pattern, two-job chaining]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The Java MapReduce model as used in Labs 1–4: the word count reference implementation, the anatomy of the Driver/Mapper/Reducer classes, Writable types, reading job parameters from the Configuration, chaining two jobs, the in-reducer TopK pattern, and the user-bias normalization exercise. All code quoted from the lab projects in `labs/lab-1` … `labs/lab-4`.

## The Programming Model

A MapReduce application is three classes (Lab 2): **Driver** (configures and launches the job), **Mapper** (processes input key-value pairs and emits intermediate pairs), **Reducer** (receives, for each key, all values emitted for that key and emits the final pairs). Lab 2's first remark: *"Analyze the problem specification and decide if you really need all classes"* — some problems need no reducer (filter-only), some need no mapper logic.

Between map and reduce, the framework performs the **shuffle**: all pairs with the same key are routed to the same reducer. This is why the key you choose determines what your reducer sees — and why counting emitted pairs matters (Lab 2's complexity questions).

## Reference Implementation: Word Count (Lab 1)

From `labs/lab-1/mapreducebasicprojectwindows/src/main/java/it/polito/bigdata/hadoop/`:

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

## Writable Types Seen in the Labs

`LongWritable` (line offset, mapper input key), `Text` (strings), `IntWritable`, `FloatWritable` (Lab 4 averages), `NullWritable` (Lab 3 imports). Custom data types must implement the `Writable`/`Comparable` interfaces to travel between mappers and reducers — the Lab 3 template provides `WordCountWritable` (a `word,count` pair, usable as the *value* of emitted pairs).

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

Job parameters reach distributed classes through the `Configuration`. Lab 2 ex.1 ("accept the prefix as a command-line parameter") reads it in the **driver** with a `Scanner` and stores it:

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

## Pattern: Filtering (map-only style)

Lab 2 ex.1 keeps only lines `word\tfreq` whose word starts with a prefix. With `KeyValueTextInputFormat` the mapper already receives `(word, freq)` pairs, so the mapper applies the predicate and re-emits the pair unchanged. The reducer is only a pass-through (or can be omitted with 0 reducers — the lab keeps it and the framework copies pairs through).

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

**Job 2 — local TopK in the reducer.** Lab 3's `reducerbigdata2.java` keeps a `TopKVector<WordCountWritable>` as a *local, in-memory* structure and emits results only in `cleanup`:

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

## Pattern: Chaining Two Jobs

Labs 3 and 4 both run **two jobs in one driver**: job 2 reads the output folder of job 1 (`FileInputFormat.addInputPath(job2, outputDir)`) and writes to a new folder `args[3]`. Structure from the drivers:

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

The second job uses exactly **1 reducer** in both labs — the final output is a single sorted/part file.

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

## Pattern: Combiner

The Hadoop exercise set ships "withcombiner" variants (e.g. `exercises/hadoop/solutionsexmapreduce/exercise5withcombiner/`): a combiner is set in the driver with one extra line, between mapper and reducer:

```java
job.setMapperClass(MapperBigData.class);
// Set combiner class
job.setCombinerClass(CombinerBigData.class);
job.setReducerClass(ReducerBigData.class);
```

A combiner runs on map output *locally on each mapper*, pre-aggregating values to reduce shuffle traffic — safe when the reduce operation is associative and commutative (like the sum of ones in word count).

## Key Takeaways

- Template: Driver (job config) + Mapper (emit `(key, value)`) + Reducer (aggregate per key); the chosen key defines the shuffle and thus the whole computation.
- Always set: input/output paths and formats, mapper class + map output types, reducer class + final output types, number of reducers.
- Headers are filtered in code (counter on first line, or string check on the first field).
- Non-command-line parameters travel via `Configuration` (`conf.set` in driver, `context.getConfiguration().get` in mapper/reducer).
- Two-job chains feed job 1's HDFS output into job 2 (often with `KeyValueTextInputFormat` and 1 reducer).
- TopK is computed **in-memory in the reducer** (`TopKVector` + `setup`/`cleanup`), never by sorting all data.
- Keep sparse-group data local (HashMap in the reducer) instead of shuffling it.
