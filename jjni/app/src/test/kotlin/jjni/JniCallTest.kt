import jjni.JniCall
import kotlin.test.*

class JniCallTest {
    lateinit var jniCall: JniCall

    @BeforeTest
    fun setUp() {
        jniCall = JniCall
    }
    
    @Test
    fun testCallInvCountFromJni() {
        jniCall.incCountFromJni()
        assertEquals(jniCall.count,1)
    }
    
    @Test
    fun testCallGetUserFromJni() {
        assertEquals("Alice",jniCall.user.name)
    }
}