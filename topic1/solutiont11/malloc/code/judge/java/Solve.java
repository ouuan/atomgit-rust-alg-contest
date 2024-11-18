public class Solve {

    
    MyUtils myUtils = new MyUtils();

  
    // your code below 
    final int WSIZE = 4;

    int align(int size) {
        return (((size) + (Config.ALIGNMENT - 1)) & ~0x7);
    }

    int myInit() {
        return 0;
    }


    // return Pos
    int myMalloc(int size) {
        int alingedSize = align(size + align(WSIZE));

        int ptr = myUtils.memSbrk(alingedSize);
        if (ptr == -1)
            return -1;//null
        else {
            myUtils.setPtrValue(ptr, size);
            return ptr + align(WSIZE);
        }
    }

    void myFree(int ptr) {

    }

    int myRealloc(int ptr, int size) {
        //your code here,you can remove all the code below if you need.
        //The code below is just a very navie approach.
        //In this way, realloc is implemented directly using my_malloc and my_free.
        int newptr;
        int copysize;
        if (size == 0) {
            myFree(ptr);
            return -1;//null
        }

        if (ptr == -1) {
            return myMalloc(size);
        }

        newptr = myMalloc(size);
        if (newptr < 0) {
            return -1;//null
        }

        copysize = myUtils.getPtrValueInt(ptr - align(WSIZE));
        if (size < copysize)
            copysize = size;
        myUtils.memcpy(newptr, ptr, copysize);
        myFree(ptr);
        return newptr;
    }

}
