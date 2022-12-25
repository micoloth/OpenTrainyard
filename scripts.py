# Get name of all files in assets/samples: 

import os

def get_samples():
    samples = []
    for file in os.listdir("OpenTrainyard/assets/samples"):
        if file.endswith(".png"):
            samples.append(file)
    return samples

files = get_samples()