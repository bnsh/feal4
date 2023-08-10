CC=gcc
CFLAGS=-g3 -O2 -Wall -Werror

all: feal-8

feal-8: feal-8.o
	$(CC) $(CFLAGS) $(^) -o $(@)

%.o: %.c
	$(CC) -c $(CFLAGS) $(^) -o $(@)
