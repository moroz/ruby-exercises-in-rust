# Identify a number whether is zero, positive or negative number, and even or  odd.

print "Please input an integer, and hit Enter:"
x = gets.to_i

def zero?(num)
  if num < 0
    "negative"
  elsif num > 0
    "positive"
  else
    "zero"
  end
end

def even?(num)
 num % 2 == 0 ? "even" : "odd"
end

puts "This is a '#{zero?(x)}' and '#{even?(x)}' number"
