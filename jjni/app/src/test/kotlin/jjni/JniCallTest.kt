import jjni.JniCall
import jjni.User
import kotlin.test.*

class JniCallTest {
    lateinit var jniCall: JniCall

    @BeforeTest
    fun setUp() {
        jniCall = JniCall
    }

    @Test
    fun testSetUserFromJni() {
        val set = jniCall.setUserFromJni()
        assert(set)

        val user = jniCall.user
        assertEquals(user.name,"Alice From JNI !!")
    }

}
