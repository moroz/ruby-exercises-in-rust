# Task: Input a number N, please check N if is a prime number

def is_prime(n)
  (2..(n - 1)).each do |i|
    return if n % i == 0
  end
  true
end

print "Please input a number N, and hit Enter: "
n = gets

if is_prime(n.to_i)
  puts "Hooray, #{n} is a prime number."
else
  puts "Oops, #{n} is not a prime number."
end
