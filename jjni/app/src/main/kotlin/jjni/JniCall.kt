package jjni

object JniCall {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }

    lateinit var user: User

    @JvmStatic
    fun createUser(name: String) = User(name)

    external fun setUserFromJni(): Boolean
}


data class User(var name: String)
