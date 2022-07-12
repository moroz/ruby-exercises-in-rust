# Task: User able to get a right triangle's area result by inputting the width  and height value 

def calculate_area(a, b)
  a *= a
  b *= b
  Math.sqrt(a + b).round
end

print "Please input the height of the right triangle, and hit Enter: "
a = gets.to_i

print "Please input the width of the right triangle, and hit Enter: "
b = gets.to_i

answer = calculate_area(a,b)

puts "The area result is: #{answer}"
