虚拟机对象总有一个特定的非范型类型。因此，所有的类型查询只产生原始类型

1.
Pair<String> stringPair = ...;
Pair<Employee> emPair = ...;
if(stringPair.getClass() == emPair.getClass()) //they are equal

2.
不能实例化参数化类型的数组
Pair<String>[] table = new Pair<String>[10];//出错
因为类型擦除后，table类型是Pair[]，
令Object[] objarray = table;
objarray[0] = new Pair<Employee>(); //不会出错

如果需要收集参数化类型对象，只有一种安全而有效的办法，使用ArrayList:ArrayList<Pair<String>>


