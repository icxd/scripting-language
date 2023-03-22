import "std/stdio.h"

func main(argc: int, argv: const cstring*)
	var a: int = 69
	var b: int* = &a
	var c: int = *b

	printf("a = %d\n", a)
	printf("b = %d\n", b)
	printf("c = %d\n", c)
end

