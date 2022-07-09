# Task: exchange the value of varialbe a and b

a = 1
b = 2

puts "a is #{a}"
puts "b is #{b}"

a, b = b, a

puts "a should be 1, now it is #{a}"
puts "b should be 2, now it is #{b}"

