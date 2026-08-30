package it.polito.bigdata.hadoop.exercise13;

import java.io.IOException;
import java.util.HashMap;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * Reducer
 */
class ReducerBigData extends Reducer<Text, // Input key type
		DoubleWritable, // Input value type
		Text, // Output key type
		DoubleWritable> { // Output value type

	protected HashMap<String, Double> hashMap;

	protected void setup(Context context) throws IOException, InterruptedException {
		hashMap = new HashMap<>();
	}

	// The reduce method is called only once in this approach
	// All the key-value pairs emitted by the mappers have the
	// same key (NullWritable.get())
	@Override
	protected void reduce(Text key, // Input key type
			Iterable<DoubleWritable> values, // Input value type
			Context context) throws IOException, InterruptedException {

		for(DoubleWritable val : values) {
			hashMap.put(key.toString(), val.get());
		}
	}

	protected void cleanup(Context context) throws IOException, InterruptedException {
		double maximum = 0;

		for(String key : hashMap.keySet()) {
			if (hashMap.get(key) > maximum) {
				maximum = hashMap.get(key);
			}
		}

		for(String key : hashMap.keySet()) {
			if (hashMap.get(key) == maximum) {
				context.write(new Text(key), new DoubleWritable(maximum));
			}
		}

	}
}
