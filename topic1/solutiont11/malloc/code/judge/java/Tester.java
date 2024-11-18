import java.io.*;
import java.util.*;

public class Tester {

    private Solve solver;
    private int verbose;
    private int error;
    private String flag;
    Tester() {
        error=0;
        verbose = 0;
        solver = new Solve();
        flag="77796755c34b3aada95b7ab2caf33d5ec337d567e58a328cf02c139b29bc650c";
    }

    final int lineNum(int i) {
        return i + 5;
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
                return false;
            }

            if ((low < solver.myUtils.memHeapLow()) || (low > solver.myUtils.memHeapHigh()) ||
                    (high < solver.myUtils.memHeapLow()) || (high > solver.myUtils.memHeapHigh())) {
                return false;
            }


            for (Range temp = head.next; null != temp; temp = temp.next) {
                if ((low >= temp.low && low <= temp.high) ||
                        (high >= temp.low && high <= temp.high)) {
                    
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
            if(current==null)
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
                System.out.println("list null");
            else {
                for (Range cur = head.next; null != cur; cur = cur.next) {
                    System.out.println(cur.low + " " + cur.high);
                }
            }
        }

    }


    class TraceOp {
        public int type;
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
        int weight;

        public Trace() {

        }

        public Trace(int _numIds, int _numOps, String _fileName, int _weight) {
            numIds = _numIds;
            numOps = _numOps;
            ops = new TraceOp[_numOps];
            blockSize = new int[_numIds];
            blocksPos = new int[_numIds];
            fileName = _fileName;
            weight = _weight;
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
        int weight;
    }



    void run(String[] args) {
        String dataDir = Config.DATADIR;
        String[] dataFiles = Config.DEFAULT_DATAFILES;

        int numFiles = Config.FILENUM;
        int errors = 0;
        int c;
        boolean onetime = false;

        numFiles = 1;
        dataDir = "./";
        dataFiles = new String[1];
        
        dataFiles[0] = "asdasd";

        Trace trace;
        Stats[] g_stats = new Stats[numFiles]; 


        RangeList g_rangeList = new RangeList();
        solver.myUtils.memInit();

        for (int i = 0; i < numFiles; ++i) {
            
            trace = readTrace(dataDir, dataFiles[i]);
            g_stats[i] = new Stats();
            g_stats[i].fileName = trace.fileName;
            g_stats[i].ops = trace.numOps;
            g_stats[i].weight = trace.weight;
            
            

            if (verbose > 1)
                System.out.printf("Checking for correctness, ");

            g_stats[i].valid = evalValid(trace, i, g_rangeList);

            if (onetime) {
                break;
            }
            if (g_stats[i].valid) {
                if (verbose > 1)
                    System.out.printf("utilization, ");

                g_stats[i].util = evalUtil(trace, i);

                if (verbose > 1)
                    System.out.printf("and time.\n");
                g_stats[i].secs = evalSpeed(trace, i, 10);
            }

            if (g_stats[i].valid && error == 0) {
                System.out.printf("%s %d %.6f %.6f\n", flag,1, 1 - g_stats[i].util,
                       g_stats[i].secs);
            } else {
                System.out.printf("%s %d %.6f %.6f\n", flag,0, 1.0,99999999.0);
            }
        }

        
        
        
        
        
        
        
        
        
        
        
        

        

        solver.myUtils.memDeInit();

    }

