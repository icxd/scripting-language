import "std/stdio.h"
import "std/string.h"

struct String
	data: cstring
	len: int

	constructor: func(cstring, int): String*
	c_str: func(String*): cstring
	length: func(String*): int
end

func String.c_str(self: String*): cstring => self.data
func String.length(self: String*): int => self.len

func main(argc: int, argv: const cstring*): int
	var str: String* = new String("Hello, world!", strlen("Hello, world!"))
	printf("%s\n", str.c_str(str))
	return 0
end

