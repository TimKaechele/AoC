days = 256
input = Hash[File.read('./input.txt').split(",").map(&:to_i).group_by { |age| age }.map { |age,numbers| [age, numbers.length] }]
input.default = 0

def step(input)
  input[7] += input[0]
  input[9] += input[0]
  input[0] = 0
  input.transform_keys! { |age| age - 1 }
end

days.times do
  input = step(input)
end

puts input.values.sum
