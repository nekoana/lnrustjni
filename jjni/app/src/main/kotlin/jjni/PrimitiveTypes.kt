package jjni

object PrimitiveTypes {
    init {
        System.load("/home/codin/java/lnjni/rjni/target/debug/librjni.so")
    }
    external fun getBoolean():Boolean
    external fun getByte():Byte
    external fun getShort():Short
    external fun getChar():Char
    external fun getInt():Int
    external fun getLong():Long
    external fun getFloat():Float
    external fun getDouble():Double
}