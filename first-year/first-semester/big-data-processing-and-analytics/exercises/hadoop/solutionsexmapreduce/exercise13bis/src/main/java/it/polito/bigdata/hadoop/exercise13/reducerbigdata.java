package it.polito.bigdata.hadoop.exercise13;

import java.io.IOException;
import java.util.HashMap;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * Reducer
 */
class ReducerBigData extends Reducer<
                Text,           // Input key type
                Text,  // Input value type
                Text,           // Output key type
                FloatWritable> {  // Output value type

	protected HashMap<String, Float> map;

	protected void setup(Context context) throws IOException, InterruptedException {
		map = new HashMap<String, Float>();
	}
	
	// The reduce method is called only once in this approach
	// All the key-value pairs emitted by the mappers as the 
	// same key (NullWritable.get())
    @Override
    protected void reduce(
        Text key, // Input key type
        Iterable<Text> values, // Input value type
        Context context) throws IOException, InterruptedException {

		for (Text value : values) {
			map.put(key.toString(), Float.parseFloat(value.toString()));
		}
    }

	protected void cleanup(Context context) throws IOException, InterruptedException {
		float maxProfit = 0;
		float secondMaxProfit = 0;
		boolean first = false;
		boolean second = false;
		for (String key : map.keySet()) {
			if (map.get(key) > maxProfit) {
				maxProfit = map.get(key);
			}
		}

		for (String key : map.keySet()) {
			if (map.get(key) > secondMaxProfit && map.get(key) < maxProfit) {
				secondMaxProfit = map.get(key);
			}
		}

		for (String key : map.keySet()) {
			if (map.get(key) == maxProfit && !first) {
				context.write(new Text(key), new FloatWritable(maxProfit));
				first = true;
			}
			if (map.get(key) == secondMaxProfit && !second) {
				context.write(new Text(key), new FloatWritable(secondMaxProfit));
				second = true;
			}
		}
	}
}
