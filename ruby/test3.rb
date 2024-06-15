#!/usr/bin/ruby -w

#指数算术
puts 2**(1/4)#1与4的商为0，然后2的0次方为1
puts 16**(1/4.0)#1与4.0的商为0.25（四分之一），然后开四次方根

puts 'escape using "\\"';
puts 'That\'s right';

puts "相乘 : #{24*60*60}";

name="Ruby"
puts name
puts "#{name+",ok"}"


ary = [ "fred", 10, 3.14, "This is a string", "last element", ]
ary.each do |i|
    puts i
end


hsh = colors = { "red" => 0xf00, "green" => 0x0f0, "blue" => 0x00f }
hsh.each do |key, value|
    print key, " is ", value, "\n"
end
