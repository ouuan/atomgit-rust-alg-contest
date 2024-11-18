import java.nio.ByteBuffer;
import java.nio.ByteOrder;

public class MyUtils {

    private int g_memStartBrk = 0;
    private int g_memBrk;
    private int g_memMaxAddr;

    byte[] g_heap;

    final void setPtrValue(int ptr, byte value) {
        g_heap[ptr] = value;
    }

    final void setPtrValue(int ptr, char value) {
        ByteBuffer buf = ByteBuffer.allocate(2);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putChar(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, short value) {
        ByteBuffer buf = ByteBuffer.allocate(2);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putShort(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, int value) {
        ByteBuffer buf = ByteBuffer.allocate(4);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putInt(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, long value) {
        ByteBuffer buf = ByteBuffer.allocate(8);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putLong(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }
    }

    final void setPtrValue(int ptr, float value) {
        ByteBuffer buf = ByteBuffer.allocate(4);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putFloat(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }

    }

    final void setPtrValue(int ptr, double value) {
        ByteBuffer buf = ByteBuffer.allocate(8);
        buf.order(ByteOrder.LITTLE_ENDIAN);
        byte[] bytes = buf.putDouble(value).array();
        for (byte b : bytes) {
            g_heap[ptr] = b;
            ptr++;
        }

    }

    final void setPtrValue(int ptr, boolean value) {
        g_heap[ptr] = (value ? (byte) 1 : (byte) 0);
    }


    final int getPtrValuePtr(int ptr) {
        return getPtrValueInt(ptr);
    }

    final int getPtrValueInt(int ptr) {
        return (int) ((((g_heap[ptr + 3] & 0xFF) << 24)
                | ((g_heap[ptr + 2] & 0xFF) << 16)
                | ((g_heap[ptr + 1] & 0xFF) << 8)
                | ((g_heap[ptr] & 0xFF))));//big

    }

    final char getPtrValueChar(int ptr) {
        return (char) ((((char) g_heap[ptr + 1]) & 0xFF << 8) | (((char) g_heap[ptr] & 0xFF)));
    }

    final byte getPtrValueByte(int ptr) {
        return g_heap[ptr];
    }

    final short getPtrValueShort(int ptr) {
        return (short) ((((short) g_heap[ptr + 1]) & 0xff << 8) | (((short) g_heap[ptr] & 0xFF)));

    }

    final long getPtrValueLong(int ptr) {

        return ((((long) g_heap[ptr + 7] & 0xff) << 56)
                | (((long) g_heap[ptr + 6] & 0xff) << 48)
                | (((long) g_heap[ptr + 5] & 0xff) << 40)
                | (((long) g_heap[ptr + 4] & 0xff) << 32)
                | (((long) g_heap[ptr + 3] & 0xff) << 24)
                | (((long) g_heap[ptr + 2] & 0xff) << 16)
                | (((long) g_heap[ptr + 1] & 0xff) << 8) | (((long) g_heap[ptr + 0] & 0xff) << 0));

    }

    final boolean getPtrValueBool(int ptr) {
        return g_heap[ptr] != 0;
    }

    void memcpy(int dst, int src, int size) {
        if ((dst > src + size) || dst < src) {
            while (size != 0) {
                g_heap[dst] = g_heap[src];
                dst++;
                src++;
                size--;
            }
        } else {
            int d = dst + size - 1;
            int s = src + size - 1;
            while (size != 0) {
                g_heap[d] = g_heap[s];
                d--;
                s--;
                size--;
            }
        }
    }

    void memInit() {
        g_heap = new byte[Config.MAXHEAP];
        g_memMaxAddr = g_memStartBrk + Config.MAXHEAP;
        g_memBrk = g_memStartBrk;
    }

    void memDeInit() {
        g_memStartBrk = 0;
        g_memBrk = 0;
        g_memMaxAddr = 0;
    }

    int memSbrk(int incr) {
        int oldBrk = g_memBrk;
        if ((incr < 0) || ((g_memBrk + incr) > g_memMaxAddr)) {
            // System.out.println("ERROR: memSbrk failed. Ran out of memory...");
            return -1;
        }
        g_memBrk += incr;
        return oldBrk;
    }

    void memResetBrk() {
        g_memBrk = g_memStartBrk;
    }

    int memHeapLow() {
        return g_memStartBrk;
    }

    int memHeapHigh() {
        return (g_memBrk - 1);
    }

    int memHeapSize() {
        return g_memBrk - g_memStartBrk;
    }
}

