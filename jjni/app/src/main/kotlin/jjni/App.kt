/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package jjni

object App {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }
    
    external fun helloJni()
}

fun main() {
    println(App.helloJni())
}
