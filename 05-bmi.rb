# Task: BMI calculater in metric
print "Please input your weight(kilograms), and hit Enter: "
weight = gets.to_i

print "Please input your height(centimeters), and hit Enter: "
height = gets.to_i

def bmi_calulator(weight, height)
  height = (height / 100.00) **2
  (weight / height).round(1)
end

def bmi_category(index)
  case index
  when 0..18.5
    "Underweight"
  when (18.6..24.9)
    "Normal weight"
  when (25.0..29.9)
    "Overweight"
  else
    "Obesity"
  end
end

bmi = bmi_calulator(weight, height)

puts "Your BMI index is: #{bmi}, #{bmi_category(bmi)}."
