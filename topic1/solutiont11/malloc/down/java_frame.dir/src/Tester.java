import java.io.*;
import java.util.*;

public class Tester {

    private Solve solver;
    private int verbose;

    Tester() {
        verbose = 1;
        solver = new Solve();
    }

    final int lineNum(int i) {
        return i + 3;
    }

    final boolean isAligned(int pos) {
        return (pos % Config.ALIGNMENT) == 0;
    }

    class RangeList {
        class Range {
            int low;
            int high;
            Range next;

            public Range() {
                next = null;
            }

            public Range(int _low, int _high) {
                low = _low;
                high = _high;
                next = null;
            }

            public Range(int _low, int _high, Range _next) {
                low = _low;
                high = _high;
                next = _next;
            }
        }

        Range head;
        int size;

        public RangeList() {
            head = new Range();
            size = 0;
        }

        public boolean add(int low, int bsize, String filename, int opnum) {
            if (check(low, bsize, filename, opnum)) {
                Range tmp = new Range(low, low + bsize - 1);
                tmp.next = head.next;
                head.next = tmp;
                size++;
                return true;
            }
            return false;

        }

        private boolean check(int low, int bsize, String filename, int opnum) {
            int high = low + bsize - 1;
            if (!isAligned(low)) {
                mallocError(filename, opnum, "Payload address not aligned to 8 bytes");
                return false;
            }

            if ((low < solver.myUtils.memHeapLow()) || (low > solver.myUtils.memHeapHigh())
                    || (high < solver.myUtils.memHeapLow()) || (high > solver.myUtils.memHeapHigh())) {

                mallocError(filename, opnum, "Payload lies outside heap");
                return false;
            }

            for (Range temp = head.next; null != temp; temp = temp.next) {
                if ((low >= temp.low && low <= temp.high) || (high >= temp.low && high <= temp.high)) {
                    mallocError(filename, opnum, "overlaps");
                    return false;
                }
            }
            return true;
        }

        public void remove(int low) {
            Range prev = head;
            Range current = head.next;

            while (current != null && current.low != low) {
                prev = current;
                current = current.next;
            }
            if (current == null)
                return;
            prev.next = current.next;
            size--;
        }

        public void clear() {
            head.next = null;
            size = 0;
        }

        public void printList() {
            if (size == 0)
                System.out.println("链表为空");
            else {
                for (Range cur = head.next; null != cur; cur = cur.next) {
                    System.out.println(cur.low + " " + cur.high);
                }
            }
        }

    }

    class TraceOp {
        public int type;// 0 alloc 1 free 2 realloc
        public int index;
        public int size;

        public TraceOp(int _type, int _index, int _size) {
            type = _type;
            index = _index;
            size = _size;
        }

        void print() {
            System.out.printf("%d\t%d\t%d\n", type, index, size);
        }
    }

    class Trace {
        int numIds;
        int numOps;
        TraceOp[] ops;
        int[] blocksPos;
        int[] blockSize;
        String fileName;

        public Trace() {

        }

        public Trace(int _numIds, int _numOps, String _fileName) {
            numIds = _numIds;
            numOps = _numOps;
            ops = new TraceOp[_numOps];
            blockSize = new int[_numIds];
            blocksPos = new int[_numIds];
            fileName = _fileName;
        }

        void print() {
            System.out.println("trace print start");
            System.out.println(fileName);
            System.out.println(numIds);
            System.out.println(numOps);
            for (int i = 0; i < numOps; i++) {
                ops[i].print();
            }
            System.out.println("trace print end");
        }
    }

    class Stats {
        String fileName;
        double ops;
        boolean valid;
        double secs;
        double util;
    }

