import "std/stdio.h"

enum Color : const char*
	Red = "ff0000"
	Green = "00ff00"
	Blue = "0000ff"
end

func main(argc: int, argv: const cstring*)
	var color: const char* = Color.Red
	printf("Color: %s\n", color)

	color = Color.Green
	printf("Color: %s\n", color)

	color = Color.Blue
	printf("Color: %s\n", color)
end

