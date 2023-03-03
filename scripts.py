

path = "/Users/michele.tasca/My Stuff/Coding stuff/Julia/Trainyard/assets/all_puzzles_clean.json"

import json
from pathlib import Path
data = json.loads(Path(path).read_text())



# A list of dicts like this: 
data[0]
# {'local_filename_map': 'Red Line.png',
#  'name': 'Red Line',
#  'solutions_url': 'http://www.trainyard.ca/solutions/redLine',
#  'city': 'Abbotsford Puzzles',
#  'parsed_map': '00 00 00 00 00 00 00\n00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 00 00 00 00',
#  'type_': 'Regular puzzles',
#  'trackCount': 'Best track count: 3+0',
#  'big_image_url': 'http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798688_large.png',
#  'thumb': 'http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redLine_thumb.png'}

#  Get all Disstinct values in the "type_" field
type_s = set([d['type_'] for d in data])  # {'Bonus puzzles', 'Express puzzles', 'Regular puzzles'}

# Reorder data like this:
# first, alternate dict of type_ 'Regular puzzles' and 'Express puzzles' (one by one)
# Then, append all 'Bonus puzzles'

data_reordered = []

# Get all 'Regular puzzles':
data_regular = [d for d in data if d['type_'] == 'Regular puzzles']
# Get all 'Express puzzles':
data_express = [d for d in data if d['type_'] == 'Express puzzles']
# Get all 'Bonus puzzles':
data_bonus = [d for d in data if d['type_'] == 'Bonus puzzles']

# Alternate between Regular and Express puzzles:
for i in range(max(len(data_regular), len(data_express))):
    if i < len(data_regular):
        data_reordered.append(data_regular[i])
    if i < len(data_express):
        data_reordered.append(data_express[i])


# Append all 'Bonus puzzles':
data_reordered.extend(data_bonus)

len(data_reordered)

names = [d['name'] for d in data_reordered]


# Format each data in data_reordered as a STRING of rome rust code that looks like this:
    # PuzzleData {
    #     // "local_filename_map": "Red Line.png",
    #     name: "Red Line".to_string(),
    #     // "solutions_url": "http://www.trainyard.ca/solutions/redLine",
    #     city: "Abbotsford Puzzles".to_string(),
    #     parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 00 00 00 00".to_string(),
    #     type_: "Regular puzzles".to_string(),
    #     trackCount: "3/0".to_string(),
    #     // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798688_large.png",
    #     // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redLine_thumb.png"
    # },

string_template_to_format = """
    PuzzleData {{
        // "local_filename_map": "{local_filename_map}",
        name: "{name}".to_string(),
        // "solutions_url": "{solutions_url}",
        city: "{city}".to_string(),
        parsed_map: "{parsed_map}".to_string(),
        type_: "{type_}".to_string(),
        track_count: "{trackCount}".to_string(),
        // "big_image_url": "{big_image_url}",
        // "thumb": "{thumb}"
    }},"""

new_codes = []
for d in data_reordered:
    # Escape \n as \\n in d['parsed_map']:
    d = {k: v.replace("\n", "\\n") for k, v in d.items()}
    # Turn trackCount from 'Best track count: 3+0'in '3/0':
    d['trackCount'] = d['trackCount'].replace('+', '/').replace('Best track count: ', '')
    new_codes.append(string_template_to_format.format(**d))
new_code = "".join(new_codes)

# Read "/Users/michele.tasca/My Stuff/Coding stuff/OpenTrainyard/src/all_puzzles_clean_template.rs" astext file:
path_template = "/Users/michele.tasca/My Stuff/Coding stuff/OpenTrainyard/src/all_puzzles_clean_template.rs"

template = Path(path_template).read_text()

# Sobstitute "{/// HEREEEE PUT THEM HERE}" with new_code
template = template.replace("{/// HEREEEE PUT THEM HERE}", new_code)
# {PUT HERE LENGTH} with len(data_reordered)
template = template.replace("{PUT HERE LENGTH}", str(len(data_reordered)))

# Write the new file to "/Users/michele.tasca/My Stuff/Coding stuff/OpenTrainyard/src/all_puzzles_clean.rs"
path_new_file = "/Users/michele.tasca/My Stuff/Coding stuff/OpenTrainyard/src/all_puzzles_clean.rs"
Path(path_new_file).write_text(template)




path_images = "/Users/michele.tasca/My Stuff/Coding stuff/Julia/Trainyard/assets"
import os
import shutil


for i, name in enumerate(names):
    print(name)
    print(f'![]({path_images}/{name}.png)')
    i_formatted = str(i+1).zfill(3)

    # save image named name as path intto "saved_images_ordered" folder, with name i_formatted_name.png 
    # (e.g. 001_Red Line.png)
    output_path = f'{path_images}/saved_images_ordered/{i_formatted}_{name}.png'
    input_path = f'{path_images}/level_images/{name}.png'
    # !cp $input_path $output_path
    # Use a library:
    # os.system(f'cp {input_path} {output_path}')
    # Or:
    shutil.copyfile(input_path, output_path)




################################################################




# Get all repeated names:
names_json = [d['name'] for d in data]
len(names_json)
len(names_rust)

def set_diff_stats(**kwargs):
    assert len(kwargs) == 2, 'set_diff_stats() takes exactly 2 arguments'
    (name_set1, name_set2), (set1, set2) = kwargs.keys(), kwargs.values()
    set1, set2 = set(set1), set(set2)
    print(f'len({name_set1})={len(set1)}', f'len({name_set2})={len(set2)}')
    print(f'len({name_set1}.intersection({name_set2}))={len(set1.intersection(set2))}')
    print(f'len({name_set1}.difference({name_set2}))={len(set1.difference(set2))}')
    print(f'len({name_set2}.difference({name_set1}))={len(set2.difference(set1))}')

    print(f'Fraction of {name_set1} that is in {name_set2}:', len(set1.intersection(set2)) / len(set1))
    print(f'Fraction of {name_set2} that is in {name_set1}:', len(set2.intersection(set1)) / len(set2))

    print(f'Elements that are in {name_set1} but not in {name_set2}:', set1.difference(set2))
    print(f'Elements that are in {name_set2} but not in {name_set1}:', set2.difference(set1))

set_diff_stats(names_rust=names_rust, names_json=names_json)



pd.Series(names_rust)[pd.Series(names_rust).duplicated()]


# For each value in "city", get how many times it appears in the list of dicts:

from collections import Counter
Counter([d['city'] for d in data])

# Get max:

max(Counter([d['city'] for d in data]).values())
