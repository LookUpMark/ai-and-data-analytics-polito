package it.polito.bigdata.hadoop.exercise12;

import java.io.IOException;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Mapper
 */
class MapperBigData extends
		Mapper<Text, // Input key type
				Text, // Input value type
				Text, // Output key type
				FloatWritable> {// Output value type

	float threshold;

	protected void setup(Context context) {
		// I retrieve the value of the threshold only one time for each mapper
		threshold = 
			Float.parseFloat(
					context.getConfiguration().get("maxThreshold"));
	}

	protected void map(Text key, // Input key type
			Text value, // Input value type
			Context context) throws IOException, InterruptedException {

		float pm10value = Float.parseFloat(value.toString());
		if (pm10value < threshold) {
			context.write(key, new FloatWritable(pm10value));
		}
	}
}
