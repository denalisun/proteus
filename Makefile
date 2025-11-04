CC = x86_64-w64-mingw32-gcc
SRC = src/main.c
OUT = proteus.dll
CFLAGS = -I include/ -L lib/ -std=c99

build:
	$(CC) -shared -o $(OUT) $(SRC) -Wall $(CFLAGS)