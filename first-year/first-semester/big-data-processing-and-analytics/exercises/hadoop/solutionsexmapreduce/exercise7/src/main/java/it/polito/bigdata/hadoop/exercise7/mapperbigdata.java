package it.polito.bigdata.hadoop.exercise7;

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
				Text> {// Output value type

	protected void map(Text key, // Input key type
			Text value, // Input value type
			Context context) throws IOException, InterruptedException {

		String[] sentence = value.toString().split("\\s+");
		for (String word : sentence) {
			String cleanWord = word.toLowerCase();
			if (cleanWord.compareTo("and") != 0 && cleanWord.compareTo("or") != 0 && cleanWord.compareTo("not") != 0) {
				context.write(new Text(cleanWord), key);
			}
		}
	}
}