    private Trace readTrace(String traceDir, String fileName) {
        try {

            if (verbose > 1) {
                System.out.printf("Read testfile: %s\n", fileName);
            }
            
            

            
            
            BufferedReader br = new BufferedReader(new InputStreamReader(System.in));



            int i = 0;
            int weight = Integer.parseInt(br.readLine());
            int ids = Integer.parseInt(br.readLine());
            int ops = Integer.parseInt(br.readLine());
            int ignoteFlag = Integer.parseInt(br.readLine());

            if (weight < 0 || weight > 1) {
                
                System.exit(0);
            }
            if (ignoteFlag != 0 && ignoteFlag != 1) {
                
                System.exit(0);
            }

            Trace _trace = new Trace(ids, ops, fileName, weight);

            
            

            int opIndex = 0;
            int maxIndex = 0;
            int index = 0;
            String line = null;
            int size=0;


            while ((line = br.readLine()) != null) {
                String[] tmp = line.split(" ");
        
                index = Integer.parseInt(tmp[1]);
                if(tmp.length==3){
                    size=Integer.parseInt(tmp[2]);
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
                if (opIndex == _trace.numOps) break;
            }
            br.close();

            
            if (maxIndex != _trace.numIds - 1) {
                System.out.println(maxIndex);
                System.out.println("maxIndex!=_trace.numIds");
                System.exit(0);
            }
            if (opIndex != _trace.numOps) {
                System.out.println("opIndex!=_trace.numOps");
                System.exit(0);
            }
            
            return _trace;

        } catch (IOException e) {
            e.printStackTrace();
            return null;
        }
    }

    private void reinitTrace(Trace trace){
        Arrays.fill(trace.blocksPos, -1);
    }

    private boolean evalValid(Trace trace, int tracenum, RangeList list) {


        
        
        solver.myUtils.memResetBrk();
        list.clear();
        reinitTrace(trace);

        if (solver.myInit() < 0) {
            
            return false;
        }

        int i, index, size, oldSize;
        int pos, oldPos;
        for (i = 0; i < trace.numOps; ++i) {
            index = trace.ops[i].index;
            size = trace.ops[i].size;
            switch (trace.ops[i].type) {
                case 0:
                    if ((pos = solver.myMalloc(size)) < 0) {
                        
                        return false;
                    }
                    if (!list.add(pos, size, trace.fileName, i))
                        return false;

                    for (int k = 0; k < size; k++) {
                        solver.myUtils.g_heap[pos + k] = (byte) (index & 0xff);
                        
                    }

                    trace.blocksPos[index] = pos;
                    trace.blockSize[index] = size;
                    break;
                case 1:
                    if (index == -1) {
                        pos = -1;
                    } else {
                        pos = trace.blocksPos[index];
                        list.remove(pos);
                    }
                    solver.myFree(pos);
                    break;

                case 2:
                    oldPos = trace.blocksPos[index];
                    pos = solver.myRealloc(oldPos, size);
                    if (pos < 0 && size != 0) {
                        
                        return false;
                    }

                    if (pos >= 0 && size == 0) {
                        
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
                        if (solver.myUtils.g_heap[pos + j] != ((byte) (index & 0xFF))) {
                            
                            return false;
                        }
                    }

                    for (int k = 0; k < size; k++) {
                        solver.myUtils.g_heap[pos + k] = (byte) (index & 0xff);
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
            
            error++;
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
                case 0:
                    if ((pos = solver.myMalloc(size)) < 0) {
                        
                        error++;

                        return -1;
                    }

                    trace.blocksPos[index] = pos;
                    trace.blockSize[index] = size;
                    totalSize += size;
                    maxTotalSize = Math.max(maxTotalSize, totalSize);
                    break;

                case 1:
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

                case 2:
                    oldPos = trace.blocksPos[index];
                    if ((pos = solver.myRealloc(oldPos, size)) < 0 && size != 0) {
                        
                        error++;

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
            
            return;
        }
        int i, index, size;
        int pos, oldPos;
        for (i = 0; i < trace.numOps; ++i) {
            index = trace.ops[i].index;
            size = trace.ops[i].size;
            switch (trace.ops[i].type) {
                case 0:
                    if ((pos = solver.myMalloc(size)) < 0) {
                        
                        error++;

                        return;
                    }
                    trace.blocksPos[index] = pos;
                    break;

                case 1:
                    if (index < 0) {
                        pos = -1;
                    } else {
                        pos = trace.blocksPos[index];
                    }
                    solver.myFree(pos);
                    break;

                case 2:
                    oldPos = trace.blocksPos[index];
                    if ((pos = solver.myRealloc(oldPos, size)) < 0 && size != 0) {
                        
                        error++;

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


    
    
    
    
    


    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    

    

    
    
    
    
    
    
    
    

    private void print(String msg) {
        System.out.println(msg);
    }

    private void appError(String msg) {
        print(msg);
        System.exit(0);
    }

    private void unixError(String msg) {
        print(msg);
        System.exit(0);
    }

    private void mallocError(String filename, int opnum, String msg) {
        System.out.format("ERROR [trace %s, line %d]: %s\n", filename, lineNum(opnum), msg);
    }


}
