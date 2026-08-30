package it.polito.bigdata.hadoop.lab;

import java.io.IOException;

import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;

/**
 * Lab  - Mapper
 */

/* Set the proper data types for the (key,value) pairs */
class MapperBigData extends Mapper<
                    Text, // Input key type
                    Text,         // Input value type
                    Text,         // Output key type
                    IntWritable> {// Output value type
    
    protected void map(
            Text key,   // Input key type
            Text value,         // Input value type
            Context context) throws IOException, InterruptedException {

    		/* Implement the map method */
            int intValue = Integer.parseInt(value.toString());
            if(intValue >= 0 && intValue < 100) {
                context.write(new Text("Group0"), new IntWritable(1)); // "Group 0", new IntWritable(1));
            } else if(intValue >= 100 && intValue < 200) {
                context.write(new Text("Group1"), new IntWritable(1)); // "Group 1", new IntWritable(1));
            } else if(intValue >= 200 && intValue < 300) {
                context.write(new Text("Group2"), new IntWritable(1)); // "Group 2", new IntWritable(1));
            } else if(intValue >= 300 && intValue < 400) {
                context.write(new Text("Group3"), new IntWritable(1)); // "Group 3", new IntWritable(1));
            } else if(intValue >= 400 && intValue < 500) {
                context.write(new Text("Group4"), new IntWritable(1)); // "Group 4", new IntWritable(1));
            } else if(intValue >= 500) {
                context.write(new Text("Group5"), new IntWritable(1)); // "Group 5", new IntWritable(1));
            }
    }
}
