package it.polito.bigdata.hadoop.exercise4;

import java.io.IOException;

import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Exercise 4 - Mapper
 */
class MapperBigData extends
		Mapper<Text, // Input key type
				Text, // Input value type
				Text, // Output key type
				Text> { // Output value type

	private static Double PM10Threshold = new Double(50);

	protected void map(Text key, // Input key type
			Text value, // Input value type
			Context context) throws IOException, InterruptedException {

		float PM10Threshold = 50;
		if (Float.parseFloat(value.toString()) > PM10Threshold) {
			String zone = key.toString().split(",")[0];
			String date = key.toString().split(",")[1];
			context.write(new Text(zone), new Text(date));
		}
	}
}
