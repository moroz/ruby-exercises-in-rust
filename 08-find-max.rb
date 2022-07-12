# Task: Allow the user to input at least three numbers, and pick out the biggest one

arr = []
print "Program will be finished by hitting Enter
"
loop do
  print "Please input a number: "
  line = gets
  break if line.chomp.empty?
  arr << line.to_i
end

puts "The maximum number is #{arr.max}"
