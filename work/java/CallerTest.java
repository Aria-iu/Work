public class CallerTest{
	public static native void echo(int a);
	public static void main(String[] args) {
		System.loadLibrary("Test");
		Test.echo(99);
	}
}
