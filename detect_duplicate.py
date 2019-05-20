import argparse
from collections import Counter

parser = argparse.ArgumentParser()
parser.add_argument('file_path', help='重複チェックする対象のファイルパス')
args = parser.parse_args()

with open(args.file_path) as file:
    input_words = [line.split('\t')[0] for line in file]

counter = Counter(input_words)
for input_word, count in counter.items():
    if count == 1:
        continue

    print(input_word, count)
