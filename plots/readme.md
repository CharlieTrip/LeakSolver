# Generating and Plotting Data

This folder contains scripts for generating data and creating plots related to
special Hamming weight distributions of secret keys.

**AAA:** the Python scripts are dumb and require all the datasets up to the maximum index!

## Contents:

- `gen_rnd.sh`: Generates the data into the "data" folder. It includes parameters for the number of tests, starting and ending points for the generation.
- `rnd_plt.py`: Generates various plots and saves them into the "img" folder.
- `rnd_shw.py`: Generates a sample of the special Hamming weight distribution of the secret keys.

## Execution:

To execute the code:

1. `./gen_rnd.sh` to generate the data.
3. `python rnd_plt.py` to generate and save the plots into the "img" folder.
2. `python rnd_shw.py` to generate the special Hamming weight distribution.
