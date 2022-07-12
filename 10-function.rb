# Task: User able to input x, Y, Z numbers in turn, and pick out the biggest one

def find_max(x, y, z)
  arr = [x, y, z].map { |i| i.to_i }
  arr.max
end

print "Please input a number x, and hit Enter: "
x = gets

print "Please input a number y, and hit Enter: "
y = gets

print "Please input a number z, and hit Enter: "
z = gets

answer = find_max(x,y,z)

puts "The maximum number is #{answer}"
