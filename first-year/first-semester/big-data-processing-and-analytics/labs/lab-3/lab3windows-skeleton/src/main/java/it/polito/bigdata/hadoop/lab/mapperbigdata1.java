package it.polito.bigdata.hadoop.lab;

import java.io.IOException;

import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.io.NullWritable;

/**
 * Lab  - Mapper
 */

/* Set the proper data types for the (key,value) pairs */
class MapperBigData1 extends Mapper<
                    LongWritable, // Input key type
                    Text,         // Input value type
                    Text,         // Output key type
                    IntWritable> {// Output value type
    
    protected void map(
        LongWritable key,   // Input key type
        Text value,         // Input value type
        Context context) throws IOException, InterruptedException {

        /* Implement the map method */
        String[] fields = value.toString().split(",");
        if (fields.length == 2) {
            // Se c'è solo un elemento, emetti la chiave con quel singolo elemento
            context.write(new Text(fields[1] + "," + fields[1]), new IntWritable(1));
        } else {
            // Procedi normalmente se ci sono più di un elemento
            for (int i = 1; i < fields.length - 1; i++) {
                for (int j = i + 1; j < fields.length; j++) {
                    if (fields[i].compareTo(fields[j]) > 0) {
                        context.write(new Text(fields[i] + "," + fields[j]), new IntWritable(1));
                    } else {
                        context.write(new Text(fields[j] + "," + fields[i]), new IntWritable(1));
                    }
                }
            }
        }
    }
}
