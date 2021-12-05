defmodule Day5 do
  use Application

  def start(_type, _args) do
    IO.puts("Hello World")
    # We will not start a real application, so we are going to return
    # a fake pid here.

    File.read!("./input.txt")
      |> String.trim
      |> String.split("\n")
      |> Enum.map(fn line -> line |> String.split(" -> ") end)
      |> Enum.map(fn [left, right] ->
        %{
          start: build_position(left),
          stop: build_position(right),
        }
      end)
      |> Enum.flat_map(fn equation ->
        build_line_points(equation)
      end)
      |> Enum.reduce(%{}, fn point, acc ->
        val = Map.get(acc, point, 0)
        Map.put(acc, point, val + 1)
      end)
      |> Enum.filter(fn {_, count} ->
        count > 1
      end)
      |> Enum.count()
      |> IO.puts

    {:ok, :c.pid(0, 255, 0)}
  end

  def build_line_points(equation) do
    distances =
      {
        equation[:stop][:x] - equation[:start][:x],
        equation[:stop][:y] - equation[:start][:y]
      }

    case distances do
      {0, y_distance} ->
        x_zero_equation(y_distance, equation)
      {x_distance, 0} ->
        y_zero_equation(x_distance, equation)
      {x_dist, y_dist} ->
        default_equation(x_dist, y_dist, equation)
    end
  end

  def x_zero_equation(distance, equation) do
    Enum.to_list(0..distance)
    |> Enum.map(fn val ->
      %{
        x: equation[:start][:x],
        y: equation[:start][:y] + val
      }
    end)
  end

  def y_zero_equation(distance, equation) do
    Enum.to_list(0..distance)
    |> Enum.map(fn val ->
      %{
        x: equation[:start][:x] + val,
        y: equation[:start][:y]
      }
    end)
  end

  def default_equation(x_dist, y_dist, equation) do
    x_points = Enum.to_list(0..x_dist)
    y_points = Enum.to_list(0..y_dist)

    Enum.zip_with([x_points, y_points], fn [x,y] ->
      %{x: x, y: y}
    end)
    |> Enum.map(fn vec ->
      %{
        x: equation[:start][:x] + vec[:x],
        y: equation[:start][:y] + vec[:y],
      }
    end)
  end

  def build_position(raw_string) do
    [x, y] = String.split(raw_string, ",")

    {x_parsed, _} = Integer.parse(x)
    {y_parsed, _} = Integer.parse(y)

    %{
      x: x_parsed,
      y: y_parsed
    }
  end
end
