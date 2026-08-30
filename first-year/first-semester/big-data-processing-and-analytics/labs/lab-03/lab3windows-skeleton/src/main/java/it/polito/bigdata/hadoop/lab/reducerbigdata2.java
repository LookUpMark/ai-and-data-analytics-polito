package it.polito.bigdata.hadoop.lab;

import java.io.IOException;

import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * Lab - Reducer
 */

/* Set the proper data types for the (key,value) pairs */
class ReducerBigData2 extends Reducer<
                Text,           // Input key type
                IntWritable,    // Input value type
                Text,           // Output key type
                IntWritable> {  // Output value type
    
    private TopKVector<WordCountWritable> topKVector;

    protected void setup(Context context) throws IOException, InterruptedException {
        topKVector = new TopKVector<>(100);
    }
    
    @Override
    protected void reduce(
        Text key, // Input key type
        Iterable<IntWritable> values, // Input value type
        Context context) throws IOException, InterruptedException {

        int sum = 0;
        for (IntWritable value : values) {
            sum += value.get();
        }

		/* Implement the reduce method */
    	WordCountWritable wc = new WordCountWritable(key.toString(), new Integer(sum));
        topKVector.updateWithNewElement(wc);
    }

    protected void cleanup(Context context) throws IOException, InterruptedException {
        for (WordCountWritable wc : topKVector.getLocalTopK()) {
            context.write(new Text(wc.getWord()), new IntWritable(wc.getCount()));
        }
    }
}
