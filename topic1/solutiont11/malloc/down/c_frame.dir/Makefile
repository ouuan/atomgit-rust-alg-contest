CC = gcc
CFLAGS = -Wall  -Wno-unused-result -O2 -g
OBJS = tester.o solve.o utils.o

tester: $(OBJS)
	$(CC) $(CFLAGS) -o tester $(OBJS)

tester.o: tester.c tester.h utils.h solve.h
utils.o: utils.c utils.h
solve.o: solve.c solve.h utils.h

clean:
	rm -f *~ *.o tester
