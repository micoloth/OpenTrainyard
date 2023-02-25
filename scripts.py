

path = "/Users/michele.tasca/My Stuff/Coding stuff/Julia/Trainyard/assets/all_puzzles_clean.json"

import json
from pathlib import Path
data = json.loads(Path(path).read_text())



# A list of dicts like this: 
# {'local_filename_map': 'Red Line.png',
#  'name': 'Red Line',
#  'solutions_url': 'http://www.trainyard.ca/solutions/redLine',
#  'city': 'Abbotsford Puzzles',
#  'parsed_map': '00 00 00 00 00 00 00\n00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 00 00 00 00',
#  'type_': 'Regular puzzles',
#  'trackCount': 'Best track count: 3+0',
#  'big_image_url': 'http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798688_large.png',
#  'thumb': 'http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redLine_thumb.png'}

data[0]

# Get all repeated names:
names = [d['name'] for d in data]
import pandas as pd
names = pd.Series(names)
names[names.duplicated()]


# For each value in "city", get how many times it appears in the list of dicts:

from collections import Counter
Counter([d['city'] for d in data])

# Get max:

max(Counter([d['city'] for d in data]).values())