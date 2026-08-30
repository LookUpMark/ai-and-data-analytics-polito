package it.polito.bigdata.hadoop.exercise10;

import it.polito.bigdata.hadoop.exercise10.DriverBigData.MY_COUNTERS;

import java.io.IOException;

import org.apache.hadoop.io.*;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Ex. 10 Mapper
 */
class MapperBigData extends
		Mapper<LongWritable, // Input key type
				Text, // Input value type
				NullWritable, // Output key type
				IntWritable> {// Output value type

	protected int counter;

	protected void setup(Context context) throws IOException, InterruptedException {
		counter = 0;
	}

	protected void map(LongWritable key, // Input key type
			Text value, // Input value type
			Context context) throws IOException, InterruptedException {

		counter++;
	}

	protected void cleanup(Context context) throws IOException, InterruptedException {
		context.write(NullWritable.get(), new IntWritable(counter));
	}
}
