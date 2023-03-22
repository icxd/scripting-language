import "std/stdio.h"

func add(a: int, b: int): int => a + b
func sub(a: int, b: int): int => a - b
func mul(a: int, b: int): int => a * b
func div(a: int, b: int): int => a / b

enum Integer : func(int, int): int
	Add = add
	Sub = sub
	Mul = mul
	Div = div
end

func my_func(fn: func(int, int): int, a: int, b: int): int => fn(a, b)

func main(argc: int, argv: const cstring*)
	var function: func(int, int): int = Integer.Add
	var result: int = function(1, 2)
	printf("result: %d\n", result)	

	result = Integer.Sub(1, 2)
	printf("result: %d\n", result)

	printf("result: %d\n", Integer.Mul(1, 2))

	printf("result: %d\n", my_func(Integer.Div, 1, 2))
end

