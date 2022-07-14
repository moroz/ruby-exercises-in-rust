# Task: Input a number N, output N * N multiplication table

print "Please input a number N, and hit Enter: "
n = gets.to_i

i = 1
while ( i <= n )
  p "#{ n } * #{ i } = #{ n * i }"
  i += 1
end

