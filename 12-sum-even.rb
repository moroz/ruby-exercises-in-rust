# Task: Sum up all even numbers within 1 to 100.

i = 1
total = 0

while ( i <= 100 )

  if i % 2 == 0
    total += i
  end

  i+=1
end

puts total
