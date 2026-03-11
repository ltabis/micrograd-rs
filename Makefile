DOT_FILE := test.dot
SVG_FILE := test.svg

all:
	cargo test
	dot -Tsvg $(DOT_FILE) > $(SVG_FILE)
	firefox $(SVG_FILE)

clean:
	rm $(DOT_FILE) $(SVG_FILE)