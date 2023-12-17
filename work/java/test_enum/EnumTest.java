import java.util.*;
public class EnumTest{
	public static void main(String[] args)
	{
			Size size = Enum.valueOf(Size.class,"MEDIUM");	
			System.out.println("Size = "+ size);
			System.out.println("abbreviation = "+ size.getAbbreviation());
			return;
	}
	
}

enum Size{
	SMALL("S"),MEDIUM("M");
	private String abbreviation;
	private Size(String abbreviation){	this.abbreviation = abbreviation;}
	public String getAbbreviation(){
		return abbreviation;
	}
}

