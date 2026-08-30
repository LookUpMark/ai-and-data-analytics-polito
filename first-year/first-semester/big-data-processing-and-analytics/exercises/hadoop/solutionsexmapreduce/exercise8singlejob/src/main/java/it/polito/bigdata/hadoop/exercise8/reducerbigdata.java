package it.polito.bigdata.hadoop.exercise8;

import java.io.IOException;
import java.util.HashMap;

import org.apache.hadoop.io.DoubleWritable;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Reducer;

/**
 * Exercise 8 - Reducer
 */
class ReducerBigData extends Reducer<Text, // Input key type (Year)
		Text, // Input value type (Month,Income)
		Text, // Output key type
		DoubleWritable> { // Output value type

	@Override
	protected void reduce(Text key, // Input key type (Year)
						  Iterable<Text> values, // Input value type
						  Context context) throws IOException, InterruptedException {

		// HashMap to store monthly income
		HashMap<String, Double> monthlyIncome = new HashMap<>();

		// Populate the HashMap
		for (Text value : values) {
			String[] parts = value.toString().split(",");
			String month = parts[0];
			double income = Double.parseDouble(parts[1]);

			// Accumulate income for the month
			monthlyIncome.put(month, monthlyIncome.getOrDefault(month, 0.0) + income);
		}

		double totalAnnualIncome = 0.0;
		int monthsWithIncome = 0;

		// Calculate total annual income and count months with income
		for (String month : monthlyIncome.keySet()) {
			double totalMonthlyIncome = monthlyIncome.get(month);

			if (totalMonthlyIncome > 0) {
				totalAnnualIncome += totalMonthlyIncome;
				monthsWithIncome++;
			}

			// Emit (Year-Month, Total Monthly Income)
			context.write(new Text(key.toString() + "-" + month), new DoubleWritable(totalMonthlyIncome));
		}

		// Calculate average monthly income
		if (monthsWithIncome > 0) {
			double averageMonthlyIncome = totalAnnualIncome / monthsWithIncome;

			// Emit (Year, Average Monthly Income)
			context.write(key, new DoubleWritable(averageMonthlyIncome));
		}
	}
}
