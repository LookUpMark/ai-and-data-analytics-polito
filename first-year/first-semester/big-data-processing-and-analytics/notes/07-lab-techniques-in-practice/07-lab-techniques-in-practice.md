---
title: Lab Techniques in Practice
aliases: [BDPA lab recipes, spark-submit usage, JupyterHub workflow, KML output, dirty data filtering, header filtering]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> A cookbook of the recurring techniques from Labs 5–9: setting up PySpark in notebooks, running scripts with `spark-submit` and command-line arguments, filtering headers and dirty records, parsing timestamps, producing a single output file, formatting KML output, retrieving logs, and inspecting `part-*` results. Each recipe cites the lab where it appears.

## 1. Notebook Setup (every Spark lab)

Standard first cells, identical in Labs 5–8:

```python
import findspark
import pyspark

findspark.init()
sc = pyspark.SparkContext.getOrCreate()
```

For Spark SQL labs, add:

```python
from pyspark.sql import SparkSession
spark = SparkSession.builder.getOrCreate()
```

Notebook flavors on `jupyter.polito.it` (Lab 5): **PySpark (Local)** runs driver and executors on the gateway; **PySpark (Yarn)** runs them on the cluster. Data is read from/stored on HDFS in both cases.

## 2. Script + `spark-submit` (Lab 5)

A `.py` application must create and stop the context itself:

```python
from pyspark import SparkConf, SparkContext
import sys

if __name__ == "__main__":
    prefix       = sys.argv[1]   # application parameters on the command line
    inputFolder  = sys.argv[2]
    outputFolder = sys.argv[3]

    conf = SparkConf().setAppName("Lab5 Spark application")
    sc = SparkContext(conf=conf)

    # ... application ...

    sc.stop()
```

Run from a Terminal on the gateway (Lab 5):

```bash
spark-submit --master yarn --deploy-mode client lab5.py ho /data/students/bigdata-01QYD/Lab2/ ex5_out
spark-submit --master local --deploy-mode client lab5.py ...
```

The lab asks to run the **same script both ways** and check the results are consistent. The exercise solutions carry the same habit — e.g. the Ex. 39/42 notebooks store `# argv[1]`, `# argv[2]` comments next to their path variables.

## 3. Header Filtering

Every structured input of the course has a header line. Patterns actually used:

```python
# Lab 6 (Reviews.csv, comma-separated, header starts with 'Id')
filteredRDD = inputRDD.filter(lambda line: line.find('Id') == -1)

# Lab 7 (register.csv, first field of the header is 'station')
registerRDD = sc.textFile("sampleData/registerSample.csv").filter(lambda x: "station" not in x)

# Lab 4, Java: skip the first line seen by each mapper via a counter
```

Care: `find('Id') == -1` drops any line *containing* "Id" — acceptable for the numeric/product-id datasets of this course; with DataFrames prefer `header=True` (Lab 8), which consumes the header without touching the data.

## 4. Dirty-Record Filtering (Labs 7–8)

Some `register.csv` lines have `used slots = 0 and free slots = 0` (monitoring failures) and "must be filtered before performing the analysis":

```python
# RDD, exploiting the tab-separated format
registerRDD = registerRDD.filter(lambda x: "\t0\t0" not in x)

# DataFrame / SQL
filteredDF = registerDF.filter("not(used_slots = 0 and free_slots = 0)")
```

The same idea in exercises: PM10 readings above a threshold, log lines containing a substring — always one `filter` first, then analyze.

## 5. Timestamps → (weekday, hour)

Lab 7 hint (verbatim from the handout):

```python
from datetime import datetime

timestamp = "2008-05-15 12:01:00"
datetimeObject = datetime.strptime(timestamp, "%Y-%m-%d %H:%M:%S")
dayOfTheWeek = datetimeObject.strftime("%a")   # 'Thu'
hour = datetimeObject.hour                     # 12
```

In Spark SQL the built-ins replace the Python library (Lab 8): `date_format(timestamp,'EE')`, `hour(timestamp)`. Weekday names are three-letter abbreviations (`Mon`, `Tue`, …) in both worlds — Lab 7's tie-break sorts them with `day_list = ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]`.

