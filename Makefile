init:
	pip3 install -r requirements.txt

emoji:
	python3 make_emoji.py > emoji.txt
	cat dvorakjp_prime.txt emoji.txt > dvorakjp_prime_with_emoji.txt

sorted_emoji:
	python3 make_emoji.py --sort > emoji.txt
	cat dvorakjp_prime.txt emoji.txt > dvorakjp_prime_with_emoji.txt
