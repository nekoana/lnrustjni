package jjni

object JniArray {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }

    external fun doubleIntArray(array: IntArray): IntArray

    external fun mergeStringArray(arr1: Array<String>, arr2: Array<String>): Array<String>
}

