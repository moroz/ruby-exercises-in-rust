# get the maximum number from an array

# def find_max(array)
#   array.sort.last
# end

def find_max(array)
  n = array[0]
  array.each do |elem|
    if elem > n
      n = elem
    end
  end
  n
end

arr = [8, 12, 36, 53, 9, 75, 3, 71, 59, 88]

max = find_max(arr)
puts "Max is #{max}" # should be 88

