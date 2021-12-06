days = 80
input = File.read('./input.txt').split(",").map(&:to_i)

def step(input)
  input
    .flat_map { |age| age == 0 ? [age, 9] : [age] }
    .map { |age| age == 0 ? 7 : age }
    .map { |age| age - 1 }
end

days.times do |i|
  input = step(input)
end

puts input.count
