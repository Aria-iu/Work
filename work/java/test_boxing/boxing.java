package test_boxing;
public class boxing{
	public static void main(String args[])
	{
//		自动装箱要求boolean、byte、char<=127、介于-128～127的short和int被包装到固定的对象中，
//		若int = 10,则会包装到同一个Integer中，所以两个数据的地址一致，而1000时不一致。
//		Integer a = 10;
//		Integer b = 10;
		Integer a = 1000;
		Integer b = 1000;
		if(a == b)
		{
			System.out.println("a == b\n");
		}else 
		{
			System.out.println("a != b\n");
		}
	}
}
