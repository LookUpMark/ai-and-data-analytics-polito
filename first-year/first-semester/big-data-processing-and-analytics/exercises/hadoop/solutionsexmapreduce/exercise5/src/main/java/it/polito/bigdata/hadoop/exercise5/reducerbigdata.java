package it.polito.bigdata.hadoop.exercise5;

import java.io.IOException;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * WordCount Reducer
 */
class ReducerBigData extends
		Reducer<Text, // Input key type
				FloatWritable, // Input value type
				Text, // Output key type
				FloatWritable> { // Output value type

	@Override
	protected void reduce(Text key, // Input key type
			Iterable<FloatWritable> values, // Input value type
			Context context) throws IOException, InterruptedException {

		int sum = 0, count = 0;
		for (FloatWritable value : values) {
			sum += value.get();
			count++;
		}
		context.write(key, new FloatWritable(sum / count));
	}
}
