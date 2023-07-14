package jjni

object JniCall {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }

    var count = 0
    val user: User
        get() = getUserFromJni()

    external fun incCountFromJni()
    external fun callIncCountFromJni()
    private external fun getUserFromJni(): User

    fun incCount() {
        incCountFromJni()
    }

    @JvmStatic
    fun sayHi() {
        println("Hi Jni!!!!")
    }

    @JvmStatic
    external fun callSayHiFromJni()

}


data class User(var name: String)
