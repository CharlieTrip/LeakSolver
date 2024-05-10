import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import os

# Parse timing field to convert to the specified unit
def parse_timing(timing, unit='ms'):
		if 'µs' in timing:
				value = float(timing[:-2]) / 1000
		elif 'ms' in timing:
				value = float(timing[:-2])
		elif 's' in timing:
				value = float(timing[:-1]) * 1000
		else:
				value = float(timing)  # Assume milliseconds if no unit is provided
		
		if unit == 'µs':
				return value * 1000
		elif unit == 's':
				return value / 1000
		else:
				return value  # Default to milliseconds if an invalid unit is provided

def clean_dataframe(csv_file,unit='µs'):
		# Read the data into a DataFrame
		df = pd.read_csv("data/"+csv_file, sep=';')
		
		df['timing_ms'] = df['timing'].apply(lambda x: parse_timing(x, unit=unit))
		
		# Calculate mean and standard deviation of the 'timing_ms' column
		mean_timing = df['timing_ms'].mean()
		std_timing = df['timing_ms'].std()
		
		# Define the threshold for filtering (e.g., 2 sigma)
		threshold_lower = mean_timing - 2 * std_timing
		threshold_upper = mean_timing + 2 * std_timing
		
		# Filter out the rows where the 'timing_ms' values are outside the threshold range
		cleaned_df = df.loc[(df['timing_ms'] >= threshold_lower) & (df['timing_ms'] <= threshold_upper)]
		
		return cleaned_df


def plot_keySHW_analysis(df,unit='µs',disable=False,save=""):
		# Scatter plot of keySHW vs timing_ms
		sns.scatterplot(data=df, x='keySHW', y='timing_ms')
		plt.title('keySHW vs Timing')
		plt.xlabel('keySHW')
		plt.ylabel('Timing ({})'.format(unit))
		plt.grid(True)
		plt.xlim(0, 64)
		plt.ylim(0, None)
		plt.tight_layout()
		if save != "":
			plt.savefig(save + '.png')
		if not(disable):
			plt.show()
		plt.close()

def plot_finSHW_analysis(df,unit='µs',disable=False,save=""):
		# Scatter plot of finSHW vs timing_ms
		sns.scatterplot(data=df, x='finSHW', y='timing_ms')
		plt.title('finSHW vs Timing')
		plt.xlabel('finSHW')
		plt.ylabel('Timing ({})'.format(unit))
		plt.grid(True)
		plt.xlim(0, 64)
		plt.ylim(0, None)
		plt.tight_layout()
		if save != "":
			plt.savefig(save + '.png')
		if not(disable):
			plt.show()
		plt.close()

def plot_distribution(df,what=('timing_ms',30,0,100,'Timing (µs)'),unit='µs',disable=False,save=""):
		plt.figure(figsize=(8, 5))
		sns.histplot(data=df, x=what[0], bins=what[1], color='skyblue', edgecolor='black')
		plt.xlabel(what[4])
		plt.ylabel('Frequency')
		plt.grid(True)
		plt.xlim(what[2], what[3])
		plt.tight_layout()
		if save != "":
			plt.savefig(save + '.png')
		if not(disable):
			plt.show()
		plt.close()

def plot_scatter_histogram(df,unit='µs',disable=False,save=""):
		# Create a single figure with three subplots arranged horizontally
		fig, axs = plt.subplots(1, 3, figsize=(18, 6))

		# Scatter plot of keySHW vs timing_ms
		sns.scatterplot(data=df, x='keySHW', y='timing_ms', ax=axs[0])
		axs[0].set_title('keySHW vs timing')
		axs[0].set_ylabel('Timing ({})'.format(unit))
		axs[0].set_xlabel('Key Special HW')
		axs[0].set_xlim(0, 64)  # Limit the x-axis from 0 to 64
		axs[0].set_ylim(0, None)  # Limit the y-axis from 0 
		axs[0].grid(True)

		# Scatter plot of finSHW vs timing_ms
		sns.scatterplot(data=df, x='finSHW', y='timing_ms', ax=axs[1])
		axs[1].set_title('finSHW vs Timing')
		axs[1].set_ylabel('Timing ({})'.format(unit))
		axs[1].set_xlabel('Final Special HW')
		axs[1].set_xlim(0, 64)  # Limit the x-axis from 0 to 64
		axs[1].set_ylim(0, None)  # Limit the y-axis from 0 
		axs[1].grid(True)

		# Histogram of timing_ms
		sns.histplot(data=df, x='timing_ms', ax=axs[2], bins=30, color='skyblue', edgecolor='black')
		axs[2].set_title('Timing Distribution')
		axs[2].set_xlabel('Timing ({})'.format(unit))
		axs[2].set_ylabel('Frequency')
		axs[2].grid(True)

		# Adjust layout
		plt.tight_layout()
		
		if save != "":
			plt.savefig(save + '.png')

		# Show the plot
		if not(disable):
			plt.show()
		plt.close()


