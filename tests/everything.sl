import "std/stdio.h"
import "std/stdlib.h"

struct Point
	x: int
	y: int
end

enum Color : cstring
	Red = "#ff0000"
	Green = "#00ff00"
	Blue = "#0000ff"
end

type MyType = Color
type MyType2 = Point

type MyUnionType = Color | Point

struct MyStruct
	x: int

	constructor: func(int): MyStruct*
	my_func: func(MyStruct*, int)
end

func MyStruct.my_func(self: MyStruct*, x: int)
	printf("MyStruct.my_func(%d)\n", x)
end

enum MyEnum : MyStruct*
	One = MyStruct(x: 1)
	Two = MyStruct(x: 2)
end

func main(argc: int, argv: cstring): int
	var x: int = 5
	var y: Point* = Point(x: 5, y: 10)
	var z: cstring = Color.Red

	var my_array: int[5] = [1, 2, 3, 4, 5]

	var myStruct: MyStruct* = new MyStruct(5) // malloc(sizeof MyStruct) as MyStruct*

	var one: MyStruct* = MyEnum.One

	printf("x = %d\n", x)
	printf("y = %d, %d\n", y.x, y.y)
	printf("z = %s\n", z as cstring)

	printf("my_array = %d, %d, %d, %d, %d\n", my_array[0], my_array[1], my_array[2], my_array[3], my_array[4])

	myStruct.my_func(myStruct, 5)

	printf("one = %d\n", one.x)

	return 0
end
