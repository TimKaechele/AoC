horizontal_position = File.read('./input.txt').split(',').map(&:to_i)
position_distances = (horizontal_position.min..horizontal_position.max).map do |center|
  [
    center,
    horizontal_position.map do |pos|
      dist = (center - pos).abs
      # sum of all numbers between 1 and dist
      (dist / 2.0) * (1 + dist)
    end.sum
  ]
end
puts position_distances.min_by { |tuple| tuple[1] }
