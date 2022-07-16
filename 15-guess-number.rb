# Task: Number guessing game, generate a random number within 0~99 first, then the user starts guessing. The program will respond with too small, too big or correct!, the program terminates once got correct.

target = rand(100)

while (true)
  print "Please guess a number N within 0~99, and hit Enter: "
  n = gets.to_i

  if n == target
    puts "Bingo, congrates!! "
    break
  end

  if n < target
    puts "Oops, your guess is smaller than the answer." 
  else
    puts "Oops, your guess is bigger than the answer." 
  end

end
