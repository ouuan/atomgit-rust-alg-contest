class Solve {

    MyUtils myUtils = new MyUtils();

    final int WSIZE = 4;
    final int DSIZE = 8;
    final int CHUNKSIZE = 1 << 12;
    final int MINFB_SIZE = 2;


    final int PACK(int size, int alloc) {
        return size | alloc;
    }

    final int GET(int pos) {
        return myUtils.getPtrValueInt(pos);
    }

    final void PUT(int pos, int val) {
        myUtils.setPtrValue(pos, val);
    }

    final int GET_SIZE(int pos) {
        return GET(pos) & ~0x7;
    }

    final boolean GET_ALLOC(int pos) {
        return (GET(pos) & 0x1) == 1;
    }

    final int HDRP(int pos) {
        return pos - WSIZE;
    }

    final int FTRP(int pos) {
        return pos + GET_SIZE(HDRP(pos)) - DSIZE;
    }

    final int PREV_LINKNODE_RP(int pos) {
        return pos;
    }

    final int NEXT_LINKNODE_RP(int pos) {
        return pos + WSIZE;
    }

    final int NEXT_BLKP(int pos) {
        return pos + GET_SIZE(pos - WSIZE);
    }

    final int PREV_BLKP(int pos) {
        return pos - GET_SIZE(pos - DSIZE);
    }

    private int heapListP = 0;
    private int blockListStart = 0;


    final int getFreeCategoryRoot(int size) {

        int i = 0;
        if (size <= 8) i = 0;
        else if (size <= 16) i = 0;
        else if (size <= 32) i = 0;
        else if (size <= 64) i = 1;
        else if (size <= 128) i = 2;
        else if (size <= 256) i = 3;
        else if (size <= 512) i = 4;
        else if (size <= 2048) i = 5;
        else if (size <= 4096) i = 6;
        else i = 7;

        return blockListStart + i * WSIZE;
    }


    int myInit() {

        if ((heapListP = myUtils.memSbrk(12 * WSIZE)) == -1)
            return -1;

        PUT(heapListP, 0);              /*block size list<=32*/
        PUT(heapListP + (1 * WSIZE), 0);    /*block size list<=64*/
        PUT(heapListP + (2 * WSIZE), 0);    /*block size list<=128*/
        PUT(heapListP + (3 * WSIZE), 0);    /*block size list<=256*/
        PUT(heapListP + (4 * WSIZE), 0);    /*block size list<=512*/
        PUT(heapListP + (5 * WSIZE), 0);    /*block size list<=2048*/
        PUT(heapListP + (6 * WSIZE), 0);    /*block size list<=4096*/
        PUT(heapListP + (7 * WSIZE), 0);    /*block size list>4096*/
        PUT(heapListP + (8 * WSIZE), 0);    /* for alignment*/
        PUT(heapListP + (9 * WSIZE), PACK(DSIZE, 1));
        PUT(heapListP + (10 * WSIZE), PACK(DSIZE, 1));
        PUT(heapListP + (11 * WSIZE), PACK(0, 1));

        blockListStart = heapListP;
        heapListP += (10 * WSIZE);

        if ((extendHeap(CHUNKSIZE / DSIZE)) == -1)
            return -1;

        return 0;
    }

    int extendHeap(int dwords) {
        int bp;
        int size;
        size = (dwords % 2 == 1) ? (dwords + 1) * DSIZE : dwords * DSIZE;
        if ((bp = myUtils.memSbrk(size)) == -1) {
            return -1;
        }

        PUT(HDRP(bp), PACK(size, 0));
        PUT(FTRP(bp), PACK(size, 0));


        /*init the prev and next free pointer fields*/
        PUT(NEXT_LINKNODE_RP(bp), 0);//null
        PUT(PREV_LINKNODE_RP(bp), 0);//null

        /*the  epilogue header*/
        PUT(HDRP(NEXT_BLKP(bp)), PACK(0, 1));

        return coalesce(bp);
    }

    int coalesce(int bp) {

        boolean prevAlloc = GET_ALLOC(FTRP(PREV_BLKP(bp)));
        boolean nextAlloc = GET_ALLOC(HDRP(NEXT_BLKP(bp)));
        int size = GET_SIZE(HDRP(bp));

        if (prevAlloc && nextAlloc) {

        } else if (prevAlloc && !nextAlloc) {
            size += GET_SIZE(HDRP(NEXT_BLKP(bp)));

            removeFreeBlock(NEXT_BLKP(bp));
            PUT(HDRP(bp), PACK(size, 0));
            PUT(FTRP(bp), PACK(size, 0));

        } else if (!prevAlloc && nextAlloc) {
            size += GET_SIZE(HDRP(PREV_BLKP(bp)));

            removeFreeBlock(PREV_BLKP(bp));
            PUT(FTRP(bp), PACK(size, 0));
            PUT(HDRP(PREV_BLKP(bp)), PACK(size, 0));
            bp = PREV_BLKP(bp);
        } else {
            size += GET_SIZE(FTRP(NEXT_BLKP(bp))) + GET_SIZE(HDRP(PREV_BLKP(bp)));

            removeFreeBlock(PREV_BLKP(bp));
            removeFreeBlock(NEXT_BLKP(bp));

            PUT(FTRP(NEXT_BLKP(bp)), PACK(size, 0));
            PUT(HDRP(PREV_BLKP(bp)), PACK(size, 0));
            bp = PREV_BLKP(bp);
        }

        /*insert the new free block*/
        insertFreeBlock(bp);
        return bp;

    }

    void removeFreeBlock(int p) {
        int root = getFreeCategoryRoot(GET_SIZE(HDRP(p)));
        int prevp = GET(PREV_LINKNODE_RP(p));
        int nextp = GET(NEXT_LINKNODE_RP(p));

        if (prevp == 0) {
            if (nextp != 0)
                PUT(PREV_LINKNODE_RP(nextp), 0);
            PUT(root, nextp);
        } else {
            if (nextp != 0)
                PUT(PREV_LINKNODE_RP(nextp), prevp);
            PUT(NEXT_LINKNODE_RP(prevp), nextp);
        }
        /*set the next and prev pointers to NULL*/
        PUT(NEXT_LINKNODE_RP(p), 0);
        PUT(PREV_LINKNODE_RP(p), 0);

    }

    void insertFreeBlock(int p) {

        int root = getFreeCategoryRoot(GET_SIZE(HDRP(p)));
        int prevp = root;
        int nextp = GET(root);

        /*find the postion to insert, smaller < p < bigger*/
        while (nextp != 0) {//null 0
            if (GET_SIZE(HDRP(nextp)) >= GET_SIZE(HDRP(p))) break;
            prevp = nextp;
            nextp = GET(NEXT_LINKNODE_RP(nextp));
        }
        /*insert*/
        if (prevp == root) {
            PUT(root, p);
            PUT(NEXT_LINKNODE_RP(p), nextp);
            PUT(PREV_LINKNODE_RP(p), 0);
            if (nextp != 0) PUT(PREV_LINKNODE_RP(nextp), p);
        } else {
            PUT(NEXT_LINKNODE_RP(prevp), p);
            PUT(PREV_LINKNODE_RP(p), prevp);
            PUT(NEXT_LINKNODE_RP(p), nextp);
            if (nextp != 0) PUT(PREV_LINKNODE_RP(nextp), p);
        }

    }

    int findFit(int size) {
        int root = getFreeCategoryRoot(size);
        int tmp;
        for (; root != (heapListP - (2 * WSIZE)); root += WSIZE) {
            tmp = GET(root);
            while (tmp != 0) {//null
                if (GET_SIZE(HDRP(tmp)) >= size)
                    return tmp;
                tmp = GET(NEXT_LINKNODE_RP(tmp));
            }
        }
        return -1;
    }


    void place(int bp, int asize) {
        int csize = GET_SIZE(HDRP(bp));
        removeFreeBlock(bp);
        if ((csize - asize) >= (MINFB_SIZE * DSIZE)) {

            PUT(HDRP(bp), PACK(asize, 1));
            PUT(FTRP(bp), PACK(asize, 1));
            bp = NEXT_BLKP(bp);

            PUT(HDRP(bp), PACK(csize - asize, 0));
            PUT(FTRP(bp), PACK(csize - asize, 0));

            PUT(NEXT_LINKNODE_RP(bp), 0);
            PUT(PREV_LINKNODE_RP(bp), 0);
            coalesce(bp);
        } else {
            PUT(HDRP(bp), PACK(csize, 1));
            PUT(FTRP(bp), PACK(csize, 1));
        }
    }


    // return Pos
    int myMalloc(int size) {

        int asize;
        int extendsize;
        int bp;
        if (size == 0)
            return -1;

        if (size <= DSIZE) {
            asize = 2 * (DSIZE);
        } else {
            asize = (DSIZE) * ((size + (DSIZE) + (DSIZE - 1)) / (DSIZE));
        }

        if ((bp = findFit(asize)) != -1) {
            place(bp, asize);
            return bp;
        }

        /*apply new block*/
        extendsize = Math.max(asize, CHUNKSIZE);
        if ((bp = extendHeap(extendsize / DSIZE)) == -1) {
            return -1;
        }

        place(bp, asize);
        return bp;
//        return ptr;
    }

    void myFree(int ptr) {
        if (ptr == -1)
            return;

        int size = GET_SIZE(HDRP(ptr));

        PUT(HDRP(ptr), PACK(size, 0));
        PUT(FTRP(ptr), PACK(size, 0));
        PUT(NEXT_LINKNODE_RP(ptr), 0);
        PUT(PREV_LINKNODE_RP(ptr), 0);
        coalesce(ptr);
    }


    int myRealloc(int ptr, int size) {

        //your code here,you can remove all the code below if you need.
        //The code below is just a very navie approach.
        //In this way, realloc is implemented directly using my_malloc and my_free.
        int newptr;
        int asize;

        if (size == 0) {
            myFree(ptr);
            return -1;
        }

        if (ptr == -1)
            return myMalloc(size);

        /*compute the total size,which contanins header + footer + payload and fit the alignment requirement*/
        if (size <= DSIZE) {
            asize = 2 * (DSIZE);
        } else {
            asize = (DSIZE) * ((size + (DSIZE) + (DSIZE - 1)) / (DSIZE));
        }

        int oldsize = GET_SIZE(HDRP(ptr));

        if (oldsize == asize) return ptr;

        if (oldsize < asize) {

            Boolean[] isNextFree = new Boolean[]{false};
            int bp = reallocCoalesce(ptr, asize, isNextFree);
            if (isNextFree[0]) { /*next block is free*/
                reallocPlace(bp, asize);
                return bp;
            } else if (!isNextFree[0] && bp != ptr) { /*previous block is free, move the point to new address,and move the payload*/
                myUtils.memcpy(bp, ptr, size);
                reallocPlace(bp, asize);
                return bp;
            } else {
                /*realloc_coalesce is fail*/
                newptr = myMalloc(size);
                myUtils.memcpy(newptr, ptr, size);
                myFree(ptr);
                return newptr;
            }
        } else {/*just change the size of ptr*/
            reallocPlace(ptr, asize);
            return ptr;
        }

    }


    void reallocPlace(int bp, int asize) {
        int csize = GET_SIZE(HDRP(bp));
        PUT(HDRP(bp), PACK(csize, 1));
        PUT(FTRP(bp), PACK(csize, 1));
    }

    int reallocCoalesce(int bp, int newSize, Boolean[] isNextFree) {
        boolean prev_alloc = GET_ALLOC(FTRP(PREV_BLKP(bp)));
        boolean next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(bp)));
        int size = GET_SIZE(HDRP(bp));
        isNextFree[0] = false;

        /*coalesce the block and change the point*/
        if (prev_alloc && next_alloc) {

        } else if (prev_alloc && !next_alloc) {
            size += GET_SIZE(HDRP(NEXT_BLKP(bp)));
            if (size >= newSize) {
                removeFreeBlock(NEXT_BLKP(bp));
                PUT(HDRP(bp), PACK(size, 1));
                PUT(FTRP(bp), PACK(size, 1));
                isNextFree[0] = true;
                return bp;
            }
        } else if (!prev_alloc && next_alloc) {
            size += GET_SIZE(HDRP(PREV_BLKP(bp)));
            if (size >= newSize) {
                removeFreeBlock(PREV_BLKP(bp));
                PUT(FTRP(bp), PACK(size, 1));
                PUT(HDRP(PREV_BLKP(bp)), PACK(size, 1));
                bp = PREV_BLKP(bp);
                return bp;
            }

        } else {
            size += GET_SIZE(FTRP(NEXT_BLKP(bp))) + GET_SIZE(HDRP(PREV_BLKP(bp)));
            if (size >= newSize) {
                removeFreeBlock(PREV_BLKP(bp));
                removeFreeBlock(NEXT_BLKP(bp));
                PUT(FTRP(NEXT_BLKP(bp)), PACK(size, 1));
                PUT(HDRP(PREV_BLKP(bp)), PACK(size, 1));
                bp = PREV_BLKP(bp);
            }

        }
        return bp;
    }

}
