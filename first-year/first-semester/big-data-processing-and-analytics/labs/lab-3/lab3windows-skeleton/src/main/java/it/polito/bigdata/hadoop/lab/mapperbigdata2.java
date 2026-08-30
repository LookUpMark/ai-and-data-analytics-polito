package it.polito.bigdata.hadoop.lab;

import java.io.IOException;

import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Mapper;
import it.polito.bigdata.hadoop.lab.TopKVector;

/**
 * Lab  - Mapper
 */

/* Set the proper data types for the (key,value) pairs */
class MapperBigData2 extends Mapper<
                    Text, // Input key type
                    Text,         // Input value type
                    Text,         // Output key type
                    IntWritable> {// Output value type

    private TopKVector<WordCountWritable> topKVector;

    protected void setup(Context context) throws IOException, InterruptedException {
        topKVector = new TopKVector<>(100);
    }
    
    protected void map(
            Text key,   // Input key type
            Text value,         // Input value type
            Context context) throws IOException, InterruptedException {

    		/* Implement the map method */
            WordCountWritable wc = new WordCountWritable(key.toString(), Integer.parseInt(value.toString()));
            topKVector.updateWithNewElement(wc);
    }

    protected void cleanup(Context context) throws IOException, InterruptedException {
        for (WordCountWritable wc : topKVector.getLocalTopK()) {
            context.write(new Text(wc.getWord()), new IntWritable(wc.getCount()));
        }
    }
}
