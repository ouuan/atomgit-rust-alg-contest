CC = g++
CPPFLAGS =-Wall -Wno-unused-result -O2 -g
OBJS =tester.o source.o utils.o

tester: $(OBJS)
	$(CC) $(CPPFLAGS) -o tester $(OBJS)

tester.o: tester.cpp tester.h utils.h solve.h
utils.o: utils.cpp utils.h
source.o: source.cpp solve.h utils.h

clean:
	rm -f *~ *.o tester
