package jjni

import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals


class JniArrayTest {

    @BeforeTest
    fun setUp() {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }


    @Test
    fun testDoubleIntArray() {
        val array = intArrayOf(1, 2, 4)
        val newArray = JniArray.doubleIntArray(array)

        assertContentEquals(newArray, intArrayOf(2, 4, 8))
    }

    @Test
    fun testMergeStringArray() {
        val arr1 = arrayOf("A","B","C")
        val arr2 = arrayOf("a","b","c")

        val arr = JniArray.mergeStringArray(arr1,arr2)
        println(arr.contentToString())
    }
}