    void run(String[] args) {
        String dataDir = Config.DATADIR;
        String[] dataFiles = Config.DEFAULT_DATAFILES;
        int numFiles = Config.FILENUM;
        int c;
        boolean onetime = false;

        GetOpt getOpt = new GetOpt(args, "f:c:hv");
        while ((c = getOpt.getNextOption()) != -1) {
            switch (c) {
            case 'h':
                usage();
                System.exit(0);
                break;
            case 'f':
                numFiles = 1;
                dataDir = "./";
                dataFiles = new String[1];
                dataFiles[0] = getOpt.getOptionArg();
                System.out.printf("Testing datafile :%s\n", dataFiles[0]);
                break;
            case 'c':
                numFiles = 1;
                onetime = true;
                dataDir = "./";
                dataFiles = new String[1];
                dataFiles[0] = getOpt.getOptionArg();
                System.out.printf("Testing datafile :%s\n", dataFiles[0]);
                break;
            case 'v':
                verbose = 2;
                break;
            default:
                usage();
                System.exit(1);
            }
        }

        if (c == -1) {
            System.out.printf("Using default datafiles in %s\n", dataDir);
        }

        if (verbose > 1) {
            System.out.printf("Testing you malloc\n");
        }

        Trace trace;
        Stats[] stats = new Stats[numFiles];

        RangeList rangeList = new RangeList();
        solver.myUtils.memInit();

        for (int i = 0; i < numFiles; ++i) {
            trace = readTrace(dataDir, dataFiles[i]);
            stats[i] = new Stats();
            stats[i].fileName = trace.fileName;
            stats[i].ops = trace.numOps;
            if (verbose > 1)
                System.out.printf("Checking for correctness, ");

            stats[i].valid = evalValid(trace, i, rangeList);

            if (onetime) {
                break;
            }
            if (stats[i].valid) {
                if (verbose > 1)
                    System.out.printf("utilization, ");

                stats[i].util = evalUtil(trace, i);
                if (verbose > 1)
                    System.out.printf("and time.\n");
                stats[i].secs = evalSpeed(trace, i, 10);
            }
        }

        if (onetime) {
            System.out.printf("Correctness check finished, by running testfile \"%s\".\n", dataFiles[numFiles - 1]);
            if (stats[numFiles - 1].valid) {
                System.out.printf(" => correct. \n");
            } else {
                System.out.printf(" => incorrect. \n");
            }
        } else {
            print("Results for your malloc:");
            printResult(numFiles, stats);
            print("\n");
        }

        solver.myUtils.memDeInit();

    }

    private Trace readTrace(String traceDir, String fileName) {
        try {

            if (verbose > 1) {
                System.out.printf("Read testfile: %s\n", fileName);
            }
            File dir = new File(traceDir);
            File fin = new File(dir.getCanonicalPath() + File.separator + fileName);

            FileInputStream fis = new FileInputStream(fin);
            BufferedReader br = new BufferedReader(new InputStreamReader(fis));

            int ids = Integer.parseInt(br.readLine());
            int ops = Integer.parseInt(br.readLine());
            Trace _trace = new Trace(ids, ops, fileName);
            int opIndex = 0;
            int maxIndex = 0;
            int index = 0;
            String line = null;
            int size = 0;

            while ((line = br.readLine()) != null) {
                String[] tmp = line.split(" ");
                index = Integer.parseInt(tmp[1]);
                if (tmp.length == 3) {
                    size = Integer.parseInt(tmp[2]);
                }
                switch (tmp[0]) {
                case "a":
                    _trace.ops[opIndex] = new TraceOp(0, index, size);
                    maxIndex = (index > maxIndex) ? index : maxIndex;

                    break;
                case "f":
                    _trace.ops[opIndex] = new TraceOp(1, index, 0);
                    break;
                case "r":
                    _trace.ops[opIndex] = new TraceOp(2, index, size);
                    maxIndex = (index > maxIndex) ? index : maxIndex;
                    break;
                default:
                    System.out.printf("Bogus type character (%s) in tracefile %s\n", tmp[0], fileName);
                    break;
                }
                opIndex++;
                if (opIndex == _trace.numOps)
                    break;
            }
            br.close();

            // assert
            if (maxIndex != _trace.numIds - 1) {
                System.out.println(maxIndex);
                System.out.println("maxIndex!=_trace.numIds");
                System.exit(1);
            }
            if (opIndex != _trace.numOps) {
                System.out.println("opIndex!=_trace.numOps");
                System.exit(1);
            }
            return _trace;

        } catch (IOException e) {
            e.printStackTrace();
            return null;
        }
    }

