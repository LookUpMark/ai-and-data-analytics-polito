---
title: Exam Exercises Review
aliases: [Spark exercises 30-46, PM10 exercises solutions, exam preparation BDPA]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> A review of the official exercise book (`exercises/spark/02-spark-exercises-bigdatanb.pdf`, exercises 30–46) with fully worked PySpark solutions for the representative ones: filtering a log (Ex. 30/31), maximum/top-k PM10 values (Ex. 32–34), and multi-input aggregation with `groupByKey`/`cogroup` (Ex. 39/42). Code is taken from the official solution notebooks in `exercises/spark/solutionsexspark30-46/`.

## The Exercise Catalog

Input formats (from the exercise book):

- **PM10 sensors** (Ex. 32–39): CSV lines `sensorId,date,PM10 value`, e.g. `s1,2016-01-01,20.5`. Asked: max value, top-k values, lines/dates at the max, average, per-sensor max, sensors with ≥2 readings > 50, critical dates per sensor (also with empty lists), order by critical days, top-k critical sensors.
- **Web server log** (Ex. 30–31): lines like `66.249.69.97 - - [24/Sep/2014:22:25:44 +0000] "GET http://www.google.com/bot.html"`. Asked: lines containing "google"; distinct IPs of connections to `www.google.com`.
- **Questions/answers** (Ex. 42): `QuestionId,Timestamp,Text` and `AnswerId,QuestionId,Timestamp,Text` → question with its list of answers.
- **Bike sharing** (Ex. 43): readings `stationId,date,hour,minute,num_of_bikes,num_of_free_slots` + neighbors file → criticality percentages, most critical (timeslot, station), full-station propagation checks.
- **Misleading profiles** (Ex. 44–45): watched movies `userid,movieid,start,end`, preferences `userid,genre`, catalog `movieid,title,genre` → users with > threshold% watched movies outside their liked genres.
- **Time series** (Ex. 46): `timestamp, temperature` (Unix, 1-minute sampling) → windows of 3 consecutive readings with increasing trend.

Official solutions on disk: `exercises/spark/solutionsexspark30-46/exNN*.ipynb` (often with alternative variants: `ex31-v2-flatmap`, `ex32-takeordered`, `ex34-top`, `ex36-v1/v2/v3`, …). Exercise data: `exercises/spark/exsparkdata30-46/data/exNN/data/`.

## Worked Exercise 1 — Log Filtering (Ex. 30 + 31)

**Ex. 30 statement.** Input: simplified web-server log, one URL request per line. Output: the lines containing the word "google", stored in an HDFS folder.

Solution (`sol_ex30.ipynb`):

```python
inputPath  = "/data/students/bigdata-01QYD/ex_data/Ex30/data/"
outputPath = "res_out_Ex30/"

# Each element of logRDD corresponds to one line of the input file
logRDD = sc.textFile(inputPath)

# Only the elements of the RDD satisfying the filter are selected
googleRDD = logRDD.filter(lambda logLine: logLine.lower().find("google") >= 0)

# Store the result in the output folder
googleRDD.saveAsTextFile(outputPath)
```

**Ex. 31 statement.** Output: the **distinct IP addresses** of connections to URLs containing `www.google.com`.

Solution based on `flatMap` (`sol_ex31-v2-flatmap.ipynb`) — one function does selection and extraction, returning an empty list for non-matching lines:

```python
def filterAndExtractIP(line):
    listIPs = []
    if line.lower().find("www.google.com") >= 0:
        IP = line.split('-')[0]
        listIPs.append(IP)
    return listIPs

IPsRDD = logRDD.flatMap(filterAndExtractIP)
distinctIPsRDD = IPsRDD.distinct()
distinctIPsRDD.saveAsTextFile(outputPath)
```

Points illustrated: `textFile` of a folder; substring match with `lower()` (case-insensitivity); `flatMap` as filter+map in one step; `distinct()` to deduplicate (shuffle — justified because the exercise demands it).

## Worked Exercise 2 — PM10 Maxima (Ex. 32, 33, 34)

