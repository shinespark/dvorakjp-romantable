import sys
import requests

URL = 'https://raw.githubusercontent.com/emojione/emojione/master/emoji.json'
r = requests.get(URL)

if r.status_code != requests.codes.ok:
    print("Couldn't get a response. HTTP Status Code: %s" % r.status_code)
    sys.exit()

for code, v in r.json().items():
    shortname = v['shortname']
    shortname_alternates = v['shortname_alternates']
    code_points = v['code_points']['base']

    # ignore surrogate pairs
    if '-' in code_points:
        continue

    emoji = chr(int(code_points, 16))
    print("%s\t%s" % (shortname, emoji))

    for i in shortname_alternates:
        print("%s\t%s" % (i, emoji))
