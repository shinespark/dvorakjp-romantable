init:
	pipenv install

emoji:
	python3 make_emoji.py --sort > ./lib/emoji.txt
	cat dvorakjp_prime.txt ./lib/emoji.txt > dvorakjp_prime_with_emoji.txt

check_duplicate:
	python3 detect_duplicate.py dvorakjp_prime.txt