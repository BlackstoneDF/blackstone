import json

# open the file and read the contents
with open('samples\\scripts\\analyze.py', encoding='utf-8') as f:
    data = json.load(f)

# access the data
# print(data['codeblocks'][0])
for i in data['actions']:
    if i['name'] == "ShiftAllDirections":
        print(i['icon']['arguments'])