All three read the same input and extract the third CSV field as a float:

```python
readingsRDD = sc.textFile(inputPath)
pm10ValuesRDD = readingsRDD.map(lambda PM10Reading: float(PM10Reading.split(',')[2]))
```

**Ex. 32 — maximum value, print on stdout.** Two one-action solutions. With `reduce` (`sol_ex32.ipynb`):

```python
maxPM10Value = pm10ValuesRDD.reduce(lambda value1, value2: max(value1, value2))
print(maxPM10Value)
```

With `takeOrdered` and an inverted key (`sol_ex32-takeordered.ipynb`): *takeOrdered returns the smallest elements*, so negate the values to obtain the maximum:

```python
maxPM10Value = pm10ValuesRDD.takeOrdered(1, lambda n: -1 * n)[0]
```

**Ex. 33 — top-3 maximum values** (`sol_ex33.ipynb`): `top(3)` returns the 3 largest elements in descending order:

```python
top3PM10Value = pm10ValuesRDD.top(3)
print(top3PM10Value)      # e.g. [60.2, 55.5, 52.5]
```

**Ex. 34 — the full line(s) achieving the maximum.** Requires two steps: compute the max, then filter the original RDD by that value (`sol_ex34-takeordered.ipynb`):

```python
maxPM10Value = pm10ValuesRDD.takeOrdered(1, lambda num: -num)[0]

selectedRecordsRDD = readingsRDD.filter(
    lambda PM10Reading: float(PM10Reading.split(',')[2]) == maxPM10Value)

selectedRecordsRDD.saveAsTextFile(outputPath)
```

Expected behavior on the book's sample input: both `s1,2016-01-02,60.2` and `s1,2016-01-03,60.2` are stored. (Variant `ex34-top.ipynb` uses `top(1)` for the first step.)

Related quick ones, same skeleton: **average** (Ex. 35/36) via `reduce` sum + `count` (the v2/`aggregate` variant does it in one action — see note 05), **per-sensor maximum** (Ex. 37/38) via `map` to `(sensorId, value)` pairs + `reduceByKey(max)`.

## Worked Exercise 3 — Critical Dates per Sensor (Ex. 39)

**Statement.** Output: one line per sensor with the sensorId and the **list of dates** with PM10 > 50 for that sensor; only sensors with at least one such date.

Solution (`sol_ex39.ipynb`) — filter → pair creation → `groupByKey` → readable lists:

```python
inputPath  = "/data/students/bigdata-01QYD/ex_data/Ex39/data/sensors.txt"  # argv[1]
outputPath = "res_out_Ex39/"                                                # argv[2]

readingsRDD = sc.textFile(inputPath)

# select only the lines with PM10 > 50
readingsHighValueRDD = readingsRDD.filter(
    lambda PM10Reading: float(PM10Reading.split(',')[2]) > 50)

# pairs (sensorId, date)
sensorsCriticalDatesRDD = readingsHighValueRDD.map(
    lambda PM10Reading: (PM10Reading.split(',')[0], PM10Reading.split(',')[1]))

# one pair per sensor with the list of its critical dates
finalSensorCriticalDates = sensorsCriticalDatesRDD.groupByKey()
finalSensorCriticalDateStringFormat = finalSensorCriticalDates.mapValues(
    lambda dates: list(dates))

finalSensorCriticalDateStringFormat.saveAsTextFile(outputPath)
```

Expected output on the book's sample: `(s1, [2016-01-02, 2016-01-03])`. The follow-up variant (Ex. 39bis in the book: sensors with **no** critical date must appear with an empty list, e.g. `(s3, [])`) needs the *complete* list of sensors too — solved in `sol_ex39bis-v1.ipynb` by keeping all sensors in the pair-creation step and filtering values > 50 inside the value.

## Worked Exercise 4 — Questions and Answers (Ex. 42)

**Statement.** Two files: questions `QuestionId,Timestamp,Text`, answers `AnswerId,QuestionId,Timestamp,Text`. Output: one line per question with its text and the list of its answers' texts.

