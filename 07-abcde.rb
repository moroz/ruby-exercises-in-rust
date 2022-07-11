# Task: Input x,y,z and output a speicifc result
# when x < 0, output "A"
# when x > 0, and
#   when y > 0, and
#     when z > 0 output "B"
#     when z < 0 output "C"
#   when y < 0
#     when z > 0 output "D"
#     when z < 0 output "E"

print "Please input an integar X, then hit Enter: "
x = gets.to_i

print "Please input an integar Y, then hit Enter: "
y = gets.to_i

print "Please input an integar Z, then hit Enter: "
z = gets.to_i

def abcde(x, y, z)
  if z < 0
    y < 0 ? "E" : "C" 
  elsif z > 0
    y > 0 ? "B" : "D"
  else
    "A"
  end
end
  
puts "The result is #{abcde(x, y, z)}"
