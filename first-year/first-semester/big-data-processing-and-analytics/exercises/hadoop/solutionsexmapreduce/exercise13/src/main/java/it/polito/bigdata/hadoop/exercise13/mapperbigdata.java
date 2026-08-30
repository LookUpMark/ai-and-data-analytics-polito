package it.polito.bigdata.hadoop.exercise13;

import java.io.IOException;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Mapper
 */
class MapperBigData extends Mapper<Text, // Input key type
		Text, // Input value type
		Text, // Output key type
		DoubleWritable> {// Output value type

	protected void map(Text key, // Input key type
			Text value, // Input value type
			Context context) throws IOException, InterruptedException {

		double valueDouble = Double.parseDouble(value.toString());
		context.write(key, new DoubleWritable(valueDouble));
	}
}