Solution (`sol_ex42.ipynb`) — both inputs keyed by `QuestionId`, then `cogroup`:

```python
questionsRDD = sc.textFile(inputPathQuestions)
questionsPairRDD = questionsRDD.map(
    lambda question: (question.split(",")[0], question.split(",")[2]))

answersRDD = sc.textFile(inputPathAnswers)
answersPairRDD = answersRDD.map(
    lambda answer: (answer.split(",")[1], answer.split(",")[3]))

# "cogroup" the two RDDs of pairs
questionsAnswersPairRDD = questionsPairRDD.cogroup(answersPairRDD)

# transform the two iterables of each pair into lists (reformat them)
questionsAnswersReformatted = questionsAnswersPairRDD.mapValues(
    lambda value: (list(value[0]), list(value[1])))

questionsAnswersReformatted.saveAsTextFile(outputPath)
```

Expected output on the book's sample: `(Q1,(['What is ..?'],['It is ..', 'I think it is ..']))`. `cogroup` (vs `join`) keeps questions with **zero** answers and returns both value lists as iterables — always finish with `mapValues(list(...))` to make them storable/readable.

## Worked Exercise 5 — Misleading Profiles (Ex. 44)

**Statement.** Three inputs (watched movies, liked genres per user, movie catalog with genre). A user has a *misleading profile* if more than `threshold`% of watched movies have a genre **not** among his/her liked genres. Output: the userids.

Solution (`sol_ex44.ipynb`) — `join` on movieid, then `cogroup` per user, then a driver-side predicate over the two lists:

```python
# movieid -> userid  (from watchedmovies.txt)
movieUserPairRDD = watchedRDD.map(lambda line: (line.split(",")[1], line.split(",")[0]))
# movieid -> genre  (from movies.txt)
movieGenrePairRDD = moviesRDD.map(lambda line: (line.split(",")[0], line.split(",")[2]))

# attach the genre to each visualization, then re-key by user
joinWatchedGenreRDD = movieUserPairRDD.join(movieGenrePairRDD)
usersWatchedGenresRDD = joinWatchedGenreRDD.map(lambda pair: (pair[1][0], pair[1][1]))

# userid -> genre (from preferences.txt)
userLikedGenresRDD = preferencesRDD.map(lambda line: (line.split(",")[0], line.split(",")[1]))

# per user: (iterable of watched genres with repetitions, iterable of liked genres)
userWatchedLikedGenres = usersWatchedGenresRDD.cogroup(userLikedGenresRDD)

def misleadingProfileFunc(userWatchedLikedGenresLists):
    likedGenres = list(userWatchedLikedGenresLists[1][1])
    numWatchedMovies = 0
    notLiked = 0
    for watchedGenre in userWatchedLikedGenresLists[1][0]:
        numWatchedMovies = numWatchedMovies + 1
        if watchedGenre not in likedGenres:
            notLiked = notLiked + 1
    if float(notLiked) > threshold * float(numWatchedMovies):
        return True
    else:
        return False

misleadingUsersRDD = userWatchedLikedGenres.filter(misleadingProfileFunc).keys()
misleadingUsersRDD.saveAsTextFile(outputPath)
```

The pattern: **join to enrich** records with reference data, **cogroup to collect** per-entity lists, then apply the business predicate locally on the (small) per-user lists — the same "keep sparse data local" principle as Lab 4's HashMap reducer.

## Worked Exercise 6 — Sliding Windows (Ex. 46)

**Statement.** `timestamp, temperature` at 1-minute sampling. Select all windows of 3 **consecutive** readings with strictly increasing temperature.

Solution (`sol_ex46.ipynb`) — flatMap each reading into the 3 windows containing it, `groupByKey` on window start, then filter:

