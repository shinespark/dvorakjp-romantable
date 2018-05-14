import argparse
import requests
import sys
from collections import OrderedDict

parser = argparse.ArgumentParser()
parser.add_argument("-s", "--sort", help="output sorted emojis list", action="store_true")
args = parser.parse_args()

URL = 'https://raw.githubusercontent.com/emojione/emojione/master/emoji.json'
r = requests.get(URL)

if r.status_code != requests.codes.ok:
    print("Couldn't get a response. HTTP Status Code: %s" % r.status_code)
    sys.exit()


pre_d = {}  # { name: emoji }
for code, v in r.json().items():
    shortname = v['shortname'].rstrip(':')
    shortname_alternates = v['shortname_alternates']
    code_points = v['code_points']['fully_qualified']

    if '-' in code_points:  # surrogate pairs
        splited_code_points = code_points.split('-')
        unicode = ''
        for code_point in splited_code_points:
            unicode += chr(int(code_point, 16))
        emoji = unicode.encode('utf-16', 'surrogatepass').decode('utf-16')
    else:
        emoji = chr(int(code_points, 16))

    pre_d[shortname] = emoji

    # only add non forward-matched emojis
    for i in shortname_alternates:
        if not i.startswith(shortname):
            pre_d[i.rstrip(':')] = emoji

# add colon to prevent unsettled conversion
d = {}
for k, v in pre_d.items():
    is_forward_match = False
    for key in pre_d.keys():
        if key == k:  # skip own
            continue
        if key.startswith(k):
            is_forward_match = True
            break

    if is_forward_match:
        d[k + ':'] = v
    else:
        d[k] = v

# output
if args.sort:
    for k, v in OrderedDict(sorted(d.items(), key=lambda t: t[0])).items():
        print("%s\t%s" % (k, v))
else:
    for k, v in d.items():
        print("%s\t%s" % (k, v))