def collect_and_plot_statistics(N,what=("effectiveHW","Effective HW","timing_ms","Timing"),unit='µs',total=(True,128),disable=False,save=""):
		# Initialize lists to store statistical values
		data = []

		# Loop through the range of N values
		for i in range(N):
				# Read the data into a DataFrame
				file = 'random_test.{:03d}.out'.format(i)
				df = clean_dataframe(file, unit=unit)
				
				# Filter the data
				filtered_df = df[(df['HW'] == df['effectiveHW']) | (df['effectiveHW'] == 128 - df['HW'])]
				
				# Append to the list
				data.append(filtered_df)
		
		# Concatenate the dataframes
		result_df = pd.concat(data)
		
		# Generate all possible values of effectiveHW
		if total[0]:
			all_values = pd.DataFrame({what[0]: range(total[1]+1)}) 

			# Merge with result_df to add missing values
			result_df = pd.merge(all_values, result_df, how='left', on=what[0])


		
		# Plot the violin plot
		plt.figure(figsize=(8, 5))
		sns.boxplot(x=what[0], y=what[2], data=result_df,showfliers=False,whis=0)
		plt.xlabel(what[1])
		plt.ylabel(what[3])
		plt.grid(True)
		
		if total[0]:
			step = int(total[1]//16)
			plt.xticks(np.arange(0, total[1]+1, step))
		
		if save != "":
			plt.savefig(save + '.png')

		# Show the plot
		if not(disable):
			plt.show()
		plt.close()




# General Visualization Parameters
unit = 's'
disable = True



# Get the list of files in the "data" folder
data_files = os.listdir("data")

# Extract the indices from the file names
indices = [int(filename.split(".")[1]) for filename in data_files]

# Get the maximum index
max_index = max(indices) + 1


for i in range(max_index):
	file = 'random_test.{:03d}.out'.format(i)
	df = clean_dataframe(file,unit=unit)
	plot_scatter_histogram(df,unit=unit,disable=disable,save="img/"+file)

total = True
collect_and_plot_statistics(max_index,what=("effectiveHW","Effective HW","timing_ms",'Timing ({})'.format(unit)),unit=unit,total=(total,128),disable=disable,save="img/totalHW.tot")
collect_and_plot_statistics(max_index,what=("outSHW","Special HW","timing_ms",'Timing ({})'.format(unit)),unit=unit,total=(total,64),disable=disable,save="img/totalSHW.tot")
collect_and_plot_statistics(max_index,what=("effectiveHW","Effective HW","solutions",'Solutions'),unit=unit,total=(total,128),disable=disable,save="img/totalSol")


total = False
collect_and_plot_statistics(max_index,what=("effectiveHW","Effective HW","timing_ms",'Timing ({})'.format(unit)),unit=unit,total=(total,128),disable=disable,save="img/totalHW")
collect_and_plot_statistics(max_index,what=("outSHW","Special HW","timing_ms",'Timing ({})'.format(unit)),unit=unit,total=(total,64),disable=disable,save="img/totalSHW")




dfs = []
for i in range(max_index):
	file = 'random_test.{:03d}.out'.format(i)
	df = clean_dataframe(file,unit=unit)
	dfs.append(df)

totdf = pd.concat(dfs)

plot_distribution(totdf,what=('keySHW',64,0,64,'Key Special HW'),unit='ms',disable=disable,save="img/keySHW")
plot_distribution(totdf,what=('outSHW',64,0,64,'Out Special HW'),unit='ms',disable=disable,save="img/outSHW")