```python
def windowElementsFunc(reading):
    fields = reading.split(",")
    t = int(fields[0])
    temperature = float(fields[1])
    # this reading belongs to the windows starting at t, t-60s, t-120s
    pairs = []
    pairs.append((t,       reading))   # first element of window starting at t
    pairs.append((t - 60,  reading))   # second element of window starting at t-60
    pairs.append((t - 120, reading))   # third element of window starting at t-120
    return pairs

windowsElementsRDD = readingsRDD.flatMap(windowElementsFunc)
timestampsWindowsRDD = windowsElementsRDD.groupByKey()

def increasingTrendFunc(pairInitialTimestampWindow):
    minTimestamp = pairInitialTimestampWindow[0]
    timestampTemp = {}
    for timestampTemperature in pairInitialTimestampWindow[1]:
        fields = timestampTemperature.split(",")
        t = int(fields[0])
        timestampTemp[t] = float(fields[1])
    if len(timestampTemp) != 3:
        increasing = False        # incomplete window: discard
    else:
        if (timestampTemp[minTimestamp] < timestampTemp[minTimestamp + 60]
                and timestampTemp[minTimestamp + 60] < timestampTemp[minTimestamp + 120]):
            increasing = True
        else:
            increasing = False
    return increasing

seletedWindowsRDD = timestampsWindowsRDD.filter(increasingTrendFunc)
seletedWindowsRDD.values().map(lambda window: list(window)).saveAsTextFile(outputPath)
```

On the book's sample input the two selected windows are `1451606400,12.1,1451606460,12.2,1451606520,13.5` and `1451606460,12.2,1451606520,13.5,1451606580,14.0`. Key insight: a reading belongs to `windowSize` (3) different windows, so emit it once per window with the window's start timestamp as key.

## Solution Strategy Checklist

1. **Read the statement twice**: output on **stdout** (`print`) vs **HDFS folder** (`saveAsTextFile`) vs **driver-side list** (`collect`/`take`).
2. Sketch the transformation chain on the toy input given in the book: parse (`split` on the right separator) → filter (headers, thresholds, dirty lines) → key selection → aggregate → sort/top-k → format → store.
3. Max/top-k: `reduce(max)`, `top(k)`, `takeOrdered(k, key=lambda x: -x)`; ties resolved afterwards with a filter (Ex. 34).
4. Per-entity lists: `groupByKey` + `mapValues(list)` (Ex. 39); two datasets: `join` (enrich, exact keys) or `cogroup` (collect two lists, keep unmatched) (Ex. 42/44).
5. Two-pass problems (max then select): compute the scalar with one action, use it to close over the lambda in the next — beware the extra action.
6. Windowing (Ex. 46): replicate each element into the windows it belongs to, keyed by window start.
7. Always end scripts with `sc.stop()`; take paths from `sys.argv` when asked (Labs 5 and the exercise solutions' `# argv` comments).

## Where to Practice More

- Alternative official variants: `solutionsexspark30-46/` contains `ex31-v1`, `ex32-top`, `ex33-takeorderd`, `ex34-takeorderedwrong` (a deliberately wrong variant worth studying), `ex36-v1/v2/v3`, `ex39bis-v1`, `ex41-v1/v2`, `ex45` (profile update with the ≥5-movies rule).
- The exercise statement PDF with input/output examples: `exercises/spark/02-spark-exercises-bigdatanb.pdf`.
- Bike-sharing variants (Ex. 43): input data under `exercises/spark/exsparkdata30-46/data/ex43/` (`readings.txt`, `neighbors.txt`) with reference outputs in `critical-stations/` and `critical-stations-timeslots/`.

## Key Takeaways

- Most exam exercises reduce to: filter → map to pairs → aggregate (`reduce`/`reduceByKey`/`groupByKey`) → select (`top`/`takeOrdered`/filter on computed value) → output.
- Read the required output channel carefully (stdout vs HDFS folder) — it decides between `print`, `take`, and `saveAsTextFile`.
- `top`/`takeOrdered` with key functions solve max/top-k including tie handling; `aggregate` solves multi-statistics-in-one-pass.
- `cogroup`/`join` cover all multi-input exercises; finish with `mapValues(list(...))`.
- Replicating elements per window (Ex. 46) generalizes to any sliding-window exam question.
