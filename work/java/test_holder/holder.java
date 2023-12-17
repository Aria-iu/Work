import org.omg.CORBA.IntHolder;
public class holder{
	public static void main(String args[])
	{
			int t = 3;
			triple(t);
			System.out.println(t);
			return;
	}
	private static void triple(IntHolder x)
	{
		x.value = 3 * x.value;
		return;
	}
}
