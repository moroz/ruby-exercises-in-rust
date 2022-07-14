# Task: Input a number N, please check N if is a prime number

def is_prime(n)
# ....
end

print "Please input a number N, and hit Enter: "
n = gets.to_i

if is_prime(n.to_i)
  puts "Hooray, this is a prime number."
else
  puts "Oops, this is not a prime number."
end
