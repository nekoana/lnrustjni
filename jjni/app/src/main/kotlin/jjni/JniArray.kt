package jjni

object JniArray {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }

    external fun doubleIntArray(array: IntArray): IntArray
}

