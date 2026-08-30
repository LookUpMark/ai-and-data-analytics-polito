package it.polito.bigdata.hadoop.exercise3;

import java.io.IOException;

import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Exercise 3 - Mapper
 */
class MapperBigData extends Mapper<
                    Text, 		  // Input key type
                    Text, 		  // Input value type
                    Text,         // Output key type
                    IntWritable> {// Output value type
    
	private static Double PM10Threshold = new Double(50);
	
    protected void map(
            Text key,   		// Input key type
            Text value,         // Input value type
            Context context) throws IOException, InterruptedException {

            float PM10Threshold = 50;
            if (Float.parseFloat(value.toString()) > PM10Threshold) {
                String sensor = key.toString().split(",")[0];
                context.write(new Text(sensor), new IntWritable(1));
            }
    }
}