    private void reinitTrace(Trace trace) {
        Arrays.fill(trace.blocksPos, -1);
    }

    private boolean evalValid(Trace trace, int tracenum, RangeList list) {

        solver.myUtils.memResetBrk();
        list.clear();
        reinitTrace(trace);

        if (solver.myInit() < 0) {
            mallocError(trace.fileName, 0, "my init failed in evalValid.");
            return false;
        }

        int i, index, size, oldSize;
        int pos, oldPos;
        for (i = 0; i < trace.numOps; ++i) {
            index = trace.ops[i].index;
            size = trace.ops[i].size;
            switch (trace.ops[i].type) {
            case 0:// alloc
                if ((pos = solver.myMalloc(size)) < 0) {
                    mallocError(trace.fileName, i, "my_malloc failed in evalValid.");
                    return false;
                }
                if (!list.add(pos, size, trace.fileName, i))
                    return false;

                for (int k = 0; k < size; k++) {
                    solver.myUtils.heap[pos + k] = (byte) (index & 0xff);
                }

                trace.blocksPos[index] = pos;
                trace.blockSize[index] = size;
                break;
            case 1:// free
                if (index == -1) {
                    pos = -1;
                } else {
                    pos = trace.blocksPos[index];
                    list.remove(pos);
                }
                solver.myFree(pos);
                break;

            case 2:// realloc
                oldPos = trace.blocksPos[index];
                pos = solver.myRealloc(oldPos, size);
                if (pos < 0 && size != 0) {
                    mallocError(trace.fileName, i, "my_realloc failed in evalValid.");
                    return false;
                }

                if (pos >= 0 && size == 0) {
                    mallocError(trace.fileName, i, "my_realloc with size 0 returned non-NULL.");
                    return false;
                }

                list.remove(oldPos);

                if (size > 0) {
                    if (!list.add(pos, size, trace.fileName, i))
                        return false;
                }

                oldSize = trace.blockSize[index];
                if (size < oldSize)
                    oldSize = size;

                for (int j = 0; j < oldSize; j++) {
                    if (solver.myUtils.heap[pos + j] != ((byte) (index & 0xFF))) {
                        mallocError(trace.fileName, i, "my_realloc did not preserve the data from old block");
                        return false;
                    }
                }

                for (int k = 0; k < size; k++) {
                    solver.myUtils.heap[pos + k] = (byte) (index & 0xff);
                }

                trace.blocksPos[index] = pos;
                trace.blockSize[index] = size;
                break;

            default:
                appError("Nonexistent request type in evalValid");
                break;
            }
        }

        return true;
    }

    private double evalUtil(Trace trace, int tracenum) {
        reinitTrace(trace);
        solver.myUtils.memResetBrk();

        if (solver.myInit() < 0) {
            mallocError(trace.fileName, 0, "my_init failed in evalUtil.");
            return -1;
        }

        int i, index, size, oldSize;
        int totalSize = 0;
        int maxTotalSize = 0;
        int pos, oldPos;
        for (i = 0; i < trace.numOps; ++i) {
            index = trace.ops[i].index;
            size = trace.ops[i].size;
            switch (trace.ops[i].type) {
            case 0:// alloc
                if ((pos = solver.myMalloc(size)) < 0) {
                    mallocError(trace.fileName, i, "my_malloc failed in evalUtil.");
                    return -1;
                }

                trace.blocksPos[index] = pos;
                trace.blockSize[index] = size;
                totalSize += size;
                maxTotalSize = Math.max(maxTotalSize, totalSize);
                break;

            case 1:// free
                if (index < 0) {
                    size = 0;
                    pos = -1;
                } else {
                    pos = trace.blocksPos[index];
                    size = trace.blockSize[index];
                }
                solver.myFree(pos);
                totalSize -= size;
                break;

            case 2:// realloc
                oldPos = trace.blocksPos[index];
                if ((pos = solver.myRealloc(oldPos, size)) < 0 && size != 0) {
                    mallocError(trace.fileName, i, "my_realloc failed in evalUtil.");
                    return -1;
                }

                oldSize = trace.blockSize[index];

                trace.blocksPos[index] = pos;
                trace.blockSize[index] = size;
                totalSize += (size - oldSize);
                maxTotalSize = Math.max(maxTotalSize, totalSize);
                break;

            default:
                appError("Nonexistent request type in evalUtil");
                break;
            }
        }
        return (double) maxTotalSize / (double) solver.myUtils.memHeapSize();
    }

