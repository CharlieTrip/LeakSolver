import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import random


def plot_SHW_distribution(df, disable=False, save=""):
	"""
	Plots the distribution of the 'SHW' variable in the DataFrame.

	Args:
		df (DataFrame): The DataFrame containing the 'SHW' variable.
		disable (bool): If True, the plot will not be displayed (default: False).
		save (str): If provided, the plot will be saved with the given filename (default: "").
	"""
	plt.figure(figsize=(10, 6))
	sns.histplot(data=df, x='SHW', bins=64, color='skyblue', edgecolor='black')
	plt.xlabel('Special HW')
	plt.ylabel('Frequency')
	plt.grid(True)
	plt.xlim(0, 64)

	if save:
		plt.savefig(save + '.png')

	if not disable:
		plt.show()


def hw(n):
	return bin(n).count("1")

def rnd_key():
	# Generate a list of 16 elements, each being a random 8-bit integer
	return [random.randint(0, 255) for _ in range(16)]

def shw(key):
	sw = 0
	for num in key:
		w = hw(num)
		sw += 4 - abs(4 - w)
	return sw
		

def create_dataframe(N):
	data = []
	generated_keys = set()  # Keep track of generated keys
	while len(data) < N:
		key = tuple(rnd_key())  # Convert the list to a tuple to make it hashable
		if key not in generated_keys:  # Check if the key has already been generated
			shw_value = shw(key)
			data.append({'key': key, 'SHW': shw_value})
			generated_keys.add(key)  # Add the key to the set of generated keys
	return pd.DataFrame(data)


disable = True

df = create_dataframe(2**15) 
plot_SHW_distribution(df, disable=disable, save="img/shw")
