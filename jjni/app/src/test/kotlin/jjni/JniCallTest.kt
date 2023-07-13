import jjni.JniCall
import kotlin.test.*

class JniCallTest {
    lateinit var jniCall: JniCall

    @BeforeTest
    fun setUp() {
        jniCall = JniCall
    }
    
    @Test
    fun testInvCountFromJni() {
        jniCall.incCountFromJni()
        assertEquals(jniCall.count,1)
    }
    
    @Test
    fun testGetUserFromJni() {
        assertEquals("Alice",jniCall.user.name)
    }

    @Test
    fun testCallIncCountFromJni(){
        jniCall.callIncCountFromJni()
        assertEquals(1,jniCall.count)
    }
}