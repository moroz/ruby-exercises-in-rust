# Task: User able to get a right triangle's area result by inputting the width  and height value 

print "Please input the height of the right triangle, and hit Enter: "
a = gets.to_i

print "Please input the width of the right triangle, and hit Enter: "
b = gets.to_i

a *= a
b *= b

result = Math.sqrt(a + b)

puts "The area result is: #{result}"
