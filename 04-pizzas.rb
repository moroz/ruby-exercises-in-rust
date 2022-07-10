# Task: Input how many current pizzas and people, output how many slices per person and how many pizzas remain.

print "Please input how many pizzas, and hit Enter: "
pizzas = gets.to_i

print "Please input how many people, and hit Enter: "
people = gets.to_i

per_capita = pizzas / people
remainder = pizzas % people

puts "Every persion gets: #{per_capita}"
puts "Remaining pizzas: #{remainder}"
