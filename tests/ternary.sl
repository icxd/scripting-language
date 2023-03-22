import "std/stdio.h"
import "std/stdbool.h"

func main(argc: int, argv: const cstring*): int
	var a: int = 0
	var b: const bool = true if a == 0 else false

	printf("b = %d\n", b)

	return 0
end

