import java.nio.ByteBuffer;
import java.nio.ByteOrder;

public class MyUtils {

    private int memStartBrk = 0;
    private int memBrk;
    private int memMaxAddr;

    byte[] heap;

    final void setPtrValue(int ptr, byte value) {
        heap[ptr] = value;
    }

    final void setPtrValue(int ptr, char value) {
        ByteBuffer buf = ByteBuffer.allocate(2);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putChar(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, short value) {
        ByteBuffer buf = ByteBuffer.allocate(2);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putShort(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, int value) {
        ByteBuffer buf = ByteBuffer.allocate(4);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putInt(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, long value) {
        ByteBuffer buf = ByteBuffer.allocate(8);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putLong(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, float value) {
        ByteBuffer buf = ByteBuffer.allocate(4);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putFloat(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }

    }

    final void setPtrValue(int ptr, double value) {
        ByteBuffer buf = ByteBuffer.allocate(8);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putDouble(value).array();
        for (byte b : bytes) {
            heap[ptr] = b;
            ptr++;
        }

    }

    final void setPtrValue(int ptr, boolean value) {
        heap[ptr] = (value ? (byte) 1 : (byte) 0);
    }


    final int getPtrValuePtr(int ptr) {
        return getPtrValueInt(ptr);
    }

    final int getPtrValueInt(int ptr) {
        return (int) ((((heap[ptr + 3] & 0xFF) << 24)
                | ((heap[ptr + 2] & 0xFF) << 16)
                | ((heap[ptr + 1] & 0xFF) << 8)
                | ((heap[ptr] & 0xFF))));//big

    }

    final char getPtrValueChar(int ptr) {
        return (char) ((((char) heap[ptr + 1]) & 0xFF << 8) | (((char) heap[ptr] & 0xFF)));
    }

    final byte getPtrValueByte(int ptr) {
        return heap[ptr];
    }

    final short getPtrValueShort(int ptr) {
        return (short) ((((short) heap[ptr + 1]) & 0xff << 8) | (((short) heap[ptr] & 0xFF)));

    }

    final long getPtrValueLong(int ptr) {

        return ((((long) heap[ptr + 7] & 0xff) << 56)
                | (((long) heap[ptr + 6] & 0xff) << 48)
                | (((long) heap[ptr + 5] & 0xff) << 40)
                | (((long) heap[ptr + 4] & 0xff) << 32)
                | (((long) heap[ptr + 3] & 0xff) << 24)
                | (((long) heap[ptr + 2] & 0xff) << 16)
                | (((long) heap[ptr + 1] & 0xff) << 8) | (((long) heap[ptr + 0] & 0xff) << 0));

    }

    final boolean getPtrValueBool(int ptr) {
        return heap[ptr] != 0;
    }

    void memcpy(int dst, int src, int size) {
        if ((dst > src + size) || dst < src) {
            while (size != 0) {
                heap[dst] = heap[src];
                dst++;
                src++;
                size--;
            }
        } else {
            int d = dst + size - 1;
            int s = src + size - 1;
            while (size != 0) {
                heap[d] = heap[s];
                d--;
                s--;
                size--;
            }
        }
    }

    void memInit() {
        heap = new byte[Config.MAXHEAP];
        memMaxAddr = memStartBrk + Config.MAXHEAP;
        memBrk = memStartBrk;
    }

    void memDeInit() {
        memStartBrk = 0;
        memBrk = 0;
        memMaxAddr = 0;
    }

    int memSbrk(int incr) {
        int oldBrk = memBrk;
        if ((incr < 0) || ((memBrk + incr) > memMaxAddr)) {
            System.out.println("ERROR: memSbrk failed. Ran out of memory...");
            return -1;
        }
        memBrk += incr;
        return oldBrk;
    }

    void memResetBrk() {
        memBrk = memStartBrk;
    }

    int memHeapLow() {
        return memStartBrk;
    }

    int memHeapHigh() {
        return (memBrk - 1);
    }

    int memHeapSize() {
        return memBrk - memStartBrk;
    }
}

