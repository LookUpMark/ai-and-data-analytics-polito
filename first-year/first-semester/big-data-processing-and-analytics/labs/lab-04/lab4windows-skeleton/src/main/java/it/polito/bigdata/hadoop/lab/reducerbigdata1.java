package it.polito.bigdata.hadoop.lab;

import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.HashMap;

import org.apache.hadoop.io.FloatWritable;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * Lab - Reducer
 */

/* Set the proper data types for the (key,value) pairs */
class ReducerBigData1 extends Reducer<
                Text,           // Input key type
                Text,           // Input value type
                Text,           // Output key type
                FloatWritable> {  // Output value type

    @Override
    protected void reduce(
        Text key, // Input key type
        Iterable<Text> values, // Input value type
        Context context) throws IOException, InterruptedException {

        /* Implement the reduce method */
        HashMap<String, Float> hashmap = new HashMap<>();
        float mean = 0;

        for (Text value : values) {
            String[] fields = value.toString().split(",");
            hashmap.put(fields[0], Float.parseFloat(fields[1]));
        }
        for (String k : hashmap.keySet()) {
            mean += hashmap.get(k);
        }
        mean /= hashmap.size();
        for (String k : hashmap.keySet()) {
            hashmap.replace(k, hashmap.get(k) - mean);
        }
        for (String k : hashmap.keySet()) {
            context.write(new Text(k), new FloatWritable(hashmap.get(k)));
        }
    }
}