    private void evalSpeedFunc(Trace trace, int tracenum) {
        reinitTrace(trace);
        solver.myUtils.memResetBrk();
        if (solver.myInit() < 0) {
            mallocError(trace.fileName, 0, "my_init failed in evalSpeedFunc.");
            return;
        }
        int i, index, size;
        int pos, oldPos;
        for (i = 0; i < trace.numOps; ++i) {
            index = trace.ops[i].index;
            size = trace.ops[i].size;
            switch (trace.ops[i].type) {
            case 0:// alloc
                if ((pos = solver.myMalloc(size)) < 0) {
                    mallocError(trace.fileName, i, "my_malloc failed in evalSpeedFunc.");
                    return;
                }
                trace.blocksPos[index] = pos;
                break;

            case 1:// free
                if (index < 0) {
                    pos = -1;
                } else {
                    pos = trace.blocksPos[index];
                }
                solver.myFree(pos);
                break;

            case 2:// realloc
                oldPos = trace.blocksPos[index];
                if ((pos = solver.myRealloc(oldPos, size)) < 0 && size != 0) {
                    mallocError(trace.fileName, i, "my_realloc failed in evalSpeedFunc.");
                    return;
                }
                trace.blocksPos[index] = pos;
                break;
            default:
                appError("Nonexistent request type in evalSpeedFunc");
                break;
            }
        }
    }

    private double evalSpeed(Trace trace, int tracenum, int n) {
        long start = System.nanoTime();
        for (int i = 0; i < n; ++i) {
            evalSpeedFunc(trace, tracenum);
        }
        long end = System.nanoTime();
        double diff = (double) (end - start) / 1000000000L;
        return diff / n;
    }

    private void printResult(int n, Stats[] stats) {
        int i;
        System.out.printf("%4s%10s%10s%10s %s\n", "id", " valid", "util", "secs", "filename");

        for (i = 0; i < n; i++) {
            if (stats[i].valid) {
                System.out.printf("%4d%10s%10.6f%10.6f %s\n", i, "yes", stats[i].util, stats[i].secs,
                            stats[i].fileName);
            } else {
                System.out.printf("%4d%10s%10s%10s %s\n", i, "no", "-", "-", stats[i].fileName);
            }
        }

    }

    private void usage() {
        print("Usage: java MyMain [-h] [-v] [-f/c <file>]\n");
        print("Options\n");
        print("\t-f <file>  Use <file> as the test file.\n");
        print("\t-c <file>  Run test file <file> once, check for correctness only.\n");
        print("\t-v         Print extra info as each testfile is run.\n");
        print("\t-h         Print help.\n");
    }

    private void print(String msg) {
        System.out.println(msg);
    }

    private void appError(String msg) {
        print(msg);
        System.exit(1);
    }

    private void unixError(String msg) {
        print(msg);
        System.exit(1);
    }

    private void mallocError(String filename, int opnum, String msg) {
        System.out.format("ERROR [Testfile %s, line %d]: %s\n", filename, lineNum(opnum), msg);
    }

}