## 6. Single Output File and `part-*` Inspection

- `saveAsTextFile(folder)` writes **one file per partition**. Lab 7 hint: "To create one single output file, set the number of partitions of the final RDD to 1 by using `coalesce(1)` before invoking `saveAsTextFile()`."
- Lab 6 shows partition control explicitly:

```python
orderedByValueRDD.getNumPartitions()      # inspect
orderedByValueRDD = orderedByValueRDD.coalesce(2)
orderedByValueRDD.saveAsTextFile("output")
```

- Inspect results in HUE or read the `part-00000…` files. Lab 6's stored output (`labs/lab-6/output/`) looks like:

```
('B1,B3', 3)
('B1,B5', 3)
('B3,B5', 3)
```

Python tuples are stringified as-is; if the required output format is `word,freq` without quotes/parentheses, `map` the pairs into formatted strings before saving.

## 7. KML Output (Lab 7)

Required marker format, one per line, station id in `<name>`, timeslot fields and criticality in `<ExtendedData>`, coordinates in `<Point>`:

```
<Placemark><name>44</name><ExtendedData><Data name="DayWeek"><value>Mon</value></Data><Data name="Hour"><value>3</value></Data><Data name="Criticality"><value>0.5440729483282675</value></Data></ExtendedData><Point><coordinates>2.189700,41.379047</coordinates></Point></Placemark>
```

The handout's recipe: produce the placemark strings as RDD elements, `coalesce(1)`, `saveAsTextFile`, then copy the generated part file inside a KML wrapper:

```xml
<kml xmlns="http://www.opengis.net/kml/2.2"><Document>
<!-- placemark lines pasted here -->
</Document></kml>
```

Visualize on `https://kmlviewer.nsspot.net` or `https://www.gpsvisualizer.com`. Station coordinates come from `stations.csv` (`stationId\tlongitude\tlatitude\tname`) — stations below the threshold are simply not emitted.

## 8. Logs and Application IDs (Labs 6–9)

```bash
# find your applications (finished included), then read their logs
yarn application -list -appStates ALL | grep 'sXXXXXX'
yarn logs -applicationId application_1521819176307_2195
```

Alternatives for the id: the submission output on the terminal, or the HUE job browser (`hue.polito.it/hue/jobbrowser/#!jobs`). This is the standard loop when working from outside Polito.

## 9. Small Samples Before the Big File

Every lab ships a small companion for local reasoning: `SampleLocalFile.csv` (Lab 5), `ReviewsSample.csv` (Labs 6, 9 — also available from the course web page), `registerSample.csv`/`stations.csv` (Labs 7–8), `AmazonTransposedDataset_Sample.txt` (Lab 3), `ReviewsSample.csv` for Lab 4. The workflow is always: develop/check on the sample, then run on the HDFS shared folder under `/data/students/bigdata-01QYD/…`.

## 10. Complexity Bookkeeping (Lab 2's questions, forever)

Before submitting, ask for each step:

- pairs/bytes **read from HDFS**;
- pairs/bytes **emitted by mappers** (RDD producers) and hence shuffled **on the network**;
- can data be pre-aggregated before the shuffle (`reduceByKey` instead of `groupByKey`, combiner in MapReduce)?
- can per-group data be kept **local** (in-memory HashMap/TopK in a reducer, Lab 3/4 pattern)?

## 11. End-of-Session Checklist

1. Save notebooks/scripts; download them if you need them later.
2. Stop the JupyterHub container: `File -> Hub Control Panel -> Stop My Server` (required at the end of every lab session).
3. Optionally clean output folders on HDFS (`hdfs dfs -rm -r`) so the next run succeeds.

## Key Takeaways

- Same skeleton every time: `findspark.init()` → `SparkContext.getOrCreate()` → read/filter → transform → act/save.
- Headers and dirty lines are always removed **first**, with a pattern chosen for the separator at hand.
- One output file per partition; `coalesce(1)` when a single file (KML, sorted result) is required.
- `yarn application -list -appStates ALL | grep <user>` + `yarn logs -applicationId <id>` for debugging; HUE job browser as the GUI alternative.
- Sample files first, shared HDFS folders after; stop the container when leaving.
