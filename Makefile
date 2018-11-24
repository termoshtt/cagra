DOT = $(wildcard *.dot)
PNG = $(patsubst %.dot,%.png,$(DOT))

all: $(PNG)

%.png: %.dot
	dot -Tpng $< -o $@
