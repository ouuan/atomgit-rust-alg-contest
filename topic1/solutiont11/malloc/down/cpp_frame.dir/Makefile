CC = g++
CPPFLAGS =-Wall -Wno-unused-result -O2 -g
OBJS =tester.o solve.o utils.o

tester: $(OBJS)
	$(CC) $(CPPFLAGS) -o tester $(OBJS)

tester.o: tester.cpp tester.h utils.h solve.h
utils.o: utils.cpp utils.h
solve.o: solve.cpp solve.h utils.h

clean:
	rm -f *~ *.o tester
