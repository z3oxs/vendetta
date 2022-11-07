install:
	cargo install --path .
	mkdir ${HOME}/.config/vendetta/
	cp ./assets/* ${HOME}/.config/vendetta/

update:
	cargo install --path .
