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
class MapperBigData1 extends Mapper<
                    LongWritable, // Input key type
                    Text, // Input value type
                    Text, // Output key type
                    Text> {// Output value type
    
    private int counter;
    
    protected void setup(Context context) throws IOException, InterruptedException {
    		/* Implement the setup method */
        counter = 0;
    }
    
    protected void map(
            LongWritable key,   // Input key type
            Text value,         // Input value type
            Context context) throws IOException, InterruptedException {

    		/* Implement the map method */
            String[] fields = value.toString().split(",");
            if (counter == 0) {
                counter++;
            } else {
                context.write(new Text(fields[2]), new Text(fields[1] + "," + fields[6]));
            }
    }
